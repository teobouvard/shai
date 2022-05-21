use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::model::constraints::Constraints;
use crate::model::person::Person;
use crate::model::planning::Planning;

#[derive(Serialize, Deserialize)]
pub struct Config {
    planning: Planning,
    members: HashMap<Person, Constraints>,
}
