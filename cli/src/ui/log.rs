use crate::{
    app::{CommandOutput, CommandResult, CpuStepResult},
    command::Command,
};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Widget,
};

pub struct Log<'a, T> {
    items: &'a [T],
    spacing: u16,
}

impl<'a, T> Log<'a, T> {
    pub fn new(items: &'a [T]) -> Self {
        Self { items, spacing: 1 }
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, T> Widget for Log<'a, T>
where
    T: AsLogItem,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut line: u16 = 0;

        for item in self.items.iter().rev() {
            let log_items = item.as_log_items();
            let len = log_items.len().try_into().unwrap_or(u16::MAX);

            if len.saturating_add(line) > area.height {
                break;
            }

            for spans in log_items {
                let width = spans.width().try_into().unwrap_or(u16::MAX);
                buf.set_spans(area.left(), area.top() + line, &spans, width);

                line += 1;
            }

            line += self.spacing;
        }
    }
}

trait AsLogItem {
    fn as_log_items(&self) -> Vec<Spans>;
}

impl AsLogItem for CpuStepResult {
    fn as_log_items(&self) -> Vec<Spans> {
        vec![vec![
            Span::styled(
                format!("${:04X} | ", self.address),
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw(format!("{}", self.operation)),
        ]
        .into()]
    }
}

impl AsLogItem for CommandResult {
    fn as_log_items(&self) -> Vec<Spans> {
        let mut content = self.command.as_log_items();
        content.extend(self.output.as_log_items());

        content
    }
}

impl AsLogItem for Command {
    fn as_log_items(&self) -> Vec<Spans> {
        vec![vec![
            Span::styled(">> ", Style::default().fg(Color::DarkGray)),
            Span::raw(format!("{self}")),
        ]
        .into()]
    }
}

impl AsLogItem for Option<CommandOutput> {
    fn as_log_items(&self) -> Vec<Spans> {
        self.as_ref()
            .map(|item| item.as_log_items())
            .unwrap_or_default()
    }
}

impl AsLogItem for CommandOutput {
    fn as_log_items(&self) -> Vec<Spans> {
        let value_style = Style::default().fg(Color::DarkGray);

        match self {
            Self::Auto { info } => vec![info.as_str().into()],
            Self::CartInfo { title, mbc_kind } => vec![
                Spans::from(vec![Span::raw("Title: "), Span::styled(title, value_style)]),
                Spans::from(vec![
                    Span::raw("MBC: "),
                    Span::styled(format!("{mbc_kind}"), value_style),
                ]),
            ],
            Self::ReadByte { address, value } => vec![format!("${address:04X} = {value}").into()],
            Self::ReadWord { address, value } => vec![format!("${address:04X} = {value}").into()],
            Self::WriteByte { address, value } => {
                vec![format!("Set ${address:04X} = {value}").into()]
            }
            Self::WriteWord { address, value } => {
                vec![format!("Set ${address:04X} = {value}").into()]
            }
        }
    }
}
