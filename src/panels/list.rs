use std::io::Result;

use crossterm::event::KeyCode;
use hackernews::{StoryType, get_items::ItemResponse};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, block::Title},
};
use tokio::sync::watch;

use crate::{
    components::{Component, Loading}, ChannelAction, ChannelData
};

pub struct ListBlock {
    pub data: Vec<ItemResponse>,
    pub selected: usize,
    pub topic: StoryType,
    pub focus: bool,
    list_top_cursor: usize,
    loading: Loading,
}

impl ListBlock {
    pub fn new(data: Vec<ItemResponse>, topic: StoryType, focus: bool) -> Self {
        Self {
            data,
            topic,
            focus,
            selected: 0,
            list_top_cursor: 0,
            loading: Loading::new(),
        }
    }

    fn next_topic(&mut self) {
        self.topic = match self.topic {
            StoryType::Show => StoryType::Best,
            StoryType::Best => StoryType::Jobs,
            StoryType::Jobs => StoryType::Top,
            StoryType::Top => StoryType::New,
            StoryType::New => StoryType::Show,
        };
    }

    fn prev_topic(&mut self) {
        self.topic = match self.topic {
            StoryType::Show => StoryType::New,
            StoryType::New => StoryType::Top,
            StoryType::Top => StoryType::Jobs,
            StoryType::Jobs => StoryType::Best,
            StoryType::Best => StoryType::Show,
        };
    }

    pub fn set_data(&mut self, data: Vec<ItemResponse>) {
        self.data = data;
        self.selected = 0;
        self.list_top_cursor = 0;
        self.loading.set_loading(false);
    }

    pub fn reset(&mut self) {
        self.loading.set_loading(true);
        self.data.clear();
        self.selected = 0;
    }
}

impl Component for ListBlock {
    fn draw(
        &mut self,
        f: &mut Frame,
        rect: Rect,
        _data: watch::Receiver<ChannelData>,
    ) -> Result<()> {
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
            .title(Title::from(Line::from(vec![
                self.loading.to_span_mut().unwrap_or(Span::raw("")),
                Span::raw("<"),
                Span::styled("T", Style::default().fg(Color::Red)),
                Span::raw(format!(
                    " - {}({}/{})>",
                    self.topic,
                    self.selected
                        .saturating_add(if self.data.is_empty() { 0 } else { 1 }),
                    self.data.len()
                )),
            ])));

        let list_items = self
            .data
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let mut list_item =
                    ListItem::new(item.title.clone().unwrap_or("No title".to_string()));
                if idx == self.selected {
                    list_item = list_item.style(Style::default().bg(Color::Blue));
                }

                list_item
            })
            .collect::<Vec<_>>();

        let list_len: usize = list_items.len();
        if self.selected < self.list_top_cursor {
            self.list_top_cursor = self.list_top_cursor.saturating_sub(1);
        } else if self.selected >= self.list_top_cursor + height.saturating_sub(2)
            && self.selected < list_len
        {
            self.list_top_cursor = self.list_top_cursor.saturating_add(1);
        }
        let top: usize = self.list_top_cursor;
        let bottom: usize = (self.selected + height)
            .min(list_items.len());
        let list = List::new(list_items[top..bottom].to_vec()).block(left_block);

        f.render_widget(list, rect);
        Ok(())
    }

    fn event(&mut self, key: crossterm::event::KeyEvent, _action: watch::Sender<ChannelAction>) {
        if key.code == KeyCode::Char('j') {
            self.selected = self
                .selected
                .saturating_add(1)
                .min(self.data.len().saturating_sub(1));
        } else if key.code == KeyCode::Char('k') {
            self.selected = self.selected.saturating_sub(1);
        } else if key.code == KeyCode::Tab {
            self.next_topic();
        } else if key.code == KeyCode::BackTab {
            self.prev_topic();
        }
    }
}
