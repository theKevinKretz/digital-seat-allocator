use crate::train::{RouteSegment};


struct Data {
    wifi_data: WiFiData,
    reservation_data: ReservationData,
    komfort_check_in_data: KomfortCheckInData,
    checked_passengers_data: CheckedPassengersData,
}

struct WiFiData {                   // WiFi Data (Data comes in during the trip)
    router_data: Vec<Router>,
}

struct Router {
    coach: i32,                     // Coach number
    position_in_coach: (f64, f64),  // x, y
    connections: Vec<f64>,          // Strength of connection to devices (0.0 - 1.0)
}

struct ReservationData {            // Reservation Data (Data comes in before the trip)
    reservations: Vec<Reservation>,
}

struct Reservation {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}

struct KomfortCheckInData {         // Komfort Check-In (Data comes in during the trip)
    check_ins: Vec<KomfortCheckIn>,
}

struct KomfortCheckIn {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}

struct CheckedPassengersData {      // Checked Passengers (Data comes in during the trip)
    checked_passengers: Vec<CheckedPassenger>,
}

struct CheckedPassenger {
    coach: i32,
    seat: i32,
    route_segment: RouteSegment,
}