use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    #[default]
    Static,
    Breathing,
    Neon,
    Wave,
    Shifting,
    Zoom,
}
impl Effect {
    pub const ALL: [Self; 6] = [
        Self::Static,
        Self::Breathing,
        Self::Neon,
        Self::Wave,
        Self::Shifting,
        Self::Zoom,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Static => "Static",
            Self::Breathing => "Breathing",
            Self::Neon => "Neon",
            Self::Wave => "Wave",
            Self::Shifting => "Shifting",
            Self::Zoom => "Zoom",
        }
    }
    pub fn id(self) -> usize {
        Self::ALL.iter().position(|e| *e == self).unwrap_or(0)
    }
    pub fn uses_color(self) -> bool {
        !matches!(self, Self::Neon | Self::Wave)
    }
}
