#![feature(proc_macro_hygiene, decl_macro)]
use corgis::airlines::*;
use rocket::{get, routes, State};
use rocket_contrib::json::Json;
use rustic_hal::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Gets airports from ```DataStore``` and wrap in ```HAL```
/// The `HAL` resource provides links to each airport in the collection
/// Returns the JSON of the HAL of Airports
#[get("/airports")]
fn get_airports(data_store: State<DataStore>) -> Json<Vec<HalResource>> {
    // Take the records from DataStore, and store the airports in a hash set
    let airports: HashSet<_> = data_store
        .records
        .iter()
        .map(|record| record.airport.clone())
        .collect();

    // Airports is now wrapped in the HAL with links
    let airports = airports
        .iter()
        .map(|airport| {
            let code = airport.code.clone();
            HalResource::new(airport)
                .with_link("self", format!("/airports/{}", code))
                .with_link("statistics", format!("/statistics?airport_code={}", code))
        })
        .collect();

    Json(airports)
}

#[get("/airports/<airport_code>")]
fn get_airport(airport_code: String, data_store: State<DataStore>) -> Json<HalResource> {
    let record = data_store
        .records
        .iter()
        .find(|record| record.airport.code == airport_code)
        .expect("404 - TODO: Replace with re-direct");

    let airport = record.airport.clone();

    let hal = HalResource::new(airport).with_link("self", format!("/airports/{}", airport_code));

    Json(hal)
}

#[get("/carriers?<airport_code>")]
fn get_carriers(
    airport_code: Option<String>,
    data_store: State<DataStore>,
) -> Json<HashSet<Carrier>> {
    let carriers: HashSet<_> = data_store
        .records
        .iter()
        .filter(|&record| {
            airport_code
                .clone()
                .map_or(true, |airport_code| record.airport.code == airport_code)
        })
        .map(|record| record.carrier.clone())
        .collect();
    Json(carriers)
}

#[get("/statistics?<carrier_code>&<airport_code>&<month>")]
fn get_statistics(
    carrier_code: Option<String>,
    airport_code: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<Record>> {
    Json(
        data_store
            .records
            .iter()
            .filter(|&record| {
                carrier_code
                    .clone()
                    .map_or(true, |carrier_code| record.carrier.code == carrier_code)
                    && airport_code
                        .clone()
                        .map_or(true, |airport_code| record.airport.code == airport_code)
                    && month.map_or(true, |month| record.time.month == month)
            })
            .cloned()
            .collect(),
    )
}

#[get("/statistics/on_time?<carrier_code>&<airport_code>&<month>")]
fn get_on_time_statistics(
    carrier_code: Option<String>,
    airport_code: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<(String, String, i32)>> {
    Json(
        data_store
            .records
            .iter()
            .filter(|&record| {
                carrier_code
                    .clone()
                    .map_or(true, |carrier_code| record.carrier.code == carrier_code)
                    && airport_code
                        .clone()
                        .map_or(true, |airport_code| record.airport.code == airport_code)
                    && month.map_or(true, |month| record.time.month == month)
            })
            .map(|record| {
                (
                    record.carrier.code.clone(),
                    record.airport.code.clone(),
                    record.statistics.flights.on_time,
                )
            })
            .collect(),
    )
}

#[get("/statistics/delayed?<carrier_code>&<airport_code>&<month>")]
fn get_delayed_statistics(
    carrier_code: Option<String>,
    airport_code: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<(String, String, i32)>> {
    Json(
        data_store
            .records
            .iter()
            .filter(|&record| {
                carrier_code
                    .clone()
                    .map_or(true, |carrier_code| record.carrier.code == carrier_code)
                    && airport_code
                        .clone()
                        .map_or(true, |airport_code| record.airport.code == airport_code)
                    && month.map_or(true, |month| record.time.month == month)
            })
            .map(|record| {
                (
                    record.carrier.code.clone(),
                    record.airport.code.clone(),
                    record.statistics.flights.delayed,
                )
            })
            .collect(),
    )
}

#[get("/statistics/cancelled?<carrier_code>&<airport_code>&<month>")]
fn get_cancelled_statistics(
    carrier_code: Option<String>,
    airport_code: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<(String, String, i32)>> {
    Json(
        data_store
            .records
            .iter()
            .filter(|&record| {
                carrier_code
                    .clone()
                    .map_or(true, |carrier_code| record.carrier.code == carrier_code)
                    && airport_code
                        .clone()
                        .map_or(true, |airport_code| record.airport.code == airport_code)
                    && month.map_or(true, |month| record.time.month == month)
            })
            .map(|record| {
                (
                    record.carrier.code.clone(),
                    record.airport.code.clone(),
                    record.statistics.flights.cancelled,
                )
            })
            .collect(),
    )
}

#[get("/statistics/minutes_delayed?<carrier_code>&<airport_code>&<month>&<reason>")]
fn get_minutes_delayed_statistics(
    carrier_code: Option<String>,
    airport_code: Option<String>,
    month: Option<u32>,
    reason: Option<String>,
    data_store: State<DataStore>,
) -> Json<Vec<(String, i32, Time)>> {
    Json(
        data_store
            .records
            .iter()
            .filter(|&record| {
                carrier_code
                    .clone()
                    .map_or(true, |carrier_code| record.carrier.code == carrier_code)
                    && airport_code
                        .clone()
                        .map_or(true, |airport_code| record.airport.code == airport_code)
                    && month.map_or(true, |month| record.time.month == month)
            })
            .map(|record| {
                (
                    record.carrier.code.clone(),
                    if let Some(reason) = &reason {
                        match reason.as_str() {
                            "carrier" => record.statistics.minutes_delayed.carrier,
                            "late_aircraft" => record.statistics.minutes_delayed.carrier,
                            "national_aviation_system" => {
                                record.statistics.minutes_delayed.national_aviation_system
                            }
                            "security" => record.statistics.minutes_delayed.security,
                            "total" => record.statistics.minutes_delayed.total,
                            "weather" => record.statistics.minutes_delayed.weather,
                            _ => record.statistics.minutes_delayed.total,
                        }
                    } else {
                        record.statistics.minutes_delayed.total
                    },
                    record.time.clone(),
                )
            })
            .collect(),
    )
}

#[get("/statistics/connection?<airport_1_code>&<airport_2_code>&<carrier_code>")]
fn get_connection_statistics(
    airport_1_code: String,
    airport_2_code: String,
    carrier_code: Option<String>,
) -> Json<Vec<Statistics>> {
    unimplemented!();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes!(
                get_airports,
                get_airport,
                get_carriers,
                get_statistics,
                get_on_time_statistics,
                get_delayed_statistics,
                get_cancelled_statistics,
                get_minutes_delayed_statistics,
                get_connection_statistics,
            ),
        )
        .manage(DataStore::new())
}

fn main() {
    rocket().launch();
}
