use ratatui::style::{Color, Style};

use super::types::Field;

#[derive(Default, Clone)]
pub struct LoginFormState {
    pub(super) username:     String,
    pub(super) password:     String,
    pub(super) active_field: Field,
}

impl LoginFormState {
    pub(super) fn get_style_for(&self, field: Field) -> Style {
        if field == self.active_field {
            return Style::default().fg(Color::Yellow);
        }
        
        Style::default()
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
}
