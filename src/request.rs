use crate::train::{RouteSegment, SequenceClass};

pub struct Request {
    requested_seats: Vec<SeatRequirements>, // List of wished seats [SeatRequirements]
    train_id: String,                   // Train number (e.g. "ICE 608')
    route_segment: RouteSegment,        // Route segment (e.g. "Berlin - Hamburg")
    class: SequenceClass,               // Class (e.g. "1st Class")
}


impl Request {
    pub fn example() -> Request {
        let example_request: Request = Request {
            requested_seats: vec![
                SeatRequirements {
                    window: (true, 0.8),
                    limited_view: (false, 0.8),
                    close_to_exit: (true, 0.2),
                    close_to_dining: (true, 0.0),
                },
                SeatRequirements {
                    window: (false, 0.1),
                    limited_view: (false, 0.0),
                    close_to_exit: (true, 0.0),
                    close_to_dining: (true, 0.0),
                }
            ],
            train_id: "ICE 608".to_string(),
            class: SequenceClass::First,
            
            route_segment: RouteSegment::example(),
        };

        example_request
    }
}


pub struct SeatRequirements {
    window: (bool, f64),                // Window seat (required, weight)
    limited_view: (bool, f64),          // Limited view if window seat (required, weight)
    close_to_exit: (bool, f64),         // Close to exit (required, weight)
    close_to_dining: (bool, f64),       // Close to dining car (required, weight)
}