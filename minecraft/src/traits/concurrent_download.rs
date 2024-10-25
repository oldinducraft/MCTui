use std::future::Future;
use std::path::Path;
use std::sync::Arc;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use tokio::sync::Semaphore;
use tokio::task::JoinError;

use super::download::Download;

pub trait ConcurrentDownload {
    fn download_concurrently<T, F>(
        self,
        max_concurrent: usize,
        root: T,
        retries: u32,
        on_progress: &F,
    ) -> impl Future<Output = Result<(), JoinError>> + Send
    where
        T: AsRef<Path> + Send,
        F: Fn(usize, usize) + Send + Sync;
}

impl<I: Download + Sync + Send + 'static> ConcurrentDownload for Vec<I> {
    async fn download_concurrently<T, F>(
        self,
        max_concurrent: usize,
        root: T,
        retries: u32,
        on_progress: &F,
    ) -> Result<(), JoinError>
    where
        T: AsRef<Path>,
        F: Fn(usize, usize),
    {
        let assets_len = self.len();
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        let mut tasks = self
            .into_iter()
            .map(|asset| {
                let semaphore = Arc::clone(&semaphore);
                let root = root.as_ref().to_path_buf();

                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    asset.download_if_needed(&root, retries, &|_, _| {}).await.unwrap();

                    Some(())
                })
            })
            .collect::<FuturesUnordered<_>>()
            .enumerate();

        while let Some((i, result)) = tasks.next().await {
            match result {
                Ok(_) => on_progress(i, assets_len),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
