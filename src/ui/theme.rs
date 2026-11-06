use crate::model::Rgb;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType},
};
pub(super) const BG: Color = Color::Rgb(12, 17, 27);
pub(super) const MUTED: Color = Color::Rgb(145, 160, 180);
pub(super) const ACCENT: Color = Color::Rgb(67, 221, 202);
pub(super) const PRESETS: [(&str, Rgb); 7] = [
    ("Red", Rgb(255, 0, 0)),
    ("Green", Rgb(0, 255, 0)),
    ("Blue", Rgb(0, 80, 255)),
    ("Purple", Rgb(160, 40, 255)),
    ("White", Rgb(255, 255, 255)),
    ("Amber", Rgb(255, 140, 0)),
    ("Cyan", Rgb(0, 255, 255)),
];
pub(super) fn panel(title: &str) -> Block<'_> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(Style::default().fg(Color::Rgb(50, 68, 90)))
}
pub(super) fn focus(active: bool) -> Style {
    if active {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(MUTED)
    }
}
