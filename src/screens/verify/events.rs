use super::VerifyScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for VerifyScreen {
    fn on_screen_changed(&mut self) {
        if !self.libs.shared_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
            return;
        }

        let libs = self.libs.clone();

        self.cancel();
        self.handle = Some(tokio::spawn(VerifyScreen::verify(libs)));
    }

    fn on_exit(&mut self) { self.cancel(); }

    fn on_tick(&mut self) { self.loader_state.on_tick(); }
}
