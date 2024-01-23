use app::{App, Outcome};
use clap::Parser;
use cli::Cli;
use std::error::Error;

mod app;
mod auto;
mod cli;
mod command;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut app = App::from_file(&cli.cart_file)?;

    let mut terminal = ui::create()?;
    ui::set_up(&mut terminal)?;

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        match app.run() {
            Ok(Outcome::Reset) => {
                app = App::from_file(&cli.cart_file)?;
            }
            Ok(Outcome::Quit) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        };
    }

    ui::tear_down(&mut terminal)?;

    Ok(())
}
