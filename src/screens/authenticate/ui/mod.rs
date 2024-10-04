use loader::Loader;
use ratatui::layout::Constraint;
use ratatui::Frame;

use super::AuthenticateScreen;
use crate::screens::RenderableScreenTrait;
use crate::utils::ui::center::center;
use crate::widgets::window::Window;

pub mod loader;
pub mod loader_state;

const KEY_HINTS: [(&str, &str); 1] = [("Esc/Ctrl+C", "Exit")];

impl RenderableScreenTrait for AuthenticateScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Authenticate".into(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Length(10));

        frame.render_widget(window, area);
        frame.render_stateful_widget(Loader, area, &mut self.loader_state);
    }
}
