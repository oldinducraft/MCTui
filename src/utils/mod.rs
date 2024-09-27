use std::sync::RwLock;

use config::Config;
use immediate_rw_lock::ImmediateRwLock;
use in_memory::InMemory;

use crate::screens::Screen;

pub mod config;
pub mod immediate_rw_lock;
pub mod in_memory;
pub mod ui;
pub mod yggdrasil;

pub struct Libs {
    pub screen:    ImmediateRwLock<Screen>,
    pub config:    Config,
    pub in_memory: RwLock<InMemory>,
}

impl Libs {
    pub fn new() -> Self {
        Self {
            screen:    ImmediateRwLock::default(),
            config:    Config::new(),
            in_memory: RwLock::new(InMemory::default()),
        }
    }
}
