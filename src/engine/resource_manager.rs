use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use image::DynamicImage;

pub struct ResourceManager {
    base_path: PathBuf,
    image_cache: HashMap<String, Arc<DynamicImage>>,
    audio_cache: HashMap<String, Arc<Vec<u8>>>,
}

impl ResourceManager {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            image_cache: HashMap::new(),
            audio_cache: HashMap::new(),
        }
    }
    
    pub fn load_image(&mut self, path: &str) -> Result<Arc<DynamicImage>> {
        if let Some(cached) = self.image_cache.get(path) {
            return Ok(Arc::clone(cached));
        }
        
        let full_path = self.base_path.join(path);
        let image = image::open(full_path)?;
        let arc_image = Arc::new(image);
        self.image_cache.insert(path.to_string(), Arc::clone(&arc_image));
        
        Ok(arc_image)
    }
    
    pub fn load_audio(&mut self, path: &str) -> Result<Arc<Vec<u8>>> {
        if let Some(cached) = self.audio_cache.get(path) {
            return Ok(Arc::clone(cached));
        }
        
        let full_path = self.base_path.join(path);
        let data = std::fs::read(full_path)?;
        let arc_data = Arc::new(data);
        self.audio_cache.insert(path.to_string(), Arc::clone(&arc_data));
        
        Ok(arc_data)
    }
    
    pub fn clear_cache(&mut self) {
        self.image_cache.clear();
        self.audio_cache.clear();
    }
}
