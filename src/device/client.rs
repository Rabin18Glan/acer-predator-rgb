use super::{DEVICE, Error, Status, node, status};
use crate::model::Settings;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};
pub trait Transport {
    fn command(&mut self, command: &str) -> Result<(), Error>;
    fn status(&mut self) -> Result<Status, Error>;
}
pub struct Device;
impl Device {
    pub(crate) fn apply(&mut self, settings: &Settings) -> Result<Status, Error> {
        apply(settings, self)
    }
}
impl Transport for Device {
    fn command(&mut self, command: &str) -> Result<(), Error> {
        node::check()?;
        let mut file = OpenOptions::new().write(true).open(DEVICE).map_err(|e| {
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

fn apply(settings: &Settings, transport: &mut impl Transport) -> Result<Status, Error> {
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
#[path = "../../tests/unit/device.rs"]
mod tests;
