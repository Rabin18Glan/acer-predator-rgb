use super::{
    AppUi,
    hit::HitTarget,
    theme::{ACCENT, BG, MUTED, PRESETS, panel},
};
use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};
pub(super) struct Palette;

impl Palette {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let block = panel(" Palette ");
        let inner = block.inner(area);
        f.render_widget(block, area);
        if let Some(input) = &app.hex_input {
            f.render_widget(
                Paragraph::new(format!(
                    "HEX  #{input}▏\n\nSix digits apply live · Esc cancels"
                ))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(ACCENT)),
                inner,
            );
            return;
        }
        for (i, (label, c)) in PRESETS.iter().enumerate() {
            let w = (inner.width / 4).max(1);
            let rect = Rect::new(
                inner.x + (i % 4) as u16 * w,
                inner.y + if i < 4 { 0 } else { 2 },
                w.saturating_sub(1).max(1),
                1,
            );
            let text = if c.0 as u16 + c.1 as u16 + c.2 as u16 > 400 {
                BG
            } else {
                Color::White
            };
            f.render_widget(
                Paragraph::new(*label)
                    .alignment(Alignment::Center)
                    .style(Style::default().bg(Color::Rgb(c.0, c.1, c.2)).fg(text)),
                rect,
            );
            ui.hits.push((rect, HitTarget::Preset(*c)));
        }
        let info = if !app.settings.effect.uses_color() {
            "Automatic colors".into()
        } else {
            format!("{}   h edit", String::from(app.color()))
        };
        f.render_widget(
            Paragraph::new(info).style(Style::default().fg(MUTED)),
            Rect::new(inner.x, inner.y + 4, inner.width, 1),
        );
        if inner.height > 5 {
            f.render_widget(
                Paragraph::new("R G B P W O C · color shortcuts").style(Style::default().fg(MUTED)),
                Rect::new(inner.x, inner.y + 5, inner.width, 1),
            );
        }
    }
}
