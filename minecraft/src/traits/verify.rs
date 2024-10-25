use std::future::Future;
use std::path::Path;

pub trait Verify {
    type Error: Send + std::fmt::Debug;

    fn expected_hash(&self) -> impl Future<Output = String> + Send;

    fn actual_hash<T>(&self, root: T) -> impl Future<Output = Result<String, Self::Error>> + Send
    where
        T: AsRef<Path> + Send + Sync;

    fn verify<T>(&self, root: T) -> impl Future<Output = Result<bool, Self::Error>> + Send
    where
        T: AsRef<Path> + Send + Sync,
        Self: Sync,
    {
        async move { Ok(self.actual_hash(root).await? == self.expected_hash().await) }
    }
}
