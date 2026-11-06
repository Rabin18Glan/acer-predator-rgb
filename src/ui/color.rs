use super::{
    AppUi,
    hit::HitTarget,
    palette::Palette,
    theme::{ACCENT, MUTED, focus, panel},
};
use crate::{
    app::App,
    model::{Effect, Rgb},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};
pub(super) struct ColorPanel;

impl ColorPanel {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let cols = Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)])
            .spacing(1)
            .split(area);
        let block = panel(" Adjust ");
        let inner = block.inner(cols[0]);
        f.render_widget(block, cols[0]);
        let target = if app.settings.effect != Effect::Static {
            app.settings.effect.name().into()
        } else if app.zone == 0 {
            "All zones".into()
        } else {
            format!("Zone {}", app.zone)
        };
        f.render_widget(
            Paragraph::new(format!(
                "{} Target      ‹ {target} ›",
                if app.selected == 0 { "›" } else { " " }
            ))
            .style(focus(app.selected == 0)),
            Rect::new(inner.x, inner.y, inner.width, 1),
        );
        if app.settings.effect == Effect::Static {
            ui.hits.push((
                Rect::new(inner.x, inner.y, inner.width, 1),
                HitTarget::Zone((app.zone + 1) % 5),
            ));
        }
        let Rgb(r, g, b) = app.color();
        for (i, (label, value, max, color)) in [
            ("Red", r, 255, Color::Rgb(240, 110, 125)),
            ("Green", g, 255, Color::Rgb(107, 205, 152)),
            ("Blue", b, 255, Color::Rgb(108, 159, 244)),
            ("Brightness", app.settings.brightness, 100, ACCENT),
        ]
        .into_iter()
        .enumerate()
        {
            let y = inner.y + 1 + i as u16;
            let gauge = Rect::new(inner.x + 12, y, inner.width.saturating_sub(17), 1);
            let active = app.selected == i + 1;
            f.render_widget(
                Paragraph::new(format!("{} {label}", if active { "›" } else { " " }))
                    .style(focus(active)),
                Rect::new(inner.x, y, 12, 1),
            );
            f.render_widget(
                ratatui::widgets::LineGauge::default()
                    .label("")
                    .ratio(value as f64 / max as f64)
                    .line_set(ratatui::symbols::line::THICK)
                    .filled_style(Style::default().fg(color))
                    .unfilled_style(Style::default().fg(Color::Rgb(38, 51, 69))),
                gauge,
            );
            f.render_widget(
                Paragraph::new(format!("{value:>3}{}", if i == 3 { "%" } else { " " }))
                    .style(Style::default().fg(if active { Color::White } else { MUTED })),
                Rect::new(inner.right().saturating_sub(4), y, 4, 1),
            );
            ui.hits.push((gauge, HitTarget::Slider(i + 1)));
        }

        Palette::render(f, cols[1], app, ui);
    }
}
