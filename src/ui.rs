use crate::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn ui(frame: &mut Frame, app: &App) {
    frame.render_widget(
        Block::default()
            .title("Jelmers galgje voor Susan :o <3 (ESC om af te sluiten)")
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
    frame.render_widget(get_hangman_widget(app), chunks[2]);

    show_end_game_popup(app, frame);
}

fn show_end_game_popup(app: &App, frame: &mut Frame) {
    if !app.game_finished {
        return;
    }

    let popuparea = centered_rect(60, 40, frame.area());

    let (title, message) = if app.has_won {
        (
            "joepie de poepie!",
            format!(
                "mulder de eindbaas heeft het weer voor elkaar! 🥳\n\nDruk op 'R' om opnieuw te starten. \n \nHet woord was: {}",
                app.word_to_guess.clone()
            ),
        )
    } else {
        (
            "loserrrr",
            format!(
                "tsjongejonge, wie had dat nou weer verwacht 😢\n\nDruk op 'R' om opnieuw te starten.\n\nHet woord was: {}",
                app.word_to_guess.clone()
            ),
        )
    };

    frame.render_widget(
        Paragraph::new(message)
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: true }),
        popuparea,
    );
}
// Hangman display and panic title
fn get_hangman_widget(app: &App) -> Paragraph {
    // ASCII frames with consistent dimensions (7 lines tall)
    const FRAMES: [&str; 10] = [
        // Frame 0 - empty gallows
        r#"
+---+
|  |
|
|
|
|
========="#,
        // Frame 1 - head
        r#"
+---+
|  |
|  o
|
|
|
========="#,
        // Frame 2 - head + torso
        r#"
+---+
|  |
|  o
|  |
|
|
========="#,
        // Frame 3 - head + torso + left arm
        r#"
   +---+
|  |
|  o
| /|
|
|
========="#,
        // Frame 4 - head + torso + both arms
        r#"
+---+
|  | 
|  o
| /|\
|
|
========="#,
        // Frame 5 - head + torso + both arms + left leg
        r#"
+---+
|  |
|  o
| /|\
| /
|
========="#,
        // Frame 6 - complete body
        r#"
+---+
|  |
|  o
| /|\
| / \
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
        0..=2 => "ewajaaa fucking chillings hiero bij die galg tent, ga zo biertje halen denk ik",
        3..=4 => "w..wwacht eens even",
        5..=6 => "owjeee",
        7..=9 => "ummmmmm pipi...!!",
        _ => "doei druif",
    };

    Paragraph::new(drawing)
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
