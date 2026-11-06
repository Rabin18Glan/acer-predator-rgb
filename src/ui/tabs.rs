use super::{
    AppUi,
    hit::HitTarget,
    theme::{ACCENT, MUTED},
};
use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Tabs},
};
pub(super) struct TabsBar;

impl TabsBar {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let titles = ["1 Color selection", "2 Effect", "3 Speed", "4 Direction"];
        f.render_widget(
            Tabs::new(titles)
                .select(app.tab.index())
                .highlight_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
                .style(Style::default().fg(MUTED))
                .padding(" ", " ")
                .divider(" ")
                .block(
                    Block::default()
                        .borders(ratatui::widgets::Borders::BOTTOM)
                        .border_style(Style::default().fg(Color::Rgb(39, 51, 68))),
                ),
            area,
        );
        let mut x = area.x;
        for (i, title) in titles.iter().enumerate() {
            let w = title.len() as u16 + 2;
            ui.hits
                .push((Rect::new(x, area.y, w, 1), HitTarget::Tab(Tab::ALL[i])));
            x += w + 1;
        }
    }
}
