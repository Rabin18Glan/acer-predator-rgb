use crate::{
    config, device, keyboard,
    model::{Direction, Effect, Rgb, Settings},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Tabs, Wrap},
};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Color,
    Speed,
    Direction,
    Effect,
}
impl Tab {
    pub const ALL: [Self; 4] = [Self::Color, Self::Speed, Self::Direction, Self::Effect];
    pub fn index(self) -> usize {
        match self {
            Self::Color => 0,
            Self::Speed => 1,
            Self::Direction => 2,
            Self::Effect => 3,
        }
    }
}
#[derive(Clone, Copy)]
enum Action {
    Tab(Tab),
    Zone(usize),
    Preset(Rgb),
    Slider(usize),
    Effect(Effect),
    Direction(Direction),
}

pub struct App {
    pub settings: Settings,
    pub tab: Tab,
    pub selected: usize,
    pub zone: usize,
    pub status: String,
    pub connection: String,
    pub error: bool,
    pub hex_input: Option<String>,
    pending: Option<Instant>,
    started: Instant,
    hits: Vec<(Rect, Action)>,
}
impl App {
    fn from_settings(settings: Settings, status: String) -> Self {
        Self {
            settings,
            tab: Tab::Color,
            selected: 0,
            zone: 0,
            status,
            connection: String::new(),
            error: false,
            hex_input: None,
            pending: None,
            started: Instant::now(),
            hits: vec![],
        }
    }
    pub fn new() -> Self {
        let mut app = match config::load() {
            Ok((s, msg)) => Self::from_settings(s, msg),
            Err(e) => {
                let mut a = Self::from_settings(Settings::default(), e);
                a.error = true;
                a
            }
        };
        app.refresh();
        app
    }
    pub fn refresh(&mut self) {
        match device::status() {
            Ok(s) => {
                self.connection = s.describe();
                self.error = false;
            }
            Err(e) => {
                self.connection = "Driver unavailable".into();
                self.status = e.to_string();
                self.error = true;
            }
        }
    }
    pub fn select_tab(&mut self, tab: Tab) {
        self.tab = tab;
        self.selected = 0;
    }
    pub fn select_zone(&mut self, zone: usize) {
        self.zone = zone;
        self.select_tab(Tab::Color);
        self.status = format!(
            "Zone {zone} selected. {}",
            if self.settings.effect == Effect::Static {
                "Color edits now affect this zone."
            } else {
                "Choose Static in Effect for per-zone colors; animations use shared colors."
            }
        );
    }
    pub fn color(&self) -> Rgb {
        if self.settings.effect == Effect::Static {
            self.settings.zones[self.zone.saturating_sub(1)]
        } else {
            self.settings.color
        }
    }
    pub fn set_color(&mut self, c: Rgb) {
        if !self.settings.effect.uses_color() {
            self.status =
                "Neon and Wave generate colors automatically. Choose another effect to edit RGB."
                    .into();
            return;
        }
        if self.settings.effect == Effect::Static {
            if self.zone == 0 {
                self.settings.zones.fill(c);
            } else {
                self.settings.zones[self.zone - 1] = c;
            }
        } else {
            self.settings.color = c;
        }
    }
    pub fn move_selection(&mut self, delta: i16) {
        match self.tab {
            Tab::Color => self.selected = (self.selected as i16 + delta).rem_euclid(5) as usize,
            _ => self.adjust(delta),
        }
    }
    pub fn adjust(&mut self, delta: i16) {
        let clamp = |v: u8, max: i16| (i16::from(v) + delta).clamp(0, max) as u8;
        match self.tab {
            Tab::Color => match self.selected {
                0 => {
                    if self.settings.effect == Effect::Static {
                        self.zone = (self.zone as i16 + delta.signum()).rem_euclid(5) as usize;
                    }
                }
                1..=3 => {
                    let Rgb(r, g, b) = self.color();
                    let mut c = [r, g, b];
                    let i = self.selected - 1;
                    c[i] = clamp(c[i], 255);
                    self.set_color(Rgb(c[0], c[1], c[2]));
                }
                4 => self.settings.brightness = clamp(self.settings.brightness, 100),
                _ => {}
            },
            Tab::Speed => {
                if self.settings.effect != Effect::Static {
                    self.settings.speed = clamp(self.settings.speed, 9);
                } else {
                    self.status =
                        "Static has no animation speed. Select an animation in the Effect tab."
                            .into();
                }
            }
            Tab::Direction => {
                if self.settings.effect != Effect::Static {
                    self.settings.direction = if delta > 0 {
                        Direction::LeftToRight
                    } else {
                        Direction::RightToLeft
                    };
                } else {
                    self.status =
                        "Static has no animation direction. Select an animation in the Effect tab."
                            .into();
                }
            }
            Tab::Effect => {
                self.settings.effect = Effect::ALL
                    [(self.settings.effect.id() as i16 + delta.signum()).rem_euclid(6) as usize]
            }
        }
    }
    /// Called only for an actual settings change, never navigation or startup.
    pub fn changed(&mut self, before: &Settings) {
        if self.settings != *before {
            self.pending = Some(Instant::now());
            self.status = "Sending live change…".into();
            self.error = false;
        }
    }
    pub fn tick(&mut self) {
        // Coalesce fast repeats for 40ms; no Enter or confirmation needed.
        if self
            .pending
            .is_some_and(|t| t.elapsed() >= Duration::from_millis(40))
        {
            self.apply();
        }
    }
    pub fn apply(&mut self) {
        self.pending = None;
        match device::apply(&self.settings, &mut device::Device) {
            Ok(s) => {
                self.connection = s.describe();
                self.status = "Live change applied · firmware status 0".into();
                self.error = false;
            }
            Err(e) => {
                self.status = format!("Live change failed: {e}. Press r to check connection.");
                self.error = true;
            }
        }
    }
    pub fn save(&mut self) {
        match config::save(&self.settings) {
            Ok(p) => {
                self.status = format!("Saved {}", p.display());
                self.error = false;
            }
            Err(e) => {
                self.status = e;
                self.error = true;
            }
        }
    }
    pub fn load(&mut self) {
        match config::load() {
            Ok((s, msg)) => {
                self.settings = s;
                self.status = msg;
                self.error = false;
            }
            Err(e) => {
                self.status = e;
                self.error = true;
            }
        }
    }
    pub fn click(&mut self, x: u16, y: u16) {
        let hit = self
            .hits
            .iter()
            .rev()
            .find(|(r, _)| r.contains((x, y).into()))
            .copied();
        if let Some((r, a)) = hit {
            match a {
                Action::Tab(t) => self.select_tab(t),
                Action::Zone(z) => {
                    self.zone = z;
                    self.select_tab(Tab::Color);
                }
                Action::Preset(c) => self.set_color(c),
                Action::Effect(e) => self.settings.effect = e,
                Action::Direction(d) => {
                    if self.settings.effect != Effect::Static {
                        self.settings.direction = d;
                    }
                }
                Action::Slider(field) => {
                    let max = if field == 4 {
                        100
                    } else if self.tab == Tab::Speed {
                        9
                    } else {
                        255
                    };
                    let value =
                        ((x - r.x) as u32 * max / (r.width.saturating_sub(1).max(1) as u32)) as u8;
                    if self.tab == Tab::Speed {
                        if self.settings.effect != Effect::Static {
                            self.settings.speed = value;
                        }
                    } else if field == 4 {
                        self.settings.brightness = value;
                        self.selected = 4;
                    } else {
                        let Rgb(red, green, blue) = self.color();
                        let mut c = [red, green, blue];
                        c[field - 1] = value;
                        self.set_color(Rgb(c[0], c[1], c[2]));
                        self.selected = field;
                    }
                }
            }
        }
    }
}

