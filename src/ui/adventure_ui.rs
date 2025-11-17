use crate::games::adventure::Adventure;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_adventure_game(game: &Adventure, frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(area);

    // --- Log Window ---
    let log_text = game
        .log()
        .iter()
        .map(|l| Line::raw(l.clone()))
        .collect::<Vec<_>>();

    let log_widget = Paragraph::new(log_text)
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .wrap(Wrap { trim: false });

    frame.render_widget(log_widget, layout[0]);

    // --- Input Line ---
    let input_widget = Paragraph::new(game.input().to_string())
        .block(Block::default().borders(Borders::ALL).title("Command"))
        .wrap(Wrap { trim: false });

    frame.render_widget(input_widget, layout[1]);
}
