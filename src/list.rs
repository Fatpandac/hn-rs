use std::io::Result;

use hackernews::{StoryType, get_items::ItemResponse};
use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem},
};

pub struct LeftBlock {
    data: Vec<ItemResponse>,
    selected: usize,
    topic: StoryType,
    list_top_cursor: usize,
    focus: bool,
}

impl LeftBlock {
    pub fn new(
        data: Vec<ItemResponse>,
        selected: usize,
        topic: StoryType,
        list_top_cursor: usize,
        focus: bool,
    ) -> Self {
        Self {
            data,
            selected,
            topic,
            list_top_cursor,
            focus,
        }
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    pub fn set_data(&mut self, data: Vec<ItemResponse>) {
        self.data = data;
    }

    pub fn set_selected(&mut self, selected: usize) {
        self.selected = selected;
    }

    pub fn set_topic(&mut self, topic: StoryType) {
        self.topic = topic;
    }

    pub fn draw(&mut self, f: &mut Frame, rect: ratatui::layout::Rect) -> Result<()> {
        let height = rect.height as usize;
        let left_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style({
                if self.focus {
                    Style::new().blue()
                } else {
                    Style::new()
                }
            })
            .title(Line::from(vec![
                Span::raw("<"),
                Span::styled("T", Style::default().fg(Color::Red)),
                Span::raw(format!(
                    " - {}({}/{})>",
                    self.topic.to_string(),
                    self.selected.saturating_add({
                        if self.data.is_empty() {
                            0
                        } else {
                            1
                        }
                    }),
                    self.data.len()
                )),
            ]));

        let list_items = self
            .data
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let mut list_item = ListItem::new(item.title.clone());
                if idx == self.selected.try_into().unwrap_or(0) {
                    list_item = list_item.style(Style::default().bg(Color::Blue));
                }

                list_item
            })
            .collect::<Vec<_>>();

        let list_len: usize = list_items.len().try_into().unwrap_or(0);
        if self.selected < self.list_top_cursor {
            self.list_top_cursor = self.list_top_cursor.clone().saturating_sub(1);
        } else if self.selected >= self.list_top_cursor + height.saturating_sub(2)
            && self.selected < list_len
        {
            self.list_top_cursor = self.list_top_cursor.clone().saturating_add(1);
        }
        let top: usize = self.list_top_cursor;
        let bottom: usize = (self.selected + height)
            .try_into()
            .unwrap_or(0)
            .min(list_items.len());
        let list = List::new(list_items[top..bottom].to_vec()).block(left_block);

        f.render_widget(list, rect);
        Ok(())
    }
}
