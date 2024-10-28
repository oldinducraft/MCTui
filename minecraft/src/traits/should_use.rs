pub trait ShouldUse {
    /// Should this item be used in the current context
    fn should_use(&self) -> bool;
}
