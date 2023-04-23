use rand::Rng;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::train::{Train, RouteSegment};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Passenger {
    id: Uuid,                      // Passenger UUID (e.g. "4a7c7d56-fa63-4d8b-8b6d-3e3d4b4ea4dd")
    route_segment: RouteSegment,   // Start station (e.g. "Hamburg")
    start_position: (f64, f64),    // (x, y - relative to train base coordinates)
    wish_to_seat: bool,            // true: passenger wants to sit, false: passenger wants to stand
    seat: Option<Uuid>,            // Seat UUID (e.g. "e7a6ef08-9b9a-4e8d-9dce-94c76f0e8e29") - None if standing
}

impl Passenger {
    pub fn new(route_segment: RouteSegment, start_position: (f64, f64), wish_to_seat: bool) -> Passenger {
        Passenger {
            id: Uuid::new_v4(),
            route_segment,
            start_position,
            wish_to_seat,
            seat: None,
        }
    }

    /// Choose a seat in the train and sit down
    pub fn board(&mut self, train: &Train, passengers: &Vec<Passenger>) {
        if self.wish_to_seat {
            // Choose closest coach
            let mut closest_coach_distance = 999990.0; // TODO - Make elegant
            let mut closest_coach = &train.coaches()[0];

            for coach in train.coaches() {
                let coach_center_y = coach.base_coordinates().1 + (train.coach_dimensions().1 / 2.0);
                let distance = (coach_center_y - self.start_position.1).abs();

                if distance < closest_coach_distance {
                    closest_coach_distance = distance;
                    closest_coach = coach;
                }
            }

            let coach = closest_coach;

            // Start position relative to coach
            let start_position_relative_to_coach = (
                (self.start_position.0 % train.coach_dimensions().0) / train.coach_dimensions().0,
                (self.start_position.1 % train.coach_dimensions().1) / train.coach_dimensions().1,
            );

            // Choose closest door
            let closest_door = match start_position_relative_to_coach.1 {   // 0: back, 1: front
                0.0..=0.5 => 0,
                0.5..=1.0 => 1,
                _ => panic!("Invalid start position relative to coach"),
            };

            // Start y position in coach relative to coach
            let rel_start_y = closest_door as f64 * train.coach_dimensions().1;

            // Choose seat group
            // Get seat groups
            let seat_groups = coach.seat_groups();

            // Evaluate seat groups
            let mut seat_group_evaluations = Vec::new();

            for seat_group in &seat_groups {
                // Count occupied seats in seat group
                let mut occupied_seats = 0;
                for seat_id in seat_group.seats() {
                    for passenger in passengers {
                        if let Some(passenger_seat_id) = passenger.seat_id() {
                            if *seat_id == passenger_seat_id {
                                occupied_seats += 1;
                            }
                        }
                    }
                }
            
                let capacity = 1 - occupied_seats;

                // Calculate relative available seat capacity
                let relative_available_seat_capacity = 1.0 - (occupied_seats as f64 / seat_group.size() as f64);

                // Calculate distance to start position in coach
                let distance = (seat_group.center_coordinates(train).1 - rel_start_y).abs(); // TODO - Make elegant (Do not require train)

                // Calculate evaluation
                let evaluation = relative_available_seat_capacity * (1.0 - distance);

                // Add evaluation to list
                seat_group_evaluations.push((seat_group.id(), capacity, evaluation));
            }

            // Filter seat groups by capacity
            seat_group_evaluations.retain(|seat_group_evaluation| seat_group_evaluation.1 > 0);

            if seat_group_evaluations.len() > 0 {

                // Sort seat groups by evaluation
                seat_group_evaluations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                // Choose seat group
                let seat_group_id = seat_group_evaluations[0].0;

                // Get seat group
                let seat_group = seat_groups.iter().find(|seat_group| seat_group.id() == seat_group_id).unwrap();

                // Filter seats by occupation
                let mut seats = seat_group.seats().clone();
                seats.retain(|seat_id| !self.occupied_seat(passengers, Some(*seat_id)));

                // Choose seat in seat group
                let seat_id = seats[rand::thread_rng().gen_range(0..seats.len())]; // Random seat in seat group

                // Sit down
                self.sit(Some(seat_id));
            }
        }
    }

    fn occupied_seat(&self, passengers: &Vec<Passenger>, seat_id: Option<Uuid>) -> bool {
        passengers.iter().any(|passenger| passenger.seat_id() == seat_id)
    }

    pub fn sit(&mut self, seat: Option<Uuid>) {
        self.seat = seat;
    }

    pub fn exit(&mut self) {
        self.seat = None;
    }

    // Getters
    pub fn route_segment(&self) -> &RouteSegment {
        &self.route_segment
    }

    pub fn seat_id(&self) -> Option<Uuid> {
        self.seat
    }
}
