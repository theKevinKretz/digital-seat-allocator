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

use crate::simulation::Simulation;

// fn main() {
//     // Run simulation
//     let mut simulation: Simulation = Simulation::example();
//     simulation.run();
//     simulation.save("simulation.json");

//     // Get example request
//     let request = Request::example();

//     // Process request
//     let answer = request.process_on_simulation(&simulation);

//     // Print answer seat
//     let answer_seat = simulation.train().get_seat(&answer.seats()[0]);
//     println!("Seat: {:#?}", answer_seat);

//     // Evaluate answer
// }

#[post("/example", data = "<request>")]
fn process_request(request: Json<Request>) -> Json<Answer> {
    let mut simulation: Simulation = Simulation::example();
    simulation.run();
    Json(request.process_on_simulation(&simulation))
}

#[post("/example")]
fn get_train() -> Json<Train> {
    Json(Train::example())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/allocator", routes![process_request])
        .mount("/dev", routes![get_train])
}
