use std::{fs, path::Path, sync::RwLock};
use serde::{Deserialize, Serialize};

pub struct Config {
    pub path: String,
    pub inner: RwLock<ConfigInner>,
}

impl Config {
    pub fn new(path: Option<String>) -> Self {
        let this = Self {
            path: path.unwrap_or_else(|| "config.json".to_string()),
            inner: RwLock::new(Default::default()),
        };
        this.load();
        this
    }

    pub fn load(&self) {
        if !self.config_exists() {
            self.save();
            return;
        }

        let mut lock = self.inner.write().unwrap();
        let content = fs::read_to_string(&self.path).unwrap();
        *lock = serde_json::from_str(&content).unwrap();
    }
    
    pub fn save(&self) {
        let lock = self.inner.read().unwrap();
        let content = serde_json::to_string(&*lock).unwrap();
        fs::write(&self.path, content).unwrap();
    }

    fn config_exists(&self) -> bool {
        Path::new(self.path.as_str()).exists()
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ConfigInner {
    pub username: Option<String>,
    pub password: Option<String>,
}
