use super::AppUi;
use crate::{
    app::{Action, ColorChannel, Tab},
    model::{Direction, Effect, Rgb},
};

/// Rendered hit regions are separate from lighting state.
#[derive(Clone, Copy)]
pub(super) enum HitTarget {
    Tab(Tab),
    Zone(usize),
    Preset(Rgb),
    Slider(usize),
    Effect(Effect),
    Direction(Direction),
}

impl AppUi {
    pub(crate) fn action_at(&self, x: u16, y: u16) -> Option<Action> {
        let (rect, target) = self
            .hits
            .iter()
            .rev()
            .find(|(rect, _)| rect.contains((x, y).into()))?;
        Some(match *target {
            HitTarget::Tab(tab) => Action::SelectTab(tab),
            HitTarget::Zone(zone) => Action::SelectZone(zone),
            HitTarget::Preset(color) => Action::SetColor(color),
            HitTarget::Effect(effect) => Action::SetEffect(effect),
            HitTarget::Direction(direction) => Action::SetDirection(direction),
            HitTarget::Slider(field) => {
                let max = match field {
                    0 => 9,
                    4 => 100,
                    _ => 255,
                };
                let value =
                    ((x - rect.x) as u32 * max / rect.width.saturating_sub(1).max(1) as u32) as u8;
                match field {
                    0 => Action::SetSpeed(value),
                    4 => Action::SetBrightness(value),
                    1..=3 => Action::SetChannel(ColorChannel::ALL[field - 1], value),
                    _ => return None,
                }
            }
        })
    }
}
