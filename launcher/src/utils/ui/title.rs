use ratatui::layout::Alignment;
use ratatui::text::Span;
use ratatui::widgets::block::{Position, Title};

pub fn title(title: Span<'static>) -> Title<'static> {
    Title::from(vec!["< ".into(), title, " >".into()])
        .alignment(Alignment::Center)
        .position(Position::Top)
}
