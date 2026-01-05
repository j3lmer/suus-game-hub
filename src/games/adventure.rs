use crate::games::Game;
use crate::ui::adventure_ui;
use crate::utils::image as image_utils;

use ratatui::crossterm::event::KeyCode;
use ratatui_image::protocol::StatefulProtocol;

use serde::Deserialize;

use std::cell::RefCell;
use std::collections::HashMap;

use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Clone)]
pub struct CommandAction {
    pub action: String,
    pub text: Option<String>,
    pub target: Option<String>,
    pub reason: Option<String>,
    pub file_path: Option<String>,
    pub set_flag: Option<String>,
    pub requires_flag: Option<String>,
    pub requires_all: Option<Vec<String>>,
    pub actions: Option<Vec<CommandAction>>,
    pub else_actions: Option<Vec<CommandAction>>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum CommandJson {
    Simple(Vec<CommandAction>),
    Wrapped {
        once: Option<bool>,
        actions: Vec<CommandAction>,
        fallback: Option<Vec<CommandAction>>,
    },
}

#[derive(Deserialize)]
pub struct SceneJson {
    pub id: String,
    pub scene_enter: String,
    pub scene_art: String,
    pub scene_image: Option<String>,
    pub commands: HashMap<String, CommandJson>,
}

#[derive(Deserialize)]
pub struct AdventureJsonRoot {
    pub scenes: Vec<SceneJson>,
    pub global_commands: HashMap<String, CommandJson>,
}

pub struct Scene {
    pub enter_text: String,
    pub scene_art: String,
    pub scene_image: Option<RefCell<Box<dyn StatefulProtocol>>>,
    pub commands: HashMap<String, CommandJson>,
}

pub struct AdventureStats {
    pub moves_done: i32,
}

pub struct Adventure {
    scenes: HashMap<String, Scene>,
    current_scene: String,
    global_commands: HashMap<String, CommandJson>,
    log: Vec<String>,

    pub input_buffer: String,

    pub flags: HashMap<String, bool>,

    pub is_playing_audio: bool,

    pub autocomplete_matches: Vec<String>,
    pub autocomplete_index: usize,

    pub log_scroll: u16,
    pub auto_scroll: bool,

    pub art_shown: bool,

