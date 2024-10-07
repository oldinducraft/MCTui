use ratatui::layout::Constraint;
use ratatui::Frame;

use super::VerifyScreen;
use crate::screens::RenderableScreen;
use crate::utils::ui::center::center;
use crate::widgets::loader::Loader;
use crate::widgets::window::Window;

const KEY_HINTS: [(&str, &str); 1] = [("Esc/Ctrl+C", "Exit")];

impl RenderableScreen for VerifyScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Verify".into(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Length(10));

        frame.render_widget(window, area);
        frame.render_stateful_widget(
            Loader {
                text: "Verifying client...".to_string(),
            },
            area,
            &mut self.loader_state,
        );
    }
}
