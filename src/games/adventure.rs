use crate::games::Game;
use crate::ui::adventure_ui;
use ratatui::crossterm::event::KeyCode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CommandAction {
    pub action: String,
    pub text: Option<String>,
    pub target: Option<String>,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct SceneJson {
    pub id: String,
    pub scene_enter: String,
    pub scene_art: String,
    pub commands: HashMap<String, CommandAction>,
}

#[derive(Deserialize)]
pub struct AdventureJsonRoot {
    pub scenes: Vec<SceneJson>,
}

pub struct Scene {
    pub id: String,
    pub enter_text: String,
    pub scene_art: String,
    pub commands: HashMap<String, CommandAction>,
}

pub struct AdventureStats {
    pub moves_done: i32,
}

pub struct Adventure {
    scenes: HashMap<String, Scene>,
    current_scene: String,

    log: Vec<String>,
    pub input_buffer: String,

    pub autocomplete_matches: Vec<String>,
    pub autocomplete_index: usize,

    // Scrolling state
    pub log_scroll: u16,
    pub auto_scroll: bool,
}

impl Adventure {
    pub fn new() -> Self {
        let file =
            std::fs::read_to_string("data/adventure.json").expect("Could not read adventure.json");

        let root: AdventureJsonRoot = serde_json::from_str(&file).expect("Invalid adventure.json");

        let first_scene_id = root.scenes.first().expect("No scenes in JSON").id.clone();

        let mut scenes = HashMap::new();

        for s in root.scenes {
            scenes.insert(
                s.id.clone(),
                Scene {
                    id: s.id,
                    enter_text: s.scene_enter,
                    scene_art: s.scene_art,
                    commands: s.commands,
                },
            );
        }

        let first_scene_enter = scenes
            .get(&first_scene_id)
            .expect("First scene not found")
            .enter_text
            .clone();

        Adventure {
            scenes,
            current_scene: first_scene_id,
            log: vec![first_scene_enter],
            input_buffer: String::new(),
            autocomplete_matches: vec![],
            autocomplete_index: 0,
            log_scroll: 0,
            auto_scroll: true,
        }
    }

    pub fn start_new_game(&mut self) {
        let first_scene_id = self.current_scene.clone();

        self.log.clear();
        self.input_buffer.clear();
        self.autocomplete_matches.clear();
        self.autocomplete_index = 0;
        self.log_scroll = 0;
        self.auto_scroll = true;

        let first: &Scene = &self.scenes[&first_scene_id];

        self.log.push(first.enter_text.clone());
        self.update_autocomplete();
    }

    pub fn inventory(&self) -> Vec<&'static str> {
        vec!["📱"]
    }

    pub fn stats(&self) -> AdventureStats {
        AdventureStats { moves_done: 0 }
    }

    pub fn current_scene_art(&self) -> &str {
        &self.scenes[&self.current_scene].scene_art
    }

    pub fn update(&mut self) {}

    fn all_commands(&self) -> Vec<String> {
        let scene = self.scenes.get(&self.current_scene).unwrap();
        scene.commands.keys().cloned().collect()
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

    pub fn autocomplete_suggestion(&self) -> Option<&str> {
        self.autocomplete_matches
            .get(self.autocomplete_index)
            .map(|s| s.as_str())
    }

    fn process_command(&mut self, input: &str) {
        let input = input.trim().to_lowercase();
        self.log.push(format!("> {}", input));

        let scene = self.scenes.get(&self.current_scene).unwrap();

        if let Some(action) = scene.commands.get(&input) {
            match action.action.as_str() {
                "log" => {
                    self.log.push(action.text.clone().unwrap_or_default());
                }
                "change_scene" => {
                    let target = action.target.as_ref().expect("Missing target");
                    self.current_scene = target.clone();
                    let new_scene = self.scenes.get(target).unwrap();
                    self.log.push(new_scene.enter_text.clone());
                }
                "die" => {
                    let reason = action.reason.clone().unwrap_or("You died".to_string());
                    self.log.push(format!("GAME OVER: {}", reason));
                }
                _ => self
                    .log
                    .push("Ik weet niet wat ik hiermee moet..".to_string()),
            }
        } else {
            self.log.push("Dat kan niet.".to_string());
        }

        // Re-enable auto-scroll when new content is added
        self.auto_scroll = true;
        self.log_scroll = 0; // Reset manual scroll position
        self.update_autocomplete();
    }

    pub fn scroll_up(&mut self) {
        self.auto_scroll = false;
        self.log_scroll = self.log_scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.log_scroll = self.log_scroll.saturating_add(1);
        // Check if we've scrolled to the bottom, re-enable auto-scroll
        let total_lines = self.total_log_lines();
        if self.log_scroll as usize >= total_lines {
            self.auto_scroll = true;
        }
    }

    pub fn total_log_lines(&self) -> usize {
        self.log.iter().map(|entry| entry.lines().count()).sum()
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
            KeyCode::Up => {
                self.scroll_up();
            }
            KeyCode::Down => {
                self.scroll_down();
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
