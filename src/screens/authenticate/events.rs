use super::AuthenticateScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for AuthenticateScreen {
    fn on_tick(&mut self) { self.loader_state.on_tick(); }

    fn on_screen_changed(&mut self) {
        if self.libs.config.get_username().is_none() {
            self.libs.screen.goto(Screen::Login(None));
            return;
        }

        self.handle = Some(AuthenticateScreen::spawn_auth(self.libs.clone()));
    }

    fn on_exit(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}
