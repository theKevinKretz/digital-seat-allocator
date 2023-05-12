use serde::{Deserialize, Serialize};
use uuid::Uuid;

use ndarray::Array2;
use image::{ImageBuffer, Rgb};
use plotters::prelude::*; // TODO - Remove
use rand::prelude::*;

use crate::data::Data;
use crate::simulation::Simulation;
use crate::train::{RouteSegment, Seat, Train, Position, self};

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

    pub fn process_std_train(&self) -> Answer {
        // TODO - Get real data from API (cooperation needed)
        // Get exaple data from simulation

        let scale: usize = 10; // TODO - Make this a parameter

        // Run simulation
        let mut simulation = Simulation::example();
        simulation.run();
        simulation.save("simulation.json");

        // Get train
        let train = simulation.train();

        // Get probabilities at start station
        let current_stop = 4; // TODO - Get real current stop

        let data = simulation.data(current_stop);

        // Generate heatmaps
        let mut i = 0;
        let mut wifi_heatmaps = Vec::new();

        for coach in train.coaches() {
            // Generate heatmap
            let mut box_size = coach.dimensions();
            box_size = (box_size.0 * scale, box_size.1 * scale);  // Higher resolution
            let mut heatmap = Array2::<f64>::zeros(box_size);

            // Add Wi-Fi data
            let tolerance = 5.0;  // TODO - Make this a parameter

            for sensor in coach.routers() {
                let sensor_pos = sensor.position().coordinates();
            
                for x in 0..box_size.0 {
                    for y in 0..box_size.1 {
                        let point_distance = ((x as f64 / scale as f64 - sensor_pos.0).powi(2) + (y as f64 / scale as f64 - sensor_pos.1).powi(2)).sqrt(); // Divide by 10 to get real coordinates

                        for connection in data.router_data(sensor.id()).connections() {
                            let device_distance = 1.0 / connection.strength();

                            let difference = (device_distance - point_distance).abs();

                            let value = (tolerance - difference) / tolerance; // Doc: hm_wifi

                            if value > 0.0 {
                                heatmap[(x, y)] += value; // TODO - dont ignore existing values
                            }
                        }
                    }
                }
            }
        
            // Normalize heatmap // TODO - Make unnecessary
            // let max_value: f64 = heatmap.fold(0.0, |acc: f64, &x| x.max(acc));
            // println!("Max value: {}", max_value);
            // heatmap.mapv_inplace(|x| x / max_value);

            Self::plot_heatmap(&heatmap, i).unwrap();
            i += 1;
            wifi_heatmaps.push(heatmap);
        }

        // Process seats
        let mut answer = AnswerTrain {
            seats: Vec::new(),
        };

        for train_seat in train.seats() {
            let mut free_chance = 0.0;

            // TODO - Calculate chance to be free

            // From Wi-Fi data
            let mut wifi_sum = 0.0;

            let heatmap = &wifi_heatmaps[train_seat.coach_number() as usize];
            
            let low_x = (train_seat.base_coordinates().x() * scale as f64) as usize;
            let low_y = (train_seat.base_coordinates().y() * scale as f64) as usize;
            let high_x = ((train_seat.base_coordinates().x() + train_seat.dimensions().0) * scale as f64) as usize;
            let high_y = ((train_seat.base_coordinates().y() + train_seat.dimensions().1) * scale as f64) as usize;

            for x in low_x..high_x {
                for y in low_y..high_y {
                    if low_x < x && x < high_x && low_y < y && y < high_y {
                        wifi_sum += heatmap[(x, y)];
                    }
                }
            }

            free_chance += wifi_sum;


            // Calculate requirements fits for each seat request
            let mut requirements_fit = self.calculate_seat_fits(train_seat);

            answer.seats.push(AnswerSeat {
                id: Uuid::new_v4(),
                position: train_seat.base_coordinates().clone(),
                free_chance,
                requirements_fit,
                score: 0.0,
            });
        }

        let mut best_seats = Vec::new();
        // TODO - Implement algorithm to find best seats (calculate scores)

        // Create answer
        let mut answer_best_seats = Answer::new();

        // answer.seat_probabilities = probabilities;
        answer_best_seats.seats = best_seats;
        answer_best_seats
    }

    fn plot_heatmap(heatmap: &Array2<f64>, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut imgbuf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(heatmap.nrows() as u32, heatmap.ncols() as u32);
    
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let value = (heatmap[(x as usize, y as usize)] * 255.0).round() as u8;
            let color = Rgb([value, value, value]);
            *pixel = color;
        }
    
        let path = format!("heatmap_{}.png", id);

        imgbuf.save(path)?;
    
        Ok(())
    }

    fn calculate_seat_fits(&self, seat: &Seat) -> Vec<f64> {
        let mut scores = Vec::new();

        for seat_request in &self.seats {
            let mut score = 0.0;

            // TODO - Calculate how well the seat fits the requirements
            if seat_request.window.value == seat.is_window() {
                score += seat_request.window.weight;
            }

            if seat_request.close_to_car_end.value {
                score += seat_request.close_to_car_end.weight * seat.distance_to_car_end();
            } else {
                score -= seat_request.close_to_car_end.weight * seat.distance_to_car_end();
            }

            // if requirements.close_to_dining.value = seat.is_close_to_dining() {
            //     score += requirements.close_to_dining.weight;
            // }

            scores.push(score)
        }

        scores
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Journey {
    route_segment: RouteSegment, // Route segment (e.g. "Berlin - Hamburg")
    train_name: String,          // Train number (e.g. "ICE 608')
    seat_class: String,          // Class (e.g. "First")
}

impl Journey {
    fn example() -> Journey {
        Journey {
            route_segment: RouteSegment::example(),
            train_name: "ICE 608".to_string(),
            seat_class: "First".to_string(),
        }
    }

    fn seat_class(&self) -> &str {
        &self.seat_class
    }

    fn train_name(&self) -> &str {
        &self.train_name
    }

    fn route_segment(&self) -> &RouteSegment {
        &self.route_segment
    }

    fn start(&self) -> &str {
        self.route_segment.start()
    }

    fn end(&self) -> &str {
        self.route_segment.end()
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SeatRequirements {
    window: SingleBinaryRequirement,            // Window seat (required, weight)
    close_to_car_end: SingleBinaryRequirement,  // Close to exit (required, weight)
    close_to_dining: SingleBinaryRequirement,   // Close to dining car (required, weight)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SingleBinaryRequirement {
    value: bool,    // Required
    weight: f64,    // Weight
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


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HeatTrain {
    seats: Vec<HeatSeat>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HeatSeat {
    id: Uuid,
    position: Position,
    probability: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HeatMap {
    points: Vec<Vec<f64>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AnswerTrain {
    seats: Vec<AnswerSeat>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AnswerSeat {
    id: Uuid,
    position: Position,
    free_chance: f64,
    requirements_fit: Vec<f64>,
    score: f64,
}