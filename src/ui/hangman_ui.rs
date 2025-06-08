use crate::games::hangman::HangmanGame;
use colorgrad::GradientBuilder;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Wrap};
use tui_gradient_block::gradient_block::GradientBlock;
use tui_gradient_block::types::G;

pub fn render_hangman_game(game: &HangmanGame, frame: &mut Frame, area: Rect) {
    let block = get_gradient_block(
        "💖 Jelmers galgje voor Susan :o <3 (Backspace to return | F5 to restart) 💖",
    );
    frame.render_widget(block, area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // used letters & panic meter top part
            Constraint::Length(3), // word progress
            Constraint::Min(1),    // game (hangman)
        ])
        .split(area);

    // Only render game elements if not in the 'all words exhausted' state
    if !game.all_words_exhausted {
        let top_horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Used letters
                Constraint::Percentage(25), // Remaining guesses
                Constraint::Percentage(25), // Panic Meter
            ])
            .split(main_chunks[0]);

        // Used letters
        frame.render_widget(
            Paragraph::new(
                game.used_characters
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
                            .style(Style::default().fg(Color::Rgb(255, 204, 229))),
                    ),
            )
            .wrap(Wrap { trim: true }),
            top_horizontal_chunks[0],
        );

        // remaining guesses
        frame.render_widget(
            Paragraph::new((game.max_guesses - game.current_guess_index).to_string())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_set(symbols::border::ROUNDED)
                        .title(
                            Line::from("💌 Aantal pogingen 💌")
                                .style(Style::default().fg(Color::Cyan).bold()),
                        ),
                )
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            top_horizontal_chunks[1],
        );

        // Panic Meter
        render_panic_meter(game, frame, top_horizontal_chunks[2]);

        // Word progress display
        render_current_word_progress(game, frame, main_chunks[1]);

        // Game body: hangman display with centered content
        render_hangman_area(game, frame, main_chunks[2]);
    }

    // Always try to show the popup if game is finished or words are exhausted
    show_end_game_popup(game, frame);
}

fn render_hangman_area(game: &HangmanGame, frame: &mut Frame, area: Rect) {
    // Create the outer border block
    let bad_guesses = game.get_bad_guess_amount();
    let title = match bad_guesses {
        0..=2 => Line::from(
            "ewajaaa fucking chillings hiero bij die galg tent, ga zo biertje halen denk ik",
        )
        .style(Style::default().fg(Color::Rgb(255, 192, 203)).italic()),
        3..=4 => Line::from("w..wwacht eens even")
            .style(Style::default().fg(Color::Rgb(255, 218, 185)).italic()),
        5..=6 => {
            Line::from("owjeee").style(Style::default().fg(Color::Rgb(255, 99, 71)).bold().italic())
        }
        7..=9 => Line::from("ummmmmm pipi...!!")
            .style(Style::default().fg(Color::Rgb(255, 0, 0)).bold().italic()),
        _ => Line::from("doei druif")
            .style(Style::default().fg(Color::Rgb(128, 0, 0)).bold().italic()),
    };

    let border_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .title(title);

    // Render the border
    frame.render_widget(border_block, area);

    // Create inner area and use layout chunks to center horizontally
    let inner_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    // Create horizontal layout to center the hangman paragraph
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left spacer
            Constraint::Percentage(50), // Hangman content in middle
        ])
        .split(inner_area);

    // Render the hangman drawing in the middle chunk
    frame.render_widget(get_hangman_paragraph(game), horizontal_chunks[1]);
}

fn get_hangman_paragraph(game: &HangmanGame) -> Paragraph<'static> {
    const FRAMES: [&str; 10] = [
        // Frame 0 - empty gallows
        r#"
+---+
|   |
|
|
|
|
========"#,
        // Frame 1 - head
        r#"
+---+
|   |
|   o
|
|
|
========"#,
        // Frame 2 - head + torso
        r#"
+---+
|   |
|   o
|   |
|
|
========"#,
        // Frame 3 - head + torso + left arm
        r#"
+---+
|   |
|   o
|  /|
|
|
========"#,
        // Frame 4 - head + torso + both arms
        r#"
+---+
|   |
|   o
|  /|\
|
|
========"#,
        // Frame 5 - head + torso + both arms + left leg
        r#"
+---+
|   |
|   o
|  /|\
|  /
|
========"#,
        // Frame 6 - complete body
        r#"
+---+
|   |
|   o
|  /|\
|  / \
|
========"#,
        // Frame 7 - face details
        r#"
+---+
|   |
|  (o)
|  /|\
|  / \
|
========"#,
        // Frame 8 - dead eyes
        r#"
+---+
|   |
|  (x)
|  /|\
|  / \
|
========"#,
        // Frame 9 - final dead posture
        r#"
+---+
|   |
|  (x)
| _/|\_
|  / \
|
========"#,
    ];

    let bad_guesses = game.get_bad_guess_amount();
    let frame_index = bad_guesses.min(9) as usize;
    let drawing = FRAMES[frame_index].trim_start();

    Paragraph::new(drawing).wrap(Wrap { trim: true })
}

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

