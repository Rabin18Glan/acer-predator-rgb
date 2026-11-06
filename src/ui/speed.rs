use super::{
    AppUi,
    hit::HitTarget,
    theme::{ACCENT, MUTED, panel},
};
use crate::{app::App, model::Effect};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Wrap},
};
pub(super) struct SpeedPanel;

impl SpeedPanel {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let block = panel(" Speed ");
        let inner = block.inner(area);
        f.render_widget(block, area);
        if app.settings.effect == Effect::Static {
            f.render_widget(Paragraph::new("Choose an animated effect to adjust its speed.\n4 Effect → Breathing, Neon, Wave, Shifting or Zoom.")
            .wrap(Wrap{trim:true}).style(Style::default().fg(MUTED)),inner);
            return;
        }
        f.render_widget(
            Paragraph::new(format!("{} / 9", app.settings.speed))
                .alignment(Alignment::Center)
                .style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            inner,
        );
        let gauge = Rect::new(inner.x + 3, inner.y + 2, inner.width.saturating_sub(6), 1);
        f.render_widget(
            ratatui::widgets::LineGauge::default()
                .label("")
                .ratio(app.settings.speed as f64 / 9.0)
                .line_set(ratatui::symbols::line::THICK)
                .filled_style(Style::default().fg(ACCENT))
                .unfilled_style(Style::default().fg(Color::Rgb(38, 51, 69))),
            gauge,
        );
        ui.hits.push((gauge, HitTarget::Slider(0)));
        f.render_widget(
            Paragraph::new("← Slower                   Faster →\n0 Freeze · 9 Fastest")
                .alignment(Alignment::Center)
                .style(Style::default().fg(MUTED)),
            Rect::new(inner.x, inner.y + 4, inner.width, 2),
        );
    }
}
