//! A module dealing with the "/airports" routes.

use crate::DataStore;
use corgis::airlines::Airport;
use rocket::get;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;

/// Get the JSON representation of the airports in the data store.
#[get("/", format = "application/json", rank = 1)]
pub fn get_airports_json(data_store: State<DataStore>) -> Json<Vec<Airport>> {
    Json(data_store.airports().collect())
}

/// Get the CSV representation of the airports in the data store.
#[get("/", format = "text/csv", rank = 2)]
pub fn get_airports_csv(data_store: State<DataStore>) -> Csv<Vec<Airport>> {
    fn convertor(airports: &Vec<Airport>) -> String {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        for airport in airports {
            wtr.serialize(airport).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    Csv::new(data_store.airports().collect(), convertor)
}

/// Get the default representation of the airports in the data store. This executed if the other routes are not matched.
#[get("/", rank = 3)]
pub fn get_airports_default(data_store: State<DataStore>) -> Json<Vec<Airport>> {
    get_airports_json(data_store)
}

/// Get the JSON representation of an airport in the data store.
#[get("/<code>", format = "application/json", rank = 1)]
pub fn get_airport_json(code: String, data_store: State<DataStore>) -> Option<Json<Airport>> {
    data_store
        .airports()
        .find(|airport| *airport.code() == code)
        .map(Json)
}

/// Get the CSV representation of an airport in the data store.
#[get("/<code>", format = "text/csv", rank = 2)]
pub fn get_airport_csv(code: String, data_store: State<DataStore>) -> Option<Csv<Airport>> {
    fn convertor(airport: &Airport) -> String {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.serialize(airport).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    data_store
        .airports()
        .find(|airport| *airport.code() == code)
        .map(|airport| Csv::new(airport, convertor))
}

/// Get the default representation of an airport in the data store. This executed if the other routes are not matched.
#[get("/<code>", rank = 3)]
pub fn get_airport_default(code: String, data_store: State<DataStore>) -> Option<Json<Airport>> {
    get_airport_json(code, data_store)
}
