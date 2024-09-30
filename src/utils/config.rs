use std::fs;
use std::path::Path;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::constants::CONFIG_PATH;

pub struct Config {
    inner: RwLock<ConfigInner>,
}

impl Config {
    pub fn new() -> Self {
        let this = Self {
            inner: RwLock::new(Default::default()),
        };
        this.reload();
        this
    }

    pub fn reload(&self) {
        if !self.config_exists() {
            self.save();
            return;
        }

        let mut lock = self.inner.write().unwrap();
        let content = fs::read_to_string(CONFIG_PATH).unwrap();
        *lock = serde_json::from_str(&content).unwrap();
    }

    pub fn save(&self) {
        let lock = self.inner.read().unwrap();
        let content = serde_json::to_string(&*lock).unwrap();
        fs::write(CONFIG_PATH, content).unwrap();
    }

    fn config_exists(&self) -> bool { Path::new(CONFIG_PATH).exists() }

    pub fn set_username(&self, username: Option<String>) {
        let mut lock = self.inner.write().unwrap();
        lock.username = username;
    }

    pub fn set_password(&self, password: Option<String>) {
        let mut lock = self.inner.write().unwrap();
        lock.password = password;
    }

    pub fn get_username(&self) -> Option<String> { self.inner.read().unwrap().username.clone() }
    
    pub fn get_password(&self) -> Option<String> { self.inner.read().unwrap().password.clone() }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ConfigInner {
    pub username: Option<String>,
    pub password: Option<String>,
}
