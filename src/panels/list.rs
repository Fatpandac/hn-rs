use std::io::Result;

use hackernews::{StoryType, get_items::ItemResponse};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, block::Title},
};

use crate::{
    app::Environment,
    components::{Component, DrawableComponet, Loading},
    storages::ReadHistory,
};

pub struct ListBlock {
    pub data: Vec<ItemResponse>,
    pub selected: u16,
    pub topic: StoryType,
    pub focus: bool,
    list_top_cursor: u16,
    loading: Loading,
    height: u16,
    readed_history: ReadHistory,
}

impl ListBlock {
    pub fn new(_env: &Environment, focus: bool) -> Self {
        Self {
            data: vec![],
            topic: StoryType::Show,
            focus,
            selected: 0,
            list_top_cursor: 0,
            height: 0,
            loading: Loading::new(),
            readed_history: ReadHistory::new(100),
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

    pub fn set_readed(&mut self) -> Result<()> {
        let current_item = self.data.get(self.selected as usize).unwrap();
        self.readed_history
            .add_read_item(self.topic, current_item.id)?;
        Ok(())
    }
}

impl DrawableComponet for ListBlock {
    fn draw(&mut self, f: &mut Frame, rect: Rect) -> Result<()> {
        self.height = rect.height.saturating_sub(2);
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
                Span::styled("S-T", Style::default().fg(Color::Red)),
                Span::raw(format!(
                    " {}({}/{}) ",
                    self.topic,
                    self.selected
                        .saturating_add(if self.data.is_empty() { 0 } else { 1 }),
                    self.data.len()
                )),
                Span::styled("T", Style::default().fg(Color::Red)),
                Span::raw(">"),
            ])));

        let list_items = self
            .data
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let is_readed = self.readed_history.id_is_readed(self.topic, item.id);
                let list_item = ListItem::new(item.title.clone().unwrap_or("No title".to_string()));
                let mut style = Style::default().fg(Color::White);

                if idx == self.selected as usize {
                    style = style.bg(Color::Blue);
                } else if is_readed {
                    style = style.fg(Color::DarkGray);
                }

                list_item.style(style)
            })
            .collect::<Vec<_>>();

        let list_len: u16 = list_items.len() as u16;
        if self.selected < self.list_top_cursor {
            self.list_top_cursor = self
                .list_top_cursor
                .saturating_sub(self.list_top_cursor.saturating_sub(self.selected));
        } else if self.selected >= self.list_top_cursor + self.height {
            self.list_top_cursor = self
                .list_top_cursor
                .saturating_add(
                    self.selected
                        .saturating_sub(self.list_top_cursor + self.height.saturating_sub(1)),
                )
                .min(list_len);
        }
        let top: usize = self.list_top_cursor.into();
        let bottom: usize = (self.selected + self.height).min(list_len).into();
        let list = List::new(list_items[top..bottom].to_vec()).block(left_block);

        f.render_widget(list, rect);
        Ok(())
    }
}

impl Component for ListBlock {
    fn event(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('j') {
            self.selected = self
                .selected
                .saturating_add(1)
                .min(self.data.len().saturating_sub(1) as u16);
        } else if key.code == KeyCode::Char('k') {
            self.selected = self.selected.saturating_sub(1);
        } else if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('b') {
            self.selected = self.selected.saturating_sub(self.height);
        } else if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('f') {
            self.selected = self
                .selected
                .saturating_add(self.height)
                .min(self.data.len().saturating_sub(1) as u16);
        } else if key.code == KeyCode::Tab {
            self.next_topic();
        } else if key.code == KeyCode::BackTab {
            self.prev_topic();
        }
    }
}
