use std::io::Result;

use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};

pub trait Component {
    fn event(&mut self, key: KeyEvent);
}

pub trait DrawableComponet {
    fn draw(&mut self, f: &mut Frame, rect: Rect) -> Result<()>;
}
