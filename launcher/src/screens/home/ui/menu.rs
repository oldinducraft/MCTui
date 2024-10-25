use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct Menu {
    pub username: String,
}

impl Widget for Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            Line::from(vec!["Hi, ".bold(), self.username.light_magenta().bold(), "!".bold()]),
            Line::from(vec![]),
            Line::from(vec![
                "To ".into(),
                "play".cyan().bold(),
                " press ".into(),
                "<Enter>".cyan().bold(),
                ",".into(),
            ]),
            Line::from(vec![
                "To ".into(),
                "log out".red().bold(),
                " press ".into(),
                "<Del>".red().bold(),
                ",".into(),
            ]),
            Line::from(vec![
                "To ".into(),
                "exit".yellow().bold(),
                " press ".into(),
                "<Ctrl+C".yellow().bold(),
                " / ".into(),
                "Esc>".yellow().bold(),
            ]),
        ];

        Paragraph::new(text).alignment(Alignment::Center).render(area, buf);
    }
}
