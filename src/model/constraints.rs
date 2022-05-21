use chrono::NaiveDate;

pub struct Constraints {
    pub vacations: Vec<NaiveDate>,

    pub shifts_include: Vec<String>,
    pub shifts_exclude: Vec<String>,
}
