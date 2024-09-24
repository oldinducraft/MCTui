use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Constraint;
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;

use super::ScreenTrait;
use crate::utils::center::center;
use crate::utils::key_hint::key_hint_title;
use crate::utils::title::title;
use crate::widgets::login_form::LoginForm;
use crate::widgets::window::Window;

pub struct HomeScreen {
    window: Window,
    form:   LoginForm,
}

const KEY_HINTS: [(&str, &str); 3] = [("Esc/Ctrl+C", "Exit"), ("Enter", "Submit"), ("Tab", "Next field")];

impl HomeScreen {
    pub fn new() -> Self {
        Self {
            window: Window::new(
                title(
                    "Who tf are you",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                key_hint_title(&KEY_HINTS),
            ),
            form:   LoginForm::default(),
        }
    }
}

impl ScreenTrait for HomeScreen {
    fn render(&mut self, frame: &mut Frame) {
        // 10 = 2 (window) + 3 (username) + 3 (password) + 2 (placeholder)
        let area = center(frame.area(), Constraint::Percentage(50), Constraint::Length(9));
        frame.render_widget(&mut self.window, area);

        self.form.render(area, frame);
    }

    fn on_key_pressed(&mut self, event: KeyEvent) -> Option<()> {
        self.form.on_key_pressed(event)?;

        if event.code == KeyCode::Enter {
            self.form.update_error(Some("Unimplemented".to_string()));
            return None;
        }

        Some(())
    }

    fn on_tick(&mut self) { self.form.on_tick(); }
}
