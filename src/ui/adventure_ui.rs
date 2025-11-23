use crate::games::adventure::Adventure;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui_image::StatefulImage;

pub fn render_adventure_game(game: &Adventure, frame: &mut Frame, area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let top_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[0]);

    // === LEFT PANEL (Log Window) with scrolling ===
    let log_text = game
        .log()
        .iter()
        .flat_map(|entry| {
            entry
                .lines()
                .map(|line| Line::raw(line.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut log_with_padding = log_text;
    log_with_padding.push(Line::raw(""));
    log_with_padding.push(Line::raw(""));

    let log_height = top_split[0].height.saturating_sub(2) as usize;
    let total_lines = log_with_padding.len();

    let scroll_offset = if game.auto_scroll && total_lines > log_height {
        total_lines - log_height
    } else if game.auto_scroll {
        0
    } else {
        let max_scroll = total_lines.saturating_sub(log_height);
        (game.log_scroll as usize).min(max_scroll)
    };

    let log_widget = Paragraph::new(log_with_padding)
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset as u16, 0));

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

    // === SCENE DISPLAY - Image or Fallback Text ===
    let scene = game.current_scene();

    if let Some(ref protocol_cell) = scene.scene_image {
        let mut protocol = protocol_cell.borrow_mut();
        let image = StatefulImage::new(None);
        frame.render_stateful_widget(image, right_split[1], &mut *protocol);
    } else {
        let scene_text = Paragraph::new(scene.scene_art.clone())
            .block(Block::default().borders(Borders::ALL).title("Scene"))
            .wrap(Wrap { trim: true });

        frame.render_widget(scene_text, right_split[1]);
    }

    let stats_lines = vec![Line::raw(format!(
        "Dingen gedaan: {}",
        game.stats().moves_done
    ))];

    let stats_widget =
        Paragraph::new(stats_lines)
        .block(Block::default()
        .borders(Borders::ALL)
        .title("Stats"));

    frame.render_widget(stats_widget, right_split[2]);

    // === Bottom Input Line with autocomplete ===
    let input_widget = render_input_line(game);
    frame.render_widget(input_widget, main_layout[1]);
}

fn render_input_line(game: &Adventure) -> Paragraph {
    let input = game.input().to_string();
    let spans: Vec<Span> = if let Some(suggestion) = game.autocomplete_suggestion() {
        if suggestion.starts_with(&input) {
            let typed_len = input.len();
            let typed_part = &suggestion[..typed_len];
            let suggested_part = &suggestion[typed_len..];
            vec![
                Span::raw(typed_part.to_string()),
                Span::styled(
                    suggested_part.to_string(),
                    Style::default().fg(Color::DarkGray),
                ),
            ]
        } else {
            vec![Span::raw(input)]
        }
    } else {
        vec![Span::raw(input)]
    };

    Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title("Command"))
        .wrap(Wrap { trim: false })
}
