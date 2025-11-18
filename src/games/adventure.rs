use crate::games::Game;
use crate::ui::adventure_ui;
use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

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
    input_buffer: String,
}

// TODO: dit moet ik laden uit een json file or smth
impl Adventure {
    pub fn new() -> Self {
        Self {
            scenes: vec![
                Scene::new(
                    "bedroom_in_bed",
                    "Het is zondag, je vind jezelf in bedje, met een flinke kater\n
                    jelmer en jij hebben de hele avond weer een of ander nieuw spelletje gespeeld wat hij je aan heeft gesmeerd.\n
                    om heel eerlijk te zijn vond je het best leuk, maar je kan je niet herinneren wat het nou eigenlijk was.\n\n
                    Je voelt je vies, alsof er een laag smots over je heen zit. je vraagt je af wanneer de laatste keer was dat je hebt gedouched.

                    Maar wacht eens even, alles is zwart! ben ik blind geworden? wat is er aan de hand?!
                ",
                    ""
                ),
                Scene::new(
                    "bedroom_towards_closet",
                    "Je staat naast het bed. Je kijkt naar de kast.",
                    ""
                ),
            ],
            current_scene: 0,
            log: Vec::new(),
            input_buffer: String::new(),
        }
    }

    pub fn start_new_game(&mut self) {
        self.current_scene = 0;
        self.log.clear();
        self.input_buffer.clear();

        let first = &self.scenes[self.current_scene];
        self.log.push(first.enter_text.to_string());
    }

    pub fn die(&mut self, reason: &str) {
        self.log.push(format!("GAME OVER: {}", reason));
    }

    pub fn inventory(&self) -> Vec<&'static str> {
        vec!["📱"]
    }

    // TODO: dit moet worden geupdate bij elke enter
    pub fn stats(&self) -> AdventureStats {
        AdventureStats {
            moves_done: 0
        }
    }

    pub fn current_scene_art(&self) -> &str {
        self.scenes[self.current_scene].scene_art
    }

    pub fn update(&mut self) {}

    /// Basic prototype command parser
    fn process_command(&mut self, cmd: &str) {
        let cmd = cmd.trim().to_lowercase();
        self.log.push(format!("> {}", cmd));

        match self.scenes[self.current_scene].id {
            "bedroom_in_bed" => match cmd.as_str() {
                "doe ogen open" => {
                    self.log.push("Ah, dat is beter.".to_string());
                }
                "sta op" | "uit bedje" | "opstaan" => {
                    self.current_scene = 1;
                    let s = &self.scenes[self.current_scene];
                    self.log.push(s.enter_text.to_string());
                }
                "doomscrollen" | "doomscroll" => {
                    self.die("Instagram heeft je opgegeten.");
                }
                _ => self.log.push("Dat kan niet.".to_string()),
            },

            "bedroom_towards_closet" => match cmd.as_str() {
                "naar badkamer" | "go bathroom" => {
                    self.log.push("(Not implemented)".to_string());
                }
                _ => self.log.push("Dat kan niet.".to_string()),
            },

            _ => {}
        }
    }
}

impl Game for Adventure {
    fn render(&self, frame: &mut Frame, area: Rect) {
        adventure_ui::render_adventure_game(self, frame, area);
    }

    fn restart(&mut self) {
        self.start_new_game();
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                let input = self.input_buffer.clone();
                self.input_buffer.clear();
                self.process_command(&input);
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
