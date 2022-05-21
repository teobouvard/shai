pub mod model;

use model::person::Person;

fn main() {
    let p = Person {
        name: "".to_string(),
        surname: "".to_string(),
    };
    dbg!(p);
}
