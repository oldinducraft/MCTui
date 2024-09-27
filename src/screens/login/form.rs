use layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, StatefulWidget};

use super::form_state::LoginFormState;
use super::types::Field;

#[derive(Default)]
pub struct LoginForm;

impl StatefulWidget for LoginForm {
    type State = LoginFormState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)])
            .flex(Flex::Center)
            .split(area);

        Paragraph::new(state.username.as_str())
            .block(Block::bordered().title("Username"))
            .style(state.get_style_for(Field::Username))
            .render(chunks[0], buf);

        Paragraph::new("*".repeat(state.password.len()))
            .block(Block::bordered().title("Password"))
            .style(state.get_style_for(Field::Password))
            .render(chunks[1], buf);
    }
}
