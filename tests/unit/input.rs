use super::*;
use crate::model::Settings;

fn app() -> App {
    App::from_settings(Settings::default(), "Ready".into())
}

fn press(app: &mut App, code: KeyCode) -> InputOutcome {
    app.handle_key(KeyEvent::new(code, KeyModifiers::NONE))
}

#[test]
fn zone_shortcut_changes_navigation_without_scheduling_writes() {
    let mut app = app();
    app.select_tab(Tab::Effect);
    press(&mut app, KeyCode::Char('@'));
    assert_eq!(app.tab, Tab::Color);
    assert_eq!(app.zone, 2);
    assert!(!app.is_pending());
}

#[test]
fn complete_hex_input_updates_only_the_selected_zone_without_enter() {
    let mut app = app();
    app.select_zone(3);
    let original = app.settings.zones;
    press(&mut app, KeyCode::Char('h'));
    for digit in "0066f".chars() {
        press(&mut app, KeyCode::Char(digit));
        assert!(!app.is_pending());
        assert_eq!(app.settings.zones, original);
    }
    press(&mut app, KeyCode::Char('f'));
    assert!(app.hex_input.is_none());
    assert!(app.is_pending());
    assert_eq!(app.settings.zones[2], Rgb(0, 102, 255));
    assert_eq!(app.settings.zones[0], original[0]);
}

#[test]
fn control_c_exits_even_while_editing_hex() {
    let mut app = app();
    press(&mut app, KeyCode::Char('h'));
    assert!(matches!(
        app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        InputOutcome::Quit
    ));
    assert!(!app.is_pending());
}

#[test]
fn release_events_do_not_apply_a_preset_twice() {
    let mut app = app();
    let before = app.settings.clone();
    let mut key = KeyEvent::new(KeyCode::Char('R'), KeyModifiers::NONE);
    key.kind = KeyEventKind::Release;
    app.handle_key(key);
    assert_eq!(app.settings, before);
    assert!(!app.is_pending());
}
