use std::io;

use app::App;
use ratatui::{TerminalOptions, Viewport};

mod app;
mod calendar;
fn main() -> io::Result<()> {
    let mut app = App::new();
    println!("\n");

    let mut terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(9),
    })?;

    app.run(&mut terminal)?;

    ratatui::try_restore()?;

    println!("\n\n\n\n\n");

    Ok(())
}