const BG: Color = Color::Rgb(12, 17, 27);
const MUTED: Color = Color::Rgb(145, 160, 180);
const ACCENT: Color = Color::Rgb(67, 221, 202);
const PRESETS: [(&str, Rgb); 7] = [
    ("Red", Rgb(255, 0, 0)),
    ("Green", Rgb(0, 255, 0)),
    ("Blue", Rgb(0, 80, 255)),
    ("Purple", Rgb(160, 40, 255)),
    ("White", Rgb(255, 255, 255)),
    ("Amber", Rgb(255, 140, 0)),
    ("Cyan", Rgb(0, 255, 255)),
];
fn panel(title: &str) -> Block<'_> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(Style::default().fg(Color::Rgb(50, 68, 90)))
}
pub fn draw(f: &mut Frame, app: &mut App) {
    app.hits.clear();
    let area = f.area();
    f.render_widget(
        Block::default().style(Style::default().fg(Color::White).bg(BG)),
        area,
    );
    if area.width < 76 || area.height < 24 {
        f.render_widget(
            Paragraph::new(
                "Acer RGB Studio\nEnlarge terminal to 76 × 24 or larger.\nq / Ctrl+C quit.",
            )
            .wrap(Wrap { trim: true }),
            area,
        );
        return;
    }
    // Keep controls readable on wide terminals instead of stretching every key.
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
    let header = Layout::horizontal([Constraint::Min(25), Constraint::Length(24)]).split(rows[0]);
    f.render_widget(
        Paragraph::new(vec![
            Line::from("ACER / RGB STUDIO").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from("Keyboard lighting").style(Style::default().fg(MUTED)),
        ]),
        header[0],
    );
    let target = if app.settings.effect != Effect::Static {
        "Animation".into()
    } else if app.zone == 0 {
        "All zones".into()
    } else {
        format!("Zone {}", app.zone)
    };
    f.render_widget(
        Paragraph::new(vec![
            Line::from("● LIVE").style(Style::default().fg(ACCENT)),
            Line::from(format!("{} / {target}", app.settings.effect.name()))
                .style(Style::default().fg(MUTED)),
        ])
        .alignment(Alignment::Right),
        header[1],
    );

    let titles = ["1 Color selection", "2 Speed", "3 Direction", "4 Effect"];
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
        rows[1],
    );
    let mut x = rows[1].x;
    for (i, title) in titles.iter().enumerate() {
        let w = title.len() as u16 + 2;
        app.hits
            .push((Rect::new(x, rows[1].y, w, 1), Action::Tab(Tab::ALL[i])));
        x += w + 1;
    }
    preview(f, rows[2], app);
    match app.tab {
        Tab::Color => colors(f, rows[3], app),
        Tab::Speed => speed(f, rows[3], app),
        Tab::Direction => direction(f, rows[3], app),
        Tab::Effect => effects(f, rows[3], app),
    }
    let (message, color) = if app.error {
        (app.status.as_str(), Color::Rgb(255, 129, 135))
    } else if app.pending.is_some() {
        ("Updating…", MUTED)
    } else {
        (app.status.as_str(), MUTED)
    };
    f.render_widget(
        Paragraph::new(message)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(color)),
        rows[4],
    );
    f.render_widget(
        Paragraph::new("Tab tabs   Shift+1–4 zones   Arrows edit   h hex   s save   q quit")
            .style(Style::default().fg(MUTED)),
        rows[5],
    );
}

