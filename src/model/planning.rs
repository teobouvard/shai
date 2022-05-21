use chrono::NaiveDateTime;

use crate::model::shift::Shift;

#[derive(Debug)]
pub struct Planning {
    pub name: String,

    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,

    pub shifts: Vec<Shift>,
}
