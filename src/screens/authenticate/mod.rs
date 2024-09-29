use std::sync::Arc;

use loader::Loader;
use loader_state::LoaderState;
use ratatui::layout::Constraint;
use ratatui::Frame;

use super::{Screen, ScreenTrait};
use crate::utils::ui::center::center;
use crate::utils::yggdrasil::types::{AuthenticateRequest, YggdrasilResponse};
use crate::utils::yggdrasil::Yggdrasil;
use crate::utils::Libs;
use crate::widgets::window::Window;

mod loader;
mod loader_state;

pub struct AuthenticateScreen {
    loader_state: LoaderState,
}

const KEY_HINTS: [(&str, &str); 1] = [("Esc/Ctrl+C", "Exit")];

impl ScreenTrait for AuthenticateScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Authenticate".into(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Length(10));

        frame.render_widget(window, area);
        frame.render_stateful_widget(Loader, area, &mut self.loader_state);
    }

    fn new(libs: Arc<Libs>) -> AuthenticateScreen {
        if libs.config.get_username().is_none() {
            libs.screen.goto(Screen::Login);
        }

        tokio::spawn(async move {
            let auth = AuthenticateScreen::authenticate(libs.clone()).await;
            if auth.is_none() {
                libs.screen.goto(Screen::Login);
                return;
            }

            let profile = AuthenticateScreen::get_profile(libs.clone()).await;
            if profile.is_none() {
                libs.screen.goto(Screen::Login);
                return;
            }

            libs.screen.goto(Screen::Home);
        });

        AuthenticateScreen {
            loader_state: LoaderState::default(),
        }
    }

    fn on_tick(&mut self) { self.loader_state.on_tick(); }
}

impl AuthenticateScreen {
    async fn authenticate(libs: Arc<Libs>) -> Option<()> {
        let auth = AuthenticateScreen::get_authenticate_request(libs.clone())?;

        let client = Yggdrasil::new();
        let res = client.authenticate(auth).await.ok()?;

        match res {
            YggdrasilResponse::Success(tokens) => {
                libs.in_memory.set_access_token(tokens.access_token);
                libs.in_memory.set_client_token(tokens.client_token);

                Some(())
            },
            YggdrasilResponse::Error(_) => None,
        }
    }

    async fn get_profile(libs: Arc<Libs>) -> Option<()> {
        let username = libs.config.get_username()?;

        let client = Yggdrasil::new();
        let res = client.get_profile(&username).await.ok()?;

        match res {
            YggdrasilResponse::Success(profile) => {
                libs.in_memory.set_username(profile.username);
                libs.in_memory.set_uuid(profile.uuid);
                
                Some(())
            },
            YggdrasilResponse::Error(_) => None,
        }
    }

    fn get_authenticate_request(libs: Arc<Libs>) -> Option<AuthenticateRequest> {
        let username = libs.config.get_username()?;
        let password = libs.config.get_password()?;

        Some(AuthenticateRequest { username, password })
    }
}
