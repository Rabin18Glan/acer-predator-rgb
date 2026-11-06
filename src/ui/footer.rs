use super::theme::MUTED;
use ratatui::{Frame, layout::Rect, style::Style, widgets::Paragraph};
pub(super) struct Footer;

impl Footer {
    pub(super) fn render(frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Tab tabs   Shift+1–4 zones   Arrows edit   h hex   s save   q quit")
                .style(Style::default().fg(MUTED)),
            area,
        );
    }
}
