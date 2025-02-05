use std::io;

use app::App;
use clap::Parser;
use ratatui::{TerminalOptions, Viewport};

mod app;
mod calendar;
///Display a calendar
#[derive(Parser)]
struct Args {
    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut app = App::new();

    let mut terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    })?;

    app.run(&mut terminal, args.interactive)?;

    ratatui::try_restore()?;

    Ok(())
}
