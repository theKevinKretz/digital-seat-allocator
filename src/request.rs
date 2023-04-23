use serde::{Deserialize, Serialize};

use crate::data::Data;
use crate::simulation::Simulation;
use crate::train::{RouteSegment, Seat, SequenceClass, Train};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Request {
    security_ratio: f64,          // Security ratio (e.g. 0.8)
    journey: Journey,             // Journey request
    seats: Vec<SeatRequirements>, // List of wished seats [SeatRequirements]
}

impl Request {
    pub fn example() -> Request {
        let example_request: Request = Request {
            security_ratio: 0.8,
            seats: vec![
                SeatRequirements {
                    window: SingleBinaryRequirement {
                        value: true,
                        weight: 0.7,
                    },
                    close_to_car_end: SingleBinaryRequirement {
                        value: false,
                        weight: 0.3,
                    },
                    close_to_dining: SingleBinaryRequirement {
                        value: true,
                        weight: 0.5,
                    },
                },
                SeatRequirements {
                    window: SingleBinaryRequirement {
                        value: false,
                        weight: 0.2,
                    },
                    close_to_car_end: SingleBinaryRequirement {
                        value: true,
                        weight: 0.8,
                    },
                    close_to_dining: SingleBinaryRequirement {
                        value: false,
                        weight: 0.4,
                    },
                },
            ],
            journey: Journey {
                route_segment: RouteSegment::example(),
                train_name: "ICE 608".to_string(),
                seat_class: "First".to_string(),
            },
        };

        example_request
    }

    pub fn hello(&self) {
        let msg = format!("Dear {} class passenger, I will soon help you find {} seats on train {} from {} to {}.",
            self.journey.seat_class,
            self.seats.len(),
            self.journey.train_name,
            self.journey.route_segment.start,
            self.journey.route_segment.end,
        );
        println!("{}", msg);
    }

    pub fn process_std_train(&self) -> Answer {
        // Get exaple data from simulation
        // TODO - Get real date from API (cooperation needed)
        let train = Train::example();

        let mut simulation = Simulation::example();
        simulation.run();
        simulation.save("simulation.json");
        let current_stop = 2; // TODO - Get real current stop
        let data = simulation.data(current_stop);

        // Create answer
        let mut answer = Answer::new();

        let probabilities = self.calculate_seats_probabilities(&train, &data);
        let seat_fits = self.calculate_requirements_fits(&train.seats());

        let mut best_seats = Vec::new();
        // TODO - Implement algorithm to find best seats

        answer.seat_probabilities = probabilities;
        answer.seats = best_seats;
        answer
    }

    fn calculate_seats_probabilities(&self, train: &Train, data: &Data) -> Vec<f64> {
        let mut seat_probabilities = Vec::new();

        // TODO - Calculate probapilities for each seat in the train to be free.

        seat_probabilities
    }

    fn calculate_requirements_fits(&self, seats: &Vec<&Seat>) -> Vec<Vec<f64>> {
        let mut seat_fits = Vec::new();

        for requirements in self.seats.iter() {
            let mut fits = Vec::new();

            for seat in seats {
                fits.push(Request::calculate_seat_score(seat, requirements));
            }

            seat_fits.push(fits);
        }

        seat_fits
    }

    fn calculate_seat_score(seat: &Seat, requirements: &SeatRequirements) -> f64 {
        let mut score = 0.0;

        // TODO - Calculate how well the seat fits the requirements

        score
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Journey {
    route_segment: RouteSegment, // Route segment (e.g. "Berlin - Hamburg")
    train_name: String,          // Train number (e.g. "ICE 608')
    seat_class: String,          // Class (e.g. "First")
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SeatRequirements {
    window: SingleBinaryRequirement, // Window seat (required, weight)
    close_to_car_end: SingleBinaryRequirement, // Close to exit (required, weight)
    close_to_dining: SingleBinaryRequirement, // Close to dining car (required, weight)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SingleBinaryRequirement {
    value: bool, // Required
    weight: f64, // Weight
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    seat_probabilities: Vec<f64>,
    seats: Vec<(i32, i32)>,
}

impl Answer {
    fn new() -> Answer {
        Answer {
            seat_probabilities: Vec::new(),
            seats: Vec::new(),
        }
    }
}
