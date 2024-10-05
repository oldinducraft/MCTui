use std::sync::Arc;

use tokio::task::JoinHandle;

use super::{CreatableScreenTrait, Screen, ScreenTrait};
use crate::utils::requester::Requester;
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

impl CreatableScreenTrait for DownloadScreen {
    fn new(libs: Arc<Libs>) -> DownloadScreen {
        DownloadScreen {
            libs,
            progress_state: Arc::new(ProgressState::default()),
            handle: None,
        }
    }
}

impl DownloadScreen {
    pub async fn download(libs: Arc<Libs>, progress_state: Arc<ProgressState>) {
        Requester::new().download_client(move |progress| {
            progress_state.try_set(progress);
        }).await;

        libs.screen.goto(Screen::Unpack);
    }
}