//! Lighting settings and driver-independent command encoding.
mod color;
mod direction;
mod effect;
mod settings;

pub(crate) use color::Rgb;
pub(crate) use direction::Direction;
pub(crate) use effect::Effect;
pub(crate) use settings::Settings;

#[cfg(test)]
#[path = "../../tests/unit/model.rs"]
mod tests;
