pub mod io;
pub mod model;

use anyhow::Result;
use io::yaml::YamlReader;

use crate::io::csv::CsvReader;
use crate::io::reader::Reader;
use crate::io::writer::Writer;
use crate::io::yaml::YamlWriter;

fn main() -> Result<()> {
    env_logger::init();

    let members = CsvReader::read_members("tmp/users.csv")?;
    // dbg!(&members);

    YamlWriter::write_members("tmp/constraints.yaml", &members)?;

    let config = YamlReader::read_config("tmp/config.yaml")?;
    dbg!(&config);

    Ok(())
}
