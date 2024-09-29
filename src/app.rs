use std::collections::HashMap;
use std::io::{self, Stdout};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::screens::authenticate::AuthenticateScreen;
use crate::screens::home::HomeScreen;
use crate::screens::login::LoginScreen;
use crate::screens::{Screen, ScreenTrait};
use crate::utils::Libs;

pub struct App {
    exit:    bool,
    screens: HashMap<Screen, Box<dyn ScreenTrait>>,
    libs:    Arc<Libs>,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;
    const TICKS_PER_SECOND: f32 = App::FRAMES_PER_SECOND / 10.0;

    pub fn new() -> App {
        let libs = Arc::new(Libs::new());
        let mut screens: HashMap<Screen, Box<dyn ScreenTrait>> = HashMap::new();

        screens.insert(Screen::Login(None), Box::new(LoginScreen::new(libs.clone())));
        screens.insert(Screen::Home, Box::new(HomeScreen::new(libs.clone())));
        screens.insert(Screen::Authenticate(Instant::now()), Box::new(AuthenticateScreen::new(libs.clone())));

        Self {
            exit: false,
            screens,
            libs,
        }
    }

    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let frame_period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let tick_period = Duration::from_secs_f32(1.0 / Self::TICKS_PER_SECOND);

        let mut frames_interval = tokio::time::interval(frame_period);
        let mut ticks_interval = tokio::time::interval(tick_period);
        let mut events = EventStream::new();

        while !self.exit {
            let screen = self.libs.screen.get_current();
            let screen = self.screens.get_mut(&screen).unwrap();

            tokio::select! {
                _ = frames_interval.tick() => { terminal.draw(|frame| screen.render(frame))?; },
                _ = ticks_interval.tick() => { self.on_tick(); },
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

        let screen = self.libs.screen.get_current();
        let screen = self.screens.get_mut(&screen).unwrap();
        screen.on_key_pressed(event)?;

        Some(())
    }

    fn on_tick(&mut self) {
        let screen = self.libs.screen.get_current();
        let screen = self.screens.get_mut(&screen).unwrap();
        screen.on_tick();
    }
}
