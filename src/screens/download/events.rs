use super::DownloadScreen;
use crate::screens::{Screen, ScreenEvents};

impl ScreenEvents for DownloadScreen {
    fn on_screen_changed(&mut self) {
        if !self.libs.shared_memory.auth_args_are_set() {
            self.libs.screen.goto(Screen::Authenticate);
            return;
        }

        self.cancel();
        let libs = self.libs.clone();
        let progress_state = self.progress_state.clone();
        self.handle = Some(tokio::spawn(DownloadScreen::download(libs, progress_state)));
    }

    fn on_exit(&mut self) { self.cancel(); }
}
