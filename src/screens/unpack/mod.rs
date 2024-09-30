use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Instant;

use flate2::read::GzDecoder;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Gauge};
use ratatui::Frame;
use tar::Archive;

use super::{Screen, ScreenTrait};
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::ui::center::center;
use crate::utils::Libs;

pub struct UnpackScreen {
    percent:   Arc<ImmediateRwLock<f64>>,
    unpacking: bool,
    done:      Arc<ImmediateRwLock<bool>>,
}

impl ScreenTrait for UnpackScreen {
    fn render(&mut self, frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Percentage(100), Constraint::Length(3));
        frame.render_widget(
            Gauge::default()
                .block(Block::bordered().title("Unpacking"))
                .gauge_style(if self.done.get().unwrap() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                })
                .ratio(self.percent.get().unwrap()),
            area,
        );
    }

    fn new(libs: Arc<Libs>) -> UnpackScreen {
        if libs.in_memory.get_access_token().is_none() ||
            libs.in_memory.get_client_token().is_none() ||
            libs.in_memory.get_username().is_none() ||
            libs.in_memory.get_uuid().is_none()
        {
            libs.screen.goto(Screen::Authenticate(Instant::now()));
        }

        UnpackScreen {
            percent: Arc::new(ImmediateRwLock::new(0.0)),
            unpacking: false,
            done: Arc::new(ImmediateRwLock::new(false)),
        }
    }

    fn on_tick(&mut self) {
        if !self.unpacking {
            self.unpacking = true;
            let percent = self.percent.clone();
            let done = self.done.clone();
            tokio::spawn(UnpackScreen::unpack(percent, done));
        }
    }
}

impl UnpackScreen {
    pub async fn unpack(percent: Arc<ImmediateRwLock<f64>>, done: Arc<ImmediateRwLock<bool>>) {
        let tar_gz_path = "client.tar.gz";
        let output_dir = "client/";

        let tar_gz = File::open(tar_gz_path).unwrap();
        let total_size = tar_gz.metadata().unwrap().len();
        let tar = GzDecoder::new(BufReader::new(tar_gz));
        let mut archive = Archive::new(tar);

        let mut processed_size = 0;

        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            let file_size = entry.header().size().unwrap();
            entry.unpack_in(output_dir).unwrap();

            processed_size += file_size;
            let new_percent = processed_size as f64 / total_size as f64;
            percent.try_set(new_percent.min(1.0)).unwrap();
        }

        done.set(true).unwrap();
    }
}
