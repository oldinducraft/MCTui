use std::iter;

use itertools::Itertools;
use ratatui::layout::Alignment;
use ratatui::style::Stylize;
use ratatui::text::Span;
use ratatui::widgets::block::{Position, Title};

pub fn key_hint_title(keys: &[(&'static str, &'static str)]) -> Title<'static> {
    #[allow(unstable_name_collisions)]
    let lines = keys
        .iter()
        .map(|hint| vec![format!("<{}>", hint.0).cyan().bold(), format!(" {}", hint.1).bold()])
        .intersperse(vec![" | ".dark_gray().bold()])
        .flatten();

    let lines: Vec<Span<'_>> = iter::once("[ ".into())
        .chain(lines)
        .chain(iter::once(" ]".into()))
        .collect_vec();

    Title::from(lines)
        .alignment(Alignment::Center)
        .position(Position::Bottom)
}
