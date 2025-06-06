use std::iter;

use crate::KeyCode; // Assuming KeyCode is defined in your crate
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct App {
    pub guess_input: String,
    pub word_to_guess: String,
    pub used_characters: Vec<char>,
    pub game_finished: bool,
    pub has_won: bool,
    pub max_guesses: u32,
    pub current_guess_index: u32,
    pub previous_words: Vec<String>,
    pub all_words_exhausted: bool, // New field to track if all unique words are used
}

impl App {
    pub fn new() -> App {
        App {
            guess_input: String::new(),
            word_to_guess: String::new(), // Initialize with an empty string, will be set in start_new_game
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
        // If all words are exhausted, pressing 'R' here should clear previous_words
        // to allow recycling words. Otherwise, no new game can start.
        if self.all_words_exhausted {
            self.previous_words.clear(); // Clear history to allow replaying all words
            self.all_words_exhausted = false; // Reset the exhausted state
            // Continue with the new game logic below
        }

        // Before resetting, store the current word if a game was played and a word was set
        if !self.word_to_guess.is_empty() {
            self.previous_words.push(self.word_to_guess.clone());
        }

        let mut new_app = App::new();
        // Carry over previous words to the new app instance
        new_app.previous_words = self.previous_words.clone();
        *self = new_app; // Reset the app state to a fresh instance
        self.word_to_guess = self.get_word_to_guess();

        // Check if get_word_to_guess couldn't find a new word (returned empty string)
        if self.word_to_guess.is_empty() {
            self.all_words_exhausted = true;
            self.game_finished = true; // Mark game as finished to trigger end screen
            self.has_won = false; // Indicate no win/loss, but end-of-content
        }
    }

    pub fn handle_user_press(&mut self, key: KeyCode) {
        // If all words are exhausted, only allow 'r' to restart the whole sequence
        if self.all_words_exhausted {
            if let KeyCode::Char('r') | KeyCode::Char('R') = key {
                self.start_new_game(); // This will clear previous_words and restart
            }
            return; // Don't process other keys if all words are exhausted
        }

        // Allow restarting the game when a regular game is finished
        if self.game_finished {
            if let KeyCode::Char('r') | KeyCode::Char('R') = key {
                self.start_new_game();
            }
            return;
        }

        match key {
            KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                let c = c.to_ascii_lowercase(); // normalize case
                if !self.used_characters.contains(&c) {
                    self.used_characters.push(c);

                    if self.word_to_guess.contains(c) {
                        self.guess_input.push(c); // Still pushing here, but not directly used for win condition

                        let all_guessed = self
                            .word_to_guess
                            .chars()
                            .filter(|ch| *ch != ' ') // Ignore spaces for win condition
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
            _ => {
                // Ignore other keys
            }
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

        let mut available_words: Vec<&str> = all_words
            .iter()
            .filter(|word| !self.previous_words.contains(&word.to_string()))
            .cloned()
            .collect();

        if available_words.is_empty() {
            // All unique words have been used.
            // Return an empty string to signal this to the caller.
            return String::new();
        }

        let mut rng = thread_rng();

        let word = available_words
            .choose(&mut rng)
            .unwrap_or(&"ERROR_WORD") // Fallback, though should not be hit if available_words is not empty
            .to_string();

        word
    }
}
