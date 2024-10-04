use super::DownloadScreen;
use crate::screens::{Screen, ScreenEventsTrait};

impl ScreenEventsTrait for DownloadScreen {
    fn on_screen_changed(&mut self) {
        if !self.libs.in_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
            return;
        }

        let libs = self.libs.clone();
        let progress_state = self.progress_state.clone();
        self.handle = Some(tokio::spawn(DownloadScreen::download(libs, progress_state)));
    }

    fn on_exit(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}
