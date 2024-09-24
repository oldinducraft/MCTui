use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Widget};

pub struct Window {
    title:    Title<'static>,
    key_hint: Title<'static>,
}

impl Window {
    pub fn new(title: Title<'static>, key_hint: Title<'static>) -> Self { Self { title, key_hint } }
}

impl Widget for &mut Window {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.clone())
            .title(self.key_hint.clone());

        block.render(area, buf);
    }
}
