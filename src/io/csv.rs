use std::iter;

use anyhow::Result;
use log::warn;

use crate::io::reader::Reader;
use crate::io::writer::Writer;
use crate::model::config::Config;
use crate::model::dispatch::Dispatch;
use crate::model::person::Person;

pub struct CsvReader;
pub struct CsvWriter;

impl Reader for CsvReader {
    fn read_members(path: &str) -> Result<Vec<Person>> {
        let mut members = vec![];
        let reader = csv::Reader::from_path(path)?;
        let mut iter = reader.into_records();

        while let Some(result) = iter.next() {
            if let Ok(record) = result {
                if let (Some(name), Some(surname), Some(email)) =
                    (record.get(0), record.get(1), record.get(2))
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

    fn read_config(path: &str) -> Result<Config> {
        todo!("{path}")
    }
}

impl Writer for CsvWriter {
    fn write_members(path: &str, members: &[Person]) -> Result<()> {
        todo!("{path} {members:?}")
    }

    fn write_planning(path: &str, dispatch: &Dispatch) -> Result<()> {
        let mut writer = csv::Writer::from_path(path)?;

        let shift_names = dispatch
            .config
            .planning
            .shifts
            .iter()
            .map(|shift| shift.name.as_str());

        let header: Vec<&str> = iter::once("Date").chain(shift_names.clone()).collect();
        writer.write_record(header)?;

        for day in dispatch
            .config
            .planning
            .date_start
            .iter_days()
            .take_while(|day| day < &dispatch.config.planning.date_end)
        {
            let empty_alloc = "".to_string();
            let date_str = day.format("%d/%m/%Y").to_string();
            let mut row = vec![&date_str];
            for shift_name in shift_names.clone() {
                let assigned_members = dispatch
                    .allocations
                    .iter()
                    .filter(|alloc| alloc.shift.name == shift_name && alloc.date == day)
                    .map(|alloc| &alloc.person.expect("Planning should be allocated").name)
                    .next()
                    .unwrap_or(&empty_alloc);
                row.push(assigned_members);
            }
            writer.write_record(row)?;
        }

        Ok(())
    }
}
