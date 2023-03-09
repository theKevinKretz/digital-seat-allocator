use rand::Rng;

use crate::passenger::Passenger;
use crate::train::{Train, RouteSegment};

/// Simulate journey.
pub fn simulate_full_journey(train: &Train, passengers_count: i32, wish_to_seat_chance: f64) -> Journey {

    // Generate passengers
    let mut passengers = Vec::new();

    for passenger_id in 0..passengers_count {
        // Choose random start station
        let route_segment = train.route().random_segment();                        // TODO - Make route segment random
        let distance_from_train = rand::thread_rng().gen_range(0.0..15.0);                  // Random distance from train
        let y_position = rand::thread_rng().gen_range(0.0..train.dimensions().1);           // Random y position near train
        let wish_to_seat = rand::thread_rng().gen_bool(wish_to_seat_chance);               // Random wish to seat

        let passenger = Passenger::new(passenger_id, route_segment, (-distance_from_train, y_position), wish_to_seat);

        passengers.push(passenger);
    }

    // Simulate journey
    let mut journey = Journey::new();

    for stop in train.route().stops() {

        let passengers2 = passengers.clone();
        
        // Simulate stop
        for passenger in &mut passengers {

            // Check if passenger is at stop
            if &passenger.route_segment().start_station == stop {

                // Simulate passenger
                passenger.choose_seat_and_sit(train, &passengers2); // TODO - Use real passengers
            }
        };

        // Add stop to journey
        let stop = Stop {
            station: stop.clone(),
            passengers: passengers.clone(),
        };

        journey.stops.push(stop);
    }

    // Return journey
    journey
}

#[derive(Debug)]
pub struct Journey {
    stops: Vec<Stop>,
}

impl Journey {
    fn new() -> Journey {
        Journey {
            stops: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Stop {
    station: String,
    passengers: Vec<Passenger>,
}