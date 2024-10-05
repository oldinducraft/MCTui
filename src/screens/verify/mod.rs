use std::collections::HashMap;
use std::fs::{self};
use std::sync::Arc;

use merkle_hash::{Algorithm, Encodable, MerkleTree};
use tokio::task::JoinHandle;

use super::{CreatableScreenTrait, Screen, ScreenTrait};
use crate::constants::HASH_URL;
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
        if fs::metadata("client").is_err() {
            libs.screen.goto(Screen::Download);
            return;
        }

        let tree = MerkleTree::builder("client")
            .algorithm(Algorithm::Blake3)
            .build()
            .unwrap();

        let hash = reqwest::get(HASH_URL)
            .await
            .unwrap_or_else(|err| panic!("Failed to get hash: {}", err))
            .text()
            .await
            .unwrap();
        let hash = serde_json::from_str::<HashMap<String, String>>(&hash)
            .unwrap_or_else(|err| panic!("Failed to parse hash: {}", err));

        for item in tree {
            if !item.path.absolute.is_file() {
                continue;
            }

            let right = hash
                .get(&item.path.relative.to_string())
                .unwrap_or_else(|| panic!("Failed to get hash: {}", item.path.relative));
            if item.hash.to_hex_string() != *right {
                libs.screen.goto(Screen::Download);
                return;
            }
        }
    }
}
