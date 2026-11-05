use crate::model::Settings;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    os::unix::fs::FileTypeExt,
    path::Path,
};
use thiserror::Error;

const DEVICE: &str = "/dev/predator-kbd";

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Invalid(String),
    #[error("{0}: {1}")]
    Io(&'static str, io::Error),
}

#[derive(Debug, Clone)]
pub struct Status {
    pub method: i32,
    pub code: i32,
    pub restored: u32,
}
impl Status {
    pub fn parse(text: &str) -> Result<Self, Error> {
        let get = |key: &str| {
            text.split_whitespace()
                .find_map(|field| field.strip_prefix(key))
        };
        Ok(Self {
            method: get("last_method=")
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| Error::Invalid("Malformed driver status".into()))?,
            code: get("last_status=")
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| Error::Invalid("Missing firmware status".into()))?,
            restored: get("restored_after_sleep=")
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| Error::Invalid("Missing resume count".into()))?,
        })
    }
    pub fn describe(&self) -> String {
        let result = match self.code {
            -1 => "no commands sent".into(),
            0 => "last request accepted".into(),
            n => format!("firmware status {n}"),
        };
        format!(
            "Connected · {result} · method {} · {} resume restores",
            self.method, self.restored
        )
    }
}

fn check() -> Result<(), Error> {
    let metadata = fs::symlink_metadata(DEVICE).map_err(|e| {
        Error::Io(
            "Cannot access /dev/predator-kbd (requires the working predator_kbd_rgb driver)",
            e,
        )
    })?;
    if !metadata.file_type().is_char_device() {
        return Err(Error::Invalid(
            "/dev/predator-kbd is not a character device".into(),
        ));
    }
    let taint = fs::read_to_string("/proc/sys/kernel/tainted")
        .map_err(|e| Error::Io("Cannot inspect kernel state", e))?;
    let taint: u64 = taint
        .trim()
        .parse()
        .map_err(|_| Error::Invalid("Invalid kernel taint value".into()))?;
    if taint & 128 != 0 {
        return Err(Error::Invalid(
            "Kernel recorded an Oops; restart before accessing keyboard devices".into(),
        ));
    }
    Ok(())
}

pub fn status() -> Result<Status, Error> {
    check()?;
    let mut text = String::new();
    File::open(DEVICE)
        .map_err(|e| Error::Io("Cannot read driver status", e))?
        .take(256)
        .read_to_string(&mut text)
        .map_err(|e| Error::Io("Cannot read driver status", e))?;
    Status::parse(&text)
}

pub trait Transport {
    fn command(&mut self, command: &str) -> Result<(), Error>;
    fn status(&mut self) -> Result<Status, Error>;
}
pub struct Device;
impl Transport for Device {
    fn command(&mut self, command: &str) -> Result<(), Error> {
        check()?;
        let mut file = OpenOptions::new()
            .write(true)
            .open(Path::new(DEVICE))
            .map_err(|e| {
                Error::Io(
                    "Cannot open keyboard for writing; check device permissions",
                    e,
                )
            })?;
        write_record(&mut file, command.as_bytes())
            .map_err(|e| Error::Io("Keyboard write failed", e))
    }
    fn status(&mut self) -> Result<Status, Error> {
        status()
    }
}

// One write syscall per command. A short write must not be retried as a new command.
fn write_record(writer: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
    if writer.write(bytes)? != bytes.len() {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "incomplete command",
        ));
    }
    Ok(())
}

pub fn apply(settings: &Settings, transport: &mut impl Transport) -> Result<Status, Error> {
    let commands = settings.commands().map_err(Error::Invalid)?;
    let mut last = None;
    for command in commands {
        transport.command(&command)?;
        let status = transport.status()?;
        let method = if command.starts_with("zone ") { 6 } else { 20 };
        if status.code != 0 || status.method != method {
            return Err(Error::Invalid(format!(
                "Request '{command}' not confirmed: method={}, firmware status={}. Some settings may already have changed.",
                status.method, status.code
            )));
        }
        last = Some(status);
    }
    last.ok_or_else(|| Error::Invalid("No commands generated".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        commands: Vec<String>,
        fail_at: usize,
    }
    impl Transport for Mock {
        fn command(&mut self, c: &str) -> Result<(), Error> {
            self.commands.push(c.into());
            Ok(())
        }
        fn status(&mut self) -> Result<Status, Error> {
            Ok(Status {
                method: if self.commands.len() == 1 { 20 } else { 6 },
                code: i32::from(self.commands.len() == self.fail_at),
                restored: 0,
            })
        }
    }
    #[test]
    fn stops_on_firmware_rejection_instead_of_claiming_success() {
        let mut mock = Mock {
            commands: vec![],
            fail_at: 2,
        };
        assert!(apply(&Settings::default(), &mut mock).is_err());
        assert_eq!(mock.commands.len(), 2);
    }
    #[test]
    fn sends_all_zones_on_success() {
        let mut mock = Mock {
            commands: vec![],
            fail_at: 99,
        };
        assert!(apply(&Settings::default(), &mut mock).is_ok());
        assert_eq!(mock.commands.len(), 5);
    }
    #[test]
    fn distinguishes_initial_status_from_rejection() {
        let s = Status::parse("last_method=0 last_status=-1 restored_after_sleep=0\n").unwrap();
        assert!(s.describe().contains("no commands"));
        assert!(Status::parse("garbage").is_err());
    }
    #[test]
    fn short_write_is_not_retried() {
        struct Short(usize);
        impl Write for Short {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                self.0 += 1;
                Ok(1)
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut writer = Short(0);
        assert!(write_record(&mut writer, b"effect 0 0 100 0 0 0 0").is_err());
        assert_eq!(writer.0, 1);
    }
}
