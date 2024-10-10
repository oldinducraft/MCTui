use std::process::Stdio;
use std::sync::{Arc, RwLock};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::task::JoinHandle;

use super::ScreenTrait;
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

    pub async fn run(lines: Arc<RwLock<Vec<String>>>) {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("sleep 2 && echo hello world")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        loop {
            if let Some(line) = stdout_reader.next_line().await.unwrap() {
                lines.write().expect("Failed to get lines lock").push(line);
            }

            if let Some(line) = stderr_reader.next_line().await.unwrap() {
                lines.write().expect("Failed to get lines lock").push(line);
            }
        }
    }
}
