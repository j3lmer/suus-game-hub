use crate::games::Game;
use crate::ui::snake_ui;
use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct SnakeGame {
    pub snake: Vec<Position>,
    direction: Direction,
    pub food: Position,
    area: Option<Rect>,
}

impl SnakeGame {
    pub fn new() -> Self {
        let snake = vec![
            Position { x: 5, y: 5 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 5 },
        ];
        let food = Position { x: 10, y: 10 };

        Self {
            snake,
            direction: Direction::Right,
            food,
            area: None,
        }
    }

    pub fn start_new_game(&mut self) {
        self.snake = vec![
            Position { x: 5, y: 5 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 5 },
        ];
        self.direction = Direction::Right;
        self.food = Position { x: 10, y: 10 };
    }

    fn update(&mut self) {
        let mut new_head = self.snake[0].clone();

        match self.direction {
            Direction::Up => {
                if new_head.y > 0 {
                    new_head.y -= 1;
                }
            }
            Direction::Down => {
                new_head.y += 1;
            }
            Direction::Left => {
                if new_head.x > 0 {
                    new_head.x -= 1;
                }
            }
            Direction::Right => {
                new_head.x += 1;
            }
        }

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.snake.insert(0, new_head);
            self.food = Position {
                x: (new_head.x + 5) % 50,
                y: (new_head.y + 3) % 20,
            }; // basic respawn logic
        } else {
            self.snake.insert(0, new_head);
            self.snake.pop();
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        if (self.direction == Direction::Up && dir != Direction::Down)
            || (self.direction == Direction::Down && dir != Direction::Up)
            || (self.direction == Direction::Left && dir != Direction::Right)
            || (self.direction == Direction::Right && dir != Direction::Left)
        {
            self.direction = dir;
        }
    }

    pub fn snake(&self) -> &Vec<Position> {
        &self.snake
    }

    pub fn food(&self) -> &Position {
        &self.food
    }
}

impl Game for SnakeGame {
    fn render(&self, frame: &mut Frame, area: Rect) {
        // Save area so game logic can reference size if needed

        snake_ui::render_snake_game(frame, area, self);
    }

    fn restart(&mut self) {
        self.start_new_game();
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.change_direction(Direction::Up),
            KeyCode::Down => self.change_direction(Direction::Down),
            KeyCode::Left => self.change_direction(Direction::Left),
            KeyCode::Right => self.change_direction(Direction::Right),
            KeyCode::Char(' ') => self.update(), // Advance manually for now
            _ => {}
        }
    }

    fn tick(&mut self) {
        self.update(); // or whatever method advances the snake
    }
}
