mod data;
mod passengers;
mod request;
mod simulation;
mod train;

use request::{Answer, Request};
use train::Train;

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

fn main() {
    let example_request = Request::example();
    let answer = example_request.process_std_train();
    println!("{:#?}", answer);
}

// #[post("/example", data = "<request>")]
// fn process_request(request: Json<Request>) -> Json<Answer> {
//     Json(request.into_inner().process_std_train())
// }

// #[post("/example")]
// fn get_train() -> Json<Train> {
//     Json(Train::example())
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/allocator", routes![process_request])
//         .mount("/dev", routes![get_train])
// }
