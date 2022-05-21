use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub email: String,
}
