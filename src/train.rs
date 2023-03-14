extern crate piston_window;
use piston_window::*;

use rand::Rng;


// Train Structs
// These structs are used to store the train properties

#[derive(Debug)]
pub struct Train {
//    id: String,                         // Train id (e.g. "ICE 608")
    base_coordinates: (f64, f64),       // (x, y)
    coach_dimensions: (f64, f64),       // Dimensions of one coach (x, y)
    coaches: Vec<Coach>,                // [coach]
    route: Route,                       // Route
}

impl Train {
    /// Generate an example train with parameters.
    ///
    /// # Arguments
    /// * `coach_count` - Number of coaches in the train.
    /// * `coach_size` - Number of rows of seats in each coach.
    pub fn new(coach_count: i32, coach_size: i32, route: Route) -> Train {
        // Generate train
        let mut train = Train {
            base_coordinates: (0.0, 0.0),
            coach_dimensions: (10., 60.), // TODO make this related to coach_size
            coaches: Vec::new(),
            route,
        };

        let coach_x_coord = 0.0;
        let mut coach_y_coord = 0.0;

        // Coach spacing in Y direction
        let coach_y_spacing = coach_size as f64 * 1.2;

        // Generate coaches
        for coach_number in 1..coach_count + 1 {
            // Random coach class (1 = first, 2 = second)
            let coach_class = match &coach_number { // TODO - Make this a parameter
                1..=2 => SequenceClass::First,
                _ => SequenceClass::Second
            };

            // Aisle afer seat (0 - 3)
            let aisle_after_seat = match &coach_class {
                SequenceClass::First => rand::thread_rng().gen_range(0..2),
                _ => 1,
            };

            // Row length (3 - 4)
            let row_length = match &coach_class {
                SequenceClass::First => 3,
                _ => 4,
            };

            // Seat spacing in X direction
            let seat_x_spacing = match &coach_class {
                SequenceClass::First => 1.2,
                _ => 1.0,
            };

            // Seat spacing in Y direction
            let seat_y_spacing = match &coach_class {
                SequenceClass::First => 1.2,
                _ => 1.0,
            };

            // Aisle width
            let aisle_width = match &coach_class {
                SequenceClass::First => 0.8,
                _ => 0.6,
            };

            let max_row_no = coach_size;

            let mut rows = Vec::new();
            let mut seat_y_coord = 0.0;
            let mut full_seat_no = 1;

            for row_no in 0..max_row_no {
                let mut row = Vec::new();
                let mut seat_x_coord = 0.0;

                let row_backward = match row_no {
                    0 => false,
                    n if n == &max_row_no -1 => true,
                    _ => rand::thread_rng().gen_bool(0.5),
                };

                for seat_no_in_row in 0..row_length {

                    let rows_from_exit = (row_no as f64 - max_row_no as f64 / 2.).abs();
                    let rel_distance_to_exit = rows_from_exit as f64 / max_row_no as f64; // Doc 01

                    let seat = Seat {
                        id: 1000 * coach_number + full_seat_no,
                        number: full_seat_no,
                        class: coach_class.clone(),
                        base_coordinates: (seat_x_coord, seat_y_coord),
                        orientation: if row_backward { Orientation::Backward } else { Orientation::Forward },
                        window: match seat_no_in_row {
                            0 => true,
                            n if n == row_length -1 => true,
                            _ => false,
                        },
                        limited_view: false, // TODO - implement
                        distance_to_exit: rel_distance_to_exit,
                        distance_to_dining: 0., // TODO - implement
                    };

                    if seat_no_in_row == aisle_after_seat {
                        seat_x_coord += aisle_width;
                    }

                    row.push(seat);
                    full_seat_no += 1;
                    seat_x_coord += seat_x_spacing;
                }

                rows.push(row);
                seat_y_coord += seat_y_spacing;
            }

            // Generate coach
            let mut coach = Coach {
                number: coach_number,
                base_coordinates: (coach_x_coord, coach_y_coord),
                seats: rows,
            };

            train.coaches.push(coach);
            coach_y_coord += coach_y_spacing;
        }
        train
    }

    /// Draw a train with seats.
    pub fn visualize(&self) {
        let window_size = [800, 800];
        let window_title = "Train seats";
        let mut window: PistonWindow = WindowSettings::new(window_title, window_size)
            .exit_on_esc(true)
            .build()
            .unwrap();

        while let Some(event) = window.next() {
            window.draw_2d(&event, |context, graphics, _| {
                clear([1.0; 4], graphics);

                for coach in self.coaches.iter() {
                    for row in coach.seats.iter() {
                        for seat in row.iter() {
                            let seat_color = match seat.orientation {
                                Orientation::Forward => [0.0, 1.0, 0.0, 1.0],
                                Orientation::Backward => [0.0, 0.0, 1.0, 1.0],
                            };
                            let (seat_width, seat_height) = match &seat.class {
                                SequenceClass::First => (1.0, 1.0),
                                _ => (0.8, 0.8)
                            };
                            let seat_x = self.base_coordinates.0 + coach.base_coordinates.0 + seat.base_coordinates.0;
                            let seat_y = self.base_coordinates.1 + coach.base_coordinates.1 + seat.base_coordinates.1;
                            let seat_rect = [
                                seat_x + 0.1,
                                seat_y + 0.1 ,
                                seat_x + seat_width - 0.1,
                                seat_y + seat_height - 0.1,
                            ];
                            rectangle(seat_color, seat_rect, context.transform, graphics);
                        }
                    }
                }
            });
        }
    }

