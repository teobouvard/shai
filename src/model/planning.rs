use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use crate::model::shift::Shift;

#[derive(Debug, Serialize, Deserialize)]
pub struct Planning {
    pub name: String,

    pub date_start: NaiveDate,
    pub date_end: NaiveDate,

    pub shifts: Vec<Shift>,
}
