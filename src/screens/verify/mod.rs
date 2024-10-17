use std::sync::Arc;

use tokio::task::JoinHandle;

use super::{Screen, ScreenTrait};
use crate::constants::CLIENT_FOLDER_NAME;
use crate::utils::requester::Requester;
use crate::utils::Libs;
use crate::widgets::loader_state::LoaderState;

mod events;
mod ui;

pub struct VerifyScreen {
    loader_state: LoaderState,
    handle:       Option<JoinHandle<()>>,
    libs:         Arc<Libs>,
}

impl ScreenTrait for VerifyScreen {}

impl VerifyScreen {
    pub fn new(libs: Arc<Libs>) -> VerifyScreen {
        VerifyScreen {
            loader_state: LoaderState::default(),
            handle: None,
            libs,
        }
    }

    fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }

    pub async fn verify(libs: Arc<Libs>) {
        let folder_path = libs.config.data_dir.join(CLIENT_FOLDER_NAME);
        if !folder_path.exists() {
            libs.screen.goto(Screen::Download);
            return;
        }

        let hash_map = Requester::new().get_client_hash().await;

        for (file_name, expected_hash) in hash_map {
            let file_path = folder_path.join(VerifyScreen::convert_path(file_name));
            if !file_path.exists() {
                libs.screen.goto(Screen::Download);
                return;
            }

            let bytes = std::fs::read(file_path).unwrap();
            let actual_hash = sha256::digest(&bytes);

            if actual_hash != expected_hash {
                libs.screen.goto(Screen::Download);
                return;
            }
        }

        libs.screen.goto(Screen::Run);
    }

    fn convert_path(path: String) -> String {
        if cfg!(windows) {
            return path.replace('\\', "/");
        }

        path
    }
}
