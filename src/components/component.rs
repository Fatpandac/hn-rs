use std::io::Result;

use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use tokio::sync::watch;

use crate::{ChannelAction, ChannelData};

pub trait Component {
    fn draw(&mut self, f: &mut Frame, rect: Rect, data: watch::Receiver<ChannelData>)
    -> Result<()>;

    fn event(&mut self, key: KeyEvent, action: watch::Sender<ChannelAction>);
}
