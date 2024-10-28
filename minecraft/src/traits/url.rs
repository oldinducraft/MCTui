pub trait Url {
    /// Returns the URL to download an item
    fn url(&self, host: &str) -> String;
}
