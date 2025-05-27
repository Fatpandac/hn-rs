use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::Layout,
    widgets::{Block, BorderType},
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(
            event::read().expect("failed to read event"),
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            })
        ) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let horizontal = Layout::horizontal([
        ratatui::layout::Constraint::Percentage(30),
        ratatui::layout::Constraint::Percentage(70),
    ]);
    let [left, right] = horizontal.areas(frame.area());
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Left Side"),
        left,
    );
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Right Side"),
        right,
    );
}
