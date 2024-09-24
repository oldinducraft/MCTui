use crossterm::event::KeyEvent;
use ratatui::Frame;

pub mod home;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Screen {
    Home,
}

pub trait ScreenTrait {
    fn render(&mut self, frame: &mut Frame);
    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        let _ = event;
        Some(())
    }
}
