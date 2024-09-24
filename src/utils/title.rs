use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::block::{Position, Title};

pub fn title(title: &'static str, style: Style) -> Title<'static> {
    Title::from(vec!["< ".into(), Span::styled(title, style), " >".into()])
        .alignment(Alignment::Center)
        .position(Position::Top)
}
