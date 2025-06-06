use crate::App;

use colorgrad::Gradient;
use colorgrad::GradientBuilder;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Wrap}; // Import Gauge
use tui_gradient_block::gradient_block::GradientBlock;
use tui_gradient_block::types::G;
use tui_rule::{create_raw_spans, generate_gradient_text};

pub fn ui(frame: &mut Frame, app: &App) {
    let block = get_gradient_block(
        "💖 Jelmers galgje voor Susan :o <3 (ESC om af te sluiten | f5 om te herstarten) 💖",
    );
    frame.render_widget(block, frame.area());

    let main_chunks = Layout::default() // Changed to main_chunks for clarity
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // used letters & panic meter top part
            Constraint::Length(3), // word progress
            Constraint::Min(1),    // game (hangman)
        ])
        .split(frame.area());

    // Only render game elements if not in the 'all words exhausted' state
    if !app.all_words_exhausted {
        let top_horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Used letters
                Constraint::Percentage(25), // Remaining guesses
                Constraint::Percentage(25), // Panic Meter (new)
            ])
            .split(main_chunks[0]); // Use the first main chunk

        // Used letters
        frame.render_widget(
            Paragraph::new(
                app.used_characters
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" - "),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title(
                        Line::from("✨ Gebruikte letters ✨")
                            .style(Style::default().fg(Color::Magenta).bold()),
                    ),
            )
            .wrap(Wrap { trim: true }),
            top_horizontal_chunks[0],
        );

        // remaining guesses
        frame.render_widget(
            Paragraph::new((app.max_guesses - app.current_guess_index).to_string())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_set(symbols::border::ROUNDED)
                        .title(
                            Line::from("💌 Aantal pogingen 💌")
                                .style(Style::default().fg(Color::Cyan).bold()),
                        ),
                )
                .wrap(Wrap { trim: true }),
            top_horizontal_chunks[1],
        );

        // Panic Meter
        render_panic_meter(app, frame, top_horizontal_chunks[2]); // New function call

        // Word progress display
        render_current_word_progress(app, frame, main_chunks[1]);

        // Game body: left (hangman) and right (word reveal)
        frame.render_widget(get_hangman_widget(app), main_chunks[2]);
    }

    // Always try to show the popup if game is finished or words are exhausted
    show_end_game_popup(app, frame);
}

// --- New function for the panic meter ---
fn render_panic_meter(app: &App, frame: &mut Frame, area: Rect) {
    let bad_guesses = app.get_bad_guess_amount();
    let max_bad_guesses = app.max_guesses; // Use max_guesses as the max for panic

    // Calculate ratio, clamped to 0.0 to 1.0
    let panic_ratio = (bad_guesses as f64 / max_bad_guesses as f64)
        .min(1.0)
        .max(0.0);

    let (face, title_color) = match bad_guesses {
        0 => ("😄", Color::Green),           // Happy
        1..=2 => ("🙂", Color::LightGreen),  // Slightly happy
        3..=4 => ("😐", Color::LightYellow), // Neutral
        5..=6 => ("😟", Color::LightRed),    // Worried
        7..=8 => ("😨", Color::Red),         // Scared
        _ => ("😱", Color::DarkGray),        // Panicked / Dead (beyond max_guesses)
    };

    let panic_level_text = format!("{:.0}%", panic_ratio * 100.0);

    let title_line = Line::from(vec![
        Span::styled("Paniek meter! ", Style::default().fg(title_color).bold()),
        Span::from(face),
    ]);

    let gauge_color = match panic_ratio {
        _ if panic_ratio >= 0.75 => Color::Red,    // High panic
        _ if panic_ratio >= 0.50 => Color::Yellow, // Medium panic
        _ => Color::Green,                         // Low panic
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(title_line),
        )
        .gauge_style(Style::default().fg(gauge_color).bg(Color::Black)) // Foreground is the bar color, background is the empty space
        .percent((panic_ratio * 100.0) as u16); // Gauge takes percentage from 0-100

    frame.render_widget(gauge, area);
}

// --- Rest of your ui.rs functions (unchanged for brevity, but include them) ---
fn show_end_game_popup(app: &App, frame: &mut Frame) {
    // Only show the popup if the game is finished (win/loss) OR if all words are exhausted
    if !app.game_finished && !app.all_words_exhausted {
        return;
    }

    let popuparea = centered_rect(60, 40, frame.area());

    let (title, message_lines) = if app.all_words_exhausted {
        // Specific end screen for when all unique words have been used
        (
            Line::from("💖 ALLE WOORDEN GERADEN! 💖")
                .style(Style::default().fg(Color::Rgb(255, 192, 203)).bold()), // Light pink
            vec![
                Line::from("Je hebt alle unieke woorden in de lijst geraden! 🤩".to_string()),
                Line::from(""),
                Line::from("Bedankt voor het spelen!").style(Style::default().italic()),
                Line::from(""),
                Line::from("Druk op 'R' om alle woorden opnieuw te starten.")
                    .style(Style::default().fg(Color::LightYellow).bold().italic()),
            ],
        )
    } else if app.has_won {
        // Regular win screen
        (
            Line::from("🎉 joepie de poepie!")
                .style(Style::default().fg(Color::Rgb(255, 105, 180)).bold()), // Hot pink
            vec![
                Line::from("mulder de eindbaas heeft het weer voor elkaar! 🥳".to_string()),
                Line::from(""),
                Line::from("Druk op 'R' om opnieuw te starten. ".to_string().italic()),
                Line::from(""),
                Line::from(format!("Het woord was: {}", app.word_to_guess.clone()))
                    .style(Style::default().fg(Color::Green).bold()),
            ],
        )
    } else {
        // Regular loss screen
        (
            Line::from("💀 loserrrr").style(Style::default().fg(Color::LightRed).bold()),
            vec![
                Line::from(" tsjongejonge, wie had dat nou weer verwacht 😢".to_string()),
                Line::from(""),
                Line::from("Druk op 'R' om opnieuw te starten.".to_string().italic()),
                Line::from(""),
                Line::from(format!("Het woord was: {}", app.word_to_guess.clone()))
                    .style(Style::default().fg(Color::DarkGray).bold()),
            ],
        )
    };

    frame.render_widget(
        Paragraph::new(message_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title(title)
                    .style(Style::default().fg(Color::Rgb(255, 204, 229))), // Light pink background
            )
            .wrap(Wrap { trim: true }),
        popuparea,
    );
}

