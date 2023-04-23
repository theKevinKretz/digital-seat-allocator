use rand::Rng;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::passenger::Passenger;
use crate::train::Train;
use crate::data::Data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    train: Train,
    passengers_count: i32,
    wish_to_seat_chance: f64,
}

impl Parameters {
    pub fn new(train: Train, passengers_count: i32, wish_to_seat_chance: f64) -> Parameters {
        Parameters {
            train,
            passengers_count,
            wish_to_seat_chance,
        }
    }

    pub fn example() -> Parameters {
        Parameters::new(Train::example(), 20, 0.8)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simulation {
    stops: Vec<Stop>,
    parameters: Parameters,
}

impl Simulation {
    pub fn new(parameters: Parameters) -> Simulation {
        Simulation {
            stops: Vec::new(),
            parameters,
        }
    }

    pub fn example() -> Simulation {
        Simulation::new(Parameters::example())
    }

    /// Simulate journey.
    pub fn run(&mut self) {
        // Generate passengers
        let mut passengers = Vec::new();

        for passenger_id in 0..self.parameters.passengers_count {
            // Choose random start station
            let route_segment = self.parameters.train.route().random_segment(); // Make route segment random
            let distance_from_train = rand::thread_rng().gen_range(0.0..15.0); // Random distance from train
            let y_position =
                rand::thread_rng().gen_range(0.0..self.parameters.train.dimensions().1); // Random y position near train
            let wish_to_seat = rand::thread_rng().gen_bool(self.parameters.wish_to_seat_chance); // Random wish to seat

            let passenger = Passenger::new(
                route_segment,
                (-distance_from_train, y_position),
                wish_to_seat,
            );

            passengers.push(passenger);
        }

        // Simulate journey
        // Collect passengers at each stop
        // Collect data at each stop

        for stop in self.parameters.train.route().stops() {
            // Simulate stop

            // Simulate passengers
            for passenger_id in 0..self.parameters.passengers_count {
                let mut passenger_clone = passengers[passenger_id as usize].clone();

                // Check if passenger enters at stop
                if &passenger_clone.route_segment().start == stop {
                    passenger_clone.board(&self.parameters.train, &passengers); // TODO - Use real passengers
                }

                // Check if passenger exits at stop
                if &passenger_clone.route_segment().end == stop {
                    passenger_clone.exit();
                }

                passengers[passenger_id as usize] = passenger_clone;
            }

            // Generate Data
            let data = Data::generate(&self.parameters.train, &passengers);

            // Add stop to journey
            let stop = Stop {
                station: stop.clone(),
                passengers: passengers.clone(),  // Collect passengers at stop
                data                             // Collect data at stop
            };

            self.stops.push(stop);
        }
    }

    pub fn data(&self, at_stop_no: usize) -> &Data {
        &self.stops[at_stop_no].data
    }

    // Write simulation to file
    pub fn save(&self, path: &str) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, data).expect("Unable to write file");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Stop {
    station: String,
    passengers: Vec<Passenger>,
    data: Data,
}
