use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub email: String,
}
