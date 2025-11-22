pub mod hangman;
pub mod snake;
pub mod twozerofoureight;
pub mod adventure;


use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};
// use twozerofoureight::Game2048;

#[derive(Clone)]
pub enum GameType {
    Hangman,
    // Snake,
    // Game2048,
    Adventure,
}

impl GameType {
    pub fn name(&self) -> &str {
        match self {
            GameType::Hangman => "Galgje",
            // GameType::Snake => "Snake",
            // GameType::Game2048 => "2048",
            GameType::Adventure => "Pipis avontuurtje!",
        }
    }
}

// Game trait remains the same
pub trait Game {
    fn handle_input(&mut self, key: KeyCode);
    fn render(&self, frame: &mut Frame, area: Rect);
    fn restart(&mut self);
    fn tick(&mut self) {}
}
