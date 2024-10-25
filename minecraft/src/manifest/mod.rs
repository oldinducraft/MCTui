use std::path::{Path, PathBuf};

use download::{Download, Library, Local, Resource};
use jvm_arg::JvmArg;
use serde::{Deserialize, Serialize};
use tokio::fs::create_dir_all;
use tokio::task::JoinError;

use crate::traits::concurrent_download::ConcurrentDownload;
use crate::traits::download::Download as DownloadTrait;
use crate::traits::into_invalid::IntoInvalid;
use crate::traits::path::Path as _;
use crate::traits::retain_usable::IntoUsable;

pub mod download;
pub mod jvm_arg;
pub mod platform;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub name:          String,
    pub version:       String,
    #[serde(rename = "javaVersion")]
    pub java_version:  u8,
    pub r#type:        String,
    #[serde(rename = "mainClass")]
    pub main_class:    String,
    pub jvm:           Vec<JvmArg>,
    #[serde(rename = "assetIndexes")]
    pub asset_indexes: Download<Local>,
    pub assets:        Download<Local>,
    pub client:        Download<Local>,
    pub authlib:       Download<Local>,
    pub libraries:     Vec<Download<Library>>,

    #[serde(skip)]
    pub root:               PathBuf,
    #[serde(skip)]
    pub libraries_root:     PathBuf,
    #[serde(skip)]
    pub natives_root:       PathBuf,
    #[serde(skip)]
    pub versions_root:      PathBuf,
    #[serde(skip)]
    pub asset_indexes_root: PathBuf,
    #[serde(skip)]
    pub assets_root:        PathBuf,
}

impl Manifest {
    pub async fn new<T>(raw: &str, root: T) -> Result<Self, serde_json::Error>
    where
        T: AsRef<Path>,
    {
        let mut manifest: Manifest = serde_json::from_str(raw)?;

        manifest.jvm.retain_usable();
        manifest.libraries.retain_usable();

        let root = root.as_ref();
        manifest.root = Manifest::create_and_canonicalize(root).await.unwrap();
        manifest.libraries_root = Manifest::create_and_canonicalize(root.join("libraries")).await.unwrap();
        manifest.natives_root = Manifest::create_and_canonicalize(root.join("natives")).await.unwrap();
        manifest.versions_root = Manifest::create_and_canonicalize(root.join("versions")).await.unwrap();
        manifest.asset_indexes_root = Manifest::create_and_canonicalize(root.join("assets/indexes"))
            .await
            .unwrap();
        manifest.assets_root = Manifest::create_and_canonicalize(root.join("assets/objects"))
            .await
            .unwrap();

        Ok(manifest)
    }

    async fn create_and_canonicalize<T>(path: T) -> Result<PathBuf, std::io::Error>
    where
        T: AsRef<Path>,
    {
        create_dir_all(path.as_ref()).await?;
        dunce::canonicalize(path)
    }

    pub async fn download<T>(&self, on_progress: &T) -> Result<(), DownloadError>
    where
        T: Fn(usize, usize) + Send + 'static + Sync,
    {
        let invalid_libraries = self.libraries.clone().into_invalid(16, &self.libraries_root).await?;

        self.authlib.download_if_needed(&self.root, 5, on_progress).await?;
        self.assets.download_if_needed(&self.root, 5, on_progress).await?;
        self.client
            .download_if_needed(&self.versions_root, 5, on_progress)
            .await?;
        self.asset_indexes
            .download_if_needed(&self.asset_indexes_root, 5, on_progress)
            .await?;

        invalid_libraries
            .download_concurrently(8, &self.libraries_root, 5, on_progress)
            .await?;

        let assets_path = self.assets.resolve(&self.root).await?;
        let assets_str = tokio::fs::read_to_string(assets_path).await?;
        let asset_downloads: Vec<Download<Resource>> = serde_json::from_str(&assets_str)?;
        let invalid_asset_downloads = asset_downloads.into_invalid(16, &self.assets_root).await?;

        invalid_asset_downloads
            .download_concurrently(8, &self.assets_root, 5, on_progress)
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum DownloadError {
    FailedInOneOfThreads(JoinError),
    DownloadError(crate::traits::download::Error),
    AssetsParsedFailed(serde_json::Error),
    PathResolveError(crate::traits::path::Error),
    Io(std::io::Error),
}

impl From<JoinError> for DownloadError {
    fn from(err: JoinError) -> Self { DownloadError::FailedInOneOfThreads(err) }
}

impl From<crate::traits::download::Error> for DownloadError {
    fn from(err: crate::traits::download::Error) -> Self { DownloadError::DownloadError(err) }
}

impl From<serde_json::Error> for DownloadError {
    fn from(err: serde_json::Error) -> Self { DownloadError::AssetsParsedFailed(err) }
}

impl From<crate::traits::path::Error> for DownloadError {
    fn from(err: crate::traits::path::Error) -> Self { DownloadError::PathResolveError(err) }
}

impl From<std::io::Error> for DownloadError {
    fn from(err: std::io::Error) -> Self { DownloadError::Io(err) }
}
