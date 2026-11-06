use super::{DEVICE, Error, node};
use std::{fs::File, io::Read};
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

pub(crate) fn read() -> Result<Status, Error> {
    node::check()?;
    let mut text = String::new();
    File::open(DEVICE)
        .map_err(|e| Error::Io("Cannot read driver status", e))?
        .take(256)
        .read_to_string(&mut text)
        .map_err(|e| Error::Io("Cannot read driver status", e))?;
    Status::parse(&text)
}
