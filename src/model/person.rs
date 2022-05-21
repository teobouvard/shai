use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub email: String,
}
