use crate::games::adventure::Adventure;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

// TODO: de main log view moet scrollable worden
pub fn render_adventure_game(game: &Adventure, frame: &mut Frame, area: Rect) {
    // Split whole area into: [top big area, bottom input]
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),   // top panel
            Constraint::Length(3) // command input
        ])
        .split(area);

    // --- Split top into left(log) and right(info panel) ---
    let top_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // left
            Constraint::Percentage(40), // right
        ])
        .split(main_layout[0]);

    // === LEFT PANEL (Log Window) ===
    let log_text = game
        .log()
        .iter()
        .map(|l| Line::raw(l.clone()))
        .collect::<Vec<_>>();

    let log_widget = Paragraph::new(log_text)
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .wrap(Wrap { trim: false });

    frame.render_widget(log_widget, top_split[0]);

    // === RIGHT PANEL ===
    // Right side is split vertically:
    // [inventory small row, scene preview large, stats]
    let right_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // inventory row
            Constraint::Min(5),      // scene preview
            Constraint::Length(5),   // stats box
        ])
        .split(top_split[1]);

    // --- Inventory ---
    let inventory_items = game.inventory().iter()
        .map(|item| Line::raw(item.to_lowercase()))  // assuming .emoji() returns "🗡️"
        .collect::<Vec<_>>();

    let inventory_widget = Paragraph::new(inventory_items)
        .block(Block::default().borders(Borders::ALL).title("Inventory"))
        .wrap(Wrap { trim: false });

    frame.render_widget(inventory_widget, right_split[0]);

    // --- Scene Preview ---
    let scene_text = Paragraph::new(game.current_scene_art())
        .block(Block::default().borders(Borders::ALL).title("Scene"))
        .wrap(Wrap { trim: true });

    frame.render_widget(scene_text, right_split[1]);

    // --- Stats Box ---
    let stats_lines = vec![
        Line::raw(format!("Dingen gedaan: {}", game.stats().moves_done)),
    ];

    let stats_widget = Paragraph::new(stats_lines)
        .block(Block::default().borders(Borders::ALL).title("Stats"));

    frame.render_widget(stats_widget, right_split[2]);

    // === Bottom: Input Line ===
    let input_widget = Paragraph::new(game.input().to_string())
        .block(Block::default().borders(Borders::ALL).title("Command"))
        .wrap(Wrap { trim: false });

    frame.render_widget(input_widget, main_layout[1]);
}
