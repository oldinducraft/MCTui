use std::cmp::min;
use std::future::Future;
use std::path::Path;
use std::time::Duration;

use futures::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::should_use::ShouldUse;
use super::url::Url;
use super::verify::Verify;
use crate::manifest::download::VerifyError;

pub trait Download {
    type Error: std::fmt::Debug;

    fn download<F, P>(&self, root: P, on_progress: &F) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync;

    fn download_if_needed<F, P>(
        &self,
        root: P,
        retries: u32,
        on_progress: &F,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync;
}

impl<T: super::path::Path + Url + ShouldUse + Sync + Verify> Download for T
where
    crate::traits::download::Error: From<<T as Verify>::Error>,
{
    type Error = Error;

    async fn download<F, P>(&self, root: P, on_progress: &F) -> Result<(), Self::Error>
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync,
    {
        let url = self.url();
        let dst = self.resolve(&root).await?;

        let res = reqwest::get(url).await?;
        let total_size = res.content_length().ok_or(Error::MissingContentLength)? as usize;

        let mut file = File::create(&dst).await?;

        let mut downloaded = 0;
        let mut stream = res.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;

            downloaded = min(downloaded + chunk.len(), total_size);
            on_progress(downloaded, total_size);
        }

        Ok(())
    }

    async fn download_if_needed<F, P>(&self, root: P, mut retries: u32, on_progress: &F) -> Result<(), Self::Error>
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync,
    {
        if !self.should_use() || self.verify(&root).await.unwrap_or(false) {
            return Ok(());
        }

        loop {
            match self.download(&root, on_progress).await {
                Ok(()) => {
                    if self.verify(&root).await? {
                        break Ok(());
                    }

                    if retries == 0 {
                        break Err(Error::HashMismatch);
                    }
                },
                Err(err) if retries == 0 => break Err(err),
                Err(_) => tokio::time::sleep(Duration::from_secs(1)).await,
            };

            retries -= 1;
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    MissingContentLength,
    MissingParentForDst,
    CantResolveDst(super::path::Error),
    VerifyFailed(VerifyError),
    HashMismatch,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self { Error::Reqwest(err) }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { Error::Io(err) }
}

impl From<super::path::Error> for Error {
    fn from(err: super::path::Error) -> Self { Error::CantResolveDst(err) }
}

impl From<VerifyError> for Error {
    fn from(err: VerifyError) -> Self { Error::VerifyFailed(err) }
}
