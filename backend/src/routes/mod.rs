//! A module dealing with the routes served by the web-server.

mod airports;
mod carriers;
mod statistics;

use rocket::routes;
use rocket::Rocket;

/// Mount the routes to a moved `rocket` and return the resulting `rocket`.
pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount(
            "/airports",
            routes!(
                airports::get_airports_json,
                airports::get_airports_csv,
                airports::get_airports_default,
                airports::get_airport_json,
                airports::get_airport_csv,
                airports::get_airport_default,
            ),
        )
        .mount(
            "/carriers",
            routes!(
                carriers::get_carriers_json,
                carriers::get_carriers_csv,
                carriers::get_carriers_default,
                carriers::get_carriers_operating_at_airport_json,
                carriers::get_carriers_operating_at_airport_csv,
                carriers::get_carriers_operating_at_airport_default,
                carriers::get_carrier_json,
                carriers::get_carrier_csv,
                carriers::get_carrier_default,
            ),
        )
        .mount(
            "/statistics",
            routes!(
                statistics::get_statistics_json,
                statistics::get_statistics_csv,
                statistics::get_statistics_default,
                statistics::get_statistics_flights_json,
                statistics::get_statistics_flights_csv,
                statistics::get_statistics_flights_default,
                statistics::get_statistics_minutes_delayed_json,
                statistics::get_statistics_minutes_delayed_csv,
                statistics::get_statistics_minutes_delayed_default,
            ),
        )
}
