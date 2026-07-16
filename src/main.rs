use crate::event::KeyCode;
// use crate::event::KeyEventKind;
use hub::GameHub;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use ui::render_ui;

use std::sync::mpsc;
use std::thread;

mod games;
mod hub;
mod ui;
mod utils;

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
    let tick_rate = Duration::from_millis(150);

    // Spawn a dedicated input-reading thread. It blocks on real terminal
    // events and forwards them; it never controls game timing.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            // This can block indefinitely — that's fine, it's not our clock.
            if let Ok(event) = event::read() {
                if tx.send(event).is_err() {
                    break; // main thread went away
                }
            }
        }
    });

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| render_ui(f, hub))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        // recv_timeout is a real OS timer wait, independent of terminal input.
        match rx.recv_timeout(timeout) {
            Ok(Event::Key(key)) => {
                if key.kind != KeyEventKind::Release {
                    match key.code {
                        KeyCode::Esc => return Ok(true),
                        _ => hub.handle_input(key.code),
                    }
                }
            }
            Ok(_) => {} // ignore mouse/resize/etc.
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(true),
        }

        if last_tick.elapsed() >= tick_rate {
            hub.update();
            last_tick = Instant::now();
        }
    }
}

// fn run_app<B: Backend>(terminal: &mut Terminal<B>, hub: &mut GameHub) -> Result<bool, io::Error> {
//     let tick_rate = Duration::from_millis(100);
//     let mut last_tick = Instant::now();
//
//     loop {
//         terminal.draw(|f| render_ui(f, hub))?;
//
//         let timeout = tick_rate
//             .checked_sub(last_tick.elapsed())
//             .unwrap_or_else(|| Duration::from_secs(0));
//
//         if crossterm::event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Release {
//                     continue;
//                 }
//
//                 match key.code {
//                     KeyCode::Esc => return Ok(true),
//                     _ => hub.handle_input(key.code),
//                 }
//             }
//         }
//
//         if last_tick.elapsed() >= tick_rate {
//             hub.update();
//             last_tick = Instant::now();
//         }
//     }
// }
