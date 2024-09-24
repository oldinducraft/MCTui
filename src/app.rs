use std::collections::HashMap;
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::screens::home::HomeScreen;
use crate::screens::{Screen, ScreenTrait};

pub struct App {
    exit:           bool,
    current_screen: Screen,
    screens:        HashMap<Screen, Box<dyn ScreenTrait>>,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new() -> App {
        let mut screens: HashMap<Screen, Box<dyn ScreenTrait>> = HashMap::new();
        screens.insert(Screen::Home, Box::new(HomeScreen::new()));

        Self {
            exit: false,
            current_screen: Screen::Home,
            screens,
        }
    }

    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.exit {
            let screen = self.screens.get_mut(&self.current_screen).unwrap();

            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| screen.render(frame))?; },
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

        let screen = self.screens.get_mut(&self.current_screen).unwrap();
        screen.on_key_pressed(event)?;

        Some(())
    }
}
