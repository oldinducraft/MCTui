use super::immediate_rw_lock::ImmediateRwLock;
use crate::screens::Screen;

pub struct ScreenManager {
    inner:          ImmediateRwLock<Screen>,
    screen_changed: ImmediateRwLock<bool>,
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self {
            inner:          ImmediateRwLock::new(Screen::default()),
            screen_changed: ImmediateRwLock::new(true),
        }
    }
}

impl ScreenManager {
    pub fn get_current(&self) -> Screen {
        self.inner
            .get()
            .unwrap_or_else(|err| panic!("Failed to get current screen: {}", err))
    }

    pub fn goto(&self, screen: Screen) {
        self.inner
            .set(screen)
            .unwrap_or_else(|err| panic!("Failed to set current screen: {}", err));
        self.screen_changed
            .set(true)
            .unwrap_or_else(|err| panic!("Failed to set screen changed: {}", err));
    }

    pub fn handle_screen_changed(&self) -> bool {
        let changed = self
            .screen_changed
            .get()
            .unwrap_or_else(|err| panic!("Failed to get screen changed: {}", err));
        self.screen_changed
            .set(false)
            .unwrap_or_else(|err| panic!("Failed to set screen changed: {}", err));
        changed
    }
}
