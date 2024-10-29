use std::future::Future;
use std::path::Path;
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task::JoinError;

use super::verify::Sha1Verify;

pub trait IntoInvalid<I: Sha1Verify> {
    /// Converts a list of items into a list of invalid items. See
    /// [`Sha1Verify::verify`]
    fn into_invalid<T>(self, max_concurrent: usize, root: T) -> impl Future<Output = Result<Vec<I>, JoinError>> + Send
    where
        T: AsRef<Path> + Send;
}

impl<I: Sha1Verify + Sync + Send + 'static> IntoInvalid<I> for Vec<I> {
    /// Converts a list of items into a list of invalid items. See
    /// [`Sha1Verify::verify`]
    ///
    /// Panics: if any of async tasks panics
    async fn into_invalid<T>(self, max_concurrent: usize, root: T) -> Result<Vec<I>, JoinError>
    where
        T: AsRef<Path> + Send,
    {
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        let tasks = self
            .into_iter()
            .map(|asset| {
                let permit = semaphore.clone().acquire_owned();
                let root = root.as_ref().to_path_buf();

                tokio::spawn(async move {
                    let _permit = permit.await;
                    match asset.verify(root).await {
                        Ok(true) => None,
                        _ => Some(asset),
                    }
                })
            })
            .collect::<Vec<_>>();

        let results = futures::future::join_all(tasks).await;

        Ok(results.into_iter().filter_map(|res| res.unwrap()).collect())
    }
}
