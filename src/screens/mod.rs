use std::sync::Arc;

use crossterm::event::KeyEvent;
use ratatui::Frame;
use tokio::time::Instant;

use crate::utils::config::Config;
use crate::utils::immediate_rw_lock::ImmediateRwLock;

pub mod login;
pub mod home;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum Screen {
    #[default]
    Login,
    Home,
}

pub trait ScreenTrait {
    fn new(current_screen: Arc<ImmediateRwLock<Screen>>, config: Arc<Config>) -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame);

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        let _ = event;
        Some(())
    }

    fn on_tick(&mut self, instant: Instant) { let _ = instant; }
}
