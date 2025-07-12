use crossterm::event::{KeyCode, KeyEvent};
use hackernews::{StoryType, get_items::ItemResponse};
use ratatui::{Frame, layout::Layout};
use tokio::sync::watch;

use crate::components::Component;
use crate::panels::{Article, ListBlock};

pub struct APP {
    right_block: Article,
    left_block: ListBlock,
    focus: isize,
    tx_topic: watch::Sender<StoryType>,
    rx_item: watch::Receiver<Option<Vec<ItemResponse>>>,
}

impl APP {
    pub fn new(
        tx_topic: watch::Sender<StoryType>,
        rx_item: watch::Receiver<Option<Vec<ItemResponse>>>,
    ) -> Self {
        Self {
            right_block: Article::new(None, false, 0),
            left_block: ListBlock::new(Vec::new(), 0, hackernews::StoryType::Show, true),
            focus: 0,
            tx_topic,
            rx_item,
        }
    }

    pub fn update_data(&mut self) {
        if let Some(item) = self.rx_item.borrow().clone() {
            if item.last().is_some() && item.last() == self.left_block.data.last() {
                return;
            }
            self.left_block.set_data(item);
            self.right_block
                .set_data(self.left_block.data.get(self.left_block.selected).cloned());
        } else {
            self.left_block.reset();
            self.right_block.set_data(None);
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) {
        let switch_to_left_block = (key.code == KeyCode::Char('h') || key.code == KeyCode::Esc) && self.right_block.focus;
        let switch_to_right_block = (key.code == KeyCode::Char('l') || key.code == KeyCode::Enter) && self.left_block.focus;

        if self.focus == 0 {
            self.left_block.event(key);

            self.right_block
                .set_data(self.left_block.data.get(self.left_block.selected).cloned());
            self.tx_topic.send(self.left_block.topic).unwrap();
        } else if self.focus == 1 {
            self.right_block.event(key);
        }
        if switch_to_left_block  {
            self.focus = 0;
            self.left_block.focus = true;
            self.right_block.focus = false;
        } else if switch_to_right_block {
            if self.left_block.data.is_empty() {
                return;
            }
            self.focus = 1;
            self.left_block.focus = false;
            self.right_block.focus = true;
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
