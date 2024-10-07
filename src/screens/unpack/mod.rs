use std::fs::{self, File};
use std::io::BufReader;
use std::sync::Arc;

use flate2::read::GzDecoder;
use tar::Archive;
use tokio::task::JoinHandle;

use super::{Screen, ScreenTrait};
use crate::constants::{CLIENT_ARCHIVE_FILENAME, CLIENT_FOLDER_NAME};
use crate::utils::Libs;
use crate::widgets::progress_state::ProgressState;

mod events;
mod ui;

pub struct UnpackScreen {
    progress_state: Arc<ProgressState>,
    handle:         Option<JoinHandle<()>>,
    libs:           Arc<Libs>,
}

impl ScreenTrait for UnpackScreen {}

impl UnpackScreen {
    pub fn new(libs: Arc<Libs>) -> UnpackScreen {
        UnpackScreen {
            progress_state: Arc::new(ProgressState::default()),
            handle: None,
            libs,
        }
    }
}

impl UnpackScreen {
    pub async fn unpack(progress_state: Arc<ProgressState>, libs: Arc<Libs>) {
        let archive_path = libs.config.data_dir.join(CLIENT_ARCHIVE_FILENAME);
        if !archive_path.exists() {
            libs.screen.goto(Screen::Download);
            return;
        }

        let folder_path = libs.config.data_dir.join(CLIENT_FOLDER_NAME);
        if !folder_path.exists() {
            fs::create_dir(&folder_path).unwrap_or_else(|err| panic!("Failed to create output dir: {}", err));
        }

        let tar_gz = File::open(archive_path).unwrap_or_else(|err| panic!("Failed to open tar.gz: {}", err));
        let total_size = tar_gz
            .metadata()
            .unwrap_or_else(|err| panic!("Failed to get tar.gz metadata: {}", err))
            .len();
        let tar = GzDecoder::new(BufReader::new(tar_gz));
        let mut archive = Archive::new(tar);

        let mut processed_size = 0;

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            let file_size = entry.header().size().expect("Failed to get file size");
            entry
                .unpack_in(&folder_path)
                .unwrap_or_else(|err| panic!("Failed to unpack entry: {}", err));

            processed_size += file_size;
            let new_percent = processed_size as f64 / total_size as f64;
            progress_state.try_set(new_percent.min(1.0));
        }

        libs.screen.goto(Screen::Verify);
    }
}
