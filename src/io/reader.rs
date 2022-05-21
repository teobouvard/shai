use anyhow::Result;

use crate::model::config::Config;
use crate::model::person::Person;

pub trait Reader {
    fn read_members(path: &str) -> Result<Vec<Person>>;
    fn read_config(path: &str) -> Result<Config>;
}
