use layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, StatefulWidget};
use throbber_widgets_tui::{Throbber, WhichUse};

use super::form_state::{Field, LoginFormState};
use super::login_request_state::LoginRequestState;

#[derive(Default)]
pub struct LoginForm {
    margin: u16,
}

impl LoginForm {
    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }
}

impl StatefulWidget for LoginForm {
    type State = LoginFormState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .margin(self.margin)
            .flex(Flex::Center)
            .split(area);

        Paragraph::new(state.auth.username.as_str())
            .block(Block::bordered().title("Username"))
            .style(state.get_style_for(Field::Username))
            .render(chunks[0], buf);

        Paragraph::new("*".repeat(state.auth.password.len()))
            .block(Block::bordered().title("Password"))
            .style(state.get_style_for(Field::Password))
            .render(chunks[1], buf);

        match state.login_request_state.get().unwrap() {
            LoginRequestState::Pending => {
                StatefulWidget::render(self.get_throbber(), chunks[3], buf, &mut state.throbber_state)
            },
            LoginRequestState::Fulfilled => {
                Paragraph::new(Line::from(vec![
                    "Login successful. Press ".into(),
                    "<Enter>".cyan(),
                    " again to continue".into(),
                ]))
                .style(Style::default().fg(Color::Green))
                .render(chunks[3], buf);
            },
            LoginRequestState::Rejected => {
                let error_message = state.login_request_error.get().unwrap();
                Paragraph::new(error_message)
                    .style(Style::default().fg(Color::Red))
                    .render(chunks[3], buf);
            },
            LoginRequestState::Idle => {},
        }
    }
}

impl LoginForm {
    fn get_throbber(&self) -> Throbber<'_> {
        Throbber::default()
            .label("Running...")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .throbber_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
            .throbber_set(throbber_widgets_tui::BOX_DRAWING)
            .use_type(WhichUse::Spin)
    }
}
