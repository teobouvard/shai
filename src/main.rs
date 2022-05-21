use anyhow::Result;
use io::{csv::CsvReader, reader::Reader};
pub mod io;
pub mod model;

fn main() -> Result<()> {
    env_logger::init();

    let users = CsvReader::read_members("tmp/users.csv")?;
    dbg!(users);
    Ok(())
}