fn preview(f: &mut Frame, area: Rect, app: &mut App) {
    let block = panel(" Keyboard ");
    let inner = block.inner(area);
    f.render_widget(block, area);
    for key in keyboard::keys(Rect::new(inner.x, inner.y, inner.width, 6)) {
        let Rgb(r, g, b) =
            keyboard::color(&app.settings, key.zone, app.started.elapsed().as_secs_f32());
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
        app.hits.push((key.area, Action::Zone(key.zone + 1)));
    }
    let labels = Layout::horizontal([Constraint::Ratio(1, 4); 4]).split(Rect::new(
        inner.x,
        inner.y + 6,
        inner.width,
        1,
    ));
    for (i, r) in labels.iter().enumerate() {
        let active = app.settings.effect == Effect::Static && (app.zone == 0 || app.zone == i + 1);
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
        app.hits.push((*r, Action::Zone(i + 1)));
    }
}

fn colors(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)])
        .spacing(1)
        .split(area);
    let block = panel(" Adjust ");
    let inner = block.inner(cols[0]);
    f.render_widget(block, cols[0]);
    let target = if app.settings.effect != Effect::Static {
        "Animation".into()
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
    app.hits.push((
        Rect::new(inner.x, inner.y, inner.width, 1),
        Action::Zone((app.zone + 1) % 5),
    ));
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
        app.hits.push((gauge, Action::Slider(i + 1)));
    }
    let block = panel(" Palette ");
    let inner = block.inner(cols[1]);
    f.render_widget(block, cols[1]);
    if let Some(input) = &app.hex_input {
        f.render_widget(
            Paragraph::new(format!(
                "HEX  #{input}▏\n\nSix digits apply live · Esc cancels"
            ))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(ACCENT)),
            inner,
        );
        return;
    }
    for (i, (label, c)) in PRESETS.iter().enumerate() {
        let w = (inner.width / 4).max(1);
        let rect = Rect::new(
            inner.x + (i % 4) as u16 * w,
            inner.y + if i < 4 { 0 } else { 2 },
            w.saturating_sub(1).max(1),
            1,
        );
        let text = if c.0 as u16 + c.1 as u16 + c.2 as u16 > 400 {
            BG
        } else {
            Color::White
        };
        f.render_widget(
            Paragraph::new(*label)
                .alignment(Alignment::Center)
                .style(Style::default().bg(Color::Rgb(c.0, c.1, c.2)).fg(text)),
            rect,
        );
        app.hits.push((rect, Action::Preset(*c)));
    }
    let info = if !app.settings.effect.uses_color() {
        "Automatic colors".into()
    } else {
        format!("{}   h edit", String::from(app.color()))
    };
    f.render_widget(
        Paragraph::new(info).style(Style::default().fg(MUTED)),
        Rect::new(inner.x, inner.y + 4, inner.width, 1),
    );
    if inner.height > 5 {
        f.render_widget(
            Paragraph::new("R G B P W O C · color shortcuts").style(Style::default().fg(MUTED)),
            Rect::new(inner.x, inner.y + 5, inner.width, 1),
        );
    }
}
fn focus(active: bool) -> Style {
    if active {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(MUTED)
    }
}
fn speed(f: &mut Frame, area: Rect, app: &mut App) {
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
    app.hits.push((gauge, Action::Slider(0)));
    f.render_widget(
        Paragraph::new("← Slower                   Faster →\n0 Freeze · 9 Fastest")
            .alignment(Alignment::Center)
            .style(Style::default().fg(MUTED)),
        Rect::new(inner.x, inner.y + 4, inner.width, 2),
    );
}
fn direction(f: &mut Frame, area: Rect, app: &mut App) {
    let block = panel(" Direction ");
    let inner = block.inner(area);
    f.render_widget(block, area);
    if app.settings.effect == Effect::Static {
        f.render_widget(Paragraph::new("Choose an animated effect to change its direction.\n4 Effect → Wave or Shifting makes direction easiest to see.")
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
        app.hits.push((*rect, Action::Direction(d)));
    }
    f.render_widget(
        Paragraph::new("← Right to left     → Left to right")
            .alignment(Alignment::Center)
            .style(Style::default().fg(MUTED)),
        Rect::new(inner.x, inner.y + 5, inner.width, 1),
    );
}
fn effects(f: &mut Frame, area: Rect, app: &mut App) {
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
        app.hits.push((rect, Action::Effect(e)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> App {
        App::from_settings(Settings::default(), "Ready".into())
    }
    #[test]
    fn navigation_does_not_schedule_hardware_commands() {
        let mut a = app();
        let before = a.settings.clone();
        a.select_tab(Tab::Speed);
        a.changed(&before);
        assert!(a.pending.is_none());
    }
    #[test]
    fn changed_settings_schedule_live_apply_without_enter() {
        let mut a = app();
        let before = a.settings.clone();
        a.set_color(Rgb(255, 0, 0));
        a.changed(&before);
        assert!(a.pending.is_some());
    }
    #[test]
    fn single_zone_edit_preserves_other_zones() {
        let mut a = app();
        a.zone = 3;
        a.set_color(Rgb(255, 0, 0));
        assert_eq!(a.settings.zones[2], Rgb(255, 0, 0));
        assert_eq!(a.settings.zones[0], Settings::default().zones[0]);
    }
    #[test]
    fn clicking_a_key_only_selects_its_zone() {
        let mut a = app();
        let before = a.settings.clone();
        let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24)).unwrap();
        t.draw(|f| draw(f, &mut a)).unwrap();
        let rect = a
            .hits
            .iter()
            .find_map(|(r, action)| match action {
                Action::Zone(4) => Some(*r),
                _ => None,
            })
            .unwrap();
        a.click(rect.x, rect.y);
        a.changed(&before);
        assert_eq!(a.zone, 4);
        assert!(a.pending.is_none());
    }
    #[test]
    fn clicking_effect_schedules_a_live_change() {
        let mut a = app();
        a.select_tab(Tab::Effect);
        let before = a.settings.clone();
        let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24)).unwrap();
        t.draw(|f| draw(f, &mut a)).unwrap();
        let rect = a
            .hits
            .iter()
            .find_map(|(r, action)| match action {
                Action::Effect(Effect::Breathing) => Some(*r),
                _ => None,
            })
            .unwrap();
        a.click(rect.x, rect.y);
        a.changed(&before);
        assert_eq!(a.settings.effect, Effect::Breathing);
        assert!(a.pending.is_some());
    }
    #[test]
    fn tabs_and_keyboard_fit_standard_terminal() {
        let mut a = app();
        let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24)).unwrap();
        for tab in Tab::ALL {
            a.select_tab(tab);
            t.draw(|f| draw(f, &mut a)).unwrap();
            let text: String = t
                .backend()
                .buffer()
                .content
                .iter()
                .map(|c| c.symbol())
                .collect();
            for label in [
                "Color selection",
                "Direction",
                "Effect",
                "Space",
                "Num",
                "Ready",
            ] {
                assert!(text.contains(label), "missing {label} in {tab:?}");
            }
        }
    }
}
