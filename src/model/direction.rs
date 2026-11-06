use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    RightToLeft,
    LeftToRight,
}
impl Direction {
    pub fn id(self) -> u8 {
        match self {
            Self::RightToLeft => 1,
            Self::LeftToRight => 2,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::RightToLeft => "Right to left",
            Self::LeftToRight => "Left to right",
        }
    }
}
