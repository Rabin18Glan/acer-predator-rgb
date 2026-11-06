use super::{
    AppUi,
    hit::HitTarget,
    theme::{ACCENT, BG, MUTED, panel},
};
use crate::{app::App, model::Rgb};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

pub(super) struct Palette;

impl Palette {
    pub(super) fn render(f: &mut Frame, area: Rect, app: &App, ui: &mut AppUi) {
        let block = panel(" Palette ");
        let inner = block.inner(area);
        f.render_widget(block, area);
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
        if inner.height == 0 || inner.width == 0 {
            return;
        }
        let columns = (inner.width / 3).max(1);
        let rows = inner.height.saturating_sub(1).min(10);
        for row in 0..rows {
            for column in 0..columns {
                let color = Self::swatch(column, row, columns, rows);
                let rect = Rect::new(inner.x + column * 3, inner.y + row, inner.width.min(2), 1);
                let Rgb(r, g, b) = color;
                let contrast =
                    if u32::from(r) * 299 + u32::from(g) * 587 + u32::from(b) * 114 > 140_000 {
                        BG
                    } else {
                        Color::White
                    };
                f.render_widget(
                    Paragraph::new(if color == app.color() { "•" } else { " " })
                        .alignment(Alignment::Center)
                        .style(Style::default().bg(Color::Rgb(r, g, b)).fg(contrast)),
                    rect,
                );
                ui.hits.push((rect, HitTarget::Preset(color)));
            }
        }
        let info = if !app.settings.effect.uses_color() {
            "Automatic colors".into()
        } else {
            format!("{}   h edit", String::from(app.color()))
        };
        f.render_widget(
            Paragraph::new(info).style(Style::default().fg(MUTED)),
            Rect::new(inner.x, inner.bottom() - 1, inner.width, 1),
        );
    }

    /// Hue columns, light-to-dark rows, and a final grayscale row.
    fn swatch(column: u16, row: u16, columns: u16, rows: u16) -> Rgb {
        if row == rows - 1 {
            let value =
                (u32::from(column) * 255 / u32::from(columns.saturating_sub(1).max(1))) as u8;
            return Rgb(value, value, value);
        }
        let hue = u32::from(column) * 6 * 255 / u32::from(columns);
        let step = (hue % 255) as u8;
        let base = match hue / 255 {
            0 => Rgb(255, step, 0),
            1 => Rgb(255 - step, 255, 0),
            2 => Rgb(0, 255, step),
            3 => Rgb(0, 255 - step, 255),
            4 => Rgb(step, 0, 255),
            _ => Rgb(255, 0, 255 - step),
        };
        let middle = rows.saturating_sub(2) / 2;
        let shade = |value: u8| {
            if row <= middle {
                let white = u32::from(middle - row) * 160 / u32::from(middle.max(1));
                (u32::from(value) + (255 - u32::from(value)) * white / 255) as u8
            } else {
                let level =
                    255 - u32::from(row - middle) * 170 / u32::from((rows - 2 - middle).max(1));
                (u32::from(value) * level / 255) as u8
            }
        };
        Rgb(shade(base.0), shade(base.1), shade(base.2))
    }
}
