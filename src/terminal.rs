//! Terminal lifecycle and event loop. Drop restores mouse capture and raw mode.
use crate::{
    app::{App, InputOutcome},
    ui::AppUi,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, MouseButton, MouseEventKind},
    execute,
};
use std::{
    io::{self, IsTerminal},
    time::Duration,
};

struct TerminalSession {
    terminal: ratatui::DefaultTerminal,
}

impl TerminalSession {
    fn open() -> io::Result<Self> {
        let terminal = ratatui::try_init()?;
        let session = Self { terminal };
        // If mouse setup fails, Drop still restores the terminal.
        execute!(io::stdout(), EnableMouseCapture)?;
        Ok(session)
    }

    fn run(&mut self, mut app: App) -> io::Result<()> {
        let mut view = AppUi::new();
        loop {
            app.tick();
            self.terminal.draw(|frame| view.draw(frame, &app))?;
            if !event::poll(Duration::from_millis(30))? {
                continue;
            }
            match event::read()? {
                Event::Key(key) => {
                    if matches!(app.handle_key(key), InputOutcome::Quit) {
                        break;
                    }
                }
                Event::Mouse(mouse) if app.hex_input.is_none() => match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        if let Some(action) = view.action_at(mouse.column, mouse.row) {
                            app.handle_action(action);
                        }
                    }
                    MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                        let before = app.settings.clone();
                        app.adjust(if mouse.kind == MouseEventKind::ScrollUp {
                            1
                        } else {
                            -1
                        });
                        app.changed(&before);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableMouseCapture);
        ratatui::restore();
    }
}

pub(crate) fn run() -> Result<(), String> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("Run the TUI in an interactive terminal, or use --status / --dry-run.".into());
    }
    let app = App::new();
    TerminalSession::open()
        .and_then(|mut session| session.run(app))
        .map_err(|e| e.to_string())
}
