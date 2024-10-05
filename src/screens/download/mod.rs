use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use futures::StreamExt;
use tokio::task::JoinHandle;

use super::{CreatableScreenTrait, Screen, ScreenTrait};
use crate::constants::CLIENT_URL;
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
        let res = reqwest::get(CLIENT_URL)
            .await
            .unwrap_or_else(|err| panic!("Failed to download client: {}", err));
        let total_size = res.content_length().expect("Failed to get client content length");

        let mut file =
            File::create("client.tar.gz").unwrap_or_else(|err| panic!("Failed to create client file: {}", err));
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(Ok(chunk)) = &stream.next().await {
            file.write_all(chunk).expect("Failed to write to client file");
            downloaded = min(downloaded + (chunk.len() as u64), total_size);

            progress_state.try_set(downloaded as f64 / total_size as f64);
        }

        libs.screen.goto(Screen::Unpack);
    }
}
