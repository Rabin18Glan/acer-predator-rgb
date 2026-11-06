use super::{
    AppUi,
    hit::HitTarget,
    theme::{ACCENT, MUTED, panel},
};
use crate::{
    app::App,
    keyboard,
    model::{Effect, Rgb},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};
pub(super) struct KeyboardPreview;

impl KeyboardPreview {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let block = panel(" Keyboard ");
        let inner = block.inner(area);
        f.render_widget(block, area);
        for key in keyboard::keys(Rect::new(inner.x, inner.y, inner.width, 6)) {
            let Rgb(r, g, b) =
                keyboard::color(&app.settings, key.zone, ui.started.elapsed().as_secs_f32());
            let active =
                app.settings.effect != Effect::Static || app.zone == 0 || app.zone == key.zone + 1;
            let scale = if active { 1.0 } else { 0.35 };
            let shade = |n: u8| ((n as f32 * 0.38 + 18.0) * scale) as u8;
            let style = Style::default()
                .bg(Color::Rgb(shade(r), shade(g), shade(b)))
                .fg(if active {
                    Color::Rgb(235, 242, 250)
                } else {
                    Color::Rgb(100, 113, 133)
                });
            f.render_widget(
                Paragraph::new(key.label)
                    .alignment(Alignment::Center)
                    .style(style),
                key.area,
            );
            ui.hits.push((key.area, HitTarget::Zone(key.zone + 1)));
        }
        let labels = Layout::horizontal([Constraint::Ratio(1, 4); 4]).split(Rect::new(
            inner.x,
            inner.y + 6,
            inner.width,
            1,
        ));
        for (i, r) in labels.iter().enumerate() {
            let active =
                app.settings.effect == Effect::Static && (app.zone == 0 || app.zone == i + 1);
            f.render_widget(
                Paragraph::new(format!(
                    "{} Zone {} · ⇧{}",
                    if active { "●" } else { "○" },
                    i + 1,
                    i + 1
                ))
                .alignment(Alignment::Center)
                .style(Style::default().fg(if active { ACCENT } else { MUTED })),
                *r,
            );
            ui.hits.push((*r, HitTarget::Zone(i + 1)));
        }
    }
}
