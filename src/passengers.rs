use rand::Rng;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::simulation::Parameters;
use crate::train::{RouteSegment, Train, Position};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Passengers {
    passengers: Vec<Passenger>,
}

impl Passengers {
    pub fn generate(parameters: &Parameters) -> Passengers {
        let mut passengers = Vec::new();

        let mut rng = rand::thread_rng();
        for _ in 0..parameters.passengers_count() {
            let route_segment = parameters.train().route().random_segment();
            let distance_from_train = rand::thread_rng().gen_range(0.0..15.0); // Random distance from train
            let y_position = rand::thread_rng().gen_range(0.0..parameters.train().dimensions().1); // Random y position near train
            let start_position = Position::new( -distance_from_train, y_position );
            let wish_to_seat = rng.gen_range(0.0..1.0) < parameters.wish_to_seat_chance();
            let devices_count = (rng.gen_range(0.0..2.0) * parameters.average_devices_per_passenger())
                .round() as u32;
            let uses_komfort_check_in = rng.gen_range(0.0..1.0) < parameters.komfort_check_in_chance();

            passengers.push(Passenger::new(
                route_segment,
                start_position,
                wish_to_seat,
                devices_count,
                uses_komfort_check_in,
            ));
        };

        Passengers { passengers }
    }

    pub fn board_all(&mut self, train: &Train, stop: &String) {
        for i in 0..self.passengers.len() {
            let current_passengers: Passengers = self.clone();
            self.passengers[i].board(train, current_passengers, stop);
        }
    }

    // pub fn get(&self, id: &Uuid) -> Option<&Passenger> {
    //     self.passengers
    //         .iter()
    //         .find(|passenger| passenger.id() == id)
    // }

    pub fn all(&self) -> &Vec<Passenger> {
        &self.passengers
    }

    pub fn occupied_seats(&self) -> Vec<Uuid> {
        self.passengers
            .iter()
            .filter_map(|passenger| {
                if let Some(seat_id) = passenger.seat_id() {
                    if seat_id != Uuid::nil() {
                        return Some(seat_id);
                    }
                }
                None
            })
            .collect()
    }

    // pub fn devices_count(&self) -> u32 {
    //     self.passengers
    //         .iter()
    //         .map(|passenger| passenger.devices())
    //         .sum()
    // }

    pub fn devices(&self) -> Vec<Uuid> {
        self.on_train()
            .iter()
            .flat_map(|passenger| passenger.devices.clone())
            .collect()
    }

    pub fn on_train(&self) -> Vec<&Passenger> {
        self.passengers
            .iter()
            .filter(|passenger| passenger.on_train())
            .collect()
    }

