use crate::event::KeyCode;
use crate::event::KeyEventKind;
use app::App;
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
use ui::ui;

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Running
    let mut app = App::new();
    app.start_new_game();
    run_app(&mut terminal, &mut app)?;

    //Nazorg
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool, io::Error> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // handle events
        if let Event::Key(key) = read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            if KeyEventKind::Press == key.kind {
                match key.code {
                    KeyCode::Esc => return Ok(true),
                    KeyCode::F(5) => app.start_new_game(),
                    _ => {
                        app.handle_user_press(key.code);
                    }
                }
            }
        }
    }
}
