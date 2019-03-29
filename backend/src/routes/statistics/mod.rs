//! A module dealing with the "/statistics" routes.

mod delete_statistic;
mod get_statistic;
mod get_statistics;
mod get_statistics_connection;
mod get_statistics_flights;
mod get_statistics_minutes_delayed;
mod patch_statistic;
mod post_statistics;
mod put_statistic;
mod views;

use rocket::routes;

/// Mount the statistics routes to the provided and `rocket` and return the resulting `rocket`.
pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount(
        "/statistics",
        routes!(
            get_statistics::get_statistics_json,
            get_statistics::get_statistics_csv,
            get_statistics::get_statistics_hal,
            get_statistics::get_statistics_default,
            post_statistics::post_statistic_json,
            // post_statistics::post_statistic_csv,
            post_statistics::post_statistic_hal,
            post_statistics::post_statistic_default,
            get_statistic::get_statistics_json,
            get_statistic::get_statistics_csv,
            get_statistic::get_statistics_hal,
            get_statistic::get_statistics_default,
            delete_statistic::delete_statistics_json,
            delete_statistic::delete_statistics_csv,
            delete_statistic::delete_statistics_hal,
            delete_statistic::delete_statistics_default,
            // get_statistics_connection::get_statistics_connection_json,
            // get_statistics_connection::get_statistics_connection_csv,
            // get_statistics_connection::get_statistics_connection_hal,
            // get_statistics_connection::get_statistics_connection_default,
            get_statistics_flights::get_statistics_flights_json,
            get_statistics_flights::get_statistics_flights_csv,
            get_statistics_flights::get_statistics_flights_hal,
            get_statistics_flights::get_statistics_flights_default,
            get_statistics_minutes_delayed::get_statistics_minutes_delayed_json,
            get_statistics_minutes_delayed::get_statistics_minutes_delayed_csv,
            get_statistics_minutes_delayed::get_statistics_minutes_delayed_hal,
            get_statistics_minutes_delayed::get_statistics_minutes_delayed_default,
            // patch_statistic::patch_statistics_json,
            // patch_statistic::patch_statistics_csv,
            // patch_statistic::patch_statistics_hal,
            // patch_statistic::patch_statistics_default,

            // put_statistic::put_statistics_json,
            // put_statistic::put_statistics_csv,
            // put_statistic::put_statistics_hal,
            // put_statistic::put_statistics_default,
        ),
    )
}