    pub stats: AdventureStats,
}

enum CommandSource {
    Scene,
    Global,
}

impl Adventure {
    pub fn new() -> Self {
        let file =
            std::fs::read_to_string("data/adventure.json").expect("Could not read adventure.json");

        let root: AdventureJsonRoot = serde_json::from_str(&file).expect("Invalid adventure.json");

        let first_scene_id = root.scenes.first().expect("No scenes in JSON").id.clone();

        let mut scenes = HashMap::new();

        for s in root.scenes {
            let scene_image = s
                .scene_image
                .as_ref()
                .and_then(|img_path| image_utils::load_scene_image(img_path).ok())
                .map(RefCell::new);

            scenes.insert(
                s.id.clone(),
                Scene {
                    enter_text: s.scene_enter,
                    scene_art: s.scene_art,
                    scene_image,
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
            art_shown: false,
            stats: AdventureStats { moves_done: 0 },
            global_commands: root.global_commands,
            is_playing_audio: false,
            flags: HashMap::new(),
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
        self.stats.moves_done = 0;
        self.flags.clear();

        let first = &self.scenes[&first_scene_id];

        self.log.push(first.enter_text.clone());
        self.update_autocomplete();
    }

    pub fn inventory(&self) -> Vec<&'static str> {
        vec!["📱"]
    }

    pub fn current_scene(&self) -> &Scene {
        &self.scenes[&self.current_scene]
    }

    fn all_commands(&self) -> Vec<String> {
        let mut cmds: Vec<String> = self.scenes[&self.current_scene]
            .commands
            .keys()
            .cloned()
            .collect();

        cmds.extend(self.global_commands.keys().cloned());

        cmds.sort();
        cmds.dedup();
        cmds
    }

    pub fn update_autocomplete(&mut self) {
        let input = self.input_buffer.to_lowercase();
        self.autocomplete_matches = self
            .all_commands()
            .into_iter()
            .filter(|cmd| cmd.starts_with(&input))
            .collect();

        // TODO: merge with global commands
        self.autocomplete_index = 0;
    }

    pub fn autocomplete_suggestion(&self) -> Option<&str> {
        // TODO: merge with global commands
        self.autocomplete_matches
            .get(self.autocomplete_index)
            .map(|s| s.as_str())
    }

    fn run_actions(&mut self, actions: &[CommandAction]) {
        for cmd in actions {
            if let Some(req) = &cmd.requires_flag {
                if !self.flags.get(req).cloned().unwrap_or(false) {
                    // If the requirement isn't met, skip THIS action
                    // (Optional: log a message saying "I can't do that yet")

                    continue;
                }
            }

            match cmd.action.as_str() {
                "check_logic" => {
                    if let Some(reqs) = &cmd.requires_all {
                        if self.check_flags(reqs) {
                            // Success: Run the nested 'actions'
                            if let Some(success_branch) = &cmd.actions {
                                self.run_actions(success_branch);
                            }
                        } else {
                            // Failure: Run 'else_actions'
                            if let Some(fail_branch) = &cmd.else_actions {
                                self.run_actions(fail_branch);
                            }
                        }
                    }
                }
                "log" => {
                    self.log.push(cmd.text.clone().unwrap_or_default());
                }
                "set_flag" => {
                    if let Some(flag_name) = &cmd.set_flag {
                        self.flags.insert(flag_name.clone(), true);
                    }
                }
                "change_scene" => {
                    let target = cmd.target.as_ref().unwrap();

                    self.current_scene = target.clone();

                    let new_scene = self.scenes.get(target).unwrap();

                    self.log.push(new_scene.enter_text.clone());
                }
                "show_scene_art" => {
                    self.art_shown = true;
                }
                "sound" => {
                    if let Some(file_path) = &cmd.file_path {
                        Adventure::play_sound(self, file_path);
                    }
                }
                "die" => {
                    let reason = cmd.reason.clone().unwrap_or("You died".to_string());
                    self.log.push(format!("GAME OVER: {}", reason));
                }
                "check_logic" => {
                    if let Some(reqs) = &cmd.requires_all {
                        if self.check_flags(reqs) {
                            // Success: Run the nested 'actions'
                            if let Some(success_branch) = &cmd.actions {
                                self.run_actions(success_branch);
                            }
                        } else {
                            // Failure: Run 'else_actions'
                            if let Some(fail_branch) = &cmd.else_actions {
                                self.run_actions(fail_branch);
                            }
                        }
                    }
                }
                _ => self
                    .log
                    .push("Ik weet niet wat ik hiermee moet..".to_string()),
            }
        }

        self.stats.moves_done += 1;
    }

    pub fn check_flags(&self, requirements: &[String]) -> bool {
        requirements
            .iter()
            .all(|flag| *self.flags.get(flag).unwrap_or(&false))
    }

    fn play_sound(&mut self, file_path: &str) {
        self.is_playing_audio = true;

        // We don't spawn a thread here because you want it to block
        let mut stream_handle = match OutputStreamBuilder::open_default_stream() {
            Ok(h) => h,
            Err(_) => {
                self.is_playing_audio = false;
                return;
            }
        };
        stream_handle.log_on_drop(false);

        let sink = Sink::connect_new(stream_handle.mixer());
        let file = match File::open(file_path) {
            Ok(f) => BufReader::new(f),
            Err(_) => {
                self.is_playing_audio = false;
                return;
            }
        };

        if let Ok(source) = Decoder::try_from(file) {
            sink.append(source);
            // This blocks the thread
            sink.sleep_until_end();
        }

        self.is_playing_audio = false;
    }

    fn process_command(&mut self, input: &str) {
        let input = input.trim().to_lowercase();
        if input.is_empty() {
            return;
        }

        self.log.push(format!("> {}", input));

        // 1. Try to get the command from the Scene first
        let scene_cmd = self
            .scenes
            .get(&self.current_scene)
            .and_then(|s| s.commands.get(&input).cloned());

        if let Some(cmd_json) = scene_cmd {
            // EXECUTE SCENE COMMAND
            self.handle_command_json(&input, cmd_json, CommandSource::Scene);
        } else {
            // 2. Fallback: Try Global commands
            let global_cmd = self.global_commands.get(&input).cloned();

            if let Some(cmd_json) = global_cmd {
                // EXECUTE GLOBAL COMMAND
                self.handle_command_json(&input, cmd_json, CommandSource::Global);
            } else {
                // 3. Final Fallback: Error message
                self.log
                    .push("Ik weet niet wat ik hiermee moet.".to_string());
            }
        }

        self.auto_scroll = true;
        self.log_scroll = 0;
        self.update_autocomplete();
    }

    pub fn scroll_up(&mut self) {
        self.auto_scroll = false;
        self.log_scroll = self.log_scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.log_scroll = self.log_scroll.saturating_add(1);

        let total_lines = self.total_log_lines();
        if self.log_scroll as usize >= total_lines {
            self.auto_scroll = true;
        }
    }

    pub fn total_log_lines(&self) -> usize {
        self.log.iter().map(|entry| entry.lines().count()).sum()
    }

    fn handle_command_json(&mut self, input: &str, cmd_json: CommandJson, source: CommandSource) {
        match cmd_json {
            CommandJson::Simple(actions) => {
                self.run_actions(&actions);
            }
            CommandJson::Wrapped {
                once,
                actions,
                fallback,
            } => {
                // Run the primary actions
                self.run_actions(&actions);

                // If it's a 'once' command, remove it or replace it with fallback
                if once.unwrap_or(false) {
                    let next_val = fallback.map(CommandJson::Simple);

                    match (source, next_val) {
                        // Replace with fallback actions if they exist, otherwise remove entirely
                        (CommandSource::Scene, Some(fb)) => {
                            self.scenes
                                .get_mut(&self.current_scene)
                                .unwrap()
                                .commands
                                .insert(input.to_string(), fb);
                        }
                        (CommandSource::Scene, None) => {
                            self.scenes
                                .get_mut(&self.current_scene)
                                .unwrap()
                                .commands
                                .remove(input);
                        }
                        (CommandSource::Global, Some(fb)) => {
                            self.global_commands.insert(input.to_string(), fb);
                        }
                        (CommandSource::Global, None) => {
                            self.global_commands.remove(input);
                        }
                    }
                }
            }
        }
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
            KeyCode::Up => self.scroll_up(),
            KeyCode::Down => self.scroll_down(),
            _ => {}
        }
    }
}

impl Adventure {
    pub fn log(&self) -> &Vec<String> {
        &self.log
    }

    pub fn input(&self) -> &str {
        &self.input_buffer
    }
}
