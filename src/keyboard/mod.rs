//! Keyboard geometry and visual effect simulation; never touches hardware.
mod animation;
mod layout;

pub(crate) use animation::color;
pub(crate) use layout::keys;

#[cfg(test)]
#[path = "../../tests/unit/keyboard.rs"]
mod tests;
