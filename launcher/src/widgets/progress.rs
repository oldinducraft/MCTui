use std::sync::Arc;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Gauge, StatefulWidget};

use super::progress_state::ProgressState;

pub struct Progress {
    pub title: String,
}

impl StatefulWidget for Progress {
    type State = Arc<ProgressState>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Gauge::default()
            .block(Block::bordered().title(self.title))
            .ratio(
                state
                    .percent
                    .get()
                    .unwrap_or_else(|err| panic!("Failed to get progress: {}", err)),
            )
            .render(area, buf);
    }
}
