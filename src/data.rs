use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{train::{Train, RouteSegment}, passengers::Passengers};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    wifi_data: WiFiData,
    reservation_data: ReservationData,
    komfort_check_in_data: KomfortCheckInData,
    checked_passengers_data: CheckedPassengersData,
}

impl Data {
    pub fn generate(train: &Train, passengers: &Passengers) -> Data {
        // TODO - Generate data
        // let occupation = passengers.occupied_seats();

        // Wi-Fi Data
        let mut wifi_data = WiFiData::new();

        for router in train.routers() {
            wifi_data.router_data.push(RouterData {
                id: router.id(),
                connections: Vec::new(),
            });
        }

        for device_id in passengers.devices() {
            let device_position = passengers.get_device_position(device_id).unwrap();

            // Find closest router
            let mut closest_router = None;
            for router in train.routers() {
                if closest_router.is_none() {
                    closest_router = Some(router);
                } else {
                    let current_distance = router.position().distance_to(device_position);
                    let closest_distance = closest_router.unwrap().position().distance_to(device_position);
                    if current_distance < closest_distance {
                        closest_router = Some(router);
                    }
                }
            }

            // Add connection
            if let Some(router) = closest_router {
                let connection: Connection = Connection::new(
                    device_id,
                    1.0 / router.position().distance_to(device_position),
                );

                for router_data in wifi_data.router_data.iter_mut() {
                    if router_data.id == router.id() {
                        router_data.connections.push(connection.clone());
                    }
                }
            }
        }

        // TODO - Reservation Data

        // TODO - Komfort Check-In Data
        let mut komfort_check_in_data = KomfortCheckInData::new();

        for passenger in passengers.all() {
            if passenger.uses_komfort_check_in() && passenger.seat_id().is_some() {
                let check_in = KomfortCheckIn {
                    seat_id: passenger.seat_id().unwrap(),
                    route_segment: passenger.route_segment().clone(),
                };

                komfort_check_in_data.check_ins.push(check_in);
            }
        }

        // TODO - Checked Passengers Data

        // Return data
        Data {
            wifi_data,
            reservation_data: ReservationData {
                reservations: Vec::new(),
            },
            komfort_check_in_data,
            checked_passengers_data: CheckedPassengersData {
                checked_passengers: Vec::new(),
            },
        }
    }

    pub fn wifi_data(&self) -> &WiFiData {
        &self.wifi_data
    }

    pub fn reservation_data(&self) -> &ReservationData {
        &self.reservation_data
    }

    pub fn komfort_check_in_data(&self) -> &KomfortCheckInData {
        &self.komfort_check_in_data
    }

    pub fn checked_passengers_data(&self) -> &CheckedPassengersData {
        &self.checked_passengers_data
    }

    pub fn router_data(&self, router_id: Uuid) -> &RouterData {
        for router_data in self.wifi_data.router_data.iter() {
            if router_data.id == router_id {
                return router_data;
            }
        }

        panic!("Router with ID {} not found!", router_id);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WiFiData {                   // WiFi Data (Data comes in during the trip)
    router_data: Vec<RouterData>,
}

impl WiFiData {
    fn new() -> WiFiData {
        WiFiData {
            router_data: Vec::new(),
        }
    }

    pub fn router_data(&self) -> &Vec<RouterData> {
        &self.router_data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouterData {
    id: Uuid,                       // Router ID
    // position_in_coach: (f64, f64),  // x, y
    connections: Vec<Connection>,   // Connections to devices
}

impl RouterData {
    pub fn connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    device_id: Uuid,                // Device ID
    strength: f64,                  // Strength of connection (0.0 - 1.0)
}

impl Connection {
    fn new(device_id: Uuid, strength: f64) -> Connection {
        Connection {
            device_id,
            strength,
        }
    }

    pub fn device_id(&self) -> Uuid {
        self.device_id
    }

    pub fn strength(&self) -> f64 {
        self.strength
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ReservationData {            // Reservation Data (Data comes in before the trip)
    reservations: Vec<Reservation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reservation {
    seat_id: Uuid,
    route_segment: RouteSegment,
}

#[derive(Debug, Serialize, Deserialize)]
struct KomfortCheckInData {         // Komfort Check-In (Data comes in during the trip)
    check_ins: Vec<KomfortCheckIn>,
}

impl KomfortCheckInData {
    fn new() -> KomfortCheckInData {
        KomfortCheckInData {
            check_ins: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct KomfortCheckIn {
    seat_id: Uuid,
    route_segment: RouteSegment,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckedPassengersData {      // Checked Passengers (Data comes in during the trip)
    checked_passengers: Vec<CheckedPassenger>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckedPassenger {
    seat_id: Uuid,
    route_segment: RouteSegment,
}