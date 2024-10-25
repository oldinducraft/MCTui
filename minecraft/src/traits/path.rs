use std::{future::Future, path::PathBuf};

use tokio::fs::create_dir_all;

pub trait Path {
    fn path(&self) -> PathBuf;

    fn resolve<T>(&self, root: T) -> impl Future<Output = Result<PathBuf, Error>> + Send
    where
        T: AsRef<std::path::Path> + Sync + Send,
        Self: Sync
    {
        async move {
            let path = self.path();
            let dst = root.as_ref().join(&path);
            create_dir_all(dst.parent().ok_or(Error::MissingParentForDst)?).await?;

            Ok(dst)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    MissingParentForDst,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { Error::Io(err) }
}
