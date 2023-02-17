use crate::{interact::Command, App};
use gb_rs_asm::containers::Pair;
use gb_rs_cpu::registers::FlagsRegister;
use std::io::{self, Stdout};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

pub fn create() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend)
}

pub fn layout<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints([Constraint::Length(60), Constraint::Min(1)])
        .split(f.size());

    draw_left_column(f, app, chunks[0]);
    draw_right_column(f, app, chunks[1]);
}

fn draw_left_column<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(area);

    draw_registers_display(f, app, chunks[0]);
    draw_input(f, app, chunks[1]);
    draw_messages(f, app, chunks[2]);
}

fn draw_messages<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let container = Block::default()
        .title("Command Output")
        .borders(Borders::ALL);

    let inner_area = container.inner(area.clone());
    f.render_widget(container, area);

    let chunks = Layout::default()
        .horizontal_margin(1)
        .constraints([Constraint::Min(1)])
        .split(inner_area);

    let max_lines: usize = chunks[0].height.into();
    let mut content: Vec<ListItem> = Vec::with_capacity(max_lines);

    for item in app.messages.iter().rev() {
        let items = item.as_list_items();

        if content.len() + items.len() > max_lines {
            break;
        }

        content.extend(items);

        if content.len() >= max_lines {
            break;
        }
    }

    f.render_widget(List::new(content), chunks[0]);
}

fn draw_right_column<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    draw_instruction_log(f, app, area);
}

fn draw_instruction_log<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let container = Block::default()
        .title("Instruction Log")
        .borders(Borders::ALL);

    let inner_area = container.inner(area.clone());
    f.render_widget(container, area);

    let chunks = Layout::default()
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(inner_area);

    let max_lines: usize = chunks[0].height.into();
    let skip = app.instruction_history.len().saturating_sub(max_lines);

    let log_content: Vec<_> = app
        .instruction_history
        .iter()
        .skip(skip)
        .take(max_lines)
        .map(|item| ListItem::new(item.as_ref()))
        .collect();

    let log = List::new(log_content);
    f.render_widget(log, chunks[0]);
}

fn draw_registers_display<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let container = Block::default().title("Registers").borders(Borders::ALL);
    let inner_area = container.inner(area.clone());
    f.render_widget(container, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(16),
        ])
        .horizontal_margin(1)
        .split(inner_area);

    let items = [
        create_register_item("A", app.hardware.cpu.registers.a),
        create_register_item("B", app.hardware.cpu.registers.b),
        create_register_item("D", app.hardware.cpu.registers.d),
        create_register_item("H", app.hardware.cpu.registers.h),
        create_register_item(
            "PC",
            RegisterType::Address(app.hardware.cpu.registers.program_counter),
        ),
    ];

    f.render_widget(List::new(items), chunks[0]);

    let items = [
        create_register_item("Flags", &app.hardware.cpu.registers.flags),
        create_register_item("C", app.hardware.cpu.registers.c),
        create_register_item("E", app.hardware.cpu.registers.e),
        create_register_item("L", app.hardware.cpu.registers.l),
        create_register_item(
            "SP",
            RegisterType::Address(app.hardware.cpu.registers.stack_pointer),
        ),
    ];

    f.render_widget(List::new(items), chunks[1]);

    let items = [
        ListItem::new(" "),
        create_register_item("BC", app.hardware.cpu.registers.get_pair(Pair::BC)),
        create_register_item("DE", app.hardware.cpu.registers.get_pair(Pair::DE)),
        create_register_item("HL", app.hardware.cpu.registers.get_pair(Pair::HL)),
    ];

    f.render_widget(List::new(items), chunks[2]);
}

fn create_register_item<'a, T>(label: &str, value: T) -> ListItem
where
    T: Into<RegisterType<'a>>,
{
    let value = match value.into() {
        RegisterType::Byte(n) => format!("{n:>3}"),
        RegisterType::Word(n) => format!("{n:>5}"),
        RegisterType::Address(n) => format!("${n:04X} ({n:>5})"),
        RegisterType::Flags(flags) => format!("{flags}"),
    };

    let content: Spans = vec![
        Span::raw(format!("{label:>2}: ")),
        Span::styled(value, Style::default().fg(Color::DarkGray)),
    ]
    .into();

    ListItem::new(content)
}

enum RegisterType<'a> {
    Byte(u8),
    Word(u16),
    Address(u16),
    Flags(&'a FlagsRegister),
}

impl From<u8> for RegisterType<'_> {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u16> for RegisterType<'_> {
    fn from(value: u16) -> Self {
        Self::Word(value)
    }
}

impl<'a> From<&'a FlagsRegister> for RegisterType<'a> {
    fn from(value: &'a FlagsRegister) -> Self {
        Self::Flags(value)
    }
}

fn draw_input<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let container = Block::default().title("Command").borders(Borders::ALL);
    let inner_area = container.inner(area);
    f.render_widget(container, area);

    let chunks = Layout::default()
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(inner_area);

    let default_command = app.previous_command.clone().unwrap_or(Command::Next);
    let content = Spans::from(vec![
        Span::styled(
            format!("({default_command}) > "),
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw(&app.input),
    ]);

    let content_width: u16 = content.width().try_into().unwrap_or(u16::MAX);

    let input = Paragraph::new(content);
    f.render_widget(input, chunks[0]);

    f.set_cursor(chunks[0].x + content_width, chunks[0].y);
}
