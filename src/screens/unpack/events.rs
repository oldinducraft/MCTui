use super::UnpackScreen;
use crate::screens::{Screen, ScreenEventsTrait};

impl ScreenEventsTrait for UnpackScreen {
    fn on_screen_changed(&mut self) {
        if !self.libs.in_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
            return;
        }

        let progress_state = self.progress_state.clone();
        let libs = self.libs.clone();

        self.handle = Some(tokio::spawn(UnpackScreen::unpack(progress_state, libs)));
    }

    fn on_exit(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}
