use super::{Action, App, ColorChannel, Tab};
use crate::model::{Direction, Effect, Rgb};

impl App {
    pub fn select_tab(&mut self, tab: Tab) {
        self.tab = tab;
        self.selected = 0;
    }
    pub fn select_zone(&mut self, zone: usize) {
        self.zone = zone;
        self.select_tab(Tab::Color);
        self.status = format!(
            "Zone {zone} selected. {}",
            if self.settings.effect == Effect::Static {
                "Color edits now affect this zone."
            } else {
                "Choose Static in Effect for per-zone colors; animations use shared colors."
            }
        );
    }
    pub fn color(&self) -> Rgb {
        if self.settings.effect == Effect::Static {
            self.settings.zones[self.zone.saturating_sub(1)]
        } else {
            self.settings.color
        }
    }
    pub fn set_color(&mut self, c: Rgb) {
        if !self.settings.effect.uses_color() {
            self.status =
                "Neon and Wave generate colors automatically. Choose another effect to edit RGB."
                    .into();
            return;
        }
        if self.settings.effect == Effect::Static {
            if self.zone == 0 {
                self.settings.zones.fill(c);
            } else {
                self.settings.zones[self.zone - 1] = c;
            }
        } else {
            self.settings.color = c;
        }
    }
    pub fn move_selection(&mut self, delta: i16) {
        match self.tab {
            Tab::Color => self.selected = (self.selected as i16 + delta).rem_euclid(5) as usize,
            _ => self.adjust(delta),
        }
    }
    pub fn adjust(&mut self, delta: i16) {
        let clamp = |v: u8, max: i16| (i16::from(v) + delta).clamp(0, max) as u8;
        match self.tab {
            Tab::Color => match self.selected {
                0 => {
                    if self.settings.effect == Effect::Static {
                        self.zone = (self.zone as i16 + delta.signum()).rem_euclid(5) as usize;
                    }
                }
                1..=3 => {
                    let Rgb(r, g, b) = self.color();
                    let mut c = [r, g, b];
                    let i = self.selected - 1;
                    c[i] = clamp(c[i], 255);
                    self.set_color(Rgb(c[0], c[1], c[2]));
                }
                4 => self.settings.brightness = clamp(self.settings.brightness, 100),
                _ => {}
            },
            Tab::Speed => {
                if self.settings.effect != Effect::Static {
                    self.settings.speed = clamp(self.settings.speed, 9);
                } else {
                    self.status =
                        "Static has no animation speed. Select an animation in the Effect tab."
                            .into();
                }
            }
            Tab::Direction => {
                if self.settings.effect != Effect::Static {
                    self.settings.direction = if delta > 0 {
                        Direction::LeftToRight
                    } else {
                        Direction::RightToLeft
                    };
                } else {
                    self.status =
                        "Static has no animation direction. Select an animation in the Effect tab."
                            .into();
                }
            }
            Tab::Effect => {
                self.settings.effect = Effect::ALL
                    [(self.settings.effect.id() as i16 + delta.signum()).rem_euclid(6) as usize];
            }
        }
    }

    pub(crate) fn set_channel(&mut self, channel: ColorChannel, value: u8) {
        let Rgb(r, g, b) = self.color();
        let mut channels = [r, g, b];
        channels[channel.index()] = value;
        self.set_color(Rgb(channels[0], channels[1], channels[2]));
    }

    pub(crate) fn handle_action(&mut self, action: Action) {
        let before = self.settings.clone();
        match action {
            Action::SelectTab(tab) => self.select_tab(tab),
            Action::SelectZone(zone) => {
                self.zone = zone;
                self.select_tab(Tab::Color);
            }
            Action::SetColor(color) => self.set_color(color),
            Action::SetChannel(channel, value) => {
                self.set_channel(channel, value);
                self.selected = channel.index() + 1;
            }
            Action::SetBrightness(value) => {
                self.settings.brightness = value.min(100);
                self.selected = 4;
            }
            Action::SetSpeed(value) => {
                if self.settings.effect != Effect::Static {
                    self.settings.speed = value.min(9);
                }
            }
            Action::SetDirection(direction) => {
                if self.settings.effect != Effect::Static {
                    self.settings.direction = direction;
                }
            }
            Action::SetEffect(effect) => self.settings.effect = effect,
        }
        self.changed(&before);
    }
}
