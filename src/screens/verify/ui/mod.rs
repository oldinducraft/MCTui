use ratatui::layout::Constraint;
use ratatui::Frame;

use super::VerifyScreen;
use crate::screens::RenderableScreenTrait;
use crate::utils::ui::center::center;
use crate::widgets::progress::Progress;

impl RenderableScreenTrait for VerifyScreen {
    fn render(&mut self, frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Percentage(100), Constraint::Length(3));

        frame.render_stateful_widget(
            Progress {
                title: "Verifying".to_string(),
            },
            area,
            &mut self.progress_state,
        );
    }
}
