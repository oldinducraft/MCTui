use std::hash::{Hash, Hasher};

use login::types::ErrorMessage;

pub mod authenticate;
pub mod download;
pub mod home;
pub mod login;
pub mod unpack;
pub mod verify;
pub mod run;

mod traits;
pub use traits::{RenderableScreen, ScreenTrait, ScreenEvents};

#[derive(Default, Clone, Debug)]
pub enum Screen {
    Login(ErrorMessage),
    Home,
    #[default]
    Authenticate,
    Download,
    Unpack,
    Verify,
    Run
}

impl Hash for Screen {
    fn hash<H: Hasher>(&self, state: &mut H) { std::mem::discriminant(self).hash(state); }
}

impl PartialEq for Screen {
    fn eq(&self, other: &Self) -> bool { std::mem::discriminant(self) == std::mem::discriminant(other) }
}

impl Eq for Screen {}
