use super::should_use::ShouldUse;

pub trait IntoUsable<T: ShouldUse> {
    fn retain_usable(&mut self);
}

impl<T: ShouldUse> IntoUsable<T> for Vec<T> {
    fn retain_usable(&mut self) { self.retain(|asset| asset.should_use()); }
}
