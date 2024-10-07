use crossterm::event::{KeyCode, KeyEvent};

use super::LoginScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for LoginScreen {
    fn on_key_pressed(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.form.add_char(c),
            KeyCode::Backspace => self.form.remove_char(),
            KeyCode::Tab => self.form.next_field(),
            KeyCode::Enter => self.submit(),
            _ => {},
        };
    }

    fn on_screen_changed(&mut self) {
        let screen = self.libs.screen.get_current();

        if let Screen::Login(Some(err)) = screen {
            self.error = Some(err);
        }
    }
}
