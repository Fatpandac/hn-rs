use crossterm::event::{KeyCode, KeyEvent};
use hackernews::{StoryType, get_items::ItemResponse};
use ratatui::{Frame, layout::Layout};
use tokio::sync::watch;

use crate::{list::LeftBlock, article::RightBlock};

pub struct APP {
    right_block: RightBlock,
    left_block: LeftBlock,
    data: Vec<ItemResponse>,
    selected: usize,
    focus: isize,
    current_topic: StoryType,
    tx_topic: watch::Sender<StoryType>,
    rx_item: watch::Receiver<Option<ItemResponse>>,
}

impl APP {
    pub fn new(
        tx_topic: watch::Sender<StoryType>,
        rx_item: watch::Receiver<Option<ItemResponse>>,
    ) -> Self {
        Self {
            right_block: RightBlock::new(None, false),
            left_block: LeftBlock::new(Vec::new(), 0, hackernews::StoryType::Show, 0, true),
            data: Vec::new(),
            selected: 0,
            focus: 0,
            current_topic: StoryType::Show,
            tx_topic,
            rx_item,
        }
    }

    pub fn update_data(&mut self) {
        if let Some(item) = self.rx_item.borrow().clone() {
            if self.data.last() != Some(&item) {
                self.data.push(item);
                self.left_block.set_data(self.data.clone());
            }
        } else {
            self.data.clear();
            self.selected = 0;
            self.left_block.set_data(self.data.clone());
            self.left_block.set_selected(self.selected);
        }
    }

    fn next_topic(&mut self) {
        self.current_topic = match self.current_topic {
            StoryType::Show => StoryType::Best,
            StoryType::Best => StoryType::Jobs,
            StoryType::Jobs => StoryType::Top,
            StoryType::Top => StoryType::New,
            StoryType::New => StoryType::Show,
        };
        self.tx_topic.send(self.current_topic).unwrap();
    }

    fn prev_topic(&mut self) {
        self.current_topic = match self.current_topic {
            StoryType::Show => StoryType::New,
            StoryType::New => StoryType::Top,
            StoryType::Top => StoryType::Jobs,
            StoryType::Jobs => StoryType::Best,
            StoryType::Best => StoryType::Show,
        };
        self.tx_topic.send(self.current_topic).unwrap();
    }

    pub fn handle_event(&mut self, key: KeyEvent) {
        if self.focus == 0 {
            if key.code == KeyCode::Char('j') {
                self.selected = self
                    .selected
                    .saturating_add(1)
                    .min(self.data.len().saturating_sub(1));
                self.left_block.set_selected(self.selected);
                self.right_block.set_data(self.data.get(self.selected).cloned());
            } else if key.code == KeyCode::Char('k') {
                self.selected = self.selected.saturating_sub(1);
                self.left_block.set_selected(self.selected);
                self.right_block.set_data(self.data.get(self.selected).cloned());
            } else if key.code == KeyCode::Tab {
                self.next_topic();
                self.left_block.set_topic(self.current_topic);
            } else if key.code == KeyCode::BackTab {
                self.prev_topic();
                self.left_block.set_topic(self.current_topic);
            }
        }
        if key.code == KeyCode::Char('h') {
            self.focus = 0;
            self.left_block.set_focus(true);
            self.right_block.set_focus(false);
        } else if key.code == KeyCode::Char('l') {
            self.focus = 1;
            self.left_block.set_focus(false);
            self.right_block.set_focus(true);
        }
    }

    pub fn draw(&mut self, f: &mut Frame) -> std::io::Result<()> {
        let horizontal = Layout::horizontal([
            ratatui::layout::Constraint::Percentage(30),
            ratatui::layout::Constraint::Percentage(70),
        ]);
        let [left, right] = horizontal.areas(f.area());

        self.left_block.draw(f, left)?;
        self.right_block.draw(f, right)?;

        Ok(())
    }
}
