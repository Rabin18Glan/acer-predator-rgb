//! UI components, assembled by AppUi.
mod app_ui;
mod color;
mod direction;
mod effect;
mod footer;
mod header;
mod hit;
mod layout;
mod palette;
mod preview;
mod resize_notice;
mod speed;
mod status;
mod tabs;
mod theme;

pub(crate) use app_ui::AppUi;

#[cfg(test)]
#[path = "../../tests/unit/ui.rs"]
mod tests;
