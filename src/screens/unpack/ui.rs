use ratatui::layout::Constraint;
use ratatui::Frame;

use super::UnpackScreen;
use crate::screens::RenderableScreen;
use crate::utils::ui::center::center;
use crate::widgets::progress::Progress;

impl RenderableScreen for UnpackScreen {
    fn render(&mut self, frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Percentage(100), Constraint::Length(3));

        frame.render_stateful_widget(
            Progress {
                title: "Unpacking".to_string(),
            },
            area,
            &mut self.progress_state,
        );
    }
}
