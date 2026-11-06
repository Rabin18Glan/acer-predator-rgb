use super::{
    AppUi,
    hit::HitTarget,
    theme::{BG, MUTED, focus, panel},
};
use crate::{app::App, model::Effect};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
};
pub(super) struct EffectPanel;

impl EffectPanel {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let block = panel(" Effect ");
        let inner = block.inner(area);
        f.render_widget(block, area);
        for (i, e) in Effect::ALL.into_iter().enumerate() {
            let w = inner.width / 2;
            let rect = Rect::new(
                inner.x + (i % 2) as u16 * w,
                inner.y + (i / 2) as u16 * 2,
                w.saturating_sub(1),
                2,
            );
            let description = match e {
                Effect::Static => "One color per zone",
                Effect::Breathing => "Soft fade in and out",
                Effect::Neon => "Cycle through colors",
                Effect::Wave => "Colors travel across keys",
                Effect::Shifting => "Move your color across zones",
                Effect::Zoom => "Pulse your color outward",
            };
            let active = app.settings.effect == e;
            f.render_widget(
                Paragraph::new(vec![
                    Line::from(format!("{} {}", if active { "●" } else { "○" }, e.name()))
                        .style(focus(active)),
                    Line::from(format!("  {description}")).style(Style::default().fg(MUTED)),
                ])
                .style(Style::default().bg(if active {
                    Color::Rgb(22, 39, 47)
                } else {
                    BG
                })),
                rect,
            );
            ui.hits.push((rect, HitTarget::Effect(e)));
        }
    }
}