fn get_hangman_widget(app: &App) -> Paragraph {
    // ASCII frames with consistent dimensions (7 lines tall)
    const FRAMES: [&str; 10] = [
        // Frame 0 - empty gallows
        r#"
+---+
|   |
|
|
|
|
========="#,
        // Frame 1 - head
        r#"
+---+
|   |
|   o
|
|
|
========="#,
        // Frame 2 - head + torso
        r#"
+---+
|   |
|   o
|   |
|
|
========="#,
        // Frame 3 - head + torso + left arm
        r#"
   +---+
|   |
|   o
|  /|
|
|
========="#,
        // Frame 4 - head + torso + both arms
        r#"
+---+
|   |
|   o
|  /|\
|
|
========="#,
        // Frame 5 - head + torso + both arms + left leg
        r#"
+---+
|   |
|   o
|  /|\
|  /
|
========="#,
        // Frame 6 - complete body
        r#"
+---+
|   |
|   o
|  /|\
|  / \
|
========="#,
        // Frame 7 - face details
        r#"
+---+
|   |
|  (o)
|  /|\
|  / \
|
========="#,
        // Frame 8 - dead eyes
        r#"
+---+
|   |
|  (x)
|  /|\
|  / \
|
========="#,
        // Frame 9 - final dead posture
        r#"
+---+
|   |
|  (x)
| _/|\_
|  / \
|
========="#,
    ];

    let bad_guesses = app.get_bad_guess_amount();
    let frame_index = bad_guesses.min(9) as usize;
    let drawing = FRAMES[frame_index].trim_start(); // Trim leading newline

    let title = match bad_guesses {
        0..=2 => {
            Line::from(
                "ewajaaa fucking chillings hiero bij die galg tent, ga zo biertje halen denk ik",
            )
            .style(Style::default().fg(Color::Rgb(255, 192, 203)).italic()) // Pink
        }
        3..=4 => {
            Line::from("w..wwacht eens even")
                .style(Style::default().fg(Color::Rgb(255, 218, 185)).italic()) // Peach
        }
        5..=6 => {
            Line::from("owjeee").style(Style::default().fg(Color::Rgb(255, 99, 71)).bold().italic()) // Tomato
        }
        7..=9 => {
            Line::from("ummmmmm pipi...!!")
                .style(Style::default().fg(Color::Rgb(255, 0, 0)).bold().italic()) // Red
        }
        _ => {
            Line::from("doei druif")
                .style(Style::default().fg(Color::Rgb(128, 0, 0)).bold().italic()) // Dark Red
        }
    };

    Paragraph::new(drawing)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(title),
        )
        .wrap(Wrap { trim: true })
}

// Utility function to center the popup
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_current_word_progress(app: &App, frame: &mut Frame, area: Rect) {
    let mut spans: Vec<Span> = Vec::new();
    for c in app.word_to_guess.chars() {
        if c == ' ' {
            spans.push(Span::from("  ")); // double space for better visual gap
        } else if app.used_characters.contains(&c) {
            spans.push(Span::styled(
                format!("{} ", c),
                Style::default()
                    .fg(Color::Rgb(255, 105, 180)) // Hot pink
                    .bold(),
            ));
        } else {
            spans.push(Span::styled(
                "_ ",
                Style::default().fg(Color::Rgb(220, 20, 60)).italic(), // Crimson
            ));
        }
    }

    frame.render_widget(
        Paragraph::new(Line::from(spans).alignment(Alignment::Center)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(
                    Line::from("🌸 Woord 🌸")
                        .style(Style::default().fg(Color::Rgb(255, 192, 203)).bold()), // Pink
                ),
        ),
        area,
    );
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
            Line::from(generate_gradient_text!(
                title_text,
                GradientBuilder::new()
                    .colors(&[
                        colorgrad::Color::from_rgba8(255, 153, 204, 255),
                        colorgrad::Color::from_rgba8(204, 153, 255, 255)
                    ])
                    .build::<colorgrad::LinearGradient>()
                    .unwrap()
            ))
            .centered(),
        )
}
