use std::sync::Arc;

use tokio::task::JoinHandle;

use super::{Screen, ScreenTrait};
use crate::utils::yggdrasil::types::{AuthenticateRequest, YggdrasilResponse};
use crate::utils::yggdrasil::Yggdrasil;
use crate::utils::Libs;
use crate::widgets::loader_state::LoaderState;

mod events;
mod ui;

pub struct AuthenticateScreen {
    loader_state: LoaderState,
    libs:         Arc<Libs>,
    handle:       Option<JoinHandle<()>>,
}

impl ScreenTrait for AuthenticateScreen {}

impl AuthenticateScreen {
    pub fn new(libs: Arc<Libs>) -> AuthenticateScreen {
        AuthenticateScreen {
            loader_state: LoaderState::default(),
            libs,
            handle: None,
        }
    }
}

impl AuthenticateScreen {
    fn spawn_auth(libs: Arc<Libs>) -> JoinHandle<()> {
        tokio::spawn(async move {
            let auth = AuthenticateScreen::authenticate(libs.clone()).await;
            if let Err(err) = auth {
                libs.screen.goto(Screen::Login(Some(err)));
                return;
            }

            let profile = AuthenticateScreen::get_profile(libs.clone()).await;
            if let Err(err) = profile {
                libs.screen.goto(Screen::Login(Some(err)));
                return;
            }

            libs.screen.goto(Screen::Home);
        })
    }

    async fn authenticate(libs: Arc<Libs>) -> Result<(), String> {
        let auth = AuthenticateScreen::get_authenticate_request(libs.clone())
            .ok_or("Called .authenticate() without username or password")?;

        let client = Yggdrasil::new();
        let res = client.authenticate(auth).await.map_err(|err| err.to_string())?;

        match res {
            YggdrasilResponse::Success(tokens) => {
                libs.shared_memory.set_access_token(tokens.access_token);
                libs.shared_memory.set_client_token(tokens.client_token);

                Ok(())
            },
            YggdrasilResponse::Error(err) => Err(err.error_message),
        }
    }

    async fn get_profile(libs: Arc<Libs>) -> Result<(), String> {
        let username = libs.config.get_username().ok_or("Called .get_profile() without username")?;

        let client = Yggdrasil::new();
        let res = client.get_profile(&username).await.map_err(|err| err.to_string())?;

        match res {
            YggdrasilResponse::Success(profile) => {
                libs.shared_memory.set_username(profile.username);
                libs.shared_memory.set_uuid(profile.uuid);

                Ok(())
            },
            YggdrasilResponse::Error(err) => Err(err.error_message),
        }
    }

    fn get_authenticate_request(libs: Arc<Libs>) -> Option<AuthenticateRequest> {
        let username = libs.config.get_username()?;
        let password = libs.config.get_password()?;

        Some(AuthenticateRequest { username, password })
    }
}
