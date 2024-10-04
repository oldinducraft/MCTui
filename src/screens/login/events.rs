use crossterm::event::{KeyCode, KeyEvent};

use super::LoginScreen;
use crate::screens::{Screen, ScreenEventsTrait};

impl ScreenEventsTrait for LoginScreen {
    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        match event.code {
            KeyCode::Char(c) => self.form.add_char(c),
            KeyCode::Backspace => self.form.remove_char(),
            KeyCode::Tab => self.form.next_field(),
            KeyCode::Enter => self.submit(),
            _ => return Some(()),
        };

        None
    }

    fn on_screen_changed(&mut self) {
        let screen = self.libs.screen.get_current();

        if let Screen::Login(Some(err)) = screen {
            self.error = Some(err);
        }
    }
}
