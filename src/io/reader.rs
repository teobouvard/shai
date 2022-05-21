use std::iter::Map;

use crate::model::{constraints::Constraints, person::Person};

pub trait Reader {
    fn read_members() -> Vec<Person>;
    fn read_constraints(members: Vec<Person>) -> Map<Person, Constraints>;
}
