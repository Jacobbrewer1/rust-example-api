#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket_contrib::json::Json;

mod person;

#[get("/")]
fn index() -> Json<person::Person> {
    Status::Ok;
    let p = person::Person {
        id: 1,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    return Json(p);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
