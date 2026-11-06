use super::{Direction, Effect, Rgb};
use serde::{Deserialize, Serialize};
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
