use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use form::LoginForm;
use form_state::LoginFormState;
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::Frame;
use tokio::time::Instant;
use types::{LoginRequestState, Submit};

use super::{Screen, ScreenTrait};
use crate::utils::ui::center::center;
use crate::utils::yggdrasil::types::{AuthenticateRequest, AuthenticateResponse, YggdrasilResponse};
use crate::utils::yggdrasil::Yggdrasil;
use crate::utils::Libs;
use crate::widgets::window::Window;

pub mod form;
pub mod form_state;
pub mod types;

#[derive(Clone)]
pub struct LoginScreen {
    form: LoginFormState,
    libs: Arc<Libs>,
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
            KeyCode::Enter => self.submit_or_continue((&*self).into()),
            _ => return Some(()),
        };

        None
    }

    fn on_tick(&mut self, _instant: Instant) { self.form.on_tick(); }

    fn new(libs: Arc<Libs>) -> LoginScreen {
        LoginScreen {
            form: LoginFormState::default(),
            libs,
        }
    }
}

impl LoginScreen {
    fn save_credentials(libs: Arc<Libs>, username: String, password: String) {
        libs.config.set_username(username);
        libs.config.set_password(password);
        libs.config.save();
    }

    fn save_tokens(libs: Arc<Libs>, res: AuthenticateResponse) {
        libs.in_memory.set_access_token(res.access_token);
        libs.in_memory.set_client_token(res.client_token);
    }

    fn submit_or_continue(&self, s: Submit) {
        if matches!(self.form.request_state.get().unwrap(), LoginRequestState::Fulfilled) {
            self.libs.screen.goto(Screen::Home);
            return;
        }

        tokio::spawn(async move {
            s.request_state.set(LoginRequestState::Pending).unwrap();

            let res = LoginScreen::login(s.username.clone(), s.password.clone()).await;
            match res {
                Ok(res) => {
                    s.request_state.set(LoginRequestState::Fulfilled).unwrap();
                    LoginScreen::save_tokens(s.libs.clone(), res);
                    LoginScreen::save_credentials(s.libs.clone(), s.username, s.password);
                    s.libs.screen.goto(Screen::Home);
                },
                Err(err) => {
                    s.request_state.set(LoginRequestState::Rejected(err)).unwrap();
                },
            }
        });
    }

    async fn login(username: String, password: String) -> Result<AuthenticateResponse, String> {
        if username.is_empty() || password.is_empty() {
            return Err("Have you considered filling in all fields?".to_string());
        }

        let client = Yggdrasil::new();
        let res = client.authenticate(AuthenticateRequest { username, password }).await;

        match res {
            Ok(YggdrasilResponse::Success(res)) => Ok(res),
            Ok(YggdrasilResponse::Error(err)) => Err(err.error_message),
            Err(err) => Err(err.to_string()),
        }
    }
}
