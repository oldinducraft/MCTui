use std::sync::Arc;

use crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, Flex, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use tui_widgets::big_text::{BigText, PixelSize};

use super::{Screen, ScreenTrait};
use crate::utils::Libs;

pub struct HomeScreen {
    libs: Arc<Libs>,
}

impl ScreenTrait for HomeScreen {
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .flex(Flex::Center)
            .split(frame.area());

        let big_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .lines(vec!["Oldindcraft".into()])
            .centered()
            .build();
        frame.render_widget(big_text, layout[0]);

        let text = vec![
            Line::from(vec![
                "Hi, ".bold(),
                self.libs
                    .config
                    .get_username()
                    .unwrap_or("???".to_string())
                    .light_magenta()
                    .bold(),
                "!".bold(),
            ]),
            Line::from(vec![]),
            Line::from(vec![
                "To ".into(),
                "play".cyan().bold(),
                " press ".into(),
                "<Enter>".cyan().bold(),
                ",".into(),
            ]),
            Line::from(vec![
                "To ".into(),
                "log out".red().bold(),
                " press ".into(),
                "<Del>".red().bold(),
                ",".into(),
            ]),
            Line::from(vec![
                "To ".into(),
                "exit".yellow().bold(),
                " press ".into(),
                "<Ctrl+C".yellow().bold(),
                " / ".into(),
                "Esc>".yellow().bold(),
            ]),
        ];

        frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), layout[1]);
    }

    fn new(libs: Arc<Libs>) -> HomeScreen {
        if libs.in_memory.get_access_token().is_none() || libs.in_memory.get_client_token().is_none() {
            libs.screen.goto(Screen::Authenticate);
        }

        if libs.in_memory.get_username().is_none() || libs.in_memory.get_uuid().is_none() {
            libs.screen.goto(Screen::Authenticate);
        }

        HomeScreen { libs }
    }

    fn on_key_pressed(&mut self, event: crossterm::event::KeyEvent) -> Option<()> {
        match event.code {
            KeyCode::Enter => unimplemented!(),
            KeyCode::Delete => self.log_out(),
            _ => return Some(()),
        };

        None
    }
}

impl HomeScreen {
    pub fn log_out(&self) {
        self.libs.config.set_username(None);
        self.libs.config.set_password(None);
        self.libs.config.save();
        self.libs.screen.goto(Screen::Login);
    }
}
