use std::fmt::Debug;
use rand::Rng;


use std::fs::File;
use std::io::Write;


extern crate piston_window;
use piston_window::*;

extern crate image;
use image::{ImageBuffer, Rgba};


fn main() {
    let my_request: Request = Request {
        requested_seats: vec![
            SeatRequirements {
                window: (true, 0.8),
                limited_view: (false, 0.8),
                distance_to_exit: (0.0, 0.2),
                distance_to_dining: (0.0, 0.0),
            },
            SeatRequirements {
                window: (false, 0.1),
                limited_view: (false, 0.0),
                distance_to_exit: (0.0, 0.0),
                distance_to_dining: (0.0, 0.0),
            }
        ],
        train_number: 1,
        class: SequenceClass::First,
        
        route_segment: RouteSegment {
            start_station: "Hamburg".to_string(),
            end_station: "Berlin".to_string()
        },
    };

    let new_train = generate_train(2, 5);

    println!("{:#?}", new_train);

    visualize_train(&new_train);


    // println!("{:#?}", my_train.coaches[0].seat_groups[0].seats[0].number);
}


/// Draw a train with seats.
fn visualize_train(train: &Train) {
    let window_size = [800, 800];
    let window_title = "Train seats";
    let mut window: PistonWindow = WindowSettings::new(window_title, window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);

            for coach in train.coaches.iter() {
                for row in coach.seats.iter() {
                    for seat in row.iter() {
                        let seat_color = match seat.orientation {
                            Orientation::Forward => [0.0, 1.0, 0.0, 1.0],
                            Orientation::Backward => [0.0, 0.0, 1.0, 1.0],
                        };
                        let (seat_width, seat_height) = match &coach.class {
                            SequenceClass::First => (1.0, 1.0),
                            _ => (0.8, 0.8)
                        };
                        let seat_x = train.base_coordinates.0 + coach.base_coordinates.0 + seat.base_coordinates.0;
                        let seat_y = train.base_coordinates.1 + coach.base_coordinates.1 + seat.base_coordinates.1;
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

/// Generate an example train with parameters.
///
/// # Arguments
/// * `coach_count` - Number of coaches in the train.
/// * `coach_size` - Number of rows of seats in each coach.
fn generate_train(coach_count: i32, coach_size: i32) -> Train {
    // Generate train
    let mut train = Train {
        train_type: TrainType::Custom,
        base_coordinates: (0.0, 0.0),
        coaches: Vec::new(),
    };

    let coach_x_coord = 0.0;
    let mut coach_y_coord = 0.0;

    // Coach spacing in Y direction
    let coach_y_spacing = coach_size as f64 * 1.2;

    // Generate coaches
    for coach_number in 1..coach_count + 1 {
        // Random coach class (1 = first, 2 = second)
        let coach_class = match &coach_number {
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

        let mut rows = Vec::new();
        let mut seat_y_coord = 0.0;
        let mut full_seat_no = 1;

        for row_no in 0..=coach_size {
            let mut row = Vec::new();
            let mut seat_x_coord = 0.0;

            let row_backward = match &row_no {
                0 => false,
                n if n == &coach_size => true,
                _ => rand::thread_rng().gen_bool(0.5),
            };

            for seat_no_in_row in 0..row_length {

                let seat = Seat {
                    id: 1000 * coach_number + full_seat_no,
                    number: full_seat_no,
                    base_coordinates: (seat_x_coord, seat_y_coord),
                    orientation: if row_backward { Orientation::Backward } else { Orientation::Forward },
                    window: match seat_no_in_row {
                        0 => true,
                        n if n == row_length -1 => true,
                        _ => false,
                    }
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
            class: coach_class,
            base_coordinates: (coach_x_coord, coach_y_coord),
            seats: rows,
        };

        train.coaches.push(coach);
        coach_y_coord += coach_y_spacing;
    }
    train
}


/// Generate an example request with parameters.
fn seat_requirements_fit(seat: &Seat, seat_request: &SeatRequirements) -> f64 {
    let mut score: f64 = 1.0;
    if seat.window == seat_request.window.0 {
        score *= seat_request.window.1;
    }
    // if seat.limited_view == seat_request.limited_view.0 {
    //     score *= seat_request.limited_view.1;
    // }
    // if seat.distance_to_exit >= seat_request.distance_to_exit.0 {
    //     score *= seat_request.distance_to_exit.1;
    // }
    // if seat.distance_to_dining >= seat_request.distance_to_dining.0 {
    //     score *= seat_request.distance_to_dining.1;
    // }
    score
}


// TODO - Calculate probapilities for each seat in the train to be occupied.

// TODO - Simulate boarding


// Data Structs
// These structs are used to store the data that is received from the different data sources

struct Data {
    wifi_data: WiFiData,
    reservation_data: ReservationData,
    komfort_check_in_data: KomfortCheckInData,
    checked_passengers_data: CheckedPassengersData,
}

struct WiFiData { // WiFi Data (Data comes in during the trip)
    devices: Vec<Device>,
}

struct Device {
    router: Router,
    signal_strength: f64,
}

struct Router {
    coach: i32,
    position: [f64; 2],
}

struct ReservationData { // Reservation Data (Data comes in before the trip)
    reservations: Vec<Reservation>,
}

struct Reservation {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}

struct KomfortCheckInData { // Komfort Check-In (Data comes in during the trip)
    check_ins: Vec<KomfortCheckIn>,
}

struct KomfortCheckIn {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}

struct CheckedPassengersData { // Checked Passengers (Data comes in during the trip)
    checked_passengers: Vec<CheckedPassenger>,
}

struct CheckedPassenger {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}


// Request Structs
// These structs are used to store the requests that are received from the clients

#[derive(Debug)]
struct Request {
    requested_seats: Vec<SeatRequirements>,
    train_number: i32,
    route_segment: RouteSegment,
    class: SequenceClass,
}

#[derive(Debug)]
struct SeatRequirements {
    window: (bool, f64), // (required, penalty)
    limited_view: (bool, f64),
    distance_to_exit: (f64, f64),
    distance_to_dining: (f64, f64),
}


// Train Structs
// These structs are used to store the train properties

#[derive(Debug)]
struct Train {
    train_type: TrainType,
    base_coordinates: (f64, f64),
    coaches: Vec<Coach>,
}

#[derive(Debug)]
struct Coach {
    number: i32,
    class: SequenceClass,
    base_coordinates: (f64, f64),
    seats: Vec<Vec<Seat>>,
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
struct Seat {
    id: i32,
    number: i32,
    base_coordinates: (f64, f64),
    window: bool, // TODO Add more properties
    orientation: Orientation,
    // limited_view: bool,
    // distance_to_exit: f64,
    // distance_to_dining: f64,
}

#[derive(Debug)]
enum SequenceClass {
    First,
    Second,
}

#[derive(Debug)]
enum Orientation {
    Forward,
    Backward,
}

#[derive(Debug)]
enum TrainType {
    ICE1,
    ICE2,
    ICE3,
    ICE4,
    Custom, // For testing purposes
}


// More Structs
// These structs are used to store other properties

#[derive(Debug)]
struct RouteSegment {
    start_station: String,
    end_station: String,
}