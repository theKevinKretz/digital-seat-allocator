use serde::{Deserialize, Serialize};
use uuid::Uuid;

use ndarray::{Array2, Zip};
use image::{ImageBuffer, Rgb};

use crate::data::{Data, Connection};
use crate::simulation::Simulation;
use crate::train::{RouteSegment, Seat, Coach, Train, Position, self};

const SCALE: usize = 10;
const TOLERANCE: f64 = 5.0;
const DEFAULT_STOP_NUMBER: usize = 2;

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
                        weight: 0.9,
                    },
                    close_to_dining: SingleBinaryRequirement {
                        value: true,
                        weight: 0.5,
                    },
                },
                // SeatRequirements {
                //     window: SingleBinaryRequirement {
                //         value: false,
                //         weight: 0.2,
                //     },
                //     close_to_car_end: SingleBinaryRequirement {
                //         value: true,
                //         weight: 0.8,
                //     },
                //     close_to_dining: SingleBinaryRequirement {
                //         value: false,
                //         weight: 0.4,
                //     },
                // },
            ],
            journey: Journey {
                route_segment: RouteSegment::example(),
                train_name: "ICE 608".to_string(),
                seat_class: "First".to_string(),
            },
        };

        example_request
    }

    pub fn process_on_simulation(&self, simulation: &Simulation) -> Answer {
        // TODO - Get real data from API (cooperation needed)
        // Get example data from simulation

        // Get train
        let train = simulation.train();

        // Get probabilities at start station
        let current_stop = DEFAULT_STOP_NUMBER; // TODO - Get real current stop

        let data = simulation.data(current_stop);

        // Generate heat coaches ()

        let mut i = 0;
        let mut heat_coaches = Vec::new();

        for coach in train.coaches() {
            // Generate heatmap
            let box_size = ((coach.dimensions().0 * SCALE as f64) as usize, (coach.dimensions().1 * SCALE as f64) as usize);  // Higher resolution

            // Add Wi-Fi data
            let mut heat_coach = HeatCoach::from_coach(coach);

            let mut heat_coaches_for_connections = Vec::new();

            for sensor in coach.routers() {
                let sensor_pos = sensor.position().coordinates();
                for connection in data.router_data(sensor.id()).connections() {
                    // Calculate heatmap for connection
                    let heatmap_single_connection = Self::calculate_heatmap_single_connection(coach, sensor_pos, connection, TOLERANCE, box_size);


                    // Translate into HeatCoach
                    let mut heat_coach_for_connection = HeatCoach::from_coach(coach);

                    // Add probabilities
                    for seat in heat_coach_for_connection.seats.iter_mut() {
                        seat.probability = Self::calculate_wifi_sum_from_heatmap(&heatmap_single_connection, train.get_seat(&seat.id).unwrap());
                    }

                    heat_coaches_for_connections.push(heat_coach_for_connection);

                    Self::plot_heatmap(&heatmap_single_connection, i).unwrap();
                    i += 1;
                }
            }

            // Calculate total heat coach
            for heat_coach_for_connection in heat_coaches_for_connections {
                for seat in heat_coach_for_connection.seats {
                    for seat2 in heat_coach.seats.iter_mut() {
                        if seat.id == seat2.id {
                            seat2.probability += seat.probability; // TODO - is this correct ??
                        }
                    }
                }
            }
            
            let _ = heat_coach.plot(i);
            heat_coaches.push(heat_coach);
        }


        // Process seats (generate anser)
        let mut answer_train = AnswerTrain {
            id: train.id(),
            coaches: Vec::new(),
        };

        for coach in train.coaches() {
            let mut answer_coach = AnswerCoach {
                id: coach.id(),
                seats: Vec::new(),
            };

            // Get heat coach
            let heat_coach = heat_coaches.iter().find(|hc| hc.id == coach.id()).unwrap();


            for train_seat in coach.seats() {
                let mut free_chance = 1.0;

                // Calculate chance to be free

                // From Wi-Fi data
                // Get wifi heat_seat
                let wifi_heat_seat = heat_coach.seats.iter().find(|hs| hs.id == train_seat.id()).unwrap();

                let wifi_free_prob = wifi_heat_seat.probability;

                free_chance *= wifi_free_prob;


                // Calculate requirements fits for each seat request
                let requirements_fits = self.calculate_seat_fits(train_seat);


                // Calculate scores
                let mut scores = Vec::new();

                for requirements_fit in &requirements_fits {
                    let score = self.security_ratio * free_chance + (1.0 - self.security_ratio) * requirements_fit;
                    scores.push(score);
                }

                let answer_seat = AnswerSeat {
                    id: train_seat.id(),
                    position: *train_seat.base_coordinates(),
                    free_chance,
                    requirements_fits,
                    scores,
                };

                answer_coach.seats.push(answer_seat);
            }
            answer_train.coaches.push(answer_coach);
        }

        // Algorithm to find best seats
        let number_of_seats = self.seats.len();
        let mut best_seats = Vec::new();

        if number_of_seats == 0 {
        } else if number_of_seats == 1 {
            let mut best_seat = None;
            let mut best_score = 0.0;

            for coach in answer_train.coaches {
                for seat in coach.seats {
                    let score = seat.scores[0];
                    if score > best_score {
                        best_seat = Some(seat);
                        best_score = score;
                    }
                }
            }

            if let Some(best_seat) = best_seat {
                best_seats.push(best_seat.id);
            }
        } else if number_of_seats > 1 {
            for seat_group in train.seat_groups() {
                if seat_group.seats().len() >= number_of_seats {
                    // Get answer seats
                    let mut answer_seats: Vec<Uuid> = Vec::new();

                    // TODO
                }
            }
        }

        Answer { seats: best_seats }
    }

    fn plot_heatmap(heatmap: &Array2<f64>, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Normalize heatmap values to the range [0, 1]
        let max_value = heatmap.fold(f64::MIN, |acc, &v| acc.max(v));
        let min_value = heatmap.fold(f64::MAX, |acc, &v| acc.min(v));
        let normalized_heatmap = heatmap.mapv(|v| (v - min_value) / (max_value - min_value));

        let mut imgbuf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(normalized_heatmap.nrows() as u32, normalized_heatmap.ncols() as u32);
    
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let value = (normalized_heatmap[(x as usize, y as usize)] * 255.0).round() as u8;
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

            if requirements.close_to_dining.value = seat.is_close_to_dining() {
                score += requirements.close_to_dining.weight;
            }

            scores.push(score)
        }
        scores
    }

    fn calculate_heatmap_single_connection(coach: &Coach, sensor_pos: (f64, f64), connection: &Connection, tolerance: f64, box_size: (usize, usize)) -> Array2<f64> {
        let device_distance = 1.0 / connection.strength();

        let mut heatmap = Array2::<f64>::zeros(box_size);

        for x in 0..box_size.0 {
            for y in 0..box_size.1 {
                let point_distance = ((x as f64 / SCALE as f64 - sensor_pos.0).powi(2) + (y as f64 / SCALE as f64 - sensor_pos.1).powi(2)).sqrt(); // Divide by 10 to get real coordinates

                let difference = (device_distance - point_distance).abs();

                let value = (tolerance - difference) / tolerance; // Doc: hm_wifi

                if value > 0.0 {
                    heatmap[(x, y)] = value;
                }
            }
        }

        // Normalize heatmap (everything adds up to 1.0; do this before next step, as people can be outside of seat)
        let sum = heatmap.sum();
        if sum > 0.0 {
            heatmap.mapv_inplace(|x| x / sum);
        }

        heatmap
    }

    fn calculate_wifi_sum_from_heatmap(heatmap: &Array2<f64>, seat: &Seat) -> f64 {
        let mut wifi_sum = 0.0;
                
        let low_x = (seat.base_coordinates().x() * SCALE as f64) as usize;
        let low_y = (seat.base_coordinates().y() * SCALE as f64) as usize;
        let high_x = ((seat.base_coordinates().x() + seat.dimensions().0) * SCALE as f64) as usize;
        let high_y = ((seat.base_coordinates().y() + seat.dimensions().1) * SCALE as f64) as usize;

        for x in low_x..high_x {
            for y in low_y..high_y {
                if low_x < x && x < high_x && low_y < y && y < high_y {
                    wifi_sum += heatmap[(x, y)];
                }
            }
        }

        wifi_sum
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
    //seat_probabilities: Vec<f64>,
    seats: Vec<Uuid>,
}

