use std::fs::File;
use std::iter::Map;

use anyhow::Result;

use crate::io::reader::Reader;
use crate::io::writer::Writer;
use crate::model::constraints::Constraints;
use crate::model::person::Person;
use crate::model::shift::Shift;
pub struct YamlReader;
pub struct YamlWriter;

impl Reader for YamlReader {
    fn read_shifts(path: &str) -> Result<Vec<Shift>> {
        todo!("{path}")
    }
    
    fn read_members(path: &str) -> Result<Vec<Person>> {
        todo!("{path}")
    }

    fn read_constraints(members: &[Person]) -> Result<Map<Person, Constraints>> {
        todo!("{members:?}")
    }
}

impl Writer for YamlWriter {
    fn write_members(path: &str, members: &[Person]) -> Result<()> {
        let writer = File::create(path)?;
        serde_yaml::to_writer(writer, members)?;
        Ok(())
    }

    fn write_planning(path: &str) {
        todo!("{path}")
    }
}
