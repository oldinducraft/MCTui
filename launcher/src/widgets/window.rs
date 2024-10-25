use std::cmp;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Widget};

use crate::utils::ui::{key_hint as key_hint_utils, title as title_utils};

pub struct Window {
    title:    Title<'static>,
    key_hint: Title<'static>,
}

impl Window {
    pub fn new(title: Span<'static>, key_hint: &[key_hint_utils::KeyHint]) -> Self {
        Self {
            title:    title_utils::title(title),
            key_hint: key_hint_utils::key_hint(key_hint),
        }
    }

    pub fn max_width(&self) -> usize { cmp::max(self.title.content.width(), self.key_hint.content.width()) + 2 }
}

impl Widget for Window {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .title(self.key_hint)
            .render(area, buf);
    }
}
