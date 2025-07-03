use std::io::Result;

use chrono::DateTime;
use hackernews::get_items::ItemResponse;
use html2text::config;
use ratatui::{
    Frame,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
};

pub struct RightBlock {
    data: Option<ItemResponse>,
    focus: bool,
}

impl RightBlock {
    pub fn new(data: Option<ItemResponse>, focus: bool) -> Self {
        Self {
            data,
            focus,
        }
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    pub fn set_data(&mut self, data: Option<ItemResponse>) {
        self.data = data;
    }

    pub fn draw(&mut self, f: &mut Frame, rect: ratatui::layout::Rect) -> Result<()> {
        let right_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style({
                if self.focus {
                    Style::new().blue()
                } else {
                    Style::new()
                }
            })
            .title("Content");

        let article = Paragraph::new(self.data.clone().map_or(
            "No article selected".to_string(),
            |item| {
                format!(
                    "Title: {}\nAuthor: {}\nTime: {}\nURL: {}\n\n{}",
                    item.title,
                    item.by.as_deref().unwrap_or("Unknown"),
                    DateTime::from_timestamp(item.time as i64, 0)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                    item.url.as_deref().unwrap_or("No URL"),
                    config::plain()
                        .string_from_read(
                            item.text
                                .as_deref()
                                .unwrap_or("No content available")
                                .as_bytes(),
                            rect.width.into()
                        )
                        .unwrap()
                )
            },
        ))
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(right_block);

        f.render_widget(article, rect);
        Ok(())
    }
}
