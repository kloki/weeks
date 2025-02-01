use std::io;

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Frame, Terminal,
};
pub struct App {
    index: usize,
}

impl App {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> io::Result<()> {
        loop {
            term.draw(|f| self.draw(f))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Right => self.index += 1,
                    KeyCode::Left if self.index > 0 => self.index -= 1,
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        f.render_widget(format!("index {}", self.index), f.area());
    }
}
