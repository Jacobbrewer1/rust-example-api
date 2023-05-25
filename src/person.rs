use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Person {
    // Id is the id of the person.
    pub id: i32,
    // first_name is the first name of the person.
    pub first_name: String,
    // last_name is the last name of the person.
    pub last_name: String,
    // age is the age of the person.
    pub age: u8,
}
