use std::fs;
use std::fs::File;

use anyhow::Result;

use crate::io::reader::Reader;
use crate::io::writer::Writer;
use crate::model::config::Config;
use crate::model::dispatch::Dispatch;
use crate::model::person::Person;

pub struct YamlReader;
pub struct YamlWriter;

impl Reader for YamlReader {
    fn read_members(path: &str) -> Result<Vec<Person>> {
        todo!("{path}")
    }

    fn read_config(path: &str) -> Result<Config> {
        let s = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&s)?;
        Ok(config)
    }
}

impl Writer for YamlWriter {
    fn write_members(path: &str, members: &[Person]) -> Result<()> {
        let writer = File::create(path)?;
        serde_yaml::to_writer(writer, members)?;
        Ok(())
    }

    fn write_planning(path: &str, dispatch: &Dispatch) -> Result<()> {
        todo!("{path} {dispatch:?}")
    }
}
