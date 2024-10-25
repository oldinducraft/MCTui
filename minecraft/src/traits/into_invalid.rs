use std::future::Future;
use std::path::Path;
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task::JoinError;

use super::verify::Verify;

pub trait IntoInvalid<I: Verify> {
    fn into_invalid<T>(
        self,
        max_concurrent: usize,
        root: T,
    ) -> impl Future<Output = Result<Vec<I>, JoinError>> + Send
    where
        T: AsRef<Path> + Send;
}

impl<I: Verify + Sync + Send + 'static> IntoInvalid<I> for Vec<I> {
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

        Ok(results
            .into_iter()
            .filter_map(|res| res.expect("Task panicked"))
            .collect())
    }
}
