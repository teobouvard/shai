use crate::model::{constraints::Constraints, person::Person};
use anyhow::Result;
use std::iter::Map;

pub trait Reader {
    fn read_members(path: &str) -> Result<Vec<Person>>;
    fn read_constraints(members: Vec<Person>) -> Result<Map<Person, Constraints>>;
}
