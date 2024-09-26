use std::collections::HashMap;
use std::io::{self, Stdout};
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::time::Instant;

use crate::screens::login::LoginScreen;
use crate::screens::{Screen, ScreenTrait};
use crate::utils::immediate_rw_lock::ImmediateRwLock;

pub struct App {
    exit:           bool,
    current_screen: Arc<ImmediateRwLock<Screen>>,
    screens:        HashMap<Screen, Box<dyn ScreenTrait>>,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;
    const TICKS_PER_SECOND: f32 = App::FRAMES_PER_SECOND / 10.0;

    pub fn new() -> App {
        let current_screen = Arc::new(ImmediateRwLock::default());
        let mut screens: HashMap<Screen, Box<dyn ScreenTrait>> = HashMap::new();

        screens.insert(Screen::Login, Box::new(LoginScreen::new(current_screen.clone())));

        Self {
            exit: false,
            current_screen,
            screens,
        }
    }

    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let frame_period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let tick_period = Duration::from_secs_f32(1.0 / Self::TICKS_PER_SECOND);

        let mut frames_interval = tokio::time::interval(frame_period);
        let mut ticks_interval = tokio::time::interval(tick_period);
        let mut events = EventStream::new();

        while !self.exit {
            let screen = self.screens.get_mut(&self.current_screen.get().unwrap()).unwrap();

            tokio::select! {
                _ = frames_interval.tick() => { terminal.draw(|frame| screen.render(frame))?; },
                instant = ticks_interval.tick() => { self.on_tick(instant); },
                Some(Ok(event)) = events.next() => { self.handle_event(event); },
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Option<()> {
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.on_key_pressed(event),
            _ => Some(()),
        }
    }

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        if event.code == KeyCode::Esc {
            self.exit = true;
            return None;
        }

        if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
            self.exit = true;
            return None;
        }

        let screen = self.screens.get_mut(&self.current_screen.get().unwrap()).unwrap();
        screen.on_key_pressed(event)?;

        Some(())
    }

    fn on_tick(&mut self, instant: Instant) {
        let screen = self.screens.get_mut(&self.current_screen.get().unwrap()).unwrap();
        screen.on_tick(instant);
    }
}
