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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub effect: Effect,
    pub zones: [Rgb; 4],
    pub color: Rgb,
    pub brightness: u8,
    pub speed: u8,
    pub direction: Direction,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            effect: Effect::Static,
            zones: [Rgb(0, 80, 255); 4],
            color: Rgb(0, 80, 255),
            brightness: 100,
            speed: 5,
            direction: Direction::LeftToRight,
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if self.brightness > 100 {
            return Err("brightness must be 0–100".into());
        }
        if self.speed > 9 {
            return Err("speed must be 0–9".into());
        }
        Ok(())
    }
    /// Same ordering as the working predator-rgb Python client:
    /// static mode first, then each zone as a separate command.
    pub fn commands(&self) -> Result<Vec<String>, String> {
        self.validate()?;
        if self.effect == Effect::Static {
            let mut commands = vec![format!("effect 0 0 {} 0 0 0 0", self.brightness)];
            for (i, Rgb(r, g, b)) in self.zones.iter().enumerate() {
                commands.push(format!("zone {} {r} {g} {b}", i + 1));
            }
            Ok(commands)
        } else {
            let Rgb(r, g, b) = self.color;
            Ok(vec![format!(
                "effect {} {} {} {} {r} {g} {b}",
                self.effect.id(),
                self.speed,
                self.brightness,
                self.direction.id()
            )])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn static_sequence_matches_working_python_client() {
        let s = Settings {
            zones: [
                Rgb(255, 0, 0),
                Rgb(0, 255, 0),
                Rgb(0, 0, 255),
                Rgb(255, 255, 255),
            ],
            ..Settings::default()
        };
        assert_eq!(
            s.commands().unwrap(),
            [
                "effect 0 0 100 0 0 0 0",
                "zone 1 255 0 0",
                "zone 2 0 255 0",
                "zone 3 0 0 255",
                "zone 4 255 255 255"
            ]
        );
    }
    #[test]
    fn all_animation_ids_and_directions_match_python() {
        for (i, effect) in Effect::ALL.into_iter().enumerate().skip(1) {
            for direction in [Direction::LeftToRight, Direction::RightToLeft] {
                let s = Settings {
                    effect,
                    direction,
                    color: Rgb(1, 2, 3),
                    ..Settings::default()
                };
                assert_eq!(
                    s.commands().unwrap(),
                    [format!("effect {i} 5 100 {} 1 2 3", direction.id())]
                );
            }
        }
    }
    #[test]
    fn invalid_profiles_cannot_produce_commands() {
        assert!(
            Settings {
                brightness: 101,
                ..Settings::default()
            }
            .commands()
            .is_err()
        );
        assert!(
            Settings {
                speed: 10,
                ..Settings::default()
            }
            .commands()
            .is_err()
        );
        assert!(Rgb::try_from("#éffaa".to_owned()).is_err());
    }
    #[test]
    fn imports_python_settings_without_importing_autostart() {
        let s: Settings = serde_json::from_str(r##"{"effect":"wave","zones":["#0050ff","#0050ff","#0050ff","#0050ff"],"color":"#0050ff","brightness":100,"speed":5,"direction":"left-to-right","restore_at_login":true}"##).unwrap();
        assert_eq!(s.effect, Effect::Wave);
        assert_eq!(
            serde_json::from_str::<Settings>(&serde_json::to_string(&s).unwrap()).unwrap(),
            s
        );
    }
}
