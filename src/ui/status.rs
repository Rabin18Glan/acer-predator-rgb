use super::theme::MUTED;
use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};
pub(super) struct StatusBar;

impl StatusBar {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App) {
        let (message, color) = if app.error {
            (app.status.as_str(), Color::Rgb(255, 129, 135))
        } else if app.is_pending() {
            ("Updating…", MUTED)
        } else {
            (app.status.as_str(), MUTED)
        };
        f.render_widget(
            Paragraph::new(message)
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(color)),
            area,
        );
    }
}
