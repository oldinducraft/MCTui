use std::sync::Arc;

use super::LoginScreen;
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::Libs;

#[derive(Default, PartialEq, Clone)]
pub enum LoginRequestState {
    #[default]
    Idle,
    Pending,
    Fulfilled,
    Rejected(String),
}

#[derive(Default, PartialEq, Clone)]
pub enum Field {
    #[default]
    Username,
    Password,
}

pub struct Submit {
    pub libs:          Arc<Libs>,
    pub request_state: Arc<ImmediateRwLock<LoginRequestState>>,
    pub username:      String,
    pub password:      String,
}

impl From<&LoginScreen> for Submit {
    fn from(login: &LoginScreen) -> Self {
        Self {
            libs:          login.libs.clone(),
            request_state: login.request_loader.state.clone(),
            username:      login.form.username.clone(),
            password:      login.form.password.clone(),
        }
    }
}
