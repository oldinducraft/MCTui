use std::sync::Arc;

use ratatui::style::{Color, Style};
use throbber_widgets_tui::ThrobberState;

use super::types::{Field, LoginRequestState};
use crate::utils::immediate_rw_lock::ImmediateRwLock;

#[derive(Default, Clone)]
pub struct LoginFormState {
    pub(super) username:       String,
    pub(super) password:       String,
    pub(super) active_field:   Field,
    pub(super) throbber_state: ThrobberState,
    pub(super) request_state:  Arc<ImmediateRwLock<LoginRequestState>>,
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
            Field::Username => self.username.push(c),
            Field::Password => self.password.push(c),
        }
    }

    pub fn remove_char(&mut self) {
        match self.active_field {
            Field::Username => self.username.pop(),
            Field::Password => self.password.pop(),
        };
    }

    pub fn on_tick(&mut self) {
        if matches!(self.request_state.get().unwrap(), LoginRequestState::Pending) {
            self.throbber_state.calc_next();
        }
    }
}
