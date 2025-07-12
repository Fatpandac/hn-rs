use std::io::Result;

use crossterm::event::KeyEvent;
use hackernews::get_items::ItemResponse;
use html2text::config;
use ratatui::{Frame, layout::Rect, style::Stylize, widgets::Paragraph};
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
        let content = Paragraph::new({
            if self.ids.len() == 0 {
                "No comments available".into()
            } else if self.data.is_none() {
                "Loading Comments...".into()
            } else {
                config::plain()
                    .string_from_read(
                        self.data
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|item| item.by.is_some() && item.text.is_some())
                            .map(|item| {
                                format!(
                                    "&lt;&lt;{}&gt;&gt;: {}",
                                    item.by.clone().unwrap(),
                                    item.text.clone().unwrap()
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("<br><br>")
                            .as_bytes(),
                        (rect.width - 2).into(),
                    )
                    .unwrap()
            }
        })
        .block(block);
        f.render_widget(content, rect);
        Ok(())
    }

    fn event(&mut self, _key: KeyEvent, _action: watch::Sender<ChannelAction>) {}
}
