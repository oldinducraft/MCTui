use config::Config;
use in_memory::InMemory;
use screen_manager::ScreenManager;

pub mod config;
pub mod immediate_rw_lock;
pub mod in_memory;
pub mod screen_manager;
pub mod ui;
pub mod yggdrasil;

pub struct Libs {
    pub screen:    ScreenManager,
    pub config:    Config,
    pub in_memory: InMemory,
}

impl Libs {
    pub fn new() -> Self {
        Self {
            screen:    ScreenManager::default(),
            config:    Config::new(),
            in_memory: InMemory::new(),
        }
    }
}
