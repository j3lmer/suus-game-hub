use ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::games::snake::SnakeGame;

pub fn ui(f: &mut Frame, game: &SnakeGame) {
    let board_width = game.board.width * 2 + 2;
    let board_height = game.board.height + 2;

    let container_height = 1 + board_height + 1;
    let container_area = centered_fixed_rect(board_width, container_height, f.area());

    let main_chunks = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(board_height),
            Constraint::Length(1),
        ])
        .split(container_area);

    let hud_area = main_chunks[0];
    let board_area = main_chunks[1];
    let controls_area = main_chunks[2];

    let hud_layout = Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(hud_area);

    let score_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    let best_style = Style::default().fg(Color::Yellow);

    f.render_widget(
        Paragraph::new(format!("pipis score: {}", game.score)).style(score_style),
        hud_layout[0],
    );
    f.render_widget(
        Paragraph::new(format!("pipis beste score: {}", game.high_score.best_score))
            .style(best_style)
            .alignment(ratatui::layout::Alignment::Right),
        hud_layout[1],
    );

    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, board_area);

    let inner_area = board_area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    let body = game.snake.body();
    for (i, pos) in body.iter().enumerate() {
        let screen_x = inner_area.x + (pos.x * 2);
        let screen_y = inner_area.y + pos.y;

        if screen_x < inner_area.right() && screen_y < inner_area.bottom() {
            let area = Rect::new(screen_x, screen_y, 2, 1);

            let style = if i == 0 {
                Style::default().bg(Color::LightGreen)
            } else {
                Style::default().bg(Color::Green)
            };

            let block = Block::default().style(style);
            f.render_widget(block, area);
        }
    }

    let food_pos = game.food.position();
    let screen_x = inner_area.x + (food_pos.x * 2);
    let screen_y = inner_area.y + food_pos.y;

    if screen_x < inner_area.right() && screen_y < inner_area.bottom() {
        let area = Rect::new(screen_x, screen_y, 2, 1);
        let block = Block::default().style(Style::default().bg(Color::Red));
        f.render_widget(block, area);
    }

    if game.game_over {
        let area = centered_fixed_rect(40, 9, f.area());

        f.render_widget(Clear, area);

        let block = Block::default()
            .title("Game Over")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        let msg = format!(
            "Score: {}\nBest: {}\n\n[R] Restart\n[Q] Quit",
            game.score, game.high_score.best_score
        );
        let paragraph = Paragraph::new(msg)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::White));

        f.render_widget(paragraph, area);
    }

    if game.paused {
        let area = centered_fixed_rect(20, 3, f.area());
        f.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray).fg(Color::Yellow));

        let paragraph = Paragraph::new("PAUSED")
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }

    let hint_style = Style::default().fg(Color::Gray);
    f.render_widget(
        Paragraph::new("[Space] Pauze   [R] opnieuw   [Esc] stoppen")
            .style(hint_style)
            .alignment(ratatui::layout::Alignment::Center),
        controls_area,
    );
}

fn centered_fixed_rect(width: u16, height: u16, area: Rect) -> Rect {
    let empty_x = area.width.saturating_sub(width) / 2;
    let empty_y = area.height.saturating_sub(height) / 2;

    Rect::new(
        area.x + empty_x,
        area.y + empty_y,
        width.min(area.width),
        height.min(area.height),
    )
}
