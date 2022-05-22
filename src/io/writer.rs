use anyhow::Result;

use crate::model::dispatch::Dispatch;
use crate::model::person::Person;

pub trait Writer {
    fn write_members(path: &str, members: &[Person]) -> Result<()>;
    fn write_planning(path: &str, dispatch: &Dispatch) -> Result<()>;
}
