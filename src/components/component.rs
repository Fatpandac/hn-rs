use std::io::Result;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn draw(&mut self, f: &mut Frame, rect: Rect) -> Result<()>;

    fn event(&mut self, key: KeyEvent);
}

