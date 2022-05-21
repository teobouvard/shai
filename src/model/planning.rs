use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

use crate::model::shift::Shift;

#[derive(Debug, Serialize, Deserialize)]
pub struct Planning {
    pub name: String,

    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,

    pub shifts: Vec<Shift>,
}