    pub fn seat_groups(&self, coach_number: i32, starting_point: i32) -> Vec<Vec<Seat>> {
        // Crate a vector of seat groups
        let mut seat_groups = Vec::new();

        // Crate a seat group (Vector of seats)
        let mut seat_group = Vec::new();

        // Find the coach with the given number
        let coach = self.coaches.iter().find(|coach| coach.number == coach_number).unwrap();

        // Create list of done seats
        let mut done_seats = Vec::new();

        for row in coach.seats.iter() {
            for seat in row {
                // Check if seat is already in a group
                if done_seats.contains(&seat.id) {
                    continue;
                }

                // Create new seat group
                let mut new_seat_group = Vec::new();

                // Add seat to group
                new_seat_group.push(seat.clone());

                // Add other seats to group
                for seat_group in seat_groups.iter() {
                    // Check if seats are in the same group
                    // 1. Same row and same side from aisle
                    

                    // 2. Facing row and same side from aisle
                }

                // Mark seats in new group as done
                for seat in new_seat_group.iter() {
                    done_seats.push(seat.id);
                }
            }
            seat_groups.push(seat_group);
            seat_group = Vec::new();
        }
        seat_groups
    }

    pub fn coach_dimensions(&self) -> (f64, f64) {
        self.coach_dimensions
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.coach_dimensions.1, self.coach_dimensions.1 * self.coaches.len() as f64)
    }

    pub fn route(&self) -> &Route {
        &self.route
    }

    pub fn coaches(&self) -> &Vec<Coach> {
        &self.coaches
    }

    pub fn base_coordinates(&self) -> (f64, f64) {
        self.base_coordinates
    }
}

#[derive(Debug)]
pub struct Coach {
    number: i32,                        // Coach number (e.g. 1, 2, 3, ...)
    base_coordinates: (f64, f64),       // (x, y - relative to train base coordinates)
    seats: Vec<Vec<Seat>>,              // [seat_row] = [[seat]]
}

impl Coach {
    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn base_coordinates(&self) -> (f64, f64) {
        self.base_coordinates
    }

    pub fn seats(&self) -> &Vec<Vec<Seat>> {
        &self.seats
    }
}

// #[derive(Debug)]
// struct SeatGroup {
//     // id: i32,
//     // position: [f64; 2],
//     seats: Vec<Seat>,
// }

// #[derive(Debug)]
// struct SeatRow {
//     seats: Vec<Seat>,
// }

#[derive(Debug)]
pub struct Seat {
    id: i32,                            // Seat id (e.g. 1001, 1002, 1003, ...)
    number: i32,                        // Seat number (e.g. 1, 2, 3, ...)
    base_coordinates: (f64, f64),       // (x, y - relative to coach base coordinates)
    window: bool,                       // true: window seat, false: aisle seat
    limited_view: bool,                 // if seat is next to a window
    class: SequenceClass,               // First or Second class
    orientation: Orientation,           // Forward or Backward relative to the train
    distance_to_exit: f64,              // Distance to the nearest exit
    distance_to_dining: f64,            // Distance to the nearest dining car
}

#[derive(Debug, Clone)]
pub enum SequenceClass {
    First,
    Second,
}

#[derive(Debug)]
enum Orientation {
    Forward,
    Backward,
}


// Route
#[derive(Debug)]
pub struct Route {
    stops: Vec<String>,
}

impl Route {
    pub fn new(stops: Vec<String>) -> Route {
        Route {
            stops,
        }
    }

    pub fn example() -> Route {
        Route {
            stops: vec!["Freiburg".to_string(), "Karlsruhe".to_string(), "Mannheim".to_string(), "Berlin".to_string(), "Hamburg".to_string()],
        }
    }

    pub fn random_segment(&self) -> RouteSegment {
        let start_station_no = rand::thread_rng().gen_range(0..self.stops.len()-1);
        let end_station_no = rand::thread_rng().gen_range(start_station_no+1..self.stops.len());

        let start_station = self.stops[start_station_no].clone();
        let end_station = self.stops[end_station_no].clone();
        RouteSegment::new(start_station, end_station)
    }

    pub fn stops(&self) -> &Vec<String> {
        &self.stops
    }
}

#[derive(Debug, Clone)]
pub struct RouteSegment {
    pub start_station: String,
    pub end_station: String,
}

impl RouteSegment {
    pub fn new(start_station: String, end_station: String) -> RouteSegment {
        RouteSegment {
            start_station,
            end_station,
        }
    }

    pub fn example() -> RouteSegment {
        RouteSegment {
            start_station: "Berlin".to_string(),
            end_station: "Hamburg".to_string(),
        }
    }
}


// Seat groups
#[derive(Debug)]
pub struct SeatGroup {
    properties: SeatGroupProperties,
    seats: Vec<Seat>,
}

#[derive(Debug)]
struct SeatGroupProperties {
    id: i32,
    rows: Vec<i32>,
    side_from_aisle: Side,
}


#[derive(Debug)]
enum Side {
    Left,
    Right,
}