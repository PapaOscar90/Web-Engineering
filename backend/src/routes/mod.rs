//! A module dealing with the routes served by the web-server.
use super::DataStore;
use corgis::airlines::Airport;
use rocket::{get, routes};
use rocket::{Rocket, State};
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;

/// Get the JSON representation of the airports in the data store.
#[get("/airports", format = "application/json", rank = 1)]
fn json_route(data_store: State<DataStore>) -> Json<Vec<Airport>> {
    Json(data_store.airports().collect())
}

/// Get the CSV representation of the airports in the data store.
#[get("/airports", format = "text/csv", rank = 2)]
fn csv_route(data_store: State<DataStore>) -> Csv<Vec<Airport>> {
    fn convertor(airports: &Vec<Airport>) -> String {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        for airport in airports {
            wtr.serialize(airport).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    Csv::new(data_store.airports().collect(), convertor)
}

/// Get default representation of the airports in the data store if the format is not specified.
#[get("/airports", rank = 3)]
fn default_route(data_store: State<DataStore>) -> Json<Vec<Airport>> {
    json_route(data_store)
}

/// Mount the routes to a moved `rocket` and return the resulting `rocket`.
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes!(json_route, csv_route, default_route))
}
