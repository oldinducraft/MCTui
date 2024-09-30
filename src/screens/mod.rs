use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Instant;

use authenticate::arg::CalledAt;
use crossterm::event::KeyEvent;
use login::arg::ErrorMessage;
use ratatui::Frame;

use crate::utils::Libs;

pub mod authenticate;
pub mod home;
pub mod login;
pub mod download;
pub mod unpack;

#[derive(Clone)]
pub enum Screen {
    Login(ErrorMessage),
    Home,
    Authenticate(CalledAt),
    Download,
    Unpack
}

impl Default for Screen {
    fn default() -> Self { Self::Authenticate(Instant::now()) }
}

impl Hash for Screen {
    fn hash<H: Hasher>(&self, state: &mut H) { std::mem::discriminant(self).hash(state); }
}

impl PartialEq for Screen {
    fn eq(&self, other: &Self) -> bool { std::mem::discriminant(self) == std::mem::discriminant(other) }
}

impl Eq for Screen {}

pub trait ScreenTrait {
    fn new(libs: Arc<Libs>) -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame);

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        let _ = event;
        Some(())
    }

    fn on_tick(&mut self) {}
}
