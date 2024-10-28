use std::future::Future;
use std::path::{Path, PathBuf};

use tokio::fs::create_dir_all;

pub trait LogicalPath {
    /// Logical path to an item.
    fn logical_path(&self) -> PathBuf;
}

pub trait ResolvePath {
    type Error: Send + Sync;

    /// Resolve a full path to an item
    fn resolve<T>(&self, root: T) -> impl Future<Output = Result<PathBuf, Self::Error>> + Send
    where
        T: AsRef<Path> + Sync + Send;
}

impl<I: LogicalPath + Sync> ResolvePath for I {
    type Error = std::io::Error;

    /// Join logical path of an item with a root, creating the parent directory
    /// if it doesn't exist.
    ///
    /// **Note: Path is not canonicalized**
    async fn resolve<T>(&self, root: T) -> Result<PathBuf, Self::Error>
    where
        T: AsRef<Path> + Sync + Send,
    {
        let path = self.logical_path();
        let dst = root.as_ref().join(&path);
        create_dir_all(dst.parent().unwrap()).await?;

        // Why is dst not canonicalized?
        // Since the item may not yet exist at the time this command is executed

        Ok(dst)
    }
}
