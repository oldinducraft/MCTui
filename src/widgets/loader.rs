use ratatui::prelude::*;
use ratatui::widgets::StatefulWidget;
use throbber_widgets_tui::{Throbber, WhichUse};

use super::loader_state::LoaderState;
use crate::utils::ui::center::center;

pub struct Loader {
    pub text: String,
}

impl StatefulWidget for Loader {
    type State = LoaderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = center(
            area,
            Constraint::Length(self.text.len() as u16 + 2),
            Constraint::Length(1),
        );

        StatefulWidget::render(self.get_throbber(), area, buf, &mut state.throbber_state)
    }
}

impl Loader {
    fn get_throbber(&self) -> Throbber<'_> {
        Throbber::default()
            .label("Trying to log in...")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .throbber_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
            .throbber_set(throbber_widgets_tui::BOX_DRAWING)
            .use_type(WhichUse::Spin)
    }
}
