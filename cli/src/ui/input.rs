use std::fmt::Display;

use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Widget,
    Frame,
};

pub struct Input<'a>(Spans<'a>);

impl<'a> Input<'a> {
    pub fn new<P, T>(prompt: &'a P, text: &'a T) -> Self
    where
        P: Display,
        T: Display,
    {
        Self(Spans::from(vec![
            Span::styled(
                format!("({}) > ", prompt),
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw(format!("{text}")),
        ]))
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn update_cursor<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let width = self.width().try_into().unwrap_or(u16::MAX);
        frame.set_cursor(area.left() + width, area.top());
    }
}

impl<'a> Widget for Input<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = self.width().try_into().unwrap_or(u16::MAX);
        buf.set_spans(area.left(), area.top(), &self.0, width);
    }
}
