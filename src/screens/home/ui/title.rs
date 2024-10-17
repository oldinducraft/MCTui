use ratatui::prelude::*;
use tui_widgets::big_text::{BigText, PixelSize};

use crate::constants::ORGANIZATION;

pub struct Title;

impl Widget for Title {
    fn render(self, area: Rect, buf: &mut Buffer) {
        BigText::builder()
            .pixel_size(PixelSize::Full)
            .lines(vec![ORGANIZATION.into()])
            .centered()
            .build()
            .render(area, buf);
    }
}
