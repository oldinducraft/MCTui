use std::sync::Arc;

use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::{Screen, ScreenTrait};
use crate::utils::config::Config;
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::ui::center::center;
use crate::widgets::window::Window;

pub struct HomeScreen {
    current_screen: Arc<ImmediateRwLock<Screen>>,
    config:         Arc<Config>,
}

const KEY_HINTS: [(&str, &str); 1] = [("Esc/Ctrl+C", "Exit")];

impl ScreenTrait for HomeScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Home".bold().red(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Percentage(50));

        frame.render_widget(Paragraph::new("Home"), area);
    }

    fn new(current_screen: Arc<ImmediateRwLock<Screen>>, config: Arc<Config>) -> HomeScreen {
        HomeScreen { current_screen, config }
    }
}