fn render_current_word_progress(game: &HangmanGame, frame: &mut Frame, area: Rect) {
    let mut spans: Vec<Span> = Vec::new();
    for c in game.word_to_guess.chars() {
        if c == ' ' {
            spans.push(Span::from("  "));
        } else if game.used_characters.contains(&c) {
            spans.push(Span::styled(
                format!("{} ", c),
                Style::default().fg(Color::Rgb(255, 105, 180)).bold(),
            ));
        } else {
            spans.push(Span::styled(
                "_ ",
                Style::default().fg(Color::Rgb(220, 20, 60)).italic(),
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
                        .style(Style::default().fg(Color::Rgb(255, 192, 203)).bold()),
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
        .left_gradient(pastel((255, 179, 186)))
        .bottom_gradient(pastel((186, 225, 255)))
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

fn render_panic_meter(game: &HangmanGame, frame: &mut Frame, area: Rect) {
    let bad_guesses = game.get_bad_guess_amount();
    let max_bad_guesses = game.max_guesses;

    let panic_ratio = (bad_guesses as f64 / max_bad_guesses as f64)
        .min(1.0)
        .max(0.0);

    let (face, title_color) = match bad_guesses {
        0 => ("😄", Color::Green),
        1..=2 => ("🙂", Color::LightGreen),
        3..=4 => ("😐", Color::LightYellow),
        5..=6 => ("😟", Color::LightRed),
        7..=8 => ("😨", Color::Red),
        _ => ("😱", Color::DarkGray),
    };

    let title_line = Line::from(vec![
        Span::styled("Paniek meter! ", Style::default().fg(title_color).bold()),
        Span::from(face),
    ]);

    let gauge_color = match panic_ratio {
        _ if panic_ratio >= 0.75 => Color::Red,
        _ if panic_ratio >= 0.50 => Color::Yellow,
        _ => Color::Green,
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(title_line),
        )
        .gauge_style(Style::default().fg(gauge_color).bg(Color::Black))
        .percent((panic_ratio * 100.0) as u16);

    frame.render_widget(gauge, area);
}

fn show_end_game_popup(game: &HangmanGame, frame: &mut Frame) {
    if !game.game_finished && !game.all_words_exhausted {
        return;
    }

    let popuparea = centered_rect(60, 40, frame.area());

    let (title, message_lines) = if game.all_words_exhausted {
        (
            Line::from("💖 ALLE WOORDEN GERADEN! 💖")
                .style(Style::default().fg(Color::Rgb(255, 192, 203)).bold()),
            vec![
                Line::from("Je hebt alle unieke woorden in de lijst geraden! 🤩".to_string()),
                Line::from(""),
                Line::from("Bedankt voor het spelen!").style(Style::default().italic()),
                Line::from(""),
                Line::from("Druk op 'R' of Enter om alle woorden opnieuw te starten.")
                    .style(Style::default().fg(Color::LightYellow).bold().italic()),
                Line::from("Druk op 'Backspace' om terug te gaan naar het menu.")
                    .style(Style::default().fg(Color::LightBlue).bold().italic()),
            ],
        )
    } else if game.has_won {
        (
            Line::from("🎉 joepie de poepie!")
                .style(Style::default().fg(Color::Rgb(255, 105, 180)).bold()),
            vec![
                Line::from("mulder de eindbaas heeft het weer voor elkaar! 🥳".to_string()),
                Line::from(""),
                Line::from("Druk op 'R' of Enter om opnieuw te starten.".to_string())
                    .style(Style::default().italic()),
                Line::from("Druk op 'Backspace' om terug te gaan naar het menu.".to_string())
                    .style(Style::default().italic()),
                Line::from(""),
                Line::from(format!("Het woord was: {}", game.word_to_guess.clone()))
                    .style(Style::default().fg(Color::Green).bold()),
            ],
        )
    } else {
        (
            Line::from("💀 loserrrr").style(Style::default().fg(Color::LightRed).bold()),
            vec![
                Line::from(" tsjongejonge, wie had dat nou weer verwacht 😢".to_string()),
                Line::from(""),
                Line::from("Druk op 'R' of Enter om opnieuw te starten.".to_string())
                    .style(Style::default().italic()),
                Line::from("Druk op 'Backspace' om terug te gaan naar het menu.".to_string())
                    .style(Style::default().italic()),
                Line::from(""),
                Line::from(format!("Het woord was: {}", game.word_to_guess.clone()))
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
                    .style(Style::default().fg(Color::Magenta).bold()),
            )
            .wrap(Wrap { trim: true }),
        popuparea,
    );
}
