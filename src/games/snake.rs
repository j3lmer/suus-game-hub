use crate::games::Game;
use ratatui::crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

const HIGH_SCORE_FILE: &str = "snake_highscore.json";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// Playing-field dimensions, in cells (not terminal columns/rows —
/// the UI multiplies x by 2 to draw each cell as a 2-wide block).
pub struct Board {
    pub width: u16,
    pub height: u16,
}

pub struct Snake {
    body: VecDeque<Position>,
}

impl Snake {
    pub fn body(&self) -> &VecDeque<Position> {
        &self.body
    }
}

pub struct Food {
    pos: Position,
}

impl Food {
    pub fn position(&self) -> Position {
        self.pos
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct HighScore {
    pub best_score: u32,
}

impl HighScore {
    pub fn load() -> Self {
        if Path::new(HIGH_SCORE_FILE).exists() {
            if let Ok(contents) = fs::read_to_string(HIGH_SCORE_FILE) {
                if let Ok(score) = serde_json::from_str::<HighScore>(&contents) {
                    return score;
                }
            }
        }

        HighScore::default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(HIGH_SCORE_FILE, json);
        }
    }

    pub fn update(&mut self, score: u32) {
        if score > self.best_score {
            self.best_score = score;
            self.save();
        }
    }
}

pub struct SnakeGame {
    pub board: Board,
    pub snake: Snake,
    pub food: Food,
    pub score: u32,
    pub high_score: HighScore,
    pub game_over: bool,
    pub paused: bool,
    direction: Direction,
    direction_queue: VecDeque<Direction>,
}

impl SnakeGame {
    pub fn new() -> Self {
        Self::with_size(20, 15)
    }

    pub fn with_size(width: u16, height: u16) -> Self {
        let mut game = Self {
            board: Board { width, height },
            snake: Snake {
                body: VecDeque::new(),
            },
            food: Food {
                pos: Position { x: 0, y: 0 },
            },
            score: 0,
            high_score: HighScore::load(),
            game_over: false,
            paused: false,
            direction: Direction::Right,
            direction_queue: VecDeque::new(),
        };
        game.start_new_game();
        game
    }

    pub fn start_new_game(&mut self) {
        let start_x = (self.board.width / 2).max(2);
        let start_y = self.board.height / 2;

        let mut body = VecDeque::new();
        body.push_back(Position {
            x: start_x,
            y: start_y,
        });
        body.push_back(Position {
            x: start_x - 1,
            y: start_y,
        });
        body.push_back(Position {
            x: start_x - 2,
            y: start_y,
        });

        self.snake = Snake { body };
        self.direction = Direction::Right;
        self.direction_queue.clear();
        self.score = 0;
        self.game_over = false;
        self.paused = false;
        self.spawn_food();
    }

    fn spawn_food(&mut self) {
        // Simple deterministic-ish pseudo random spawn without pulling in
        // the `rand` crate: walk cells starting from a moving seed until
        // we find one that isn't occupied by the snake.
        let mut seed = (self.score as u32)
            .wrapping_add(
                self.snake
                    .body
                    .front()
                    .map_or(0, |p| p.x as u32 * 31 + p.y as u32 * 17),
            )
            .wrapping_add(7);

        loop {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let x = (seed / 65536) % self.board.width as u32;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let y = (seed / 65536) % self.board.height as u32;

            let pos = Position {
                x: x as u16,
                y: y as u16,
            };

            if !self.snake.body.contains(&pos) {
                self.food.pos = pos;
                break;
            }
        }
    }

    fn update(&mut self) {
        if let Some(next_dir) = self.direction_queue.pop_front() {
            self.direction = next_dir;
        }

        let head = *self.snake.body.front().expect("snake has no body");
        let mut new_head = head;

        match self.direction {
            Direction::Up => {
                if new_head.y == 0 {
                    self.end_game();
                    return;
                }
                new_head.y -= 1;
            }
            Direction::Down => new_head.y += 1,
            Direction::Left => {
                if new_head.x == 0 {
                    self.end_game();
                    return;
                }
                new_head.x -= 1;
            }
            Direction::Right => new_head.x += 1,
        }

        if new_head.x >= self.board.width || new_head.y >= self.board.height {
            self.end_game();
            return;
        }

        let will_grow = new_head.x == self.food.pos.x && new_head.y == self.food.pos.y;

        // The tail cell moves away this tick unless the snake is growing,
        // so it doesn't count as a collision.
        let hits_self = if will_grow {
            self.snake.body.contains(&new_head)
        } else {
            self.snake
                .body
                .iter()
                .take(self.snake.body.len().saturating_sub(1))
                .any(|p| *p == new_head)
        };

        if hits_self {
            self.end_game();
            return;
        }

        self.snake.body.push_front(new_head);

        if will_grow {
            self.score += 1;
            self.high_score.update(self.score);
            self.spawn_food();
        } else {
            self.snake.body.pop_back();
        }
    }

    fn end_game(&mut self) {
        self.game_over = true;
        self.high_score.update(self.score);
    }

    fn change_direction(&mut self, dir: Direction) {
        let last_direction = *self.direction_queue.back().unwrap_or(&self.direction);
        if dir != last_direction.opposite()
            && dir != last_direction
            && self.direction_queue.len() < 2
        {
            self.direction_queue.push_back(dir);
        }
    }
}

impl Game for SnakeGame {
    fn render(&self, frame: &mut Frame, _area: Rect) {
        crate::ui::snake_ui::ui(frame, self);
    }

    fn restart(&mut self) {
        self.start_new_game();
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('r') | KeyCode::Char('R') => self.restart(),
            KeyCode::Char(' ') if !self.game_over => self.paused = !self.paused,
            KeyCode::Up | KeyCode::Char('k') if !self.paused && !self.game_over => {
                self.change_direction(Direction::Up)
            }
            KeyCode::Down | KeyCode::Char('j') if !self.paused && !self.game_over => {
                self.change_direction(Direction::Down)
            }
            KeyCode::Left | KeyCode::Char('h') if !self.paused && !self.game_over => {
                self.change_direction(Direction::Left)
            }
            KeyCode::Right | KeyCode::Char('l') if !self.paused && !self.game_over => {
                self.change_direction(Direction::Right)
            }
            // KeyCode::Esc for quitting is expected to be handled one level
            // up, in whatever loop owns the list of Games.
            _ => {}
        }
    }

    fn tick(&mut self) {
        if !self.game_over && !self.paused {
            self.update();
        }
    }
}
