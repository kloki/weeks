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

        let mut cal = Self { start };
        cal.prev_week();
        cal.prev_week();
        cal
    }

    pub fn next_week(&mut self) {
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

    pub fn prev_week(&mut self) {
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

        for _ in 0..(width / 3) {
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
    pub fn years(&self) -> Line {
        let mut years = vec![' '; (self.days[0].len() + 2) * 3];

        let y: Vec<char> = self.days[6][0].year().to_string().chars().collect();
        years[2] = y[0];
        years[3] = y[1];
        years[4] = y[2];
        years[5] = y[3];
        for i in 1..(self.days[6].len() - 3) {
            if self.days[6][i - 1].year() != self.days[6][i].year() {
                let y: Vec<char> = self.days[6][i].year().to_string().chars().collect();
                years[i * 3 + 2] = y[0];
                years[i * 3 + 3] = y[1];
                years[i * 3 + 4] = y[2];
                years[i * 3 + 5] = y[3];
            }
        }

        if years[6] != ' ' {
            years[2] = ' ';
            years[3] = ' ';
            years[4] = ' ';
        }

        Line::styled(
            years.into_iter().collect::<String>(),
            Style::default().fg(Color::Blue),
        )
    }
    pub fn months(&self) -> Line {
        let mut months = vec![' '; (self.days[0].len() + 2) * 3];

        let month_name = named_months(self.days[6][0].month0());
        months[2] = month_name[0];
        months[3] = month_name[1];
        months[4] = month_name[2];
        for i in 1..(self.days[6].len() - 2) {
            if self.days[6][i - 1].month() != self.days[6][i].month() {
                let month_name = named_months(self.days[6][i].month0());
                months[i * 3 + 2] = month_name[0];
                months[i * 3 + 3] = month_name[1];
                months[i * 3 + 4] = month_name[2];
            }
        }

        if months[5] != ' ' {
            months[2] = ' ';
            months[3] = ' ';
            months[4] = ' ';
        }

        Line::styled(
            months.into_iter().collect::<String>(),
            Style::default().fg(Color::Magenta),
        )
    }

    pub fn iso_weeks(&self) -> Line {
        let mut week_numbers = "  ".to_string();
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
            format!(" {}", letter),
            Style::default().fg(Color::Green),
        )];
        let today = Utc::now();
        for d in self.days[index].iter() {
            if d.day() == today.day() && d.month() == today.month() && d.year() == today.year() {
                days.push(Span::styled(
                    format!("{:3}", d.day()),
                    Style::default().fg(Color::White).bg(Color::Magenta),
                ));
                continue;
            }
            if d.month() % 2 == 0 {
                days.push(Span::styled(
                    format!("{:3}", d.day()),
                    Style::default().fg(Color::White).bg(Color::DarkGray),
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
pub fn named_months(index: u32) -> [char; 3] {
    match index {
        0 => ['J', 'a', 'n'],
        1 => ['F', 'e', 'b'],
        2 => ['M', 'a', 'r'],
        3 => ['A', 'p', 'r'],
        4 => ['M', 'a', 'y'],
        5 => ['J', 'u', 'n'],
        6 => ['J', 'u', 'l'],
        7 => ['A', 'u', 'g'],
        8 => ['S', 'e', 'p'],
        9 => ['O', 'c', 't'],
        10 => ['N', 'o', 'v'],
        11 => ['D', 'e', 'c'],
        _ => ['W', 'F', 'T'],
    }
}
