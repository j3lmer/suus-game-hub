use crate::games::Game;
use crate::ui::hangman_ui::render_hangman_game;
use rand::seq::SliceRandom;
use rand::thread_rng;
use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

pub struct HangmanGame {
    pub guess_input: String,
    pub word_to_guess: String,
    pub used_characters: Vec<char>,
    pub game_finished: bool,
    pub has_won: bool,
    pub max_guesses: u32,
    pub current_guess_index: u32,
    pub previous_words: Vec<String>,
    pub all_words_exhausted: bool,
}

impl HangmanGame {
    pub fn new() -> Self {
        Self {
            guess_input: String::new(),
            word_to_guess: String::new(),
            used_characters: vec![],
            game_finished: false,
            has_won: false,
            max_guesses: 10,
            current_guess_index: 0,
            previous_words: Vec::new(),
            all_words_exhausted: false,
        }
    }

    pub fn start_new_game(&mut self) {
        if self.all_words_exhausted {
            self.previous_words.clear();
            self.all_words_exhausted = false;
        }

        if !self.word_to_guess.is_empty() {
            self.previous_words.push(self.word_to_guess.clone());
        }

        let previous_words = self.previous_words.clone();
        *self = Self::new();
        self.previous_words = previous_words;
        self.word_to_guess = self.get_word_to_guess();

        if self.word_to_guess.is_empty() {
            self.all_words_exhausted = true;
            self.game_finished = true;
            self.has_won = false;
        }
    }

    pub fn get_bad_guess_amount(&self) -> u32 {
        self.used_characters
            .iter()
            .filter(|c| !self.word_to_guess.contains(**c))
            .count() as u32
    }

    fn get_word_to_guess(&self) -> String {
        let all_words = [
            "appeltaart",
            "computer",
            "vakantie",
            "susan",
            "mjauw",
            "miauw",
            "pipi",
            "knuffel",
            "zomer",
            "regenboog",
            "humberto tan",
            "jelmer",
            "snorfbokkel",
            "guppie",
            "poesje",
            "hottentottententententoonstelling",
            "poep",
            "snakie",
            "batsen",
            "sapje",
            "chocola",
            "bier",
            "snoepje",
            "prinses",
            "tiara",
            "soepje",
            "jammie",
            "bami",
            "simsen",
            "zuipen",
            "beerenburg",
            "friesland",
            "susan is stinky",
            "stinky",
            "negerzoen",
            "rino",
            "rhino",
            "banaan",
            "hakenkruis",
            "apenstaartje",
            "paprika",
            "bananenvla",
        ];

        let available_words: Vec<&str> = all_words
            .iter()
            .filter(|word| !self.previous_words.contains(&word.to_string()))
            .cloned()
            .collect();

        if available_words.is_empty() {
            return String::new();
        }

        let mut rng = thread_rng();
        let word = available_words
            .choose(&mut rng)
            .unwrap_or(&"ERROR_WORD")
            .to_string();

        word
    }
}

impl Game for HangmanGame {
    fn handle_input(&mut self, key: KeyCode) {
        if self.all_words_exhausted {
            if let KeyCode::Char('r') | KeyCode::Char('R') = key {
                self.start_new_game();
            }
            return;
        }

        if self.game_finished {
            if let KeyCode::Char('r') | KeyCode::Char('R') = key {
                self.start_new_game();
            }
            return;
        }

        match key {
            KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                let c = c.to_ascii_lowercase();
                if !self.used_characters.contains(&c) {
                    self.used_characters.push(c);

                    if self.word_to_guess.contains(c) {
                        self.guess_input.push(c);

                        let all_guessed = self
                            .word_to_guess
                            .chars()
                            .filter(|ch| *ch != ' ')
                            .all(|ch| self.used_characters.contains(&ch));

                        if all_guessed {
                            self.has_won = true;
                            self.game_finished = true;
                        }
                    } else {
                        self.current_guess_index += 1;
                        if self.current_guess_index >= self.max_guesses {
                            self.game_finished = true;
                            self.has_won = false;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        render_hangman_game(self, frame, area);
    }

    fn restart(&mut self) {
        self.start_new_game();
    }

    fn is_finished(&self) -> bool {
        self.game_finished || self.all_words_exhausted
    }
}
