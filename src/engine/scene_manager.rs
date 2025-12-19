use crate::vnscript::Statement;
use std::collections::HashMap;

pub struct SceneManager {
    scenes: HashMap<String, Vec<Statement>>,
    current_scene: Option<String>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
        }
    }
    
    pub fn add_scene(&mut self, name: String, statements: Vec<Statement>) {
        self.scenes.insert(name, statements);
    }
    
    pub fn get_scene(&self, name: &str) -> Option<&Vec<Statement>> {
        self.scenes.get(name)
    }
    
    pub fn set_current_scene(&mut self, name: String) {
        self.current_scene = Some(name);
    }
    
    pub fn get_current_scene(&self) -> Option<&Vec<Statement>> {
        if let Some(name) = &self.current_scene {
            self.get_scene(name)
        } else {
            None
        }
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
