//! A module dealing with the routes served by the web-server.

mod airports;
mod carriers;
mod statistics;

/// Mount the routes to the provided and `rocket` and return the resulting `rocket`.
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    // Stage 01: Mount airports
    let rocket = airports::mount(rocket);
    // Stage 02: Mount carriers
    let rocket = carriers::mount(rocket);
    // Stage 03: Mount statistics
    let rocket = statistics::mount(rocket);

    // Return the rocket
    rocket
}
