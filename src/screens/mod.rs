use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crossterm::event::KeyEvent;
use login::types::ErrorMessage;
use ratatui::Frame;

use crate::utils::Libs;

pub mod authenticate;
pub mod download;
pub mod home;
pub mod login;
pub mod unpack;
pub mod verify;

#[derive(Default, Clone, Debug)]
pub enum Screen {
    Login(ErrorMessage),
    Home,
    #[default]
    Authenticate,
    Download,
    Unpack,
    Verify,
}

impl Hash for Screen {
    fn hash<H: Hasher>(&self, state: &mut H) { std::mem::discriminant(self).hash(state); }
}

impl PartialEq for Screen {
    fn eq(&self, other: &Self) -> bool { std::mem::discriminant(self) == std::mem::discriminant(other) }
}

impl Eq for Screen {}

pub trait ScreenTrait: RenderableScreenTrait + ScreenEventsTrait + CreatableScreenTrait {}

pub trait RenderableScreenTrait {
    fn render(&mut self, frame: &mut Frame);
}

pub trait ScreenEventsTrait {
    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        let _ = event;
        Some(())
    }

    fn on_tick(&mut self) {}

    fn on_screen_changed(&mut self) {}

    fn on_exit(&mut self) {}
}

pub trait CreatableScreenTrait {
    fn new(libs: Arc<Libs>) -> Self
    where
        Self: Sized;
}
