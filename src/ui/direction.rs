use super::{
    AppUi,
    hit::HitTarget,
    theme::{MUTED, focus, panel},
};
use crate::{
    app::App,
    model::{Direction, Effect},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Paragraph, Wrap},
};
pub(super) struct DirectionPanel;

impl DirectionPanel {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let block = panel(" Direction ");
        let inner = block.inner(area);
        f.render_widget(block, area);
        if app.settings.effect == Effect::Static {
            f.render_widget(Paragraph::new("Choose an animated effect to change its direction.\n2 Effect → Wave or Shifting makes direction easiest to see.")
            .wrap(Wrap{trim:true}).style(Style::default().fg(MUTED)),inner);
            return;
        }
        let cards = Layout::horizontal([Constraint::Percentage(50); 2])
            .spacing(2)
            .split(Rect::new(
                inner.x + 1,
                inner.y,
                inner.width.saturating_sub(2),
                4,
            ));
        for (rect, d) in cards
            .iter()
            .zip([Direction::RightToLeft, Direction::LeftToRight])
        {
            let active = app.settings.direction == d;
            let block = panel(if active { " Selected " } else { " " }).border_style(focus(active));
            f.render_widget(
                Paragraph::new(vec![
                    Line::from(if d == Direction::RightToLeft {
                        "← ← ←"
                    } else {
                        "→ → →"
                    }),
                    Line::from(d.name()),
                ])
                .alignment(Alignment::Center)
                .style(focus(active))
                .block(block),
                *rect,
            );
            ui.hits.push((*rect, HitTarget::Direction(d)));
        }
        f.render_widget(
            Paragraph::new("← Right to left     → Left to right")
                .alignment(Alignment::Center)
                .style(Style::default().fg(MUTED)),
            Rect::new(inner.x, inner.y + 5, inner.width, 1),
        );
    }
}
