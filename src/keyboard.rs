//! A compact ANSI laptop keyboard with numpad, fitted to terminal cells.
use crate::model::{Direction, Effect, Rgb, Settings};
use ratatui::layout::Rect;

pub struct Key {
    pub area: Rect,
    pub label: &'static str,
    pub zone: usize,
}

pub fn keys(area: Rect) -> Vec<Key> {
    let mut result = Vec::new();
    let mut add = |x: f32, y: u16, width: f32, label: &'static str| {
        let left = area.x + (x / 20.0 * area.width as f32) as u16;
        let right = area.x + ((x + width) / 20.0 * area.width as f32) as u16;
        if y < area.height && right > left {
            result.push(Key {
                area: Rect::new(
                    left,
                    area.y + y,
                    (right - left)
                        .saturating_sub(1)
                        .max((label.chars().count() as u16).min(right - left))
                        .max(1),
                    1,
                ),
                label,
                zone: if x >= 16.0 {
                    3
                } else {
                    (((x + width / 2.0) / 5.0) as usize).min(2)
                },
            });
        }
    };
    add(0.0, 0, 1.3, "Esc");
    for (i, label) in [
        "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    ]
    .iter()
    .enumerate()
    {
        add(1.7 + i as f32 * 1.1, 0, 1.1, label);
    }
    for (i, label) in [
        "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
    ]
    .iter()
    .enumerate()
    {
        add(i as f32, 1, 1.0, label);
    }
    add(13.0, 1, 2.0, "Bksp");
    add(0.0, 2, 1.5, "Tab");
    for (i, label) in [
        "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\",
    ]
    .iter()
    .enumerate()
    {
        add(1.5 + i as f32, 2, 1.0, label);
    }
    add(0.0, 3, 1.8, "Caps");
    for (i, label) in ["A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'"]
        .iter()
        .enumerate()
    {
        add(1.8 + i as f32, 3, 1.0, label);
    }
    add(12.8, 3, 2.2, "Enter");
    add(0.0, 4, 2.3, "Shift");
    for (i, label) in ["Z", "X", "C", "V", "B", "N", "M", ",", ".", "/"]
        .iter()
        .enumerate()
    {
        add(2.3 + i as f32, 4, 1.0, label);
    }
    add(12.3, 4, 1.7, "Shift");
    add(14.0, 4, 1.0, "↑");
    for (x, w, label) in [
        (0.0, 1.3, "Ctrl"),
        (1.3, 1.0, "Fn"),
        (2.3, 1.0, "Win"),
        (3.3, 1.0, "Alt"),
        (4.3, 6.0, "Space"),
        (10.3, 1.0, "Alt"),
        (11.3, 1.0, "Ctrl"),
        (12.3, 0.9, "←"),
        (13.2, 0.9, "↓"),
        (14.1, 0.9, "→"),
    ] {
        add(x, 5, w, label);
    }
    for (y, labels) in [
        (0, ["Num", "/", "*", "-"]),
        (1, ["7", "8", "9", "+"]),
        (2, ["4", "5", "6", "+"]),
        (3, ["1", "2", "3", "Ent"]),
    ] {
        for (i, label) in labels.into_iter().enumerate() {
            add(16.0 + i as f32, y, 1.0, label);
        }
    }
    add(16.0, 4, 2.0, "0");
    add(18.0, 4, 1.0, ".");
    add(19.0, 4, 1.0, "Ent");
    result
}

pub fn color(settings: &Settings, zone: usize, seconds: f32) -> Rgb {
    let position = if settings.direction == Direction::LeftToRight {
        zone as f32
    } else {
        (3 - zone) as f32
    };
    let t = seconds
        * (0.15 + settings.speed as f32 * 0.12)
        * if settings.speed == 0 { 0.0 } else { 1.0 };
    let wave = |phase: f32| (phase.sin() + 1.0) / 2.0;
    let (c, level) = match settings.effect {
        Effect::Static => (settings.zones[zone], 1.0),
        Effect::Breathing => (settings.color, 0.12 + 0.88 * wave(t * 3.0)),
        Effect::Neon => (rainbow(t * 0.2), 1.0),
        Effect::Wave => (rainbow(t * 0.2 - position / 5.0), 1.0),
        Effect::Shifting => (settings.color, 0.12 + 0.88 * wave(t * 3.0 - position)),
        Effect::Zoom => (
            settings.color,
            0.12 + 0.88 * wave(t * 3.0 - (position - 1.5).abs() * 2.0),
        ),
    };
    let scale = settings.brightness as f32 / 100.0 * level;
    Rgb(
        (c.0 as f32 * scale) as u8,
        (c.1 as f32 * scale) as u8,
        (c.2 as f32 * scale) as u8,
    )
}
fn rainbow(h: f32) -> Rgb {
    let x = h.rem_euclid(1.0) * 6.0;
    let a = ((1.0 - (x % 2.0 - 1.0).abs()) * 255.0) as u8;
    match x as u8 {
        0 => Rgb(255, a, 0),
        1 => Rgb(a, 255, 0),
        2 => Rgb(0, 255, a),
        3 => Rgb(0, a, 255),
        4 => Rgb(a, 0, 255),
        _ => Rgb(255, 0, a),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_has_letters_modifiers_and_numpad_within_bounds() {
        let area = Rect::new(2, 3, 76, 6);
        let keys = keys(area);
        for label in ["Esc", "Q", "A", "Z", "Space", "Enter", "Num", "Ent"] {
            assert!(keys.iter().any(|k| k.label == label));
        }
        assert!(
            keys.iter()
                .all(|k| k.area.right() <= area.right() && k.area.bottom() <= area.bottom())
        );
        assert_eq!(keys.iter().find(|k| k.label == "Num").unwrap().zone, 3);
    }
}
