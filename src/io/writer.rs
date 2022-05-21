use std::path::Path;

pub trait Writer {
    fn write_planning(path: Path);
}