    pub fn get_device_position(&self, device_id: Uuid) -> Option<&Position> {
        for passenger in &self.passengers {
            for device in &passenger.devices {
                if *device == device_id {
                    return Some(&passenger.position);
                }
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Passenger {
    id: Uuid,                       // Passenger UUID (e.g. "4a7c7d56-fa63-4d8b-8b6d-3e3d4b4ea4dd")
    route_segment: RouteSegment,    // Start station (e.g. "Hamburg")
    wish_to_seat: bool,             // true: passenger wants to sit, false: passenger wants to stand
    devices_count: u32,             // Number of devices (e.g. 1)
    seat_id: Option<Uuid>,          // Seat UUID (e.g. "e7a6ef08-9b9a-4e8d-9dce-94c76f0e8e29") - None if standing
    position: Position,             // (x, y - relative to train base coordinates)
    devices: Vec<Uuid>,             // Device UUIDs (e.g. ["e7a6ef08-9b9a-4e8d-9dce-94c76f0e8e29"])
    uses_komfort_check_in: bool,    // true: passenger uses komfort check-in, false: passenger uses normal check-in
    on_train: bool,                 // true: passenger is on train, false: passenger is not on train
}

impl Passenger {
    pub fn new(
        route_segment: RouteSegment,
        start_position: Position,
        wish_to_seat: bool,
        devices_count: u32,
        uses_komfort_check_in: bool,
    ) -> Passenger {
        let mut devices = Vec::new();
        for _ in 0..devices_count {
            devices.push(Uuid::new_v4());
        }

        Passenger {
            id: Uuid::new_v4(),
            route_segment,
            position: start_position,
            wish_to_seat,
            seat_id: None,
            devices_count,
            devices,
            uses_komfort_check_in,
            on_train: false,
        }
    }

    /// Choose a seat in the train and sit down
    pub fn board(&mut self, train: &Train, passengers: Passengers, stop: &String) {
        if self.route_segment().start() == stop {

            // Choose closest coach
            let mut closest_coach_distance = std::f64::MAX;
            let mut closest_coach = &train.coaches()[0];

            for coach in train.coaches() {
                let coach_center_y =
                    coach.base_coordinates().y() + (coach.dimensions().1 as f64 / 2.0);
                let distance = (coach_center_y - self.position.y()).abs();

                if distance < closest_coach_distance {
                    closest_coach_distance = distance;
                    closest_coach = coach;
                }
            }

            let coach = closest_coach;

            let start_y_rel_to_coach_base = self.position.y() - coach.base_coordinates().y();

            // Start position relative to coach
            let start_position_relative_to_coach = Position::new (
                (self.position.x() % coach.dimensions().0 as f64) / coach.dimensions().0 as f64,
                start_y_rel_to_coach_base / coach.dimensions().1 as f64,
            );

            // Choose closest door
            let closest_door = ((start_position_relative_to_coach.y() - start_position_relative_to_coach.y()) / coach.dimensions().1 as f64).abs();
            if !(closest_door == 0.0 || closest_door == 1.0) {
                // 0: back, 1: front
                panic!("Invalid start position relative to coach")
            };

            // Enter train
            self.on_train = true;
            self.position = Position::new(
                coach.dimensions().0 as f64 * 0.5,
                coach.base_coordinates().y() + (coach.dimensions().1 as f64 * closest_door));

            if self.wish_to_seat {
                // Choose seat group
                // Get seat groups
                let seat_groups = coach.seat_groups();

                // Evaluate seat groups
                let mut seat_group_evaluations = Vec::new();

                for seat_group in &seat_groups {
                    // Count occupied seats in seat group
                    let mut occupied_seats = 0;
                    for seat_id in seat_group.seats() {
                        for passenger in passengers.all() {
                            if let Some(passenger_seat_id) = passenger.seat_id() {
                                if *seat_id == passenger_seat_id {
                                    occupied_seats += 1;
                                }
                            }
                        }
                    }

                    let capacity = 1 - occupied_seats;

                    // Calculate relative available seat capacity
                    let relative_available_seat_capacity =
                        1.0 - (occupied_seats as f64 / seat_group.size() as f64);

                    // Calculate distance to start position in coach
                    let distance = seat_group.full_center_coordinates(train).distance_to(self.position());

                    // Calculate evaluation
                    let evaluation = relative_available_seat_capacity * (1.0 - distance);

                    // Add evaluation to list
                    seat_group_evaluations.push((seat_group.id(), capacity, evaluation));
                }

                // Filter seat groups by capacity
                seat_group_evaluations.retain(|seat_group_evaluation| seat_group_evaluation.1 > 0);

                if !seat_group_evaluations.is_empty() {
                    // Sort seat groups by evaluation
                    seat_group_evaluations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                    // Choose seat group
                    let seat_group_id = seat_group_evaluations[0].0;

                    // Get seat group
                    let seat_group = seat_groups
                        .iter()
                        .find(|seat_group| seat_group.id() == seat_group_id)
                        .unwrap();

                    // Filter seats by occupation
                    let mut seats = seat_group.seats().clone();
                    seats.retain(|seat_id| !self.occupied_seat(&passengers, Some(*seat_id)));

                    // Choose seat in seat group
                    let seat_id = seats[rand::thread_rng().gen_range(0..seats.len())]; // Random seat in seat group

                    // Sit down
                    self.position = train.get_seat(&seat_id).unwrap().full_center_coordinates();
                    self.sit(Some(seat_id));
                }
            }
        }

        if self.route_segment().end() == stop {
            self.exit();
        }
    }

    fn occupied_seat(&self, passengers: &Passengers, seat_id: Option<Uuid>) -> bool {
        passengers
            .all()
            .iter()
            .any(|passenger| passenger.seat_id() == seat_id)
    }

    pub fn sit(&mut self, seat_id: Option<Uuid>) {
        self.seat_id = seat_id;
    }

    pub fn exit(&mut self) {
        self.seat_id = None;
        self.on_train = false;
    }

    // Getters
    pub fn route_segment(&self) -> &RouteSegment {
        &self.route_segment
    }

    pub fn seat_id(&self) -> Option<Uuid> {
        self.seat_id
    }

    pub fn devices(&self) -> u32 {
        self.devices_count
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn uses_komfort_check_in(&self) -> bool {
        self.uses_komfort_check_in
    }

    pub fn on_train(&self) -> bool {
        self.on_train
    }
}


// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Device {
//     id: Uuid,                       // Device UUID (e.g. "4a7c7d56-fa63-4d8b-8b6d-3e3d4b4ea4dd")
//     // position: Option<(f64, f64)>,   // (x, y - relative to train base coordinates)
// }

// impl Device {
//     pub fn new() -> Device {
//         Device {
//             id: Uuid::new_v4(),
//         }
//     }
// }