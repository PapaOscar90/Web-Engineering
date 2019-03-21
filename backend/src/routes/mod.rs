//! A module dealing with the routes served by the web-server.
use rocket::Rocket;
use rocket::{get, routes};

/// Mount the routes to a moved `rocket` and return the resulting `rocket`.
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes!())
}
