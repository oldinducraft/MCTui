use std::fs;
use std::path::PathBuf;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION, CONFIG_FILENAME, ORGANIZATION, QUALIFIER};

pub struct Config {
    inner:       RwLock<ConfigInner>,
    config_path: PathBuf,
    config_dir:  PathBuf,

    pub data_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).expect("Failed to get project dirs");
        let config_dir = dirs.config_dir().to_path_buf();
        let config_path = config_dir.join(CONFIG_FILENAME);

        let this = Self {
            inner: RwLock::new(Default::default()),
            config_dir,
            config_path,

            data_dir: dirs.data_dir().to_path_buf(),
        };

        this.create_config();
        this.reload();
        this
    }

    pub fn reload(&self) {
        let mut lock = self.write();
        let content =
            fs::read_to_string(&self.config_path).unwrap_or_else(|err| panic!("Failed to read config: {}", err));
        *lock = serde_json::from_str(&content).unwrap_or_else(|err| panic!("Failed to parse config: {}", err));
    }

    pub fn save(&self) {
        let lock = self.read();
        let content = serde_json::to_string(&*lock).unwrap_or_else(|err| panic!("Failed to serialize config: {}", err));
        fs::write(&self.config_path, content).unwrap_or_else(|err| panic!("Failed to write config: {}", err));
    }

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

    fn create_config(&self) {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir).unwrap_or_else(|err| panic!("Failed to create config dir: {}", err));
        }

        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir).unwrap_or_else(|err| panic!("Failed to create data dir: {}", err));
        }

        if !self.config_path.exists() {
            self.save();
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ConfigInner {
    pub username: Option<String>,
    pub password: Option<String>,
}
