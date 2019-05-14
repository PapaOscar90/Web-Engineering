//! A module defining the `delete_statistic` routes.

use super::views::NewStatistics;
use super::views::Statistics;

use crate::CorgisDbConn;
use diesel::{prelude::*, result::Error};
use rocket::post;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn post_statistic_data(
    conn: &diesel::PgConnection,
    statistics: NewStatistics,
) -> Result<Statistics, Error> {
    let statistics = crate::database::models::NewStatistics {
        flights_cancelled: &statistics.flights_cancelled,
        flights_delayed: &statistics.flights_delayed,
        flights_diverted: &statistics.flights_diverted,
        flights_on_time: &statistics.flights_on_time,
        minutes_delayed_carrier: &statistics.minutes_delayed_carrier,
        minutes_delayed_late_aircraft: &statistics.minutes_delayed_late_aircraft,
        minutes_delayed_national_aviation_system: &statistics
            .minutes_delayed_national_aviation_system,
        minutes_delayed_security: &statistics.minutes_delayed_security,
        minutes_delayed_weather: &statistics.minutes_delayed_weather,
        number_of_delays_carrier: &statistics.number_of_delays_carrier,
        number_of_delays_late_aircraft: &statistics.number_of_delays_late_aircraft,
        number_of_delays_national_aviation_system: &statistics
            .number_of_delays_national_aviation_system,
        number_of_delays_security: &statistics.number_of_delays_security,
        number_of_delays_weather: &statistics.number_of_delays_weather,
        time: &statistics.time,
        carrier_id: &statistics.carrier_id,
        airport_id: &statistics.airport_id,
    };
    crate::database::create_statistics(conn, statistics).map(Statistics::from)
}

/// Get the JSON representation of the posted set of statistics in the
/// database.
#[post("/", format = "application/json", data = "<statistics>", rank = 1)]
pub fn post_statistic_json(
    conn: CorgisDbConn,
    statistics: Json<NewStatistics>,
) -> Result<Json<Statistics>, Error> {
    let statistics = statistics.into_inner();
    post_statistic_data(&conn, statistics).map(Json)
}

// TODO FromDataSimple has to be implemented for CSV. To that end the csv
// portion of the `rocket_contrib_local` crate must be fleshed out and made more
// robust.
// /// Get the CSV representation of the posted set of statistics in the database.
// #[post("/", format = "text/csv", data = "<statistics>", rank = 2)]
// pub fn post_statistic_csv(
//     conn: CorgisDbConn,
//     statistics: Csv<NewStatistics>,
// ) -> Result<Csv<Statistics>, Error> {
//     fn convertor(statistics: &Statistics) -> String {
//         let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
//         wtr.serialize(statistics).unwrap();
//         String::from_utf8(wtr.into_inner().unwrap()).unwrap()
//     };

//     post_statistic_data(&conn, statistics).map(|data| Csv(data, convertor))
// }

/// Get the HAL representation of the posted set of statistics in the database.
#[post("/", format = "application/hal+json", data = "<statistics>", rank = 3)]
pub fn post_statistic_hal(
    conn: CorgisDbConn,
    statistics: Json<NewStatistics>,
) -> Result<Json<HalResource>, Error> {
    let statistics = statistics.into_inner();
    let data = post_statistic_data(&conn, statistics)?;
    let result = HalResource::new(&data)
        .with_link("self", format!("/statistics/{}", data.id))
        .with_link("delete", format!("/statistics/{}", data.id))
        .with_link("patch", format!("/statistics/{}", data.id))
        .with_link("put", format!("/statistics/{}", data.id));

    Ok(Json(result))
}

/// Get the default representation of the posted set of statistics in the data
/// store. This is executed if the other routes are not matched.
#[post("/", data = "<statistics>", rank = 4)]
pub fn post_statistic_default(
    conn: CorgisDbConn,
    statistics: Json<NewStatistics>,
) -> Result<Json<Statistics>, diesel::result::Error> {
    post_statistic_json(conn, statistics)
}
