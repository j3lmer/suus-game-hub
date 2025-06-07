use ratatui::text::{Line, Text};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::games::snake::{Direction, Position, SnakeGame};

pub fn render_snake_game(frame: &mut Frame, area: Rect, game: &SnakeGame) {
    let block = Block::default().title(" Snake ").borders(Borders::ALL);
    frame.render_widget(block, area);

    let snake = &game.snake;
    let food = &game.food;

    // Convert positions to string-based drawing
    let mut buffer = vec![vec![' '; area.width as usize]; area.height as usize];

    for Position { x, y } in snake {
        if let Some(row) = buffer.get_mut((*y % area.height) as usize) {
            if let Some(cell) = row.get_mut((*x % area.width) as usize) {
                *cell = '•'; // or 'O', or whatever char you prefer
            }
        }
    }

    if let Some(row) = buffer.get_mut((food.y % area.height) as usize) {
        if let Some(cell) = row.get_mut((food.x % area.width) as usize) {
            *cell = 'X'; // food symbol
        }
    }

    let lines = buffer
        .into_iter()
        .map(|row| {
            Line::from(Span::styled(
                row.into_iter().collect::<String>(),
                Style::default().fg(Color::Green),
            ))
        })
        .collect::<Vec<Line>>();

    let paragraph = Paragraph::new(Text::from(lines)).block(Block::default());
    frame.render_widget(paragraph, area);
}
