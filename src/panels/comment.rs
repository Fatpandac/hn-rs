use std::io::Result;

use ratatui::{style::Stylize, widgets::Paragraph};

use crate::components::Component;

#[derive(Debug)]
pub struct Comment {
    ids: Vec<usize>,
    pub focus: bool,
}

impl Comment {
    pub fn new(ids: Vec<usize>) -> Self {
        Comment { 
            ids,
            focus: false,
        }
    }
}

impl Component for Comment {
    fn draw(&mut self, f: &mut ratatui::Frame, rect: ratatui::prelude::Rect) -> Result<()> {
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
        let content = Paragraph::new(
            self.ids
                .iter()
                .map(|id| format!("Comment ID: {}", id))
                .collect::<Vec<String>>()
                .join("\n"),
        ).block(block);
        f.render_widget(content, rect);
        Ok(())
    }

    fn event(&mut self, _key: crossterm::event::KeyEvent) {}
}
