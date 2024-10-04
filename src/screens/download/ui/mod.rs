use ratatui::layout::Constraint;
use ratatui::Frame;

use super::DownloadScreen;
use crate::screens::RenderableScreenTrait;
use crate::utils::ui::center::center;
use crate::widgets::progress::Progress;

impl RenderableScreenTrait for DownloadScreen {
    fn render(&mut self, frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Percentage(100), Constraint::Length(3));

        frame.render_stateful_widget(
            Progress {
                title: "Downloading".to_string(),
            },
            area,
            &mut self.progress_state,
        );
    }
}
