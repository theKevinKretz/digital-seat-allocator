use plotters::style::text_anchor::Pos;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::Rng;

const DEFAULT_COACH_COUNT: i32 = 5;
const DEFAULT_COACH_SIZE: i32 = 10;
const DEFAULT_ROUTERS_PER_COACH: u32 = 2;

// Train Structs
// These structs are used to store the train properties

#[derive(Debug, Serialize, Deserialize)]
pub struct Train {
    id: Uuid,                          // Train id
    base_coordinates: Position,        // (x, y)
    coaches: Vec<Coach>,               // [coach]
    dimensions: (f64, f64),            // (width, length)
    route: Route,                      // Route
}

impl Train {
    /// Generate an example train with parameters.
    ///
    /// # Arguments
    /// * `coach_count` - Number of coaches in the train.
    /// * `coach_size` - Number of rows of seats in each coach.
    pub fn new(coach_count: i32, coach_size: i32, route: Route, routers_per_coach: u32) -> Train {
        // Generate train
        let mut train = Train {
            id: Uuid::new_v4(),
            base_coordinates: Position { x: 0.0, y: 0.0 },
            coaches: Vec::new(),
            dimensions: (0.0, 0.0),
            route,
        };

        let mut coach_base_coordinates = Position { x: 0.0, y: 0.0 };

        // Generate coaches
        for coach_number in 0..coach_count {
            // Random coach class (1 = first, 2 = second)
            let coach_class = match &coach_number {
                // TODO - Make this a parameter
                1..=2 => SequenceClass::First,
                _ => SequenceClass::Second,
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

            // Seat size (spacing)
            let seat_size = match &coach_class {
                SequenceClass::First => (1.2, 1.2),
                _ => (1.0, 1.0),
            };

            // Aisle width
            let aisle_width = match &coach_class {
                SequenceClass::First => 0.8,
                _ => 0.6,
            };

            let max_row_no = coach_size;

            let mut rows = Vec::new();
            let mut seat_base_coordinates = Position { x: 0.0, y: 0.0 };

            let mut full_seat_no = 1;

            for row_no in 0..max_row_no {
                let mut row = Row::new();

                seat_base_coordinates.x = 0.0;

                let row_forward = match row_no {
                    0 => true,
                    n if n == &max_row_no - 1 => false,
                    _ => rand::thread_rng().gen_bool(0.5),
                };

                let orientation = match row_forward {
                    true => Orientation::Forward,
                    false => Orientation::Backward,
                };

                let mut left_row_segment = RowSegment::new(row_no, Side::Left, &orientation);
                let mut right_row_segment = RowSegment::new(row_no, Side::Right, &orientation);

                for seat_no_in_row in 0..row_length {
                    let rows_from_exit = ((row_no - max_row_no) as f64 / 2.).abs();
                    let distance_to_exit = rows_from_exit / max_row_no as f64; // Doc 01
                    let seat_type = match seat_no_in_row {
                        0 => SeatType::Window,
                        n if n == row_length - 1 => SeatType::Window,
                        _ => SeatType::Aisle,
                    };

                    let seat = Seat {
                        id: Uuid::new_v4(),
                        number: full_seat_no,
                        coach_number,
                        sequence_class: coach_class.clone(),
                        base_coordinates: seat_base_coordinates,
                        full_base_coordinates: Position {
                            x: seat_base_coordinates.x() + coach_base_coordinates.x(),
                            y: seat_base_coordinates.y() + coach_base_coordinates.y(),
                        },
                        dimensions: seat_size,
                        orientation: orientation.clone(),
                        seat_type,
                        limited_view: false, // TODO - implement
                        distance_to_exit,
                        distance_to_dining: 0., // TODO - implement
                    };

                    if seat_no_in_row == aisle_after_seat {
                        seat_base_coordinates.x += aisle_width;
                    }

                    if seat_no_in_row <= aisle_after_seat {
                        left_row_segment.seats.push(seat);
                    } else {
                        right_row_segment.seats.push(seat);
                    }

                    full_seat_no += 1;
                    seat_base_coordinates.x += seat_size.0;
                }

                row.segments.push(left_row_segment);
                row.segments.push(right_row_segment);

                rows.push(row);
                seat_base_coordinates.y += seat_size.1;
            }

            let dimensions = (
                (seat_base_coordinates.x + seat_size.0),
                (seat_base_coordinates.y + seat_size.1),
            );

            // Add routers
            let mut routers = Vec::new();
            if routers_per_coach > 0 {
                for i in 0..routers_per_coach {
                    let x_position = dimensions.0 / 2.0;
                    let y_position = (i as f64 + 0.5) * dimensions.1 / routers_per_coach as f64;
                    let coordinates = Position { x: x_position, y: y_position };

                    let full_coordinates = Position {
                        x: train.base_coordinates().x() + coordinates.x(),
                        y: train.base_coordinates().y() + coordinates.y(),
                    };
                    routers.push(Router::new(coordinates, full_coordinates));
                }
            }

            // Generate coach
            let coach = Coach {
                id: Uuid::new_v4(),
                number: coach_number,
                base_coordinates_in_train: coach_base_coordinates,
                rows,
                dimensions,
                routers,
            };

            train.coaches.push(coach);
            coach_base_coordinates.y += dimensions.1;
        }

        let train_dimensions = (
            coach_base_coordinates.x + train.coaches[train.coaches.len() - 1].dimensions.0,
            coach_base_coordinates.y + train.coaches[train.coaches.len() - 1].dimensions.1,
        );

        train.dimensions = train_dimensions;

        train
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn example() -> Train {
        Train::new(DEFAULT_COACH_COUNT, DEFAULT_COACH_SIZE, Route::example(), DEFAULT_ROUTERS_PER_COACH)
    }

    pub fn route(&self) -> &Route {
        &self.route
    }

    pub fn coaches(&self) -> &Vec<Coach> {
        &self.coaches
    }

    pub fn coaches_count(&self) -> usize {
        self.coaches.len()
    }

    pub fn base_coordinates(&self) -> &Position {
        &self.base_coordinates
    }

    pub fn dimensions(&self) -> (f64, f64) {
        self.dimensions
    }

    pub fn seats(&self) -> Vec<&Seat> {
        let mut seats = Vec::new();
        for coach in self.coaches.iter() {
            for row in coach.rows.iter() {
                for row_segment in row.segments.iter() {
                    for seat in row_segment.seats.iter() {
                        seats.push(seat);
                    }
                }
            }
        }
        seats
    }

    pub fn seats_count(&self) -> usize {
        let mut number_of_seats = 0;
        for coach in self.coaches.iter() {
            for row in coach.rows.iter() {
                for row_segment in row.segments.iter() {
                    number_of_seats += row_segment.seats.len();
                }
            }
        }
        number_of_seats
    }

    pub fn get_seat(&self, seat_id: &Uuid) -> Option<&Seat> {
        for coach in self.coaches.iter() {
            for row in coach.rows.iter() {
                for row_segment in row.segments.iter() {
                    for seat in row_segment.seats.iter() {
                        if seat.id() == *seat_id {
                            return Some(seat);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn routers(&self) -> Vec<&Router> {
        let mut routers = Vec::new();
        for coach in self.coaches.iter() {
            for router in coach.routers.iter() {
                routers.push(router);
            }
        }
        routers
    }

    pub fn seat_groups(&self) -> Vec<SeatGroup> {
        let mut all_groups = Vec::new();
        for coach in self.coaches() {
            all_groups.extend(coach.seat_groups());
        }
        all_groups
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coach {
    id: Uuid,                     // Unique identifier
    number: i32,                  // Coach number (e.g. 0, 1, 2, 3, ...)
    base_coordinates_in_train: Position, // (x, y - relative to train base coordinates)
    rows: Vec<Row>,               // List of seat rows
    dimensions: (f64, f64),       // (width, height)
    routers: Vec<Router>,         // List of routers
}

impl Coach {
    // fn number(&self) -> i32 {
    //     self.number
    // }

    pub fn base_coordinates(&self) -> &Position {
        &self.base_coordinates_in_train
    }

    pub fn seat_groups(&self) -> Vec<SeatGroup> {
        // Create seat groups
        let mut seat_groups = Vec::new();
        let mut added_row_segments = Vec::new();

        for row in &self.rows {
            // Iterating from Back to Front
            for row_segment in &row.segments {
                // Check if the row segment has already been added to a seat group
                if added_row_segments.contains(&row_segment.id) {
                    continue;
                }

                // Create a new seat group
                let mut seat_group = SeatGroup::new();

                // Add seats from the row segment to the seat group
                for seat in &row_segment.seats {
                    seat_group.seats.push(seat.id);
                }
                added_row_segments.push(row_segment.id);

                // Add other seats to the seat group
                for row2 in &self.rows {
                    for row_segment2 in &row2.segments {
                        if row_segment2.id == row_segment.id {
                            continue;
                        }

                        if (row_segment2.side == row_segment.side)
                            && (
                                // 1. Same side from aisle and 2. Different orientation
                                row_segment2.orientation != row_segment.orientation
                            )
                            && ((row_segment.orientation == Orientation::Forward
                                && row_segment2.row_no == row_segment.row_no + 1)
                                || (row_segment.orientation == Orientation::Backward
                                    && row_segment2.row_no == row_segment.row_no - 1))
                        {
                            for seat in &row_segment2.seats {
                                seat_group.seats.push(seat.id);
                            }
                            added_row_segments.push(row_segment2.id);
                        }
                    }
                }
                seat_groups.push(seat_group);
            }
        }
        seat_groups
    }

    pub fn dimensions(&self) -> (f64, f64) {
        self.dimensions
    }

    fn center_coordinates(&self) -> Position {
        Position {
            x: self.base_coordinates_in_train.x() + self.dimensions.0 as f64 / 2.0,
            y: self.base_coordinates_in_train.y() + self.dimensions.1 as f64 / 2.0,
        }
    }

    fn seats_count(&self) -> usize {
        let mut seats_count = 0;
        for row in self.rows.iter() {
            for row_segment in row.segments.iter() {
                seats_count += row_segment.seats.len();
            }
        }
        seats_count
    }

    pub fn routers(&self) -> &Vec<Router> {
        &self.routers
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn seats(&self) -> Vec<&Seat> {
        let mut seats = Vec::new();
        for row in self.rows.iter() {
            for row_segment in row.segments.iter() {
                for seat in row_segment.seats.iter() {
                    seats.push(seat);
                }
            }
        }
        seats
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    id: Uuid,                  // Row id (e.g. 1, 2, 3, ...)
    segments: Vec<RowSegment>, // List of row segments
}

impl Row {
    fn new() -> Row {
        Row {
            id: Uuid::new_v4(),
            segments: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RowSegment {
    id: Uuid,                 // Row segment id
    row_no: i32,              // Row number (e.g. 1, 2, 3, ...)
    side: Side,               // Left or Right from aisle
    orientation: Orientation, // Forward or Backward
    seats: Vec<Seat>,         // List of seats
}

impl RowSegment {
    fn new(row_no: i32, side: Side, orientation: &Orientation) -> RowSegment {
        RowSegment {
            id: Uuid::new_v4(),
            row_no,
            side,
            orientation: orientation.clone(),
            seats: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Seat {
    id: Uuid,                     // Seat id
    number: i32,                  // Seat number (e.g. 1, 2, 3, ...)
    coach_number: i32,            // Coach number (e.g. 0, 1, 2, 3, ...)
    base_coordinates: Position, // (x, y - relative to coach base coordinates)
    full_base_coordinates: Position, // (x, y - relative to train base coordinates)
    dimensions: (f64, f64),       // (x, y)
    seat_type: SeatType,          // Window or Aisle
    limited_view: bool,           // if seat is next to a window
    sequence_class: SequenceClass, // First or Second class
    orientation: Orientation,     // Forward or Backward relative to the train
    distance_to_exit: f64,        // Distance to the nearest exit
    distance_to_dining: f64,      // Distance to the nearest dining car
}

impl Seat {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn base_coordinates(&self) -> &Position {
        &self.base_coordinates
    }

    pub fn center_coordinates(&self) -> Position {
        Position {
            x: self.base_coordinates.x() + self.dimensions.0 / 2.0,
            y: self.base_coordinates.y() + self.dimensions.1 / 2.0,
        }
    }

    pub fn full_base_coordinates(&self) -> &Position {
        &self.full_base_coordinates
    }

    pub fn full_center_coordinates(&self) -> Position {
        Position {
            x: self.full_base_coordinates.x() + self.dimensions.0 / 2.0,
            y: self.full_base_coordinates.y() + self.dimensions.1 / 2.0,
        }
    }

    pub fn area(&self) -> (Position, Position) {
        (
            Position {
                x: self.base_coordinates().x(),
                y: self.base_coordinates().y(),
            },
            Position {
                x: self.base_coordinates().x() + self.dimensions.0,
                y: self.base_coordinates().y() + self.dimensions.1,
            },
        )
    }

    pub fn dimensions(&self) -> (f64, f64) {
        self.dimensions
    }

    pub fn coach_number(&self) -> i32 {
        self.coach_number
    }

    pub fn is_window(&self) -> bool {
        self.seat_type == SeatType::Window
    }

    pub fn distance_to_car_end(&self) -> f64 {
        self.distance_to_exit
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SequenceClass {
    First,
    Second,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Orientation {
    Forward,
    Backward,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum SeatType {
    Window,
    Aisle,
}

// Route
#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    stops: Vec<String>,
}

impl Route {
    fn new(stops: Vec<String>) -> Route {
        Route { stops }
    }

    fn example() -> Route {
        Route::new(vec![
                "Freiburg".to_string(),
                "Karlsruhe".to_string(),
                "Mannheim".to_string(),
                "Berlin".to_string(),
                "Hamburg".to_string(),
            ],
        )
    }

    pub fn random_segment(&self) -> RouteSegment {
        let start_station_no = rand::thread_rng().gen_range(0..self.stops.len() - 1);
        let end_station_no = rand::thread_rng().gen_range(start_station_no + 1..self.stops.len());

        let start_station = self.stops[start_station_no].clone();
        let end_station = self.stops[end_station_no].clone();
        RouteSegment::new(start_station, end_station)
    }

    pub fn stops(&self) -> &Vec<String> {
        &self.stops
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteSegment {
    start: String,
    end: String,
}

impl RouteSegment {
    pub fn new(start: String, end: String) -> RouteSegment {
        RouteSegment { start, end }
    }

    pub fn example() -> RouteSegment {
        RouteSegment::new("Berlin".to_string(), "Hamburg".to_string())
    }

    pub fn start(&self) -> &String {
        &self.start
    }

    pub fn end(&self) -> &String {
        &self.end
    }
}

// Seat groups
#[derive(Debug, Serialize, Deserialize)]
pub struct SeatGroup {
    id: Uuid,
    seats: Vec<Uuid>,
}

impl SeatGroup {
    fn new() -> SeatGroup {
        SeatGroup {
            id: Uuid::new_v4(),
            seats: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn seats(&self) -> &Vec<Uuid> {
        &self.seats
    }

    pub fn size(&self) -> usize {
        self.seats.len()
    }

    pub fn full_center_coordinates(&self, train: &Train) -> Position {
        let mut x_sum: f64 = 0.0;
        let mut y_sum: f64 = 0.0;
        for seat_id in &self.seats {
            let seat_option = train.get_seat(seat_id);
            if let Some(seat) = seat_option {
                let seat_coordinates = seat.full_center_coordinates();
                x_sum += seat_coordinates.x();
                y_sum += seat_coordinates.y();
            } else {
                panic!("Seat not found");
            }
        }
        Position {
            x: x_sum / self.seats.len() as f64,
            y: y_sum / self.seats.len() as f64
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Router {
    id: Uuid,
    coordinates: Position,
    full_coordinates: Position,
}

impl Router {
    fn new(coordinates: Position, full_coordinates: Position) -> Router {
        Router {
            id: Uuid::new_v4(),
            coordinates,
            full_coordinates,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn position(&self) -> &Position {
        &self.full_coordinates
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }

    pub fn coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}