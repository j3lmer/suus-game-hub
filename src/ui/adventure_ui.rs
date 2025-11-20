use crate::games::adventure::Adventure;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_adventure_game(game: &Adventure, frame: &mut Frame, area: Rect) {
    // Split whole area into: [top big area, bottom input]
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    // Split top into left(log) and right(info panel)
    let top_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[0]);

    // === LEFT PANEL (Log Window) ===
    let log_text = game
        .log()
        .iter()
        .flat_map(|entry| {
            entry
                .lines() // split on newline
                .map(|line| Line::raw(line.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let log_widget = Paragraph::new(log_text)
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .wrap(Wrap { trim: false });

    frame.render_widget(log_widget, top_split[0]);

    // === RIGHT PANEL ===
    let right_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(5),
        ])
        .split(top_split[1]);

    let inventory_items = game
        .inventory()
        .iter()
        .map(|item| Line::raw(item.to_lowercase()))
        .collect::<Vec<_>>();

    let inventory_widget = Paragraph::new(inventory_items)
        .block(Block::default().borders(Borders::ALL).title("Inventory"))
        .wrap(Wrap { trim: false });

    frame.render_widget(inventory_widget, right_split[0]);

    let scene_text = Paragraph::new(game.current_scene_art())
        .block(Block::default().borders(Borders::ALL).title("Scene"))
        .wrap(Wrap { trim: true });

    frame.render_widget(scene_text, right_split[1]);

    let stats_lines = vec![Line::raw(format!(
        "Dingen gedaan: {}",
        game.stats().moves_done
    ))];

    let stats_widget =
        Paragraph::new(stats_lines).block(Block::default().borders(Borders::ALL).title("Stats"));

    frame.render_widget(stats_widget, right_split[2]);

    // === Bottom Input Line with autocomplete ===
    let input_widget = render_input_line(game);
    frame.render_widget(input_widget, main_layout[1]);
}

fn render_input_line(game: &Adventure) -> Paragraph<'_> {
    let input = game.input();

    let spans: Vec<Span> = if let Some(suggestion) = game.autocomplete_suggestion() {
        if suggestion.starts_with(input) {
            let typed_len = input.len();
            let typed_part = &suggestion[..typed_len];
            let suggested_part = &suggestion[typed_len..];

            vec![
                Span::raw(typed_part),
                Span::styled(suggested_part, Style::default().fg(Color::DarkGray)),
            ]
        } else {
            vec![Span::raw(input.to_string())]
        }
    } else {
        vec![Span::raw(input.to_string())]
    };

    Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title("Command"))
        .wrap(Wrap { trim: false })
}
