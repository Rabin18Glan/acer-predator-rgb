//! Named screen regions shared by the UI composer.
use ratatui::layout::{Constraint, Layout, Rect};

pub(super) struct ScreenLayout {
    pub header: Rect,
    pub tabs: Rect,
    pub keyboard: Rect,
    pub controls: Rect,
    pub status: Rect,
    pub footer: Rect,
}

impl ScreenLayout {
    pub(super) fn new(area: Rect) -> Option<Self> {
        if area.width < 76 || area.height < 24 {
            return None;
        }
        let width = area.width.saturating_sub(4).min(112);
        let content = Rect::new(
            area.x + (area.width - width) / 2,
            area.y,
            width,
            area.height,
        );
        let rows = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(9),
            Constraint::Min(8),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(content);
        Some(Self {
            header: rows[0],
            tabs: rows[1],
            keyboard: rows[2],
            controls: rows[3],
            status: rows[4],
            footer: rows[5],
        })
    }
}
