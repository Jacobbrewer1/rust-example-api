use std::io::Cursor;

use rocket::serde::Deserialize;
use rocket::response::{self, Responder, Response};
use rocket::http::ContentType;
use rocket::Request;
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

impl Person {
    // Create a new person.
    pub fn new() -> Person {
        Person {
            id: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            age: 0,
        }
    }
}

impl<'r> Responder<'r> for Person {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}:{}", self.name, self.age)), None)
            .raw_header("X-Person-Name", self.name)
            .raw_header("X-Person-Age", self.age.to_string())
            .header(ContentType::new("application", "x-person"))
            .ok()
    }
}
