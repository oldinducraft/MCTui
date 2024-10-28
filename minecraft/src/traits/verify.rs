use std::future::Future;
use std::path::Path;

use sha1::{Digest, Sha1};

use super::path::ResolvePath;

pub trait Sha1Hash {
    /// Expected SHA-1 hash of an item
    fn sha1_hash(&self) -> &str;
}

pub trait Sha1Verify {
    type Error: Send + Sync;

    /// Actual SHA-1 hash of an item
    fn actual_hash<T>(&self, root: T) -> impl Future<Output = Result<String, Self::Error>> + Send
    where
        T: AsRef<Path> + Send + Sync;

    /// Does actual hash match the expected hash. See [`Sha1Hash::sha1_hash`]
    fn verify<T>(&self, root: T) -> impl Future<Output = Result<bool, Self::Error>> + Send
    where
        T: AsRef<Path> + Send + Sync;
}

impl<I: Sha1Hash + ResolvePath + Sync> Sha1Verify for I {
    type Error = Sha1VerifyError<<I as ResolvePath>::Error>;

    async fn actual_hash<T>(&self, root: T) -> Result<String, Self::Error>
    where
        T: AsRef<Path> + Send + Sync,
    {
        let path = self.resolve(root).await.map_err(Sha1VerifyError::ResolveFailed)?;
        if !path.exists() || !path.is_file() {
            return Err(Sha1VerifyError::FileNotFound);
        }

        let file = tokio::fs::read(path).await?;
        let hash = Sha1::digest(file);

        Ok(format!("{:x}", hash))
    }

    async fn verify<T>(&self, root: T) -> Result<bool, Self::Error>
    where
        T: AsRef<Path> + Send + Sync,
    {
        Ok(self.actual_hash(root).await? == self.sha1_hash())
    }
}

#[derive(Debug)]
pub enum Sha1VerifyError<T> {
    ResolveFailed(T),
    FileNotFound,
    Io(std::io::Error),
}

impl<T> From<std::io::Error> for Sha1VerifyError<T> {
    fn from(err: std::io::Error) -> Self { Sha1VerifyError::Io(err) }
}
