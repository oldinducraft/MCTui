use menu::Menu;
use ratatui::layout::{Constraint, Flex, Layout};
use title::Title;

use super::HomeScreen;
use crate::screens::RenderableScreen;

pub mod menu;
pub mod title;

impl RenderableScreen for HomeScreen {
    fn render(&mut self, frame: &mut ratatui::Frame) {
        let layout = Layout::default()
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .flex(Flex::Center)
            .split(frame.area());

        frame.render_widget(Title, layout[0]);
        frame.render_widget(
            Menu {
                username: self.libs.config.get_username().unwrap_or("???".to_string()),
            },
            layout[1],
        );
    }
}
