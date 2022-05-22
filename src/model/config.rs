use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

use crate::model::constraints::Constraints;
use crate::model::person::Person;
use crate::model::planning::Planning;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub planning: Planning,
    pub members: IndexMap<Person, Constraints>,
}
