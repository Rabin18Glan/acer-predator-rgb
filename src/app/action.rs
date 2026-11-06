use crate::model::{Direction, Effect, Rgb};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Color,
    Effect,
    Speed,
    Direction,
}
impl Tab {
    pub const ALL: [Self; 4] = [Self::Color, Self::Effect, Self::Speed, Self::Direction];
    pub fn index(self) -> usize {
        match self {
            Self::Color => 0,
            Self::Effect => 1,
            Self::Speed => 2,
            Self::Direction => 3,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ColorChannel {
    Red,
    Green,
    Blue,
}
impl ColorChannel {
    pub(crate) const ALL: [Self; 3] = [Self::Red, Self::Green, Self::Blue];
    pub(crate) fn index(self) -> usize {
        match self {
            Self::Red => 0,
            Self::Green => 1,
            Self::Blue => 2,
        }
    }
}

/// Input intent, independent of terminal coordinates or device I/O.
pub(crate) enum Action {
    SelectTab(Tab),
    SelectZone(usize),
    SetColor(Rgb),
    SetChannel(ColorChannel, u8),
    SetBrightness(u8),
    SetSpeed(u8),
    SetDirection(Direction),
    SetEffect(Effect),
}
