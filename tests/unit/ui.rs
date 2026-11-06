use super::hit::HitTarget;
use super::*;
use crate::app::App;
use crate::{
    app::Tab,
    model::{Effect, Rgb, Settings},
};
fn app() -> App {
    App::from_settings(Settings::default(), "Ready".into())
}

#[test]
fn navigation_does_not_schedule_hardware_commands() {
    let mut a = app();
    let before = a.settings.clone();
    a.select_tab(Tab::Speed);
    a.changed(&before);
    assert!(!a.is_pending());
}
#[test]
fn changed_settings_schedule_live_apply_without_enter() {
    let mut a = app();
    let before = a.settings.clone();
    a.set_color(Rgb(255, 0, 0));
    a.changed(&before);
    assert!(a.is_pending());
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
    let mut view = AppUi::new();
    t.draw(|f| view.draw(f, &a)).unwrap();
    let rect = view
        .hits
        .iter()
        .find_map(|(r, action)| match action {
            HitTarget::Zone(4) => Some(*r),
            _ => None,
        })
        .unwrap();
    a.handle_action(view.action_at(rect.x, rect.y).unwrap());
    a.changed(&before);
    assert_eq!(a.zone, 4);
    assert!(!a.is_pending());
}
#[test]
fn clicking_effect_schedules_a_live_change() {
    let mut a = app();
    a.select_tab(Tab::Effect);
    let before = a.settings.clone();
    let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24)).unwrap();
    let mut view = AppUi::new();
    t.draw(|f| view.draw(f, &a)).unwrap();
    let rect = view
        .hits
        .iter()
        .find_map(|(r, action)| match action {
            HitTarget::Effect(Effect::Breathing) => Some(*r),
            _ => None,
        })
        .unwrap();
    a.handle_action(view.action_at(rect.x, rect.y).unwrap());
    a.changed(&before);
    assert_eq!(a.settings.effect, Effect::Breathing);
    assert!(a.is_pending());
}
#[test]
fn tabs_and_keyboard_fit_standard_terminal() {
    let mut a = app();
    let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, 24)).unwrap();
    for tab in Tab::ALL {
        a.select_tab(tab);
        let mut view = AppUi::new();
        t.draw(|f| view.draw(f, &a)).unwrap();
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

#[test]
fn palette_grid_scales_and_clicks_apply_the_displayed_color_to_the_selected_zone() {
    for (width, height) in [(76, 24), (120, 36)] {
        let mut app = app();
        app.zone = 2;
        let original = app.settings.zones;
        let mut ui = AppUi::new();
        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| ui.draw(frame, &app)).unwrap();
        let swatches: Vec<_> = ui
            .hits
            .iter()
            .filter_map(|(rect, target)| match target {
                HitTarget::Preset(color) => Some((*rect, *color)),
                _ => None,
            })
            .collect();
        assert!(swatches.len() >= 40);
        assert!(
            swatches
                .iter()
                .all(|(r, _)| r.right() <= width && r.bottom() <= height)
        );
        let (rect, color) = swatches.last().unwrap();
        assert_eq!(*color, Rgb(255, 255, 255));
        app.handle_action(ui.action_at(rect.x, rect.y).unwrap());
        assert_eq!(app.settings.zones[1], *color);
        assert_eq!(app.settings.zones[0], original[0]);
        assert!(app.is_pending());
    }
}
