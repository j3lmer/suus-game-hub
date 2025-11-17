pub mod dashboard;
pub mod hangman_ui;
pub mod snake_ui;
pub mod adventure_ui;

use crate::hub::{GameHub, Screen};
use ratatui::Frame;

pub fn render_ui(frame: &mut Frame, hub: &GameHub) {
    match hub.current_screen {
        Screen::Dashboard => dashboard::render_dashboard(frame, hub),
        Screen::Game => {
            if let Some(game) = &hub.current_game {
                game.render(frame, frame.area());
            }
        }
    }
}
