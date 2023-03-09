use std::{fs};

mod train;
use train::*;

mod request;
use request::*;

// mod allocator;
// use allocator::*;

mod simulation;
use simulation::*;

mod data;
use data::*;

mod passenger;
use passenger::*;


fn main() {
    let example_request = Request::example();
    let route = Route::example();

    let new_train = Train::new(2, 5, route);

    let simulation = simulate_full_journey(&new_train, 20, 0.8);

    println!("{:#?}", simulation);

    // Write simulation to file
    let data = format!("{:#?}", simulation);
    fs::write("simulation.json", data).expect("Unable to write file");


    // new_train.visualize();
}
