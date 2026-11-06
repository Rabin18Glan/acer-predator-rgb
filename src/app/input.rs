use super::{App, Tab, shortcuts::zone_shortcut};
use crate::model::Rgb;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub(crate) enum InputOutcome {
    Continue,
    Quit,
}

#[cfg(test)]
#[path = "../../tests/unit/input.rs"]
mod tests;

impl App {
    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> InputOutcome {
        if key.kind == KeyEventKind::Release {
            return InputOutcome::Continue;
        }
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return InputOutcome::Quit;
        }
        let before = self.settings.clone();
        if self.hex_input.is_some() {
            self.handle_hex(key.code);
        } else if let Some(zone) = zone_shortcut(key.code, key.modifiers) {
            self.select_zone(zone);
        } else {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return InputOutcome::Quit,
                KeyCode::BackTab => self.select_tab(Tab::ALL[(self.tab.index() + 3) % 4]),
                KeyCode::Tab => self.select_tab(Tab::ALL[(self.tab.index() + 1) % 4]),
                KeyCode::Char(c @ '1'..='4') => {
                    self.select_tab(Tab::ALL[c as usize - '1' as usize])
                }
                KeyCode::Up => self.move_selection(-1),
                KeyCode::Down => self.move_selection(1),
                KeyCode::Left => self.adjust(if key.modifiers.contains(KeyModifiers::SHIFT) {
                    -10
                } else {
                    -1
                }),
                KeyCode::Right => self.adjust(if key.modifiers.contains(KeyModifiers::SHIFT) {
                    10
                } else {
                    1
                }),
                KeyCode::Char('s') => self.save(),
                KeyCode::Char('l') => self.load(),
                KeyCode::Char('r') => {
                    self.refresh();
                    if !self.error {
                        self.status = self.connection.clone();
                    }
                }
                KeyCode::Char('h') => {
                    self.select_tab(Tab::Color);
                    self.hex_input = Some(String::new());
                }
                KeyCode::Char('R') => self.set_color(Rgb(255, 0, 0)),
                KeyCode::Char('G' | 'g') => self.set_color(Rgb(0, 255, 0)),
                KeyCode::Char('B' | 'b') => self.set_color(Rgb(0, 80, 255)),
                KeyCode::Char('P' | 'p') => self.set_color(Rgb(160, 40, 255)),
                KeyCode::Char('W' | 'w') => self.set_color(Rgb(255, 255, 255)),
                KeyCode::Char('O' | 'o') => self.set_color(Rgb(255, 140, 0)),
                KeyCode::Char('C' | 'c') => self.set_color(Rgb(0, 255, 255)),
                _ => {}
            }
        }
        self.changed(&before);
        InputOutcome::Continue
    }

    fn handle_hex(&mut self, code: KeyCode) {
        let Some(input) = self.hex_input.as_mut() else {
            return;
        };
        match code {
            KeyCode::Esc => self.hex_input = None,
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Char(c) if c.is_ascii_hexdigit() && input.len() < 6 => input.push(c),
            KeyCode::Enter => match Rgb::try_from(input.clone()) {
                Ok(color) => {
                    self.hex_input = None;
                    self.set_color(color);
                }
                Err(e) => {
                    self.status = e;
                    self.error = true;
                }
            },
            _ => {}
        }
        if let Some(input) = self.hex_input.as_ref().filter(|s| s.len() == 6)
            && let Ok(color) = Rgb::try_from(input.clone())
        {
            self.hex_input = None;
            self.set_color(color);
        }
    }
}
