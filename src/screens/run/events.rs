use super::RunScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for RunScreen {
    fn on_screen_changed(&mut self) {
        if !self.libs.shared_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
            return;
        }

        let libs = self.libs.clone();
        let lines = self.lines.clone();
        self.handle = Some(tokio::spawn(RunScreen::run(libs, lines)));
    }

    fn on_exit(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }

    fn on_tick(&mut self) { self.loader_state.on_tick(); }
}
