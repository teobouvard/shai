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

    pub fn datetime_end(&self, day: &NaiveDate) -> NaiveDateTime {
        let duration = self.time_end - self.time_start;
        self.datetime_start(day) + duration
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
