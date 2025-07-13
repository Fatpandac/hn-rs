use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Layout};
use tokio::sync::watch;

use crate::components::Component;
use crate::panels::{Article, ListBlock};
use crate::{ChannelAction, ChannelData};

pub struct APP {
    right_block: Article,
    left_block: ListBlock,
    focus: isize,
    tx_action: watch::Sender<ChannelAction>,
    rx_data: watch::Receiver<ChannelData>,
}

impl APP {
    pub fn new(
        tx_action: watch::Sender<ChannelAction>,
        rx_data: watch::Receiver<ChannelData>,
    ) -> Self {
        Self {
            right_block: Article::new(None, false),
            left_block: ListBlock::new(Vec::new(), hackernews::StoryType::Show, true),
            focus: 0,
            tx_action,
            rx_data,
        }
    }

    pub fn update_data(&mut self) {
        if let ChannelData::Story(items) = self.rx_data.borrow().clone() {
            if let Some(items) = items {
                if items.last().is_some() && items.last() == self.left_block.data.last() {
                    return;
                }
                self.left_block.set_data(items);
                self.right_block
                    .set_data(self.left_block.data.get(self.left_block.selected).cloned());
            } else {
                self.left_block.reset();
                self.right_block.set_data(None);
            }
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent, action: watch::Sender<ChannelAction>) {
        let switch_to_left_block =
            (key.code == KeyCode::Char('h') || key.code == KeyCode::Esc) && self.right_block.focus;
        let switch_to_right_block =
            (key.code == KeyCode::Char('l') || key.code == KeyCode::Enter) && self.left_block.focus;

        if self.focus == 0 {
            self.left_block.event(key, action);

            self.right_block
                .set_data(self.left_block.data.get(self.left_block.selected).cloned());
            self.tx_action
                .send(ChannelAction::Story(self.left_block.topic))
                .unwrap();
        } else if self.focus == 1 {
            self.right_block.event(key, action);
        }
        if switch_to_left_block {
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

    pub fn draw(
        &mut self,
        f: &mut Frame,
        data: watch::Receiver<ChannelData>,
    ) -> std::io::Result<()> {
        let horizontal = Layout::horizontal({
            let default_layout = vec![
                ratatui::layout::Constraint::Percentage(80),
                ratatui::layout::Constraint::Percentage(20),
            ];

            if self.left_block.focus {
                default_layout
            } else {
                default_layout.iter().cloned().rev().collect()
            }
        });
        let [left, right] = horizontal.areas(f.area());

        self.left_block.draw(f, left, data.clone())?;
        self.right_block.draw(f, right, data.clone())?;

        Ok(())
    }
}
