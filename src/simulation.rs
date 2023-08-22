use std::fs;

use serde::{Deserialize, Serialize};

use crate::passengers::Passengers;
use crate::train::Train;
use crate::data::Data;

const DEFAULT_PASSENGERS_COUNT: i32 = 1000;
const DEFAULT_WISH_TO_SEAT_CHANCE: f64 = 0.8;
const DEFAULT_AVERAGE_DEVICES_PER_PASSENGER: f64 = 0.7;
const DEFAULT_KOMFORT_CHECK_IN_CHANCE: f64 = 0.3;


#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    train: Train,
    passengers_count: i32,
    wish_to_seat_chance: f64,
    average_devices_per_passenger: f64,
    komfort_check_in_chance: f64,
}

impl Parameters {
    pub fn new(train: Train, passengers_count: i32, wish_to_seat_chance: f64, average_devices_per_passenger: f64, komfort_check_in_chance: f64) -> Parameters {
        Parameters {
            train,
            passengers_count,
            wish_to_seat_chance,
            average_devices_per_passenger,
            komfort_check_in_chance,
        }
    }

    pub fn example() -> Parameters {
        Parameters::new(
            Train::example(),
            DEFAULT_PASSENGERS_COUNT,
            DEFAULT_WISH_TO_SEAT_CHANCE,
            DEFAULT_AVERAGE_DEVICES_PER_PASSENGER,
            DEFAULT_KOMFORT_CHECK_IN_CHANCE,
        )
    }

    pub fn train(&self) -> &Train {
        &self.train
    }

    pub fn passengers_count(&self) -> i32 {
        self.passengers_count
    }

    pub fn wish_to_seat_chance(&self) -> f64 {
        self.wish_to_seat_chance
    }

    pub fn average_devices_per_passenger(&self) -> f64 {
        self.average_devices_per_passenger
    }

    pub fn komfort_check_in_chance(&self) -> f64 {
        self.komfort_check_in_chance
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
        let mut passengers = Passengers::generate(&self.parameters);

        // Simulate journey
        for station in self.parameters.train.route().stops() {
            // Simulate stop

            // Simulate passengers
            passengers.board_all(&self.parameters.train, station);

            // Generate Data
            let data = Data::generate(&self.parameters.train, &passengers);

            // Add stop to journey
            let stop = Stop {
                station: station.clone(),
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

    pub fn train(&self) -> &Train {
        &self.parameters.train
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Stop {
    station: String,
    passengers: Passengers,
    data: Data,
}
