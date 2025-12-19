use crate::vnscript::{Runtime, Statement};
use std::collections::HashMap;

pub struct GameState {
    pub runtime: Runtime,
    pub current_scene: String,
    pub statements: Vec<Statement>,
    pub characters: HashMap<String, CharacterState>,
    pub background: Option<String>,
    pub current_music: Option<String>,
    pub paused: bool,
}

#[derive(Debug, Clone)]
pub struct CharacterState {
    pub name: String,
    pub sprite: Option<String>,
    pub position: CharacterPosition,
    pub visible: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum CharacterPosition {
    Left,
    Center,
    Right,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
            current_scene: String::from("main"),
            statements: Vec::new(),
            characters: HashMap::new(),
            background: None,
            current_music: None,
            paused: false,
        }
    }
    
    pub fn load_script(&mut self, statements: Vec<Statement>) {
        self.statements = statements;
        self.runtime.index_labels(&self.statements);
    }
    
    pub fn get_current_statement(&self) -> Option<&Statement> {
        self.statements.get(self.runtime.current_index)
    }
    
    pub fn advance(&mut self) {
        if self.runtime.current_index < self.statements.len() {
            self.runtime.current_index += 1;
        }
    }
    
    pub fn is_finished(&self) -> bool {
        self.runtime.current_index >= self.statements.len()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
