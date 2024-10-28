use std::sync::Arc;

use constants::{LIBRARIES_URL, LOCAL_URL, MANIFEST_URL, RESOURCES_URL, YGGDRASIL_DOMAIN};
use minecraft::game_args::{GameArgs, Player};
use minecraft::hosts::Hosts;
use minecraft::jvm::Jvm;
use minecraft::manifest::Manifest;
use tokio::task::JoinHandle;

use super::{Screen, ScreenTrait};
use crate::utils::Libs;
use crate::widgets::progress_state::ProgressState;

mod events;
mod ui;

pub struct DownloadScreen {
    libs:           Arc<Libs>,
    progress_state: Arc<ProgressState>,
    handle:         Option<JoinHandle<()>>,
}

impl ScreenTrait for DownloadScreen {}

impl DownloadScreen {
    pub fn new(libs: Arc<Libs>) -> DownloadScreen {
        DownloadScreen {
            libs,
            progress_state: Arc::new(ProgressState::default()),
            handle: None,
        }
    }

    fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }

    pub async fn download(libs: Arc<Libs>, progress_state: Arc<ProgressState>) {
        let response = reqwest::get(MANIFEST_URL).await.unwrap();
        let manifest = response.text().await.unwrap();

        let manifest = Manifest::new(&manifest, &libs.config.data_dir, Hosts {
            misc:      LOCAL_URL.to_string(),
            resources: RESOURCES_URL.to_string(),
            libraries: LIBRARIES_URL.to_string(),
        })
        .await
        .unwrap();
        manifest
            .download_if_needed(&move |i, total| progress_state.try_set(i as f64 / total as f64))
            .await
            .unwrap();

        let jvm = Jvm::new(
            &manifest,
            GameArgs {
                player:                 Player {
                    user_type:    "mojang".to_string(),
                    username:     libs.shared_memory.get_username().unwrap(),
                    uuid:         libs.shared_memory.get_uuid().unwrap(),
                    access_token: libs.shared_memory.get_access_token().unwrap(),
                },
                quick_play_multiplayer: YGGDRASIL_DOMAIN.to_string(),
            },
            YGGDRASIL_DOMAIN,
        )
        .await;

        libs.shared_memory.set_run_cmd(jvm.command());

        libs.screen.goto(Screen::Run);
    }
}
