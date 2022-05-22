use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Duration;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Weekday;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Shift {
    pub name: String,

    pub time_start: NaiveTime,
    pub time_end: NaiveTime,

    pub days_nominal: HashSet<Weekday>,
    pub days_include: HashSet<NaiveDate>,
    pub days_exclude: HashSet<NaiveDate>,

    pub rest_needed: bool,
    pub rest_duration: Duration,
}

impl Shift {
    pub fn occurs_on_date(&self, day: &NaiveDate) -> bool {
        (self.days_nominal.contains(&day.weekday()) && !self.days_exclude.contains(day))
            || self.days_include.contains(day)
    }

    pub fn datetime_start(&self, day: &NaiveDate) -> NaiveDateTime {
        day.and_time(self.time_start)
    }

    pub fn duration(&self) -> chrono::Duration {
        if self.time_end > self.time_start {
            self.time_end - self.time_start
        } else {
            chrono::Duration::days(1) - (self.time_start - self.time_end)
        }
    }

    pub fn datetime_end(&self, day: &NaiveDate) -> NaiveDateTime {
        self.datetime_start(day) + self.duration()
    }
}

impl Hash for Shift {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Shift {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Shift {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_midnight_wrap() {
        let s = Shift {
            name: String::new(),
            time_start: NaiveTime::from_hms(18, 0, 0),
            time_end: NaiveTime::from_hms(8, 0, 0),
            days_nominal: HashSet::new(),
            days_exclude: HashSet::new(),
            days_include: HashSet::new(),
            rest_needed: false,
            rest_duration: Duration::from_secs(0),
        };

        assert_eq!(s.duration(), chrono::Duration::hours(14));
    }
}
