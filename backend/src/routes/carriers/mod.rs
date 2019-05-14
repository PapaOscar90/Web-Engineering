//! A module dealing with the "/carriers" routes.

mod get_carrier;
mod get_carriers;
mod views;

use rocket::routes;

/// Mount the carriers routes to the provided and `rocket` and return the resulting `rocket`.
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/carriers",
        routes!(
            get_carriers::get_carriers_json,
            get_carriers::get_carriers_csv,
            get_carriers::get_carriers_hal,
            get_carriers::get_carriers_default,
            get_carrier::get_carrier_json,
            get_carrier::get_carrier_csv,
            get_carrier::get_carrier_hal,
            get_carrier::get_carrier_default,
        ),
    )
}
