use std::io::Result;

use crossterm::event::KeyEvent;
use hackernews::get_items::ItemResponse;
use html2text::config;
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    widgets::{List, ListItem, Paragraph},
};
use tokio::sync::watch;

use crate::{ChannelAction, ChannelData, components::Component};

#[derive(Debug)]
pub struct Comment {
    ids: Vec<usize>,
    data: Option<Vec<ItemResponse>>,
    pub focus: bool,
}

impl Comment {
    pub fn new(ids: Vec<usize>) -> Self {
        Comment {
            ids,
            focus: false,
            data: None,
        }
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
            self.data = Some(data);
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
        let list = List::new(self.data.as_ref().map_or(vec![], |data| {
            data.iter()
                .filter(|item| item.by.is_some() && item.text.is_some())
                .map(|item| {
                    let text = format!(
                        "&lt;&lt;{}&gt;&gt;: {}",
                        item.by.clone().unwrap(),
                        item.text.clone().unwrap()
                    );
                    ListItem::new(
                        config::plain()
                            .no_link_wrapping()
                            .link_footnotes(false)
                            .string_from_read(text.as_bytes(), (rect.width - 2).into())
                            .unwrap(),
                    )
                    .style(ratatui::style::Style::new())
                })
                .collect::<Vec<_>>()
        }))
        .block(block.clone());

        if self.ids.is_empty() {
            f.render_widget(Paragraph::new("No comments available").block(block), rect);
        } else if self.data.is_none() {
            f.render_widget(Paragraph::new("Loading comments...").block(block), rect);
        } else {
            f.render_widget(list, rect);
        };

        Ok(())
    }

    fn event(&mut self, _key: KeyEvent, _action: watch::Sender<ChannelAction>) {}
}
