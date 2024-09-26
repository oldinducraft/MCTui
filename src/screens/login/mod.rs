use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use form::LoginForm;
use form_state::LoginFormState;
use login_request_state::LoginRequestState;
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::Frame;
use tokio::time::Instant;

use super::{Screen, ScreenTrait};
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::ui::center::center;
use crate::widgets::window::Window;

pub mod form;
pub mod form_state;
pub mod login_request_state;

#[derive(Default)]
pub struct LoginScreen {
    form:           LoginFormState,
    current_screen: Arc<ImmediateRwLock<Screen>>,
}

const KEY_HINTS: [(&str, &str); 3] = [("Esc/Ctrl+C", "Exit"), ("Enter", "Submit"), ("Tab", "Next field")];

impl ScreenTrait for LoginScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Who tf are you".bold().red(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Percentage(50));

        frame.render_widget(window, area);
        frame.render_stateful_widget(LoginForm::default().margin(2), area, &mut self.form);
    }

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        match event.code {
            KeyCode::Char(c) => self.form.add_char(c),
            KeyCode::Backspace => self.form.remove_char(),
            KeyCode::Tab => self.form.next_field(),
            KeyCode::Enter => {
                if self.form.login_request_state.get().unwrap() == LoginRequestState::Fulfilled {
                    return None;
                }

                self.form.submit();
            },
            _ => return Some(()),
        };

        None
    }

    fn on_tick(&mut self, _instant: Instant) { self.form.on_tick(); }

    fn new(current_screen: Arc<ImmediateRwLock<Screen>>) -> LoginScreen {
        LoginScreen {
            form: LoginFormState::default(),
            current_screen,
        }
    }
}
