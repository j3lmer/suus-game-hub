use crate::event::KeyCode;
use crate::event::KeyEventKind;
use hub::GameHub;
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, read};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::CrosstermBackend;
use std::error::Error;
use std::io;
use ui::render_ui;

mod games;
mod hub;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the game hub
    let mut game_hub = GameHub::new();

    run_app(&mut terminal, &mut game_hub)?;

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, hub: &mut GameHub) -> Result<bool, io::Error> {
    loop {
        terminal.draw(|f| render_ui(f, hub))?;

        // Handle events
        if let Event::Key(key) = read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            if KeyEventKind::Press == key.kind {
                match key.code {
                    KeyCode::Esc => return Ok(true),
                    _ => {
                        hub.handle_input(key.code);
                    }
                }
            }
        }
    }
}
