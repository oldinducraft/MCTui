use crossterm::event::KeyCode;

use super::HomeScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for HomeScreen {
    fn on_key_pressed(&mut self, event: crossterm::event::KeyEvent) {
        match event.code {
            KeyCode::Enter => self.libs.screen.goto(Screen::Verify),
            KeyCode::Delete => self.log_out(),
            _ => {},
        };
    }

    fn on_screen_changed(&mut self) {
        if !self.libs.shared_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
        }
    }
}
