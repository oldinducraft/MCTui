use form::LoginForm;
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Paragraph;

use super::LoginScreen;
use crate::screens::RenderableScreenTrait;
use crate::utils::ui::center::center;
use crate::widgets::window::Window;

pub mod form;
pub mod form_state;

const KEY_HINTS: [(&str, &str); 3] = [("Esc/Ctrl+C", "Exit"), ("Enter", "Submit"), ("Tab", "Next field")];

impl RenderableScreenTrait for LoginScreen {
    fn render(&mut self, frame: &mut ratatui::Frame) {
        let window = Window::new("Who tf are you".bold().red(), &KEY_HINTS);

        let width_constraint = Constraint::Length((frame.area().width / 2).max(window.max_width() as u16));
        let area = center(frame.area(), width_constraint, Constraint::Percentage(70));

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)].as_ref())
            .margin(2)
            .flex(Flex::Center)
            .split(area);

        frame.render_widget(window, area);
        frame.render_stateful_widget(LoginForm, layout[0], &mut self.form);

        if let Some(err) = &self.error {
            frame.render_widget(
                Paragraph::new(err.as_str())
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center),
                layout[1],
            );
        }
    }
}
