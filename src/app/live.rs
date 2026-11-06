use super::App;
use crate::{device, model::Settings};
use std::time::{Duration, Instant};

impl App {
    /// Called only for an actual settings change, never navigation or startup.
    pub fn changed(&mut self, before: &Settings) {
        if self.settings != *before {
            self.pending = Some(Instant::now());
            self.status = "Sending live change…".into();
            self.error = false;
        }
    }
    pub fn tick(&mut self) {
        // Coalesce fast repeats for 40ms; no Enter or confirmation needed.
        if self
            .pending
            .is_some_and(|t| t.elapsed() >= Duration::from_millis(40))
        {
            self.apply();
        }
    }
    pub fn apply(&mut self) {
        self.pending = None;
        match device::Device.apply(&self.settings) {
            Ok(s) => {
                self.connection = s.describe();
                self.status = "Live change applied · firmware status 0".into();
                self.error = false;
            }
            Err(e) => {
                self.status = format!("Live change failed: {e}. Press r to check connection.");
                self.error = true;
            }
        }
    }

    pub(crate) fn is_pending(&self) -> bool {
        self.pending.is_some()
    }
}
