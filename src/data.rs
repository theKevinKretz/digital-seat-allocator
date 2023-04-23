use serde::{Deserialize, Serialize};

use crate::{train::*, passenger::{self, Passenger}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    wifi_data: WiFiData,
    reservation_data: ReservationData,
    komfort_check_in_data: KomfortCheckInData,
    checked_passengers_data: CheckedPassengersData,
}

impl Data {
    fn new() -> Data {
        Data {
            wifi_data: WiFiData {
                router_data: Vec::new(),
            },
            reservation_data: ReservationData {
                reservations: Vec::new(),
            },
            komfort_check_in_data: KomfortCheckInData {
                check_ins: Vec::new(),
            },
            checked_passengers_data: CheckedPassengersData {
                checked_passengers: Vec::new(),
            },
        }
    }

    pub fn generate(train: &Train, passengers: &Vec<Passenger>) -> Data {
        // TODO - Generate data
        let occupation = train.occupation(passengers);
        let mut data = Data::new();

        // Wi-Fi Data

        // Reservation Data

        // Komfort Check-In Data

        // Checked Passengers Data

        data
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WiFiData {                   // WiFi Data (Data comes in during the trip)
    router_data: Vec<Router>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Router {
    coach: i32,                     // Coach number
    position_in_coach: (f64, f64),  // x, y
    connections: Vec<f64>,          // Strength of connection to devices (0.0 - 1.0)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReservationData {            // Reservation Data (Data comes in before the trip)
    reservations: Vec<Reservation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reservation {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}

#[derive(Debug, Serialize, Deserialize)]
struct KomfortCheckInData {         // Komfort Check-In (Data comes in during the trip)
    check_ins: Vec<KomfortCheckIn>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KomfortCheckIn {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckedPassengersData {      // Checked Passengers (Data comes in during the trip)
    checked_passengers: Vec<CheckedPassenger>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckedPassenger {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}