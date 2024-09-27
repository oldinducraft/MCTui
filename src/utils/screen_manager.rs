use super::immediate_rw_lock::ImmediateRwLock;
use crate::screens::Screen;

#[derive(Default)]
pub struct ScreenManager {
    inner: ImmediateRwLock<Screen>,
}

impl ScreenManager {
    pub fn get_current(&self) -> Screen { self.inner.get().unwrap() }

    pub fn goto(&self, screen: Screen) { self.inner.set(screen).unwrap(); }
}
