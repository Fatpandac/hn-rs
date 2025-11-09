use crossbeam_channel::Sender;
use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::Layout,
};

use crate::components::{Component, DrawableComponet};
use crate::panels::{Article, ListBlock};
use crate::{AppAction, AppData};

pub struct Environment {
    pub tx_action: Sender<AppAction>,
    pub tx_data: Sender<AppData>,
}

pub struct App {
    article: Article,
    list_block: ListBlock,
    focus: isize,
    tx_action: Sender<AppAction>,
    dirty: bool,
    pub is_running: bool,
}

impl App {
    pub fn new(tx_action: Sender<AppAction>, tx_data: Sender<AppData>) -> Self {
        let env = Environment {
            tx_action: tx_action.clone(),
            tx_data,
        };

        Self {
            dirty: true,
            is_running: true,
            list_block: ListBlock::new(&env, true),
            article: Article::new(&env),
            tx_action,
            focus: 0,
        }
    }

    pub fn should_draw(&mut self) -> bool {
        if self.list_block.is_loading() {
            return true;
        }
        if self.dirty {
            self.dirty = false;
            return true;
        }

        false
    }

    pub fn update_data(&mut self, data: AppData) {
        if let AppData::Story(items) = &data {
            if let Some(items) = items {
                if items.last().is_some() && items.last() == self.list_block.data.last() {
                    return;
                }
                self.list_block.set_data(items.to_vec());
                self.article.set_data(
                    self.list_block
                        .data
                        .get(self.list_block.selected as usize)
                        .cloned(),
                );
            } else {
                self.list_block.reset();
                self.article.set_data(None);
            }
        }
        self.article.update_data(data);
        self.dirty = true;
    }

    pub fn handle_event(&mut self, ev: Event) {
        if let Event::Key(key) = ev {
            self.dirty = true;
            let switch_to_left_block =
                (key.code == KeyCode::Char('h') || key.code == KeyCode::Esc) && self.article.focus;
            let switch_to_right_block = (key.code == KeyCode::Char('l')
                || key.code == KeyCode::Enter)
                && self.list_block.focus;

            if self.focus == 0 {
                self.list_block.event(key);

                self.article.set_data(
                    self.list_block
                        .data
                        .get(self.list_block.selected as usize)
                        .cloned(),
                );
                self.tx_action
                    .send(AppAction::Story(self.list_block.topic))
                    .unwrap();
            } else if self.focus == 1 {
                self.article.event(key);
            }
            if switch_to_left_block {
                self.focus = 0;
                self.list_block.focus = true;
                self.article.focus = false;
            } else if switch_to_right_block {
                if self.list_block.data.is_empty() {
                    return;
                }
                if self.list_block.set_readed().is_ok() {
                    self.focus = 1;
                    self.list_block.focus = false;
                    self.article.focus = true;
                }
            } else if key.code == KeyCode::Char('q') {
                self.is_running = false
            }
        } else if let Event::Resize(..) = ev {
            self.dirty = true
        }
    }

    pub fn draw(&mut self, f: &mut Frame) -> std::io::Result<()> {
        let horizontal = Layout::horizontal({
            if self.list_block.focus {
                vec![
                    ratatui::layout::Constraint::Percentage(80),
                    ratatui::layout::Constraint::Percentage(20),
                ]
            } else {
                vec![
                    ratatui::layout::Constraint::Percentage(20),
                    ratatui::layout::Constraint::Percentage(80),
                ]
            }
        });
        let [left, right] = horizontal.areas(f.area());

        self.list_block.draw(f, left)?;
        self.article.draw(f, right)?;

        Ok(())
    }
}
