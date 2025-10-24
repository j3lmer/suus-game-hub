use crate::games::{
    Game, GameType, hangman::HangmanGame, snake::SnakeGame, twozerofoureight::Game2048,
};
use ratatui::crossterm::event::KeyCode;

#[derive(PartialEq)]
pub enum Screen {
    Dashboard,
    Game,
}

pub enum MenuDirection {
    Left,
    Right,
}

pub struct GameHub {
    pub current_screen: Screen,
    pub selected_game_index: usize,
    pub current_game: Option<Box<dyn Game>>,
    pub available_games: Vec<GameType>,
}

impl GameHub {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Dashboard,
            selected_game_index: 0,
            current_game: None,
            available_games: vec![GameType::Hangman, GameType::Snake, GameType::Game2048],
        }
    }

    pub fn update(&mut self) {
        if let Some(game) = &mut self.current_game {
            game.tick();
        }
    }

    // Update the create_game method:
    fn create_game(&self, game_type: GameType) -> Box<dyn Game> {
        match game_type {
            GameType::Hangman => {
                let mut game = Box::new(HangmanGame::new());
                game.restart();
                game
            }

            GameType::Snake => {
                let mut game = Box::new(SnakeGame::new());
                game.restart();
                game
            }
            GameType::Game2048 => {
                let mut game = Box::new(Game2048::new());
                game.restart();
                game
            }
        }
    }
    pub fn handle_input(&mut self, key: KeyCode) {
        match self.current_screen {
            Screen::Dashboard => self.handle_dashboard_input(key),
            Screen::Game => self.handle_game_input(key),
        }
    }

    fn handle_dashboard_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Right | KeyCode::Char('l') => {
                self.cycle_game_selection(MenuDirection::Right);
            }

            KeyCode::Left | KeyCode::Char('h') => {
                self.cycle_game_selection(MenuDirection::Left);
            }

            KeyCode::Enter => {
                self.start_selected_game();
            }
            _ => {}
        }
    }

    fn handle_game_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Backspace => {
                // Return to dashboard
                self.current_screen = Screen::Dashboard;
                self.current_game = None;
            }
            KeyCode::F(5) => {
                // Restart current game
                if let Some(game) = &mut self.current_game {
                    game.restart();
                }
            }
            _ => {
                // Pass input to the current game
                if let Some(game) = &mut self.current_game {
                    game.handle_input(key);
                }
            }
        }
    }

    fn cycle_game_selection(&mut self, dir: MenuDirection) {
        if !self.available_games.is_empty() {
            match dir {
                MenuDirection::Left => {
                    if self.selected_game_index == 0 {
                        return;
                    }

                    self.selected_game_index = self.selected_game_index - 1;
                }
                MenuDirection::Right => {
                    self.selected_game_index =
                        (self.selected_game_index + 1) % self.available_games.len();
                }
                _ => {}
            }
        }
    }

    fn start_selected_game(&mut self) {
        if let Some(game_type) = self.available_games.get(self.selected_game_index) {
            self.current_game = Some(self.create_game(game_type.clone()));
            self.current_screen = Screen::Game;
        }
    }

    pub fn get_selected_game_name(&self) -> &str {
        if let Some(game_type) = self.available_games.get(self.selected_game_index) {
            game_type.name()
        } else {
            "No Game"
        }
    }

    pub fn get_all_game_names(&self) -> Vec<&str> {
        self.available_games.iter().map(|g| g.name()).collect()
    }
}
