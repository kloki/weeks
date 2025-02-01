use chrono::{Datelike, NaiveDate, Utc, Weekday};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
pub struct Calendar {
    start: NaiveDate,
}

impl Calendar {
    pub fn new() -> Self {
        let now = Utc::now();
        let first_day_of_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
        let start = first_day_of_month.week(Weekday::Mon).first_day();

        Self { start }
    }

    pub fn increase(&mut self) {
        self.start = self
            .start
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
    }

    pub fn decrease(&mut self) {
        self.start = self
            .start
            .pred_opt()
            .unwrap()
            .pred_opt()
            .unwrap()
            .pred_opt()
            .unwrap()
            .pred_opt()
            .unwrap()
            .pred_opt()
            .unwrap()
            .pred_opt()
            .unwrap()
            .pred_opt()
            .unwrap()
    }

    pub fn frame_data(&self, width: u16) -> Days {
        Days::new(self.start, width)
    }
}

#[derive(Clone)]
pub struct Days {
    days: [Vec<NaiveDate>; 7],
}

impl Days {
    pub fn new(start: NaiveDate, width: u16) -> Self {
        let mut start = start;
        let mut mondays = vec![];
        let mut tuesdays = vec![];
        let mut wednesdays = vec![];
        let mut thursdays = vec![];
        let mut fridays = vec![];
        let mut saturdays = vec![];
        let mut sundays = vec![];

        for _ in 0..width {
            mondays.push(start);
            start = start.succ_opt().unwrap();
            tuesdays.push(start);
            start = start.succ_opt().unwrap();
            wednesdays.push(start);
            start = start.succ_opt().unwrap();
            thursdays.push(start);
            start = start.succ_opt().unwrap();
            fridays.push(start);
            start = start.succ_opt().unwrap();
            saturdays.push(start);
            start = start.succ_opt().unwrap();
            sundays.push(start);
            start = start.succ_opt().unwrap();
        }
        Self {
            days: [
                mondays, tuesdays, wednesdays, thursdays, fridays, saturdays, sundays,
            ],
        }
    }

    pub fn iso_weeks(&self) -> Line {
        let mut week_numbers = " ".to_string();
        for d in self.days[0].iter() {
            week_numbers.push_str(&format!("{:3}", d.iso_week().week()))
        }
        Line::styled(week_numbers, Style::default().fg(Color::Green))
    }

    pub fn monday(&self) -> Line {
        self._build_days('M', 0)
    }

    pub fn tuesday(&self) -> Line {
        self._build_days('T', 1)
    }

    pub fn wednesday(&self) -> Line {
        self._build_days('W', 2)
    }

    pub fn thursday(&self) -> Line {
        self._build_days('T', 3)
    }

    pub fn friday(&self) -> Line {
        self._build_days('F', 4)
    }

    pub fn saturday(&self) -> Line {
        self._build_days('S', 5)
    }

    pub fn sunday(&self) -> Line {
        self._build_days('S', 6)
    }

    pub fn _build_days(&self, letter: char, index: usize) -> Line {
        let mut days = vec![Span::styled(
            letter.to_string(),
            Style::default().fg(Color::Green),
        )];
        let today = Utc::now();
        for d in self.days[index].iter() {
            if d.day() == today.day() && d.month() == today.month() && d.year() == today.year() {
                days.push(Span::styled(
                    format!("{:3}", d.day()),
                    Style::default().fg(Color::White).bg(Color::Red),
                ));
                continue;
            }
            if d.month() % 2 == 0 {
                days.push(Span::styled(
                    format!("{:3}", d.day()),
                    Style::default().fg(Color::White),
                ));
                continue;
            }
            days.push(Span::styled(
                format!("{:3}", d.day()),
                Style::default().fg(Color::Black).bg(Color::Gray),
            ));
        }
        days.into()
    }
}
