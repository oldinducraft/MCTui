use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::traits::path::LogicalPath;
use crate::traits::should_use::ShouldUse;
use crate::traits::url::Url;
use crate::traits::verify::Sha1Hash;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset(String);

impl LogicalPath for Asset {
    fn logical_path(&self) -> PathBuf { PathBuf::from(format!("{}/{}", &self.0[..2], self.0)) }
}

impl Url for Asset {
    fn url(&self, host: &str) -> String { format!("{host}/{}/{}", &self.0[..2], self.0) }
}

impl ShouldUse for Asset {
    fn should_use(&self) -> bool { true }
}

impl Sha1Hash for Asset {
    fn sha1_hash(&self) -> &str { &self.0 }
}
