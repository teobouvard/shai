use std::time::Duration;

use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Weekday;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
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
