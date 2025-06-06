pub mod hangman;

use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

#[derive(Clone)]
pub enum GameType {
    Hangman,
    // Add more game types here
    // TicTacToe,
    // Snake,
}

impl GameType {
    pub fn name(&self) -> &str {
        match self {
            GameType::Hangman => "Hangman",
            // GameType::TicTacToe => "Tic Tac Toe",
            // GameType::Snake => "Snake",
        }
    }
}

pub trait Game {
    fn handle_input(&mut self, key: KeyCode);
    fn render(&self, frame: &mut Frame, area: Rect);
    fn restart(&mut self);
    fn is_finished(&self) -> bool;
}
