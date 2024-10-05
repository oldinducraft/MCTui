use std::fs;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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

        let mut lock = self
            .inner
            .write()
            .unwrap_or_else(|err| panic!("Failed to lock config: {}", err));
        let content = fs::read_to_string(CONFIG_PATH).unwrap_or_else(|err| panic!("Failed to read config: {}", err));
        *lock = serde_json::from_str(&content).unwrap_or_else(|err| panic!("Failed to parse config: {}", err));
    }

    pub fn save(&self) {
        let lock = self.read();
        let content = serde_json::to_string(&*lock).unwrap_or_else(|err| panic!("Failed to serialize config: {}", err));
        fs::write(CONFIG_PATH, content).unwrap_or_else(|err| panic!("Failed to write config: {}", err));
    }

    fn config_exists(&self) -> bool { Path::new(CONFIG_PATH).exists() }

    pub fn set_username(&self, username: Option<String>) { self.write().username = username; }

    pub fn set_password(&self, password: Option<String>) { self.write().password = password; }

    pub fn get_username(&self) -> Option<String> { self.read().username.clone() }

    pub fn get_password(&self) -> Option<String> { self.read().password.clone() }

    fn write(&self) -> RwLockWriteGuard<ConfigInner> {
        self.inner
            .write()
            .unwrap_or_else(|err| panic!("Failed to lock config (write): {}", err))
    }

    fn read(&self) -> RwLockReadGuard<ConfigInner> {
        self.inner
            .read()
            .unwrap_or_else(|err| panic!("Failed to lock config (read): {}", err))
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ConfigInner {
    pub username: Option<String>,
    pub password: Option<String>,
}
