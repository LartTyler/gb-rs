use std::fmt::Display;
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

pub struct Log {
    items: Vec<String>,
}

impl Log {
    pub fn new<T>(items: &[T]) -> Self
    where
        T: Display,
    {
        Self {
            items: items.iter().map(|item| format!("{item}")).collect(),
        }
    }
}

impl Widget for Log {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let take = area.height.into();
        let skip = self.items.len().saturating_sub(take).min(1);

        for (item, index) in self.items.iter().skip(skip).take(take).zip(0..take as u16) {
            buf.set_string(area.top() + index, area.left(), item, Style::default());
        }
    }
}
