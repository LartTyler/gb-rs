pub use log::*;

use crate::app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Error, Stdout, Write};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

mod log;

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

pub fn render<B: Backend>(_f: &mut Frame<B>, _app: &App) {
    todo!()
}
