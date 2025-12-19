use eframe::egui;
use crate::engine::{GameState, ResourceManager};
use crate::media::AudioPlayer;
use crate::save::SaveManager;
use crate::vnscript::{Parser, Statement};
use directories::ProjectDirs;

pub struct StoryForgeApp {
    game_state: GameState,
    resource_manager: ResourceManager,
    audio_player: AudioPlayer,
    save_manager: SaveManager,
    
    // UI state
    current_screen: Screen,
    dialogue_text: String,
    character_name: String,
    show_dialogue_box: bool,
    waiting_for_input: bool,
    
    // Settings
    text_speed: f32,
    volume: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum Screen {
    MainMenu,
    Game,
    Settings,
    Load,
}

impl StoryForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let proj_dirs = ProjectDirs::from("com", "storyforge", "storyforge")
            .expect("Failed to get project directories");
        
        let save_dir = proj_dirs.data_dir();
        let resource_path = std::env::current_dir().unwrap_or_default();
        
        Self {
            game_state: GameState::new(),
            resource_manager: ResourceManager::new(resource_path),
            audio_player: AudioPlayer::new().unwrap_or_default(),
            save_manager: SaveManager::new(save_dir),
            
            current_screen: Screen::MainMenu,
            dialogue_text: String::new(),
            character_name: String::new(),
            show_dialogue_box: false,
            waiting_for_input: false,
            
            text_speed: 50.0,
            volume: 70.0,
        }
    }
    
    fn load_demo_script(&mut self) {
        let script = r#"
            @scene demo
            @background "demo_bg.jpg"
            @music "demo_music.mp3"
            
            @label start
            narrator: "Welcome to StoryForge!"
            narrator: "This is a demo of the new Rust-based visual novel engine."
            
            @show alice at center
            alice: "Hello! I'm Alice, and I'll be your guide today."
            
            @choice "What would you like to do?"
                @option "Learn more" -> learn_more
                @option "Start game" -> start_game
            
            @label learn_more
            alice: "StoryForge is built with Rust for performance and safety."
            alice: "It uses VNScript, a domain-specific language for visual novels."
            @jump start
            
            @label start_game
            alice: "Great! Let's begin your adventure!"
            @exit
        "#;
        
        match Parser::new(script).parse() {
            Ok(statements) => {
                self.game_state.load_script(statements);
                log::info!("Demo script loaded successfully");
            }
            Err(e) => {
                log::error!("Failed to parse demo script: {}", e);
            }
        }
    }
    
    fn show_main_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                ui.heading(egui::RichText::new("StoryForge").size(60.0));
                ui.label(egui::RichText::new("Visual Novel Engine").size(24.0));
                
                ui.add_space(50.0);
                
                if ui.button(egui::RichText::new("Start Demo").size(24.0)).clicked() {
                    self.load_demo_script();
                    self.current_screen = Screen::Game;
                    self.process_next_statement();
                }
                
                ui.add_space(10.0);
                
                if ui.button(egui::RichText::new("Load Game").size(24.0)).clicked() {
                    self.current_screen = Screen::Load;
                }
                
                ui.add_space(10.0);
                
                if ui.button(egui::RichText::new("Settings").size(24.0)).clicked() {
                    self.current_screen = Screen::Settings;
                }
                
                ui.add_space(10.0);
                
                if ui.button(egui::RichText::new("Exit").size(24.0)).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
    
    fn show_game_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Background would be rendered here
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                0.0,
                egui::Color32::from_rgb(20, 20, 40),
            );
            
            // Character sprites would be rendered here
            
            // Dialogue box
            if self.show_dialogue_box {
                egui::Area::new(egui::Id::new("dialogue_box"))
                    .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -50.0])
                    .show(ctx, |ui| {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 200))
                            .rounding(10.0)
                            .inner_margin(20.0)
                            .show(ui, |ui| {
                                ui.set_width(ui.ctx().screen_rect().width() * 0.8);
                                
                                if !self.character_name.is_empty() {
                                    ui.label(egui::RichText::new(&self.character_name)
                                        .size(20.0)
                                        .color(egui::Color32::from_rgb(100, 150, 255)));
                                }
                                
                                ui.label(egui::RichText::new(&self.dialogue_text).size(18.0));
                            });
                    });
            }
            
            // Click to advance
            if self.waiting_for_input && ui.input(|i| i.pointer.any_click()) {
                self.process_next_statement();
            }
        });
    }
    
    fn show_settings(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Settings");
                
                ui.add_space(50.0);
                
                ui.horizontal(|ui| {
                    ui.label("Volume:");
                    if ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0)).changed() {
                        self.audio_player.set_music_volume(self.volume);
                    }
                });
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("Text Speed:");
                    ui.add(egui::Slider::new(&mut self.text_speed, 10.0..=100.0));
                });
                
                ui.add_space(50.0);
                
                if ui.button(egui::RichText::new("Back").size(20.0)).clicked() {
                    self.current_screen = Screen::MainMenu;
                }
            });
        });
    }
    
    fn show_load_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Load Game");
                
                ui.add_space(50.0);
                
                match self.save_manager.list_saves() {
                    Ok(saves) => {
                        if saves.is_empty() {
                            ui.label("No save files found");
                        } else {
                            for (slot, save_data) in saves {
                                if ui.button(format!("Slot {} - {}", slot, save_data.timestamp)).clicked() {
                                    // Load game logic would go here
                                    log::info!("Loading slot {}", slot);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        ui.label(format!("Error loading saves: {}", e));
                    }
                }
                
                ui.add_space(50.0);
                
                if ui.button(egui::RichText::new("Back").size(20.0)).clicked() {
                    self.current_screen = Screen::MainMenu;
                }
            });
        });
    }
    
    fn process_next_statement(&mut self) {
        self.waiting_for_input = false;
        self.show_dialogue_box = false;
        
        if self.game_state.is_finished() {
            self.current_screen = Screen::MainMenu;
            return;
        }
        
        if let Some(statement) = self.game_state.get_current_statement() {
            match statement {
                Statement::Dialogue(dialogue) => {
                    self.character_name = dialogue.character.clone().unwrap_or_default();
                    self.dialogue_text = dialogue.text.clone();
                    self.show_dialogue_box = true;
                    self.waiting_for_input = true;
                }
                Statement::Background(bg) => {
                    log::info!("Loading background: {}", bg.image);
                    // Would load and display background here
                }
                Statement::Music(music) => {
                    if let Some(file) = &music.file {
                        log::info!("Playing music: {}", file);
                        // Would play music here
                    }
                }
                Statement::Show(show) => {
                    log::info!("Showing character: {}", show.character);
                    // Would show character sprite here
                }
                Statement::Hide(_) => {
                    log::info!("Hiding character");
                    // Would hide character sprite here
                }
                Statement::Label(_) => {
                    // Labels are just markers, skip them
                }
                Statement::Exit => {
                    self.current_screen = Screen::MainMenu;
                    return;
                }
                _ => {
                    log::info!("Unhandled statement: {:?}", statement);
                }
            }
            
            self.game_state.advance();
            
            // If we didn't hit a dialogue, process the next statement
            if !self.waiting_for_input {
                self.process_next_statement();
            }
        }
    }
}

impl eframe::App for StoryForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_screen {
            Screen::MainMenu => self.show_main_menu(ctx),
            Screen::Game => self.show_game_screen(ctx),
            Screen::Settings => self.show_settings(ctx),
            Screen::Load => self.show_load_screen(ctx),
        }
    }
}
