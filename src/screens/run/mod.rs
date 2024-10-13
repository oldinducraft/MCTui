use std::process::{self, Stdio};
use std::sync::{Arc, RwLock};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;
use tokio::task::JoinHandle;

use super::ScreenTrait;
use crate::constants::CLIENT_FOLDER_NAME;
use crate::utils::minecraft::Minecraft;
use crate::utils::Libs;
use crate::widgets::loader_state::LoaderState;

mod events;
mod ui;

pub struct RunScreen {
    loader_state: LoaderState,
    handle:       Option<JoinHandle<()>>,
    libs:         Arc<Libs>,
    lines: Arc<RwLock<Vec<String>>>,
}

impl ScreenTrait for RunScreen {}

impl RunScreen {
    pub fn new(libs: Arc<Libs>) -> RunScreen {
        RunScreen {
            loader_state: LoaderState::default(),
            handle: None,
            libs,
            lines: Arc::new(RwLock::default()),
        }
    }

    pub async fn run(libs: Arc<Libs>, lines: Arc<RwLock<Vec<String>>>) {
        let client_path = libs.config.data_dir.join(CLIENT_FOLDER_NAME);
        let mut child = Minecraft::java_cmd(&client_path, libs.shared_memory.get_access_token().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        while let Ok(None) = child.try_wait() {
            select! {
                Ok(Some(line)) = stdout_reader.next_line() => {
                    lines.write().expect("Failed to get lines lock").push(line);
                },
                Ok(Some(line)) = stderr_reader.next_line() => {
                    lines.write().expect("Failed to get lines lock").push(line);
                },
                _ = child.wait() => {}
            }
        }

        process::exit(0);
    }
}
