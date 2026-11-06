use super::{App, Tab};
use crate::{config, device, model::Settings};

impl App {
    pub(crate) fn from_settings(settings: Settings, status: String) -> Self {
        Self {
            settings,
            tab: Tab::Color,
            selected: 0,
            zone: 0,
            status,
            connection: String::new(),
            error: false,
            hex_input: None,
            pending: None,
        }
    }
    pub fn new() -> Self {
        let mut app = match config::load() {
            Ok((s, msg)) => Self::from_settings(s, msg),
            Err(e) => {
                let mut a = Self::from_settings(Settings::default(), e);
                a.error = true;
                a
            }
        };
        app.refresh();
        app
    }
    pub fn refresh(&mut self) {
        match device::status() {
            Ok(s) => {
                self.connection = s.describe();
                self.error = false;
            }
            Err(e) => {
                self.connection = "Driver unavailable".into();
                self.status = e.to_string();
                self.error = true;
            }
        }
    }
    pub fn save(&mut self) {
        match config::save(&self.settings) {
            Ok(p) => {
                self.status = format!("Saved {}", p.display());
                self.error = false;
            }
            Err(e) => {
                self.status = e;
                self.error = true;
            }
        }
    }
    pub fn load(&mut self) {
        match config::load() {
            Ok((s, msg)) => {
                self.settings = s;
                self.status = msg;
                self.error = false;
            }
            Err(e) => {
                self.status = e;
                self.error = true;
            }
        }
    }
}
