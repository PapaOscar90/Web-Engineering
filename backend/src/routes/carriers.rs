//! A module dealing with the "/carriers" routes.

use crate::DataStore;
use corgis::airlines::Carrier;
use rocket::get;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;

/// Get the JSON representation of the carriers in the data store.
#[get("/", format = "application/json", rank = 1)]
pub fn get_carriers_json(data_store: State<DataStore>) -> Json<Vec<Carrier>> {
    Json(data_store.carriers().collect())
}

/// Get the CSV representation of the carriers in the data store.
#[get("/", format = "text/csv", rank = 2)]
pub fn get_carriers_csv(data_store: State<DataStore>) -> Csv<Vec<Carrier>> {
    fn convertor(carriers: &Vec<Carrier>) -> String {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        for carrier in carriers {
            wtr.serialize(carrier).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    Csv::new(data_store.carriers().collect(), convertor)
}

/// Get the default representation of the carriers in the data store. This executed if the other routes are not matched.
#[get("/", rank = 3)]
pub fn get_carriers_default(data_store: State<DataStore>) -> Json<Vec<Carrier>> {
    get_carriers_json(data_store)
}

/// Get the JSON representation of a carrier in the data store.
#[get("/<code>", format = "application/json", rank = 1)]
pub fn get_carrier_json(code: String, data_store: State<DataStore>) -> Option<Json<Carrier>> {
    data_store
        .carriers()
        .find(|carrier| *carrier.code() == code)
        .map(Json)
}

/// Get the CSV representation of a carrier in the data store.
#[get("/<code>", format = "text/csv", rank = 2)]
pub fn get_carrier_csv(code: String, data_store: State<DataStore>) -> Option<Csv<Carrier>> {
    fn convertor(carrier: &Carrier) -> String {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.serialize(carrier).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    data_store
        .carriers()
        .find(|carrier| *carrier.code() == code)
        .map(|airport| Csv::new(airport, convertor))
}

/// Get the default representation of a carrier in the data store. This executed if the other routes are not matched.
#[get("/<code>", rank = 3)]
pub fn get_carrier_default(code: String, data_store: State<DataStore>) -> Option<Json<Carrier>> {
    get_carrier_json(code, data_store)
}
