pub mod hangman;

use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

#[derive(Clone)]
pub enum GameType {
    Hangman,
}

impl GameType {
    pub fn name(&self) -> &str {
        match self {
            GameType::Hangman => "Galgje",
        }
    }
}

// Game trait remains the same
pub trait Game {
    fn handle_input(&mut self, key: KeyCode);
    fn render(&self, frame: &mut Frame, area: Rect);
    fn restart(&mut self);
    fn is_finished(&self) -> bool;
}
