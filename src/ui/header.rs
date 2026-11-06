use super::theme::{ACCENT, MUTED};
use crate::{app::App, model::Effect};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::Paragraph,
};
pub(super) struct Header;

impl Header {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App) {
        let header = Layout::horizontal([Constraint::Min(25), Constraint::Length(24)]).split(area);
        f.render_widget(
            Paragraph::new(vec![
                Line::from("ACER / RGB STUDIO").style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Line::from("Keyboard lighting").style(Style::default().fg(MUTED)),
            ]),
            header[0],
        );
        let target = if app.settings.effect != Effect::Static {
            "Animation".into()
        } else if app.zone == 0 {
            "All zones".into()
        } else {
            format!("Zone {}", app.zone)
        };
        f.render_widget(
            Paragraph::new(vec![
                Line::from("● LIVE").style(Style::default().fg(ACCENT)),
                Line::from(format!("{} / {target}", app.settings.effect.name()))
                    .style(Style::default().fg(MUTED)),
            ])
            .alignment(Alignment::Right),
            header[1],
        );
    }
}
