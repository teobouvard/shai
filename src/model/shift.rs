use chrono::{Duration, NaiveTime, Weekday};

#[derive(Debug)]
pub struct Shift {
    pub name: String,

    pub time_start: NaiveTime,
    pub time_end: NaiveTime,

    pub days_nominal: Vec<Weekday>,

    pub rest_needed: bool,
    pub rest_duration: Duration,
}
