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
    pub fn get_current(&self) -> Screen { self.inner.get().unwrap() }

    pub fn goto(&self, screen: Screen) {
        self.inner.set(screen).unwrap();
        self.screen_changed.set(true).unwrap();
    }

    pub fn handle_screen_changed(&self) -> bool {
        let changed = self.screen_changed.get().unwrap();
        self.screen_changed.set(false).unwrap();
        changed
    }
}
