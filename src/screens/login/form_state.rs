use std::sync::Arc;

use ratatui::style::{Color, Style};
use throbber_widgets_tui::ThrobberState;

use super::login_request_state::LoginRequestState;
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::yggdrasil::types::{AuthenticateRequest, YggdrasilResponse};
use crate::utils::yggdrasil::Yggdrasil;

#[derive(Default, PartialEq)]
pub enum Field {
    #[default]
    Username,
    Password,
}

#[derive(Default)]
pub struct LoginFormState {
    pub(super) auth:                AuthenticateRequest,
    pub(super) active_field:        Field,
    pub(super) throbber_state:      ThrobberState,
    pub(super) login_request_state: Arc<ImmediateRwLock<LoginRequestState>>,
    pub(super) login_request_error: Arc<ImmediateRwLock<String>>,
}

impl LoginFormState {
    pub(super) fn get_style_for(&self, field: Field) -> Style {
        if field == self.active_field {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        }
    }

    pub fn next_field(&mut self) {
        self.active_field = match self.active_field {
            Field::Username => Field::Password,
            Field::Password => Field::Username,
        };
    }

    pub fn add_char(&mut self, c: char) {
        match self.active_field {
            Field::Username => self.auth.username.push(c),
            Field::Password => self.auth.password.push(c),
        }
    }

    pub fn remove_char(&mut self) {
        match self.active_field {
            Field::Username => self.auth.username.pop(),
            Field::Password => self.auth.password.pop(),
        };
    }

    pub fn on_tick(&mut self) {
        if self.login_request_state.get().unwrap() == LoginRequestState::Pending {
            self.throbber_state.calc_next();
        }
    }

    pub fn submit(&self) {
        tokio::spawn(Self::login(
            self.login_request_state.clone(),
            self.login_request_error.clone(),
            self.auth.clone(),
        ));
    }

    async fn login(
        login_request_state: Arc<ImmediateRwLock<LoginRequestState>>,
        login_request_error: Arc<ImmediateRwLock<String>>,
        auth: AuthenticateRequest,
    ) {
        login_request_state.set(LoginRequestState::Pending).unwrap();

        if auth.username.is_empty() || auth.password.is_empty() {
            login_request_state.set(LoginRequestState::Rejected).unwrap();
            login_request_error
                .set("Have you considered filling in all fields?".to_string())
                .unwrap();
            return;
        }

        let client = Yggdrasil::new("https://wayaway.asuscomm.com".to_string());
        let res = client.authenticate(auth).await.unwrap();

        match res {
            YggdrasilResponse::Success(_) => login_request_state.set(LoginRequestState::Fulfilled).unwrap(),
            YggdrasilResponse::Error(err) => {
                login_request_state.set(LoginRequestState::Rejected).unwrap();
                login_request_error.set(err.error_message).unwrap();
            },
        }
    }
}
