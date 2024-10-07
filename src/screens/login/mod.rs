use std::sync::Arc;

use ui::form_state::LoginFormState;

use super::{Screen, ScreenTrait};
use crate::utils::Libs;

pub mod events;
pub mod types;
pub mod ui;

pub struct LoginScreen {
    form:  LoginFormState,
    libs:  Arc<Libs>,
    error: Option<String>,
}

impl ScreenTrait for LoginScreen {}

impl LoginScreen {
    pub fn new(libs: Arc<Libs>) -> LoginScreen {
        LoginScreen {
            form: LoginFormState::default(),
            libs,
            error: None,
        }
    }
}

impl LoginScreen {
    fn submit(&self) {
        self.libs.config.set_username(Some(self.form.username.clone()));
        self.libs.config.set_password(Some(self.form.password.clone()));
        self.libs.config.save();
        self.libs.screen.goto(Screen::Authenticate);
    }
}
