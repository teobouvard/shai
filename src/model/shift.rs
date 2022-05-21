use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Weekday;

#[derive(Debug)]
pub struct Shift {
    pub name: String,

    pub time_start: NaiveTime,
    pub time_end: NaiveTime,

    pub days_nominal: Vec<Weekday>,
    pub days_include: Vec<NaiveDate>,
    pub days_exclude: Vec<NaiveDate>,

    pub rest_needed: bool,
    pub rest_duration: Duration,
}
