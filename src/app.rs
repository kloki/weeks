use std::io;

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    text::Text,
    Frame, Terminal,
};

use crate::calendar::Calendar;
pub struct App {
    calendar: Calendar,
}

impl App {
    pub fn new() -> Self {
        Self {
            calendar: Calendar::new(),
        }
    }

    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> io::Result<()> {
        loop {
            term.draw(|f| self.draw(f))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Right => self.calendar.increase(),
                    KeyCode::Left => self.calendar.decrease(),
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let days = self.calendar.frame_data(f.area().width);
        let mut text = Text::default();

        text.push_line(days.years());
        text.push_line(days.months());
        text.push_line(days.iso_weeks());
        text.push_line(days.monday());
        text.push_line(days.tuesday());
        text.push_line(days.wednesday());
        text.push_line(days.thursday());
        text.push_line(days.friday());
        text.push_line(days.saturday());
        text.push_line(days.sunday());
        f.render_widget(text, f.area());
    }
}
