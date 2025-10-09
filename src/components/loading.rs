use ratatui::{
    style::{Color, Style},
    text::Span,
};

#[derive(Debug)]
pub struct Loading {
    frames: Vec<String>,
    frame_index: usize,
    is_loading: bool,
}

impl Loading {
    pub fn new() -> Self {
        Self {
            frames: vec![
                "⠋".to_string(),
                "⠙".to_string(),
                "⠹".to_string(),
                "⠸".to_string(),
                "⠼".to_string(),
                "⠴".to_string(),
                "⠦".to_string(),
                "⠧".to_string(),
                "⠇".to_string(),
                "⠏".to_string(),
            ],
            frame_index: 0,
            is_loading: true,
        }
    }

    pub fn next_frame(&mut self) -> String {
        let frame = self.frames[self.frame_index].clone();
        self.frame_index = (self.frame_index + 1) % self.frames.len();
        frame
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn to_span_mut(&mut self) -> Option<Span<'_>> {
        if !self.is_loading {
            return None;
        }

        Some(Span::raw(format!(" {} ", self.next_frame())).style(Style::new().fg(Color::Yellow)))
    }
}
