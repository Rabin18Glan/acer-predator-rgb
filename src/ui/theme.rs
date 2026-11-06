use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType},
};
pub(super) const BG: Color = Color::Rgb(12, 17, 27);
pub(super) const MUTED: Color = Color::Rgb(145, 160, 180);
pub(super) const ACCENT: Color = Color::Rgb(67, 221, 202);
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
