use serde::{Deserialize, Serialize};

use crate::traits::should_use::ShouldUse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Platform {
    Windows,
    Linux,
    Osx,
}
impl ShouldUse for Platform {
    fn should_use(&self) -> bool {
        match self {
            Platform::Windows => cfg!(target_os = "windows"),
            Platform::Linux => cfg!(target_os = "linux"),
            Platform::Osx => cfg!(target_os = "macos"),
        }
    }
}