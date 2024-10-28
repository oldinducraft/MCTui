use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::platform::Platform;
use crate::traits::path::LogicalPath;
use crate::traits::should_use::ShouldUse;
use crate::traits::url::Url;
use crate::traits::verify::Sha1Hash;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Library {}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Resource {}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Local {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download {
    pub sha1:     String,
    pub path:     String,
    pub platform: Option<Platform>,
}

impl Url for Download {
    fn url(&self, host: &str) -> String { format!("{host}/{}", self.path) }
}

impl LogicalPath for Download {
    fn logical_path(&self) -> PathBuf { self.path.split('/').collect() }
}

impl ShouldUse for Download {
    fn should_use(&self) -> bool {
        self.platform
            .as_ref()
            .map(|platform| platform.should_use())
            .unwrap_or(true)
    }
}

impl Sha1Hash for Download {
    fn sha1_hash(&self) -> &str { &self.sha1 }
}
