use std::fs::{self};
use std::sync::Arc;

use merkle_hash::{Algorithm, Encodable, MerkleTree};
use tokio::task::JoinHandle;

use super::{CreatableScreenTrait, Screen, ScreenTrait};
use crate::constants::CLIENT_FOLDER_NAME;
use crate::utils::requester::Requester;
use crate::utils::Libs;
use crate::widgets::progress_state::ProgressState;

mod events;
mod ui;

pub struct VerifyScreen {
    progress_state: Arc<ProgressState>,
    handle:         Option<JoinHandle<()>>,
    libs:           Arc<Libs>,
}

impl ScreenTrait for VerifyScreen {}

impl CreatableScreenTrait for VerifyScreen {
    fn new(libs: Arc<Libs>) -> VerifyScreen {
        VerifyScreen {
            progress_state: Arc::new(ProgressState::default()),
            handle: None,
            libs,
        }
    }
}

impl VerifyScreen {
    pub async fn verify(libs: Arc<Libs>) {
        if fs::metadata(CLIENT_FOLDER_NAME).is_err() {
            libs.screen.goto(Screen::Download);
            return;
        }

        let tree = MerkleTree::builder(CLIENT_FOLDER_NAME)
            .algorithm(Algorithm::Blake3)
            .build()
            .unwrap();

        let hash = Requester::new().get_client_hash().await;

        for item in tree {
            if !item.path.absolute.is_file() {
                continue;
            }

            let right = hash
                .get(&item.path.relative.to_string())
                // TODO: Fix paths on Windows
                .unwrap_or_else(|| panic!("Failed to get hash: {}", item.path.relative));
            if item.hash.to_hex_string() != *right {
                libs.screen.goto(Screen::Download);
                return;
            }
        }
    }
}
