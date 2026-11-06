use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Rgb(pub u8, pub u8, pub u8);

impl TryFrom<String> for Rgb {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let hex = s.strip_prefix('#').unwrap_or(&s);
        if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("color must be six hex digits, for example #0050ff".into());
        }
        let n = u32::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        Ok(Self((n >> 16) as u8, (n >> 8) as u8, n as u8))
    }
}
impl From<Rgb> for String {
    fn from(c: Rgb) -> Self {
        format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
    }
}
