use super::UnpackScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for UnpackScreen {
    fn on_screen_changed(&mut self) {
        if !self.libs.shared_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
            return;
        }

        let progress_state = self.progress_state.clone();
        let libs = self.libs.clone();

        self.cancel();
        self.handle = Some(tokio::spawn(UnpackScreen::unpack(progress_state, libs)));
    }

    fn on_exit(&mut self) { self.cancel(); }
}
