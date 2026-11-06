//! Assemble the complete screen from independently rendered components.
use super::{
    color::ColorPanel, direction::DirectionPanel, effect::EffectPanel, footer::Footer,
    header::Header, hit::HitTarget, layout::ScreenLayout, preview::KeyboardPreview,
    resize_notice::ResizeNotice, speed::SpeedPanel, status::StatusBar, tabs::TabsBar, theme::BG,
};
use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
};
use std::time::Instant;

pub(crate) struct AppUi {
    pub(super) started: Instant,
    pub(super) hits: Vec<(Rect, HitTarget)>,
}

impl AppUi {
    pub(crate) fn new() -> Self {
        Self {
            started: Instant::now(),
            hits: Vec::new(),
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, app: &App) {
        self.hits.clear();
        frame.render_widget(
            Block::default().style(Style::default().fg(Color::White).bg(BG)),
            frame.area(),
        );
        let Some(layout) = ScreenLayout::new(frame.area()) else {
            ResizeNotice::render(frame);
            return;
        };

        Header::render(frame, layout.header, app);
        TabsBar::render(frame, layout.tabs, app, self);
        KeyboardPreview::render(frame, layout.keyboard, app, self);

        match app.tab {
            Tab::Color => ColorPanel::render(frame, layout.controls, app, self),
            Tab::Speed => SpeedPanel::render(frame, layout.controls, app, self),
            Tab::Direction => DirectionPanel::render(frame, layout.controls, app, self),
            Tab::Effect => EffectPanel::render(frame, layout.controls, app, self),
        }

        StatusBar::render(frame, layout.status, app);
        Footer::render(frame, layout.footer);
    }
}
