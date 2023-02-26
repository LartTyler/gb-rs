use self::{input::Input, log::Log, registers::Registers};
use crate::{app::App, command::Command};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Error, Stdout, Write};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame, Terminal,
};

mod input;
mod log;
mod registers;

pub fn create() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn set_up<B>(terminal: &mut Terminal<B>) -> Result<(), Error>
where
    B: Backend + Write,
{
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)
}

pub fn tear_down<B>(terminal: &mut Terminal<B>) -> Result<(), Error>
where
    B: Backend + Write,
{
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
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

    draw_registers(f, app, chunks[0]);
    draw_input(f, app, chunks[1]);
    draw_command_log(f, app, chunks[2]);
}

fn draw_registers<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let area = draw_container(f, area, "Registers");

    let registers = Registers::new(&app.hardware.cpu.registers);
    f.render_widget(registers, area);
}

fn draw_input<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let area = draw_container(f, area, "Command");

    let prompt = app.last_command.as_ref().unwrap_or(&Command::Next);
    let input = Input::new(prompt, &app.input);

    input.update_cursor(f, area);
    f.render_widget(input, area);
}

fn draw_command_log<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let area = draw_container(f, area, "Command Log");

    let log = Log::new(app.get_command_log());
    f.render_widget(log, area)
}

fn draw_right_column<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let area = draw_container(f, area, "Instruction Log");

    let log = Log::new(app.get_operation_log()).spacing(0);
    f.render_widget(log, area);
}

fn draw_container<B: Backend>(f: &mut Frame<B>, area: Rect, title: &str) -> Rect {
    let container = Block::default().title(title).borders(Borders::all());
    let inner_area = container.inner(area);
    f.render_widget(container, area);

    Rect {
        x: inner_area.x.saturating_add(1),
        y: inner_area.y,
        width: inner_area.width.saturating_sub(2).max(1),
        height: inner_area.height,
    }
}
