use rand::Rng;

use crate::train::{Train, RouteSegment};

#[derive(Debug, Clone)]
pub struct Passenger {
    id: i32,                        // Passenger id (e.g. 1, 2, 3, ...)
    route_segment: RouteSegment,    // Start station (e.g. "Hamburg")
    start_position: (f64, f64),     // (x, y - relative to train base coordinates)
    wish_to_seat: bool,             // true: passenger wants to sit, false: passenger wants to stand
    seat: i32,                      // Seat id (e.g. 1001, 1002, 1003, ...) - 0 if standing
}

impl Passenger {
    pub fn new(id: i32, route_segment: RouteSegment, start_position: (f64, f64), wish_to_seat: bool) -> Passenger {
        Passenger {
            id,
            route_segment,
            start_position,
            wish_to_seat,
            seat: 0,
        }
    }

    /// Choose a seat in the train and sit down
    pub fn choose_seat_and_sit(&mut self, train: &Train, passengers: &Vec<Passenger>) {
        // Choose closest coach
        let mut closest_coach_no = 0;
        let mut closest_coach_distance = 999990.0; // TODO - Make egegant
        let mut closest_coach = &train.coaches()[0];

        for coach in train.coaches() {
            let coach_center_y = coach.base_coordinates().1 + (train.coach_dimensions().1 / 2.0);
            let distance = (coach_center_y - self.start_position.1).abs();

            if distance < closest_coach_distance {
                closest_coach_no = coach.number();
                closest_coach_distance = distance;
                closest_coach = coach;
            }
        }

        let coach = closest_coach;

        // Relative position in coach
        let start_position_relative_to_coach = (
            (self.start_position.0 % train.coach_dimensions().0) / train.coach_dimensions().0,
            (self.start_position.1 % train.coach_dimensions().1) / train.coach_dimensions().1,
        );

        // Choose closest door
        let door = match start_position_relative_to_coach.1 {   // 0: back, 1: front
            0.0..=0.5 => 0,
            0.5..=1.0 => 1,
            _ => panic!("Invalid start position relative to coach"),
        };

        // Choose seat group
        let seat_group = match start_position_relative_to_coach.1 {
            _ => 0,
        }; // TODO - Choose optimal seat group ()

        // Choose seat
        let seat = match seat_group {
            _ => 99999,
        }; // TODO - Choose optimal seat ()

        // Mark seat as occupied
        self.sit(seat);
    }

    pub fn sit(&mut self, seat: i32) {
        self.seat = seat;
    }

    // Getters
    pub fn route_segment(&self) -> &RouteSegment {
        &self.route_segment
    }
}