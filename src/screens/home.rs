use std::cmp::min;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use atomic_float::AtomicF64;
use futures::StreamExt;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, LineGauge};
use ratatui::{symbols, Frame};

use super::ScreenTrait;

pub struct HomeScreen(Arc<State>);

// To share state between threads move it out of the struct and store it in the Arc
pub struct State {
    // Atomic types can be shared between threads without blocking
    download_state: AtomicU8,
    progress:       AtomicF64,
}

impl HomeScreen {
    const URL: &'static str = "https://ash-speed.hetzner.com/100MB.bin";

    pub fn new() -> Self {
        let state = Arc::new(State {
            download_state: AtomicU8::new(DownloadState::IDLE.bits()),
            progress:       AtomicF64::new(0.0),
        });

        // Spawn the download task cloning the state
        tokio::spawn(HomeScreen::download(Arc::clone(&state)));

        // Set the same state for the struct for rendering & event handling
        Self(state)
    }

    async fn download(state: Arc<State>) {
        // Update state to pending, it won't block any of the threads
        state
            .download_state
            .store(DownloadState::PENDING.bits(), Ordering::Relaxed);

        let res = reqwest::get(HomeScreen::URL).await.unwrap();
        let total_size = res.content_length().unwrap();

        let mut file = File::create("./test.bin").unwrap();
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.unwrap();
            file.write_all(&chunk).unwrap();
            downloaded = min(downloaded + (chunk.len() as u64), total_size);
            
            state.progress.store(downloaded as f64 / total_size as f64, Ordering::Relaxed);
        }

        state
            .download_state
            .store(DownloadState::FULFILLED.bits(), Ordering::Relaxed);
    }
}

impl ScreenTrait for HomeScreen {
    fn render(&mut self, frame: &mut Frame) {
        // Get state and progress without blocking
        let state = DownloadState::from_bits_retain(self.0.download_state.load(Ordering::Relaxed));
        let progress = self.0.progress.load(Ordering::Relaxed);

        let progress = LineGauge::default()
            .block(Block::bordered().title(state.to_string()))
            .filled_style(
                Style::default()
                    .fg(Color::Red)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .line_set(symbols::line::DOUBLE)
            .ratio(progress);

        frame.render_widget(progress, frame.area());
    }
}

#[derive(PartialEq)]
pub struct DownloadState(u8);

// Store state as bitflags because u8 is atomic
bitflags::bitflags! {
    impl DownloadState: u8 {
        const IDLE = 1;
        const PENDING = 1 << 1;
        const FULFILLED = 1 << 2;
        const REJECTED = 1 << 3;
    }
}

impl Display for DownloadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            DownloadState::IDLE => "Idle",
            DownloadState::PENDING => "Pending",
            DownloadState::FULFILLED => "Fulfilled",
            DownloadState::REJECTED => "Rejected",
            _ => panic!("unknown download state"),
        })
    }
}
