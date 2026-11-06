use crate::model::{Direction, Effect, Rgb, Settings};
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
