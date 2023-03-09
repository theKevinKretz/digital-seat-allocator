use crate::train::*;

use crate::request::*;

// TODO - Calculate probapilities for each seat in the train to be occupied.

fn seat_requirements_fit(seat: Seat, seat_request: &SeatRequirements) -> f64 {
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