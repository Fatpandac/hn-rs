use std::io::Result;

use chrono::DateTime;
use crossterm::event::{KeyCode, KeyEvent};
use hackernews::get_items::ItemResponse;
use html2text::config;
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
};

use crate::{components::Component, panels::Comment};

pub struct Article {
    pub data: Option<ItemResponse>,
    pub focus: bool,
    scroll_offset: u16,
    block_height: u16,
    block_width: u16,
    comment: Comment,
}

impl Component for Article {
    fn draw(&mut self, f: &mut Frame, rect: Rect) -> Result<()> {
        let vertical = Layout::vertical(if self.comment.focus {
            [
                ratatui::layout::Constraint::Percentage(50),
                ratatui::layout::Constraint::Percentage(50),
            ]
        } else {
            [
                ratatui::layout::Constraint::Percentage(100),
                ratatui::layout::Constraint::Percentage(0),
            ]
        });
        let [top, bottom] = vertical.areas(rect);
        self.block_height = top.height;
        self.block_width = top.width;

        let right_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style({
                if self.focus {
                    Style::new().blue()
                } else {
                    Style::new()
                }
            })
            .title("Article");

        let article = Paragraph::new(
            self.data
                .clone()
                .map_or("No article selected".to_string(), |_| {
                    self.generate_content()
                }),
        )
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(right_block)
        .scroll((self.scroll_offset, 0));

        f.render_widget(article, top);
        self.comment.draw(f, bottom)?;
        Ok(())
    }

    fn event(&mut self, key: KeyEvent) {
        if self.focus {
            if key.code == KeyCode::Char('j') {
                self.scroll(false);
            } else if key.code == KeyCode::Char('k') {
                self.scroll(true);
            } else if key.code == KeyCode::Char('o') {
                if let Some(item) = &self.data {
                    if let Some(url) = &item.url {
                        if let Err(e) = open::that(url) {
                            eprintln!("Failed to open URL: {}", e);
                        }
                    }
                }
            } else if key.code == KeyCode::Char('c') {
                self.comment.focus = true;
                self.focus = false;
            }
        } else {
            if key.code == KeyCode::Char('c') {
                self.comment.focus = false;
                self.focus = true;
            }
            self.comment.event(key);
        }
    }
}

impl Article {
    pub fn new(data: Option<ItemResponse>, focus: bool, scroll_offset: u16) -> Self {
        Self {
            data,
            focus,
            scroll_offset,
            block_height: 0,
            block_width: 0,
            comment: Comment::new(Vec::new()),
        }
    }

    pub fn set_data(&mut self, data: Option<ItemResponse>) {
        if self.data == data {
            return;
        }
        self.data = data.clone();
        self.comment = Comment::new(
            data.as_ref()
                .map_or(Vec::new(), |item| item.kids.clone().unwrap_or_default()),
        );
        self.scroll_offset = 0;
    }

    pub fn scroll(&mut self, up: bool) {
        let padding = 2;
        let content_height = {
            let content = self.generate_content();
            content.lines().fold(0, |acc, line| {
                acc + (line.len() as u16 / self.block_width).saturating_add(1)
            }) + padding
        };
        self.scroll_offset = {
            if up {
                self.scroll_offset.saturating_sub(1)
            } else if self.scroll_offset + self.block_height < content_height + padding {
                self.scroll_offset.saturating_add(1)
            } else {
                self.scroll_offset
            }
        }
    }

    fn generate_content(&self) -> String {
        self.data
            .as_ref()
            .map_or("No article selected".to_string(), |item| {
                format!(
                    "Title: {}\nAuthor: {}\nTime: {}\nURL: {}\n\n{}",
                    item.title.clone().unwrap_or("No title".to_string()),
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
                            self.block_width.into()
                        )
                        .unwrap()
                )
            })
    }
}
