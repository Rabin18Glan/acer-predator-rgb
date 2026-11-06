use ratatui::{
    Frame,
    widgets::{Paragraph, Wrap},
};
pub(super) struct ResizeNotice;

impl ResizeNotice {
    pub(super) fn render(frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new(
                "Acer RGB Studio\nEnlarge terminal to 76 × 24 or larger.\nq / Ctrl+C quit.",
            )
            .wrap(Wrap { trim: true }),
            frame.area(),
        );
    }
}
