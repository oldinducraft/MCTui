use serde::{Deserialize, Serialize};

use crate::traits::should_use::ShouldUse;

use super::platform::Platform;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum JvmArg {
    PlatformSpecific(PlatformSpecific),
    Simple(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformSpecific {
    arg:      String,
    platform: Platform,
}

impl JvmArg {
    pub fn arg(&self) -> &str {
        match self {
            JvmArg::PlatformSpecific(platform_specific) => &platform_specific.arg,
            JvmArg::Simple(arg) => arg,
        }
    }
}

impl ShouldUse for JvmArg {
    fn should_use(&self) -> bool {
        match self {
            JvmArg::PlatformSpecific(platform_specific) => platform_specific.platform.should_use(),
            JvmArg::Simple(_) => true,
        }
    }
}