use std::iter::Map;

use anyhow::Result;

use crate::model::constraints::Constraints;
use crate::model::person::Person;

pub trait Reader {
    fn read_members(path: &str) -> Result<Vec<Person>>;
    fn read_constraints(members: &[Person]) -> Result<Map<Person, Constraints>>;
}
