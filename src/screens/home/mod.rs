use std::sync::Arc;

use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
use ratatui::Frame;

use super::ScreenTrait;
use crate::utils::ui::center::center;
use crate::utils::Libs;
use crate::widgets::window::Window;

pub struct HomeScreen {
    libs: Arc<Libs>,
}

const KEY_HINTS: [(&str, &str); 1] = [("Esc/Ctrl+C", "Exit")];

impl ScreenTrait for HomeScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Home".into(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Percentage(50));
        frame.render_widget(window, area);

        let layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        frame.render_widget(
            Paragraph::new(format!(
                "Access: {:?}",
                self.libs.in_memory.read().unwrap().get_access_token()
            ))
            .block(Block::default().padding(Padding::uniform(1)))
            .wrap(Wrap { trim: false }),
            layout[0],
        );
        frame.render_widget(
            Paragraph::new(format!(
                "Client: {:?}",
                self.libs.in_memory.read().unwrap().get_client_token()
            ))
            .block(Block::default().padding(Padding::uniform(1)))
            .wrap(Wrap { trim: false }),
            layout[1],
        );
    }

    fn new(libs: Arc<Libs>) -> HomeScreen { HomeScreen { libs } }
}
