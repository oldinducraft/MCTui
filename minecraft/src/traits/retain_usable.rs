use super::should_use::ShouldUse;

pub trait RetainUsable {
    /// Retain only items that should be used. See [`ShouldUse::should_use`]
    fn retain_usable(&mut self);
}

impl<T: ShouldUse> RetainUsable for Vec<T> {
    fn retain_usable(&mut self) { self.retain(|asset| asset.should_use()); }
}
