mod data;
mod passenger;
mod request;
mod simulation;
mod train;

use request::{Answer, Request};

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

// fn main() {
//     let example_request = Request::example();
//     let answer = example_request.process_std_train();
//     println!("{:#?}", answer);
// }

#[post("/example", data = "<request>")]
fn process_request(request: Json<Request>) -> Json<Answer> {
    Json(request.into_inner().process_std_train())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![process_request])
}
