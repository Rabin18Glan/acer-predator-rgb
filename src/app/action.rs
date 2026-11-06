use crate::model::{Direction, Effect, Rgb};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Color,
    Speed,
    Direction,
    Effect,
}
impl Tab {
    pub const ALL: [Self; 4] = [Self::Color, Self::Speed, Self::Direction, Self::Effect];
    pub fn index(self) -> usize {
        match self {
            Self::Color => 0,
            Self::Speed => 1,
            Self::Direction => 2,
            Self::Effect => 3,
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
