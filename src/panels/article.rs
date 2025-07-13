use std::io::Result;

use chrono::DateTime;
use crossterm::event::{KeyCode, KeyEvent};
use hackernews::get_items::ItemResponse;
use html2text::config;
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    prelude::Color,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Wrap},
};
use tokio::sync::watch;

use crate::{ChannelAction, ChannelData, components::Component, panels::Comment};

pub struct Article {
    pub data: Option<ItemResponse>,
    pub focus: bool,
    scroll_offset: u16,
    scroll_offset_backup: u16,
    block_height: u16,
    block_width: u16,
    comment: Comment,
}

impl Component for Article {
    fn draw(
        &mut self,
        f: &mut Frame,
        rect: Rect,
        data: watch::Receiver<ChannelData>,
    ) -> Result<()> {
        let vertical = Layout::vertical(if self.comment.focus {
            [
                ratatui::layout::Constraint::Percentage(20),
                ratatui::layout::Constraint::Percentage(80),
            ]
        } else {
            [
                ratatui::layout::Constraint::Percentage(100),
                ratatui::layout::Constraint::Percentage(0),
            ]
        });
        let [top, bottom] = vertical.areas(rect);
        self.block_height = top.height.saturating_sub(2);
        self.block_width = top.width.saturating_sub(2);

        let right_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style({
                if self.focus {
                    Style::new().blue()
                } else {
                    Style::new()
                }
            })
            .title("Article")
            .title_bottom(Line::from({
                if self.focus {
                    vec![
                        Span::styled("C", Style::default().fg(Color::Red)),
                        Span::raw("omments"),
                    ]
                } else {
                    vec![]
                }
            }));

        let article = Paragraph::new(
            self.data
                .clone()
                .map_or("No article selected".to_string(), |_| {
                    self.generate_content()
                }),
        )
        .wrap(Wrap { trim: true })
        .block(right_block)
        .scroll((self.scroll_offset, 0));

        f.render_widget(article, top);
        self.comment.draw(f, bottom, data)?;
        Ok(())
    }

    fn event(&mut self, key: KeyEvent, action: watch::Sender<ChannelAction>) {
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
                self.scroll_offset_backup = self.scroll_offset;
                self.scroll_offset = 0;
                if self.data.is_some() {
                    action
                        .send(ChannelAction::Items(
                            self.data.clone().unwrap().kids.unwrap_or_default(),
                        ))
                        .unwrap();
                }
            }
        } else {
            if key.code == KeyCode::Char('c') {
                self.comment.focus = false;
                self.focus = true;
                self.scroll_offset = self.scroll_offset_backup;
                self.scroll_offset_backup = 0;
                action.send(ChannelAction::Items(Vec::new())).unwrap();
            }
            self.comment.event(key, action);
        }
    }
}

impl Article {
    pub fn new(data: Option<ItemResponse>, focus: bool) -> Self {
        Self {
            data,
            focus,
            scroll_offset: 0,
            scroll_offset_backup: 0,
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
        let content_height = {
            let content = self.generate_content();
            content.lines().fold(0, |acc, line| {
                acc + (line.len() as u16 / self.block_width).saturating_add(1)
            })
        };
        self.scroll_offset = {
            if up {
                self.scroll_offset.saturating_sub(1)
            } else if self.scroll_offset + self.block_height < content_height {
                self.scroll_offset.saturating_add(1)
            } else {
                self.scroll_offset
            }
        };
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
                        .link_footnotes(false)
                        .no_link_wrapping()
                        .string_from_read(
                            item.text
                                .as_deref()
                                .unwrap_or("No content available")
                                .as_bytes(),
                            (self.block_width - 2).into()
                        )
                        .unwrap()
                )
            })
    }
}
