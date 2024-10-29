use std::cmp::min;
use std::future::Future;
use std::path::Path;
use std::time::Duration;

use futures::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::path::ResolvePath;
use super::should_use::ShouldUse;
use super::url::Url;
use super::verify::Sha1Verify;

pub trait Download {
    type Error;

    /// Download an item, returning an error if the download fails
    fn download<F, P>(
        &self,
        host: &str,
        root: P,
        on_progress: &F,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync;

    /// Download an item, only if it needs to be downloaded. Retry up to
    /// `retries` times.
    fn download_if_needed<F, P>(
        &self,
        host: &str,
        root: P,
        retries: u32,
        on_progress: &F,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync;
}

impl<T: ResolvePath + Url + ShouldUse + Sync + Sha1Verify> Download for T {
    type Error = DownloadError<<T as ResolvePath>::Error, <T as Sha1Verify>::Error>;

    async fn download<F, P>(&self, host: &str, root: P, on_progress: &F) -> Result<(), Self::Error>
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync,
    {
        let url = self.url(host);
        let dst = self.resolve(&root).await.map_err(DownloadError::ResolveFailed)?;

        let res = reqwest::get(url).await?;
        let total_size = res.content_length().ok_or(DownloadError::MissingContentLength)? as usize;

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

    /// Download an item, returning an error if the download fails. Will retry
    /// up to `retries` times. Will not download if the file is already valid
    /// ([`Sha1Verify::verify`]). Won't download if the item should not be used
    /// ([`ShouldUse::should_use`]).
    async fn download_if_needed<F, P>(
        &self,
        host: &str,
        root: P,
        mut retries: u32,
        on_progress: &F,
    ) -> Result<(), Self::Error>
    where
        F: Fn(usize, usize) + Send + 'static + Sync,
        P: AsRef<Path> + Send + Sync,
    {
        if !self.should_use() || self.verify(&root).await.unwrap_or(false) {
            return Ok(());
        }

        loop {
            match self.download(host, &root, on_progress).await {
                Ok(()) => {
                    if self.verify(&root).await.map_err(DownloadError::VerifyFailed)? {
                        break Ok(());
                    }

                    if retries == 0 {
                        break Err(DownloadError::HashMismatch);
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
pub enum DownloadError<R, V> {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    /// The content length header is missing
    MissingContentLength,
    /// Failed to resolve the destination path of an item
    ResolveFailed(R),
    /// Failed to verify an item
    VerifyFailed(V),
    /// The hash of an item does not match the hash in the manifest
    HashMismatch,
}

impl<R, V> From<reqwest::Error> for DownloadError<R, V> {
    fn from(err: reqwest::Error) -> Self { DownloadError::Reqwest(err) }
}

impl<R, V> From<std::io::Error> for DownloadError<R, V> {
    fn from(err: std::io::Error) -> Self { DownloadError::Io(err) }
}
