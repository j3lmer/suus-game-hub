use crate::games::Game;
use crate::ui::adventure_ui;
use ratatui::crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect};

pub struct Scene {
    pub id: &'static str,
    pub enter_text: &'static str,
}

impl Scene {
    pub fn new(id: &'static str, enter_text: &'static str) -> Self {
        Self { id, enter_text }
    }
}

pub struct Adventure {
    scenes: Vec<Scene>,
    current_scene: usize,

    /// Everything printed so far
    log: Vec<String>,

    /// What player is typing
    input_buffer: String,
}

impl Adventure {
    pub fn new() -> Self {
        Self {
            scenes: vec![
                Scene::new(
                    "bedroom_in_bed",
                    "Het is zondag. Je vindt jezelf in bedje. Alles is zwart.",
                ),
                Scene::new(
                    "bedroom_towards_closet",
                    "Je staat naast het bed. Je kijkt naar de kast.",
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

    pub fn update(&mut self) {}

    /// Basic prototype command parser
    fn process_command(&mut self, cmd: &str) {
        let cmd = cmd.trim().to_lowercase();
        self.log.push(format!("> {}", cmd));

        match self.scenes[self.current_scene].id {
            "bedroom_in_bed" => match cmd.as_str() {
                "open eyes" | "ogen open" => {
                    self.log.push("Ah, dat is beter.".to_string());
                }
                "get out of bed" | "uit bed" => {
                    self.current_scene = 1;
                    let s = &self.scenes[self.current_scene];
                    self.log.push(s.enter_text.to_string());
                }
                "doomscroll" => {
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
