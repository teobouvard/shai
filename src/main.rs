pub mod io;
pub mod model;
pub mod solver;

use anyhow::Result;
use io::yaml::YamlReader;
use solver::backtracking::Solver;

use crate::io::csv::CsvReader;
use crate::io::csv::CsvWriter;
use crate::io::reader::Reader;
use crate::io::writer::Writer;
use crate::io::yaml::YamlWriter;

fn extract_user_data() -> Result<()> {
    let members = CsvReader::read_members("tmp/users.csv")?;
    YamlWriter::write_members("tmp/constraints.yaml", &members)
}

fn allocate_planning() -> Result<()> {
    let config = YamlReader::read_config("tmp/config.yaml")?;
    let result = Solver::solve(&config)?;
    CsvWriter::write_planning("tmp/planning.csv", &result)?;
    dbg!(&result);
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    extract_user_data()?;
    allocate_planning()
}
