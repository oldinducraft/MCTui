use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use futures::StreamExt;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Gauge};
use ratatui::Frame;
use sha2::{Digest, Sha256};

use super::{Screen, ScreenTrait};
use crate::constants::{CLIENT_URL, HASH_URL};
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::ui::center::center;
use crate::utils::Libs;

pub struct DownloadScreen {
    libs:        Arc<Libs>,
    percent:     Arc<ImmediateRwLock<f64>>,
    downloading: bool,
}

impl ScreenTrait for DownloadScreen {
    fn render(&mut self, frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Percentage(100), Constraint::Length(3));
        frame.render_widget(
            Gauge::default()
                .block(Block::bordered().title("Downloading"))
                .gauge_style(if self.percent.get().unwrap() == 1.0 {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                })
                .ratio(self.percent.get().unwrap()),
            area,
        );
    }

    fn new(libs: Arc<Libs>) -> DownloadScreen {
        if libs.in_memory.get_access_token().is_none() ||
            libs.in_memory.get_client_token().is_none() ||
            libs.in_memory.get_username().is_none() ||
            libs.in_memory.get_uuid().is_none()
        {
            libs.screen.goto(Screen::Authenticate(Instant::now()));
        }

        DownloadScreen {
            libs,
            percent: Arc::new(ImmediateRwLock::new(0.0)),
            downloading: false,
        }
    }

    fn on_tick(&mut self) {
        if !self.downloading {
            self.downloading = true;

            let libs = self.libs.clone();
            let percent = self.percent.clone();

            tokio::spawn(DownloadScreen::download(libs, percent));
        }
    }
}

impl DownloadScreen {
    pub async fn download(libs: Arc<Libs>, percent: Arc<ImmediateRwLock<f64>>) {
        let hash = reqwest::get(HASH_URL).await.unwrap().text().await.unwrap();
        let res = reqwest::get(CLIENT_URL).await.unwrap();
        let total_size = res.content_length().unwrap();

        let mut file = File::create("client.tar.gz").unwrap();
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();
        let mut hasher = Sha256::new();

        while let Some(Ok(chunk)) = &stream.next().await {
            hasher.update(chunk);
            file.write_all(chunk).unwrap();
            downloaded = min(downloaded + (chunk.len() as u64), total_size);

            let _ = percent.try_set(downloaded as f64 / total_size as f64);
        }

        assert_eq!(format!("{:x}", hasher.finalize()), hash);

        libs.screen.goto(Screen::Unpack);
    }
}
