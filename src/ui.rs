use crate::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn ui(frame: &mut Frame, app: &App) {
    frame.render_widget(
        Block::default()
            .title("Jelmers galgje voor Susan :o <3")
            .borders(Borders::all()),
        frame.area(),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // used letters
            Constraint::Length(3), // word progress
            Constraint::Min(1),    // game
        ])
        .split(frame.area());

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // Used letters
    frame.render_widget(
        Paragraph::new(format!(
            "Gebruikte letters: {}",
            app.used_characters
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" - ")
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Gebruikte letters"),
        )
        .wrap(Wrap { trim: true }),
        chunks[0],
    );

    // Word progress display
    render_current_word_progress(app, frame, chunks[1]);

    // Game body: left (hangman) and right (word reveal)
    frame.render_widget(get_hangman_widget(app), middle_chunks[0]);
    show_results(app, frame, middle_chunks[1]);

    show_end_game_popup(app, frame);
}

fn show_results(app: &App, frame: &mut Frame, area: Rect) {
    if app.game_finished {
        frame.render_widget(
            Paragraph::new(app.word_to_guess.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Het woord was:"),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

fn show_end_game_popup(app: &App, frame: &mut Frame) {
    if !app.game_finished {
        return;
    }

    let popuparea = centered_rect(60, 40, frame.area());

    let (title, message) = if app.has_won {
        ("joepie de poepie!", "pipi de meester gokker!! 🥳")
    } else {
        ("womp womp!", "fucking noob. 😢")
    };

    frame.render_widget(
        Paragraph::new(message)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(Style::default().bg(Color::Magenta)),
            )
            .wrap(Wrap { trim: true }),
        popuparea,
    );
}

// Hangman display and panic title
fn get_hangman_widget(app: &App) -> Paragraph {
    let bad_guesses = app.get_bad_guess_amount();

    let stages = vec![
        "       \n       \n       \n       \n       ",
        "       \n   |   \n   |   \n   |   \n_______",
        "   +---+\n   |   \n       \n       \n_______",
        "   +---+\n   |   \n   O   \n       \n_______",
        "   +---+\n   |   \n   O   \n   |   \n_______",
        "   +---+\n   |   \n   O   \n  /|   \n_______",
        "   +---+\n   |   \n   O   \n  /|\\ \n_______",
        "   +---+\n   |   \n   O   \n  /|\\ \n  /    ",
        "   +---+\n   |   \n   O   \n  /|\\ \n  / \\ ",
        "   +---+\n   |   \n   O   \n  /|\\ \n  / \\ ",
    ];

    let title = match bad_guesses {
        0..=2 => "ewajaaa fucking chillings hiero bij die galg tent, ga zo biertje halen denk ik",
        3..=4 => "w..wwacht eens even",
        5..=6 => "owjeee",
        7..=9 => "ummmmmm pipi...!!",
        _ => "doei druif",
    };

    let drawing = stages
        .get(bad_guesses as usize)
        .unwrap_or(&stages.last().unwrap());

    Paragraph::new(*drawing)
        .block(Block::default().borders(Borders::ALL).title(title))
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
    let display: String = app
        .word_to_guess
        .chars()
        .map(|c| {
            if app.used_characters.contains(&c) {
                format!("{} ", c)
            } else {
                "_ ".to_string()
            }
        })
        .collect();

    frame.render_widget(
        Paragraph::new(display.trim_end()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("het woord iss...."),
        ),
        area,
    );
}
