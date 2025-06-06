use crate::hub::GameHub;
use colorgrad::{Gradient, GradientBuilder};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use tui_big_text::{BigText, PixelSize};
use tui_gradient_block::gradient_block::GradientBlock;
use tui_gradient_block::types::G;

pub fn render_dashboard(frame: &mut Frame, hub: &GameHub) {
    let block = get_gradient_block("💖 Susan's Game Hub 💖");
    frame.render_widget(block, frame.area());

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8), // Big title
            Constraint::Min(1),    // Game selection area
            Constraint::Length(3), // Instructions
        ])
        .split(frame.area());

    // Big title using tui-big-text
    let big_title = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Rgb(255, 105, 180)).bold())
        .lines(vec!["GAME HUB".into()])
        .build();

    frame.render_widget(big_title, main_chunks[0]);

    // Game selection area
    let game_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Game tabs
            Constraint::Min(1),    // Game description
        ])
        .split(main_chunks[1]);

    // Create tabs for games
    let game_names: Vec<String> = hub
        .get_all_game_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    let tabs = Tabs::new(game_names)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(
                    Line::from("🎮 Available Games 🎮")
                        .style(Style::default().fg(Color::Rgb(255, 192, 203)).bold()),
                ),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 105, 180))
                .bg(Color::Rgb(50, 50, 50))
                .bold(),
        )
        .select(hub.selected_game_index);

    frame.render_widget(tabs, game_area[0]);

    // Game description
    let description = get_game_description(hub.get_selected_game_name());
    let description_widget = Paragraph::new(description)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(
                    Line::from("📝 Game Description 📝")
                        .style(Style::default().fg(Color::Rgb(186, 225, 255)).bold()),
                ),
        )
        .wrap(ratatui::widgets::Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    frame.render_widget(description_widget, game_area[1]);

    // Instructions
    let instructions = vec![Line::from(vec![
        Span::styled("Use ", Style::default().fg(Color::Gray)),
        Span::styled("←/→ ", Style::default().fg(Color::Yellow).bold()),
        Span::styled("to select game • ", Style::default().fg(Color::Gray)),
        Span::styled("Enter ", Style::default().fg(Color::Green).bold()),
        Span::styled("to play • ", Style::default().fg(Color::Gray)),
        Span::styled("Escape ", Style::default().fg(Color::Red).bold()),
        Span::styled("to quit", Style::default().fg(Color::Gray)),
    ])];

    let instructions_widget = Paragraph::new(instructions)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(
                    Line::from("🎯 Controls 🎯")
                        .style(Style::default().fg(Color::Rgb(186, 255, 201)).bold()),
                ),
        )
        .alignment(Alignment::Center);

    frame.render_widget(instructions_widget, main_chunks[2]);
}

fn get_game_description(game_name: &str) -> Vec<Line> {
    match game_name {
        "Hangman" => vec![
            Line::from("🎯 Classic word guessing game!"),
            Line::from(""),
            Line::from("Try to guess the hidden word by suggesting letters."),
            Line::from("You have 10 wrong guesses before the game ends."),
            Line::from(""),
            Line::from("Features:"),
            Line::from("• Unique Dutch words and phrases"),
            Line::from("• Panic meter to track your progress"),
            Line::from("• Colorful hangman animation"),
            Line::from("• All words must be completed to finish"),
        ],
        _ => vec![Line::from("🎮 Select a game to see its description!")],
    }
}

fn pastel(col: (u8, u8, u8)) -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[colorgrad::Color::from_rgba8(col.0, col.1, col.2, 255)])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}

fn get_gradient_block(title_text: &str) -> GradientBlock<'_> {
    GradientBlock::new()
        .left_gradient(pastel((255, 179, 186))) // pastel pink
        .bottom_gradient(pastel((186, 225, 255))) // pastel blue
        .top_gradient(Box::new(
            GradientBuilder::new()
                .colors(&[
                    colorgrad::Color::from_rgba8(255, 223, 186, 255),
                    colorgrad::Color::from_rgba8(255, 179, 186, 255),
                    colorgrad::Color::from_rgba8(186, 225, 255, 255),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ))
        .right_gradient(Box::new(
            GradientBuilder::new()
                .colors(&[
                    colorgrad::Color::from_rgba8(186, 255, 201, 255),
                    colorgrad::Color::from_rgba8(255, 255, 186, 255),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ))
        .title_top(
            Line::from(title_text)
                .style(Style::default().fg(Color::Rgb(255, 153, 204)).bold())
                .centered(),
        )
}
