use std::{io::Result, usize};

use crossterm::event::{KeyCode, KeyEvent};
use hackernews::get_items::ItemResponse;
use html2text::config;
use ratatui::{Frame, layout::Rect, style::Stylize, widgets::Paragraph};
use tokio::sync::watch;

use crate::{ChannelAction, ChannelData, components::Component};

#[derive(Debug)]
pub struct Comment {
    ids: Vec<usize>,
    data: Option<Vec<ItemResponse>>,
    scroll_offset: u16,
    content_height: u16,
    block_width: u16,
    pub focus: bool,
}

impl Comment {
    pub fn new(ids: Vec<usize>) -> Self {
        Comment {
            ids,
            scroll_offset: 0,
            content_height: 0,
            block_width: 0,
            focus: false,
            data: None,
        }
    }

    fn formater(&self, item: &ItemResponse, max_width: usize, indent: Option<i16>) -> String {
        let indent_space = " ".repeat(indent.unwrap_or(0) as usize);
        let mut text = format!(
            "{}&lt;&lt;{}&gt;&gt;: {}",
            indent_space,
            item.by.clone().unwrap_or("".to_string()),
            html_escape::decode_html_entities(&item.text.clone().unwrap_or("".to_string()))
                .to_string()
        );
        if let Some(children) = &item.children {
            let children_text = children
                .into_iter()
                .map(|child| {
                    let text = self.formater(child, max_width, Some(indent.unwrap_or(0) + 2));
                    return text;
                })
                .collect::<Vec<_>>()
                .join("\n");
            text = text + "\n" + &children_text;
        }

        return if indent.is_none() {
            let show_content = config::rich()
                .link_footnotes(false)
                .string_from_read(format!("<pre>{}</pre>", text).as_bytes(), max_width)
                .unwrap();

            show_content
        } else {
            text
        };
    }

    pub fn scroll(&mut self, up: bool) {
        self.scroll_offset = {
            if up {
                self.scroll_offset.saturating_sub(1)
            } else {
                self.scroll_offset
                    .saturating_add(1)
                    .min(self.content_height)
            }
        };
    }
}

impl Component for Comment {
    fn draw(
        &mut self,
        f: &mut Frame,
        rect: Rect,
        data: watch::Receiver<ChannelData>,
    ) -> Result<()> {
        if let ChannelData::Comment(Some(data)) = data.borrow().clone() {
            if let Some(mut items) = self.data.clone() {
                if items.iter().map(|item| item.id).all(|id| id != data.id) {
                    items.push(data);
                    self.data = Some(items);
                }
            } else {
                self.data = Some(vec![data]);
            }
        }
        let block = ratatui::widgets::Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Comments")
            .border_style({
                if self.focus {
                    ratatui::style::Style::new().blue()
                } else {
                    ratatui::style::Style::new()
                }
            });
        f.render_widget(
            if self.ids.is_empty() {
                Paragraph::new("No comments available").block(block)
            } else if self.data.is_none() {
                Paragraph::new("Loading comments...").block(block)
            } else {
                let content = self.data.as_ref().map_or("".to_string(), |data| {
                    data.iter()
                        .map(|item| {
                            self.formater(item, rect.width.saturating_sub(2) as usize, None)
                        })
                        .collect::<Vec<_>>()
                        .join("")
                });
                self.block_width = rect.width.saturating_sub(2);
                self.content_height = content.lines().fold(0, |acc, line| {
                    acc + (line.len() as u16 / self.block_width).saturating_add(1)
                });
                Paragraph::new(content)
                    .block(block)
                    .scroll((self.scroll_offset, 0))
            },
            rect,
        );

        Ok(())
    }

    fn event(&mut self, key: KeyEvent, _action: watch::Sender<ChannelAction>) {
        if self.focus {
            if key.code == KeyCode::Char('j') {
                self.scroll(false);
            } else if key.code == KeyCode::Char('k') {
                self.scroll(true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formater() {
        let comment = Comment::new(vec![]);
        let mut item = ItemResponse::default();
        item.children = Some(vec![
            ItemResponse::default(),
            ItemResponse::default(),
            ItemResponse::default(),
        ]);

        let str = comment.formater(&item, 80, None);
        let res = "<<Linux>>: This is a default item\n  \
                    <<Linux>>: This is a default item\n  \
                    <<Linux>>: This is a default item\n  \
                    <<Linux>>: This is a default item\n";

        assert_eq!(str, res);
    }
}
