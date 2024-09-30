use std::sync::Arc;
use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};
use form::LoginForm;
use form_state::LoginFormState;
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::{Screen, ScreenTrait};
use crate::utils::ui::center::center;
use crate::utils::Libs;
use crate::widgets::window::Window;

pub mod form;
pub mod form_state;
pub mod types;
pub mod arg;

pub struct LoginScreen {
    form:           LoginFormState,
    libs:           Arc<Libs>,
    error:          Option<String>,
}

const KEY_HINTS: [(&str, &str); 3] = [("Esc/Ctrl+C", "Exit"), ("Enter", "Submit"), ("Tab", "Next field")];

impl ScreenTrait for LoginScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Who tf are you".bold().red(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Percentage(70));

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)].as_ref())
            .margin(2)
            .flex(Flex::Center)
            .split(area);

        frame.render_widget(window, area);
        frame.render_stateful_widget(LoginForm, layout[0], &mut self.form);

        if let Some(err) = &self.error {
            frame.render_widget(
                Paragraph::new(err.as_str())
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center),
                layout[1],
            );
        }
    }

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        match event.code {
            KeyCode::Char(c) => self.form.add_char(c),
            KeyCode::Backspace => self.form.remove_char(),
            KeyCode::Tab => self.form.next_field(),
            KeyCode::Enter => self.submit(),
            _ => return Some(()),
        };

        None
    }

    fn on_tick(&mut self) {
        let screen = self.libs.screen.get_current();
        
        if let Screen::Login(Some(err)) = screen {
            self.error = Some(err);
        }
    }

    fn new(libs: Arc<Libs>) -> LoginScreen {
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
        self.libs.screen.goto(Screen::Authenticate(Instant::now()));
    }
}
