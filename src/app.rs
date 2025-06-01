use crate::KeyCode;
use rand::seq::SliceRandom;
use rand::thread_rng;

impl App {
    pub fn new() -> App {
        App {
            guess_input: String::new(),
            word_to_guess: App::get_word_to_guess(),
            used_characters: vec![],
            game_finished: false,
            has_won: false,
            max_guesses: 10,
            current_guess_index: 0,
        }
    }

    pub fn handle_user_press(&mut self, key: KeyCode) {
        // Allow restarting the game when it's finished
        if self.game_finished {
            if let KeyCode::Char('r') | KeyCode::Char('R') = key {
                *self = App::new(); // reset the game
            }
            return;
        }

        match key {
            KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                let c = c.to_ascii_lowercase(); // normalize case
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

    fn get_word_to_guess() -> String {
        let words = [
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
        ];

        let mut rng = thread_rng();

        words.choose(&mut rng).unwrap_or(&"negerzoen").to_string()
    }
}

pub struct App {
    pub guess_input: String,
    pub word_to_guess: String,
    pub used_characters: Vec<char>,
    pub game_finished: bool,
    pub has_won: bool,
    pub max_guesses: u32,
    pub current_guess_index: u32,
}
