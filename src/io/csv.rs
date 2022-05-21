use anyhow::Result;
use log::warn;
use std::iter::Map;

use crate::model::constraints::Constraints;
use crate::model::person::Person;

use crate::io::reader::Reader;

pub struct CsvReader {}

impl Reader for CsvReader {
    fn read_members(path: &str) -> Result<Vec<Person>> {
        let mut members = vec![];
        let reader = csv::Reader::from_path(path)?;
        let mut iter = reader.into_records();

        while let Some(result) = iter.next() {
            if let Ok(record) = result {
                if let (Some(name), Some(surname), Some(email)) =
                    (record.get(1), record.get(1), record.get(2))
                {
                    if name.trim().is_empty() | surname.trim().is_empty() | email.trim().is_empty()
                    {
                        let line = iter.reader().position().line();
                        warn!("Invalid record at {path}:{line}: {record:?}")
                    } else {
                        members.push(Person {
                            name: String::from(name.trim()),
                            surname: String::from(surname.trim()),
                            email: String::from(email.trim()),
                        })
                    }
                }
            } else {
                let line = iter.reader().position().line();
                warn!("Invalid record at {path}:{line}");
            }
        }

        Ok(members)
    }

    fn read_constraints(members: Vec<Person>) -> Result<Map<Person, Constraints>> {
        drop(members);
        todo!()
    }
}
