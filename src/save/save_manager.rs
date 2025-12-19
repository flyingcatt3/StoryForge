use crate::engine::GameState;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub timestamp: DateTime<Utc>,
    pub scene: String,
    pub index: usize,
    pub variables: Vec<(String, String)>,
}

pub struct SaveManager {
    save_dir: PathBuf,
}

impl SaveManager {
    pub fn new(save_dir: impl AsRef<Path>) -> Self {
        let save_dir = save_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&save_dir).ok();
        
        Self { save_dir }
    }
    
    pub fn save(&self, slot: usize, game_state: &GameState) -> Result<()> {
        let save_data = SaveData {
            timestamp: Utc::now(),
            scene: game_state.current_scene.clone(),
            index: game_state.runtime.current_index,
            variables: Vec::new(),
        };
        
        let path = self.save_dir.join(format!("save_{}.bin", slot));
        let bytes = bincode::serialize(&save_data)?;
        std::fs::write(path, bytes)?;
        
        Ok(())
    }
    
    pub fn load(&self, slot: usize) -> Result<SaveData> {
        let path = self.save_dir.join(format!("save_{}.bin", slot));
        let bytes = std::fs::read(path)?;
        let data = bincode::deserialize(&bytes)?;
        
        Ok(data)
    }
    
    pub fn list_saves(&self) -> Result<Vec<(usize, SaveData)>> {
        let mut saves = Vec::new();
        
        for entry in std::fs::read_dir(&self.save_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("save_") && name.ends_with(".bin") {
                    if let Some(slot_str) = name.strip_prefix("save_").and_then(|s| s.strip_suffix(".bin")) {
                        if let Ok(slot) = slot_str.parse::<usize>() {
                            if let Ok(data) = self.load(slot) {
                                saves.push((slot, data));
                            }
                        }
                    }
                }
            }
        }
        
        saves.sort_by_key(|(slot, _)| *slot);
        Ok(saves)
    }
}
