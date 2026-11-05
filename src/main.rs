mod config;
mod device;
mod keyboard;
mod model;
mod ui;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseButton, MouseEventKind,
    },
    execute,
};
use model::Rgb;
use std::{
    io::{self, IsTerminal},
    time::Duration,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("acer-predator-rgb: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => {}
        [arg] if arg == "--help" || arg == "-h" => {
            println!(
                "Acer RGB Studio\n\nRun without arguments for the TUI.\n--status   Read driver status (no lighting changes)\n--dry-run  Show commands for saved/imported settings\n--apply    Apply saved/imported settings explicitly\n\nUses the installed predator_kbd_rgb driver at /dev/predator-kbd.\nNo modules, services, or firmware are installed or changed automatically."
            );
            return Ok(());
        }
        [arg] if arg == "--status" => {
            println!(
                "{}",
                device::status().map_err(|e| e.to_string())?.describe()
            );
            return Ok(());
        }
        [arg] if arg == "--dry-run" || arg == "--apply" => {
            let (settings, source) = config::load()?;
            println!("{source}");
            if arg == "--dry-run" {
                for c in settings.commands()? {
                    println!("{c}");
                }
            } else {
                println!(
                    "{}",
                    device::apply(&settings, &mut device::Device)
                        .map_err(|e| e.to_string())?
                        .describe()
                );
            }
            return Ok(());
        }
        _ => return Err("Unknown arguments. Use --help.".into()),
    }
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("Run the TUI in an interactive terminal, or use --status / --dry-run.".into());
    }
    let mut app = ui::App::new();
    let mut terminal = ratatui::try_init().map_err(|e| e.to_string())?;
    let result = (|| -> io::Result<()> {
        execute!(io::stdout(), EnableMouseCapture)?;
        loop {
            app.tick();
            terminal.draw(|f| ui::draw(f, &mut app))?;
            if !event::poll(Duration::from_millis(30))? {
                continue;
            }
            let event = event::read()?;
            let before = app.settings.clone();
            if let Event::Mouse(mouse) = event {
                if app.hex_input.is_none() {
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.click(mouse.column, mouse.row)
                        }
                        MouseEventKind::ScrollUp => app.adjust(1),
                        MouseEventKind::ScrollDown => app.adjust(-1),
                        _ => {}
                    }
                    app.changed(&before);
                }
                continue;
            }
            let Event::Key(key) = event else {
                continue;
            };
            if key.kind == KeyEventKind::Release {
                continue;
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                break;
            }
            if let Some(input) = app.hex_input.as_mut() {
                match key.code {
                    KeyCode::Esc => app.hex_input = None,
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(c) if c.is_ascii_hexdigit() && input.len() < 6 => input.push(c),
                    KeyCode::Enter => match Rgb::try_from(input.clone()) {
                        Ok(color) => {
                            app.hex_input = None;
                            app.set_color(color);
                        }
                        Err(e) => {
                            app.status = e;
                            app.error = true;
                        }
                    },
                    _ => {}
                }
                if let Some(input) = app.hex_input.as_ref().filter(|s| s.len() == 6)
                    && let Ok(color) = Rgb::try_from(input.clone())
                {
                    app.hex_input = None;
                    app.set_color(color);
                }
                app.changed(&before);
                continue;
            }
            if let Some(zone) = zone_shortcut(key.code, key.modifiers) {
                app.select_zone(zone);
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::BackTab => app.select_tab(ui::Tab::ALL[(app.tab.index() + 3) % 4]),
                KeyCode::Tab => app.select_tab(ui::Tab::ALL[(app.tab.index() + 1) % 4]),
                KeyCode::Char(c @ '1'..='4') => {
                    app.select_tab(ui::Tab::ALL[c as usize - '1' as usize])
                }
                KeyCode::Up => app.move_selection(-1),
                KeyCode::Down => app.move_selection(1),
                KeyCode::Left => app.adjust(if key.modifiers.contains(KeyModifiers::SHIFT) {
                    -10
                } else {
                    -1
                }),
                KeyCode::Right => app.adjust(if key.modifiers.contains(KeyModifiers::SHIFT) {
                    10
                } else {
                    1
                }),
                KeyCode::Char('s') => app.save(),
                KeyCode::Char('l') => app.load(),
                KeyCode::Char('r') => {
                    app.refresh();
                    if !app.error {
                        app.status = app.connection.clone();
                    }
                }
                KeyCode::Char('h') => {
                    app.select_tab(ui::Tab::Color);
                    app.hex_input = Some(String::new());
                }
                KeyCode::Char('R') => app.set_color(Rgb(255, 0, 0)),
                KeyCode::Char('G' | 'g') => app.set_color(Rgb(0, 255, 0)),
                KeyCode::Char('B' | 'b') => app.set_color(Rgb(0, 80, 255)),
                KeyCode::Char('P' | 'p') => app.set_color(Rgb(160, 40, 255)),
                KeyCode::Char('W' | 'w') => app.set_color(Rgb(255, 255, 255)),
                KeyCode::Char('O' | 'o') => app.set_color(Rgb(255, 140, 0)),
                KeyCode::Char('C' | 'c') => app.set_color(Rgb(0, 255, 255)),
                _ => {}
            }
            app.changed(&before);
        }
        Ok(())
    })();
    let _ = execute!(io::stdout(), DisableMouseCapture);
    ratatui::restore();
    result.map_err(|e| e.to_string())
}

fn zone_shortcut(code: KeyCode, modifiers: KeyModifiers) -> Option<usize> {
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
mod shortcut_tests {
    use super::*;
    #[test]
    fn recognizes_shift_digits_and_terminal_symbols() {
        for (i, (digit, symbol)) in [('1', '!'), ('2', '@'), ('3', '#'), ('4', '$')]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                zone_shortcut(KeyCode::Char(digit), KeyModifiers::SHIFT),
                Some(i + 1)
            );
            assert_eq!(
                zone_shortcut(KeyCode::Char(symbol), KeyModifiers::NONE),
                Some(i + 1)
            );
            assert_eq!(
                zone_shortcut(KeyCode::Char(digit), KeyModifiers::NONE),
                None
            );
            assert_eq!(
                zone_shortcut(KeyCode::Char(symbol), KeyModifiers::ALT),
                None
            );
        }
    }
}
