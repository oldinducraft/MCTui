use std::process::{self, Stdio};
use std::sync::{Arc, RwLock};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::select;
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
    lines:        Arc<RwLock<Vec<String>>>,
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

    fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }

    pub async fn run(libs: Arc<Libs>, lines: Arc<RwLock<Vec<String>>>) {
        let cmd = libs.shared_memory.get_run_cmd().unwrap();

        let mut child = Command::new(&cmd[0])
            .current_dir(&libs.config.data_dir)
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        loop {
            select! {
                Ok(Some(line)) = stdout_reader.next_line() => {
                    lines.write().expect("Failed to get lines lock").push(line);
                },
                Ok(Some(line)) = stderr_reader.next_line() => {
                    lines.write().expect("Failed to get lines lock").push(line);
                },
                Ok(exit) = child.wait() => {
                    if !exit.success() {
                        panic!("Minecraft process exited with code {}, output:\n{}", exit.code().unwrap(), lines.read().unwrap().join("\n"));
                    }

                    break;
                }
            }
        }

        process::exit(0);
    }
}
