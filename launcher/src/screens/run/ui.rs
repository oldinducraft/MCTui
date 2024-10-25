use ratatui::layout::Constraint;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

use super::RunScreen;
use crate::screens::RenderableScreen;
use crate::utils::ui::center::center;
use crate::widgets::window::Window;

const KEY_HINTS: [(&str, &str); 1] = [("Esc/Ctrl+C", "Exit")];

impl RenderableScreen for RunScreen {
    fn render(&mut self, frame: &mut Frame) {
        let window = Window::new("Run".into(), &KEY_HINTS);
        let area = center(frame.area(), Constraint::Percentage(100), Constraint::Percentage(100));

        frame.render_widget(window, area);

        let binding = self.lines.read().expect("Failed to read lines");
        let lines = binding
            .iter()
            .map(|line| Line::from(line.as_str()))
            .rev()
            .collect::<Vec<_>>();

        frame.render_widget(
            Paragraph::new(lines).block(Block::default().padding(Padding::uniform(1))),
            area,
        );
    }
}