impl Answer {
    fn new() -> Answer {
        Answer {
            //seat_probabilities: Vec::new(),
            seats: Vec::new(),
        }
    }

    pub fn seats(&self) -> &Vec<Uuid> {
        &self.seats
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HeatCoach {
    id: Uuid,
    dimensions: (f64, f64),
    seats: Vec<HeatSeat>
}

impl HeatCoach {
    fn new(id: Uuid, dimensions: (f64, f64)) -> HeatCoach {
        HeatCoach {
            id,
            dimensions,
            seats: Vec::new(),
        }
    }

    fn from_coach(coach: &train::Coach) -> HeatCoach {
        let mut heat_coach = HeatCoach::new(coach.id(), coach.dimensions());

        for seat in coach.seats() {
            heat_coach.seats.push(HeatSeat {
                id: seat.id(),
                area: seat.area(),
                probability: 0.0,
            });
        }

        heat_coach
    }

    fn plot(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let scale = 10;
        let mut imgbuf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new((self.dimensions.0 * scale as f64) as u32, (self.dimensions.1 * scale as f64) as u32);

        for seat in self.seats.iter() {
            // Add rectangle
            let low_x = (seat.area.0.x() * scale as f64) as u32;
            let low_y = (seat.area.0.y() * scale as f64) as u32;
            let high_x = (seat.area.1.x() * scale as f64) as u32;
            let high_y = (seat.area.1.y() * scale as f64) as u32;

            for x in low_x..high_x {
                for y in low_y..high_y {
                    if x >= imgbuf.width() || y >= imgbuf.height() {
                        continue;
                    }
                    let pixel = imgbuf.get_pixel_mut(x, y);
                    *pixel = Rgb([255, 0, 0]); // TODO use (seat.probability * 255.0).abs() as u8
                }
            }
        }
    
        let path = format!("heatmap_coach_{}.png", id);

        imgbuf.save(path)?;
    
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HeatSeat {
    id: Uuid,
    area: (Position, Position),
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
    id: Uuid,
    coaches: Vec<AnswerCoach>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AnswerCoach {
    id: Uuid,
    seats: Vec<AnswerSeat>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AnswerSeat {
    id: Uuid,
    position: Position,
    free_chance: f64,
    requirements_fits: Vec<f64>,
    scores: Vec<f64>,
}
