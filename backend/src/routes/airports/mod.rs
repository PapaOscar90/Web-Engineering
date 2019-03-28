//! A module dealing with the "/airports" routes.

mod get_airport;
mod get_airports;
mod views;

use rocket::routes;

/// Mount the airports routes to the provided and `rocket` and return the resulting `rocket`.
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/airports",
        routes!(
            get_airports::get_airports_json,
            get_airports::get_airports_csv,
            get_airports::get_airports_hal,
            get_airports::get_airports_default,
            get_airport::get_airport_json,
            get_airport::get_airport_csv,
            get_airport::get_airport_hal,
            get_airport::get_airport_default,
        ),
    )
}
