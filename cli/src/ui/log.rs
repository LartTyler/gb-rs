use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

pub struct Log<'a> {
    items: &'a Vec<String>,
}

impl<'a> Log<'a> {
    pub fn new(items: &'a Vec<String>) -> Self {
        Self { items }
    }
}

impl<'a> Widget for Log<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let take = area.height.into();
        let skip = self.items.len().saturating_sub(take).min(1);

        for (item, index) in self.items.iter().skip(skip).take(take).zip(0..take as u16) {
            buf.set_string(area.top() + index, area.left(), item, Style::default());
        }
    }
}
