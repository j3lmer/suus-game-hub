use std::collections::LinkedList;

use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

use crate::games::Game;

pub struct SnakeGame {}

impl SnakeGame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_new_game(&mut self) {}
}

impl Game for SnakeGame {
    fn render(&self, frame: &mut Frame, area: Rect) {}

    fn restart(&mut self) {
        self.start_new_game();
    }

    fn handle_input(&mut self, key: KeyCode) {}
}

struct Snake {}
