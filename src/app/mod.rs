//! Application state and behavior. UI rendering and terminal setup live elsewhere.
mod action;
mod editing;
mod input;
mod live;
mod shortcuts;
mod state;

use crate::model::Settings;
pub(crate) use action::{Action, ColorChannel, Tab};
pub(crate) use input::InputOutcome;
use std::time::Instant;

pub(crate) struct App {
    pub(crate) settings: Settings,
    pub(crate) tab: Tab,
    pub(crate) selected: usize,
    /// Zero selects all static zones; 1–4 selects one.
    pub(crate) zone: usize,
    pub(crate) status: String,
    pub(crate) connection: String,
    pub(crate) error: bool,
    pub(crate) hex_input: Option<String>,
    pending: Option<Instant>,
}
