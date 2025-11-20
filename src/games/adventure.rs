use crate::games::Game;
use crate::ui::adventure_ui;
use ratatui::crossterm::event::KeyCode;

pub struct Scene {
    pub id: &'static str,
    pub enter_text: &'static str,
    pub scene_art: &'static str,
}

impl Scene {
    pub fn new(id: &'static str, enter_text: &'static str, scene_art: &'static str) -> Self {
        Self { id, enter_text, scene_art }
    }
}

pub struct AdventureStats {
    pub moves_done: i32,
}

pub struct Adventure {
    scenes: Vec<Scene>,
    current_scene: usize,

    /// Everything printed so far
    log: Vec<String>,

    /// What player is typing
    pub input_buffer: String,

    /// Autocomplete state
    pub autocomplete_matches: Vec<&'static str>,
    pub autocomplete_index: usize,
}

impl Adventure {
    pub fn new() -> Self {
        Self {
            scenes: vec![
                Scene::new(
                    "bedroom_in_bed",
                    "Het is zondag, je vind jezelf in bedje, met een flinke kater\n\
                    jelmer en jij hebben de hele avond weer een of ander nieuw spelletje gespeeld wat hij je aan heeft gesmeerd.\n\
                    om heel eerlijk te zijn vond je het best leuk, maar je kan je niet herinneren wat het nou eigenlijk was.\n\n\
                    Je voelt je vies, alsof er een laag smots over je heen zit. je vraagt je af wanneer de laatste keer was dat je hebt gedouched.\n\n\
                    Maar wacht eens even, alles is zwart! ben ik blind geworden? wat is er aan de hand?!",
                    "",
                ),
                Scene::new(
                    "bedroom_towards_closet",
                    "Je staat naast het bed. Je kijkt naar de kast.",
                    "",
                ),
            ],
            current_scene: 0,
            log: Vec::new(),
            input_buffer: String::new(),
            autocomplete_matches: Vec::new(),
            autocomplete_index: 0,
        }
    }

    pub fn start_new_game(&mut self) {
        self.current_scene = 0;
        self.log.clear();
        self.input_buffer.clear();
        self.autocomplete_matches.clear();
        self.autocomplete_index = 0;

        let first = &self.scenes[self.current_scene];
        self.log.push(first.enter_text.to_string());
        self.update_autocomplete();
    }

    pub fn die(&mut self, reason: &str) {
        self.log.push(format!("GAME OVER: {}", reason));
    }

    pub fn inventory(&self) -> Vec<&'static str> {
        vec!["📱"]
    }

    pub fn stats(&self) -> AdventureStats {
        AdventureStats { moves_done: 0 }
    }

    pub fn current_scene_art(&self) -> &str {
        self.scenes[self.current_scene].scene_art
    }

    pub fn update(&mut self) {}

    fn all_commands(&self) -> Vec<&'static str> {
        match self.scenes[self.current_scene].id {
            "bedroom_in_bed" => vec![
                "doe ogen open",
                "sta op",
                "uit bedje",
                "opstaan",
                "doomscrollen",
            ],
            "bedroom_towards_closet" => vec![
                "naar badkamer",
                "go bathroom",
            ],
            _ => vec![],
        }
    }

    pub fn update_autocomplete(&mut self) {
        let input = self.input_buffer.to_lowercase();
        self.autocomplete_matches = self
            .all_commands()
            .into_iter()
            .filter(|cmd| cmd.starts_with(&input))
            .collect();
        self.autocomplete_index = 0;
    }

    pub fn autocomplete_suggestion(&self) -> Option<&'static str> {
        self.autocomplete_matches.get(self.autocomplete_index).copied()
    }

    fn process_command(&mut self, cmd: &str) {
        let cmd = cmd.trim().to_lowercase();
        self.log.push(format!("> {}", cmd));

        match self.scenes[self.current_scene].id {
            "bedroom_in_bed" => match cmd.as_str() {
                "doe ogen open" => self.log.push("Ah, dat is beter.".to_string()),
                "sta op" | "uit bedje" | "opstaan" => {
                    self.current_scene = 1;
                    let s = &self.scenes[self.current_scene];
                    self.log.push(s.enter_text.to_string());
                }
                "doomscrollen" => self.die("Instagram heeft je opgegeten."),
                _ => self.log.push("Dat kan niet.".to_string()),
            },
            "bedroom_towards_closet" => match cmd.as_str() {
                "naar de hal" => self.log.push("(Not implemented)".to_string()),
                _ => self.log.push("Dat kan niet.".to_string()),
            },
            _ => {}
        }
        self.update_autocomplete();
    }
}

impl Game for Adventure {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        adventure_ui::render_adventure_game(self, frame, area);
    }

    fn restart(&mut self) {
        self.start_new_game();
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
                self.update_autocomplete();
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
                self.update_autocomplete();
            }
            KeyCode::Enter => {
                let input = self.input_buffer.clone();
                self.input_buffer.clear();
                self.process_command(&input);
            }
            KeyCode::Tab => {
                if !self.autocomplete_matches.is_empty() {
                    self.autocomplete_index =
                        (self.autocomplete_index + 1) % self.autocomplete_matches.len();
                    self.input_buffer =
                        self.autocomplete_matches[self.autocomplete_index].to_string();
                }
            }
            _ => {}
        }
    }

    fn tick(&mut self) {
        self.update();
    }
}

// Public getters
impl Adventure {
    pub fn log(&self) -> &Vec<String> {
        &self.log
    }

    pub fn input(&self) -> &str {
        &self.input_buffer
    }
}
