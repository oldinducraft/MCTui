use std::sync::Arc;

use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, StatefulWidget};
use throbber_widgets_tui::{Throbber, ThrobberState, WhichUse};

use super::types::LoginRequestState;
use crate::utils::immediate_rw_lock::ImmediateRwLock;
use crate::utils::ui::center::center;

pub struct RequestLoader;

impl StatefulWidget for RequestLoader {
    type State = RequestLoaderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // 12 = 10 (text) + 2 (throbber and space after it)
        let throbber_center = center(area, Constraint::Length(12), Constraint::Length(1));

        match state.state.get().unwrap() {
            LoginRequestState::Pending => {
                StatefulWidget::render(self.get_throbber(), throbber_center, buf, &mut state.throbber_state)
            },
            LoginRequestState::Fulfilled => {
                Paragraph::new(Line::from(vec![
                    "Login successful. Press ".into(),
                    "<Enter>".cyan(),
                    " again to continue".into(),
                ]))
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center)
                .render(area, buf);
            },
            LoginRequestState::Rejected(err) => {
                Paragraph::new(err)
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center)
                    .render(area, buf);
            },
            LoginRequestState::Idle => {},
        }
    }
}

impl RequestLoader {
    fn get_throbber(&self) -> Throbber<'_> {
        Throbber::default()
            .label("Running...")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .throbber_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
            .throbber_set(throbber_widgets_tui::BOX_DRAWING)
            .use_type(WhichUse::Spin)
    }
}

#[derive(Default)]
pub struct RequestLoaderState {
    throbber_state: ThrobberState,
    pub state:      Arc<ImmediateRwLock<LoginRequestState>>,
}

impl RequestLoaderState {
    pub fn on_tick(&mut self) {
        if matches!(self.state.get().unwrap(), LoginRequestState::Pending) {
            self.throbber_state.calc_next();
        }
    }
}
