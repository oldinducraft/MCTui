use std::marker::PhantomData;
use std::path::PathBuf;

use constants::{LIBRARIES_URL, LOCAL_URL, RESOURCES_URL};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use super::platform::Platform;
use crate::traits::path::Path;
use crate::traits::should_use::ShouldUse;
use crate::traits::url::Url;
use crate::traits::verify::Verify;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Library {}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Resource {}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Local {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download<Kind> {
    pub sha1:     String,
    pub path:     String,
    pub platform: Option<Platform>,
    #[serde(skip)]
    pub kind:     PhantomData<Kind>,
}

impl Url for Download<Library> {
    fn url(&self) -> String { format!("{LIBRARIES_URL}/{}", self.path) }
}

impl Url for Download<Resource> {
    fn url(&self) -> String { format!("{RESOURCES_URL}/{}", self.path) }
}

impl Url for Download<Local> {
    fn url(&self) -> String { format!("{LOCAL_URL}/{}", self.path) }
}

impl<T> Path for Download<T> {
    fn path(&self) -> PathBuf { self.path.split('/').collect() }
}

impl<T> ShouldUse for Download<T> {
    fn should_use(&self) -> bool {
        self.platform
            .as_ref()
            .map(|platform| platform.should_use())
            .unwrap_or(true)
    }
}

impl<T: Sync> Verify for Download<T> {
    type Error = VerifyError;

    async fn expected_hash(&self) -> String { self.sha1.clone() }

    async fn actual_hash<H>(&self, root: H) -> Result<String, Self::Error>
    where
        H: AsRef<std::path::Path> + Send + Sync,
    {
        let path = self.resolve(root).await?;
        if !path.exists() || !path.is_file() {
            return Err(VerifyError::FileNotFound);
        }

        let file = tokio::fs::read(path).await?;
        let hash = Sha1::digest(file);

        Ok(format!("{:x}", hash))
    }
}

#[derive(Debug)]
pub enum VerifyError {
    FileNotFound,
    ResolveFailed(crate::traits::path::Error),
    Io(std::io::Error),
}
impl From<std::io::Error> for VerifyError {
    fn from(err: std::io::Error) -> Self { VerifyError::Io(err) }
}

impl From<crate::traits::path::Error> for VerifyError {
    fn from(err: crate::traits::path::Error) -> Self { VerifyError::ResolveFailed(err) }
}
