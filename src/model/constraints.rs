use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Constraints {
    pub vacations: Vec<NaiveDate>,

    pub shifts_include: Vec<String>,
    pub shifts_exclude: Vec<String>,
}
