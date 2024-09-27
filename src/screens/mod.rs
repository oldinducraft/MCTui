use std::sync::Arc;

use crossterm::event::KeyEvent;
use ratatui::Frame;
use tokio::time::Instant;

use crate::utils::Libs;

pub mod authenticate;
pub mod home;
pub mod login;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum Screen {
    Login,
    Home,
    #[default]
    Authenticate,
}

pub trait ScreenTrait {
    fn new(libs: Arc<Libs>) -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame);

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        let _ = event;
        Some(())
    }

    fn on_tick(&mut self, instant: Instant) { let _ = instant; }
}
