use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use throbber_widgets_tui::{Throbber, ThrobberState, WhichUse};

#[derive(Default)]
pub struct LoginForm {
    active_field:   Field,
    username:       String,
    password:       String,
    status:         Option<String>,
    error:          Option<String>,
    throbber_state: ThrobberState,
}

#[derive(Default, PartialEq)]
enum Field {
    #[default]
    Username,
    Password,
}

impl LoginForm {
    fn get_style_for(&self, field: Field) -> Style {
        if field == self.active_field {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        }
    }

    fn next_field(&mut self) {
        self.active_field = match self.active_field {
            Field::Username => Field::Password,
            Field::Password => Field::Username,
        };
    }

    fn add_char(&mut self, c: char) {
        match self.active_field {
            Field::Username => self.username.push(c),
            Field::Password => self.password.push(c),
        }
    }

    fn remove_char(&mut self) {
        match self.active_field {
            Field::Username => self.username.pop(),
            Field::Password => self.password.pop(),
        };
    }

    pub fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        match event.code {
            KeyCode::Tab => {
                self.next_field();
                None
            },
            KeyCode::Char(c) => {
                self.add_char(c);
                None
            },
            KeyCode::Backspace => {
                self.remove_char();
                None
            },
            _ => Some(()),
        }
    }

    pub fn update_status(&mut self, status: Option<String>) { self.status = status; }
    pub fn update_error(&mut self, error: Option<String>) { self.error = error; }

    pub fn on_tick(&mut self) { self.throbber_state.calc_next(); }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Length(3)].as_ref())
            .margin(1)
            .flex(Flex::Center)
            .split(area);

        let username_field = Paragraph::new(self.username.as_str())
            .block(Block::bordered().title("Username"))
            .style(self.get_style_for(Field::Username));
        frame.render_widget(username_field, chunks[1]);

        let password_field = Paragraph::new("*".repeat(self.password.len()))
            .block(Block::bordered().title("Password"))
            .style(self.get_style_for(Field::Password));
        frame.render_widget(password_field, chunks[2]);

        if let Some(status) = &self.status {
            let throbber = Throbber::default()
                .label(status)
                .throbber_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
                .throbber_set(throbber_widgets_tui::ASCII)
                .use_type(WhichUse::Spin);

            frame.render_stateful_widget(throbber, chunks[0], &mut self.throbber_state);
        }

        if let Some(error) = &self.error {
            let error = Paragraph::new(error.as_str()).style(Style::default().fg(Color::Red));
            frame.render_widget(error, chunks[0]);
        }
    }
}
