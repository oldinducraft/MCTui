use std::sync::Arc;

use super::{Screen, ScreenTrait};
use crate::utils::Libs;

pub mod event;
pub mod ui;

pub struct HomeScreen {
    libs: Arc<Libs>,
}

impl ScreenTrait for HomeScreen {}

impl HomeScreen {
    pub fn new(libs: Arc<Libs>) -> HomeScreen { HomeScreen { libs } }
}

impl HomeScreen {
    pub fn log_out(&self) {
        self.libs.config.set_username(None);
        self.libs.config.set_password(None);
        self.libs.config.save();
        self.libs.screen.goto(Screen::Login(None));
    }
}
