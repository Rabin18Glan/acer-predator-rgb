use crossterm::event::{KeyCode, KeyModifiers};

pub(super) fn zone_shortcut(code: KeyCode, modifiers: KeyModifiers) -> Option<usize> {
    if modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER) {
        return None;
    }
    match code {
        // Most terminals report the resulting symbol rather than Shift+digit.
        KeyCode::Char('!') => Some(1),
        KeyCode::Char('@') => Some(2),
        KeyCode::Char('#') => Some(3),
        KeyCode::Char('$') => Some(4),
        KeyCode::Char(c @ '1'..='4') if modifiers.contains(KeyModifiers::SHIFT) => {
            Some(c as usize - '0' as usize)
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "../../tests/unit/shortcuts.rs"]
mod shortcut_tests;
