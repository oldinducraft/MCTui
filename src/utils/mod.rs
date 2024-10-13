use config::Config;
use screen_manager::ScreenManager;
use shared_memory::SharedMemory;

pub mod config;
pub mod immediate_rw_lock;
pub mod requester;
pub mod screen_manager;
pub mod shared_memory;
pub mod ui;
pub mod yggdrasil;
pub mod minecraft;

pub struct Libs {
    pub screen:        ScreenManager,
    pub config:        Config,
    pub shared_memory: SharedMemory,
}

impl Libs {
    pub fn new() -> Self {
        Self {
            screen:        ScreenManager::default(),
            config:        Config::new(),
            shared_memory: SharedMemory::new(),
        }
    }
}
