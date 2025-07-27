use crate::games::Game;
use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

unsafe extern "C" {
    fn game2048_init();
    fn game2048_tick();
    fn game2048_handle_input(key: i32);
    fn game2048_render();
    fn game2048_restart();
}

pub struct Game2048;

impl Game2048 {
    pub fn new() -> Self {
        unsafe {
            game2048_init();
        }
        Self
    }
}

impl Game for Game2048 {
    fn handle_input(&mut self, key: KeyCode) {
        let code = match key {
            KeyCode::Char('w') => 0,
            KeyCode::Char('s') => 1,
            KeyCode::Char('a') => 2,
            KeyCode::Char('d') => 3,
            _ => -1,
        };
        if code != -1 {
            unsafe {
                game2048_handle_input(code);
            }
        }
    }

    fn render(&self, _frame: &mut Frame, _area: Rect) {
        unsafe {
            game2048_render();
        }
        // 2048.c draws directly to stdout, so no Ratatui drawing here
    }

    fn restart(&mut self) {
        unsafe {
            game2048_restart();
        }
    }

    fn tick(&mut self) {
        unsafe {
            game2048_tick();
        }
    }
}
