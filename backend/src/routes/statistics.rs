//! A module dealing with the "/statistics" routes.

use crate::DataStore;
use corgis::airlines::Record;
use corgis::airlines::Statistics;
use rocket::get;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;

/// Get the JSON representation of the record in the data store.
#[get("/", format = "application/json", rank = 1)]
pub fn get_statistics_json(data_store: State<DataStore>) -> Json<Vec<Record>> {
    Json(data_store.lock().unwrap().records().cloned().collect())
}

/// Get the CSV representation of the record in the data store.
#[get("/", format = "text/csv", rank = 2)]
pub fn get_statistics_csv(data_store: State<DataStore>) -> Csv<Vec<Record>> {
    fn convertor(records: &Vec<Record>) -> String {
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(Vec::new());
        wtr.write_record(&[
            "Airport.Code",
            "Airport.Name",
            "Carrier.Code",
            "Carrier.Name",
            "Flights.Cancelled",
            "Flights.Delayed",
            "Flights.Diverted",
            "Flights.On Time",
            "Flights.Total",
            "Minutes Delayed.Carrier",
            "Minutes Delayed.Late Aircraft",
            "Minutes Delayed.National Aviation System",
            "Minutes Delayed.Security",
            "Minutes Delayed.Total",
            "Minutes Delayed.Weather",
            "# of Delays.Carrier",
            "# of Delays.Late Aircraft",
            "# of Delays.National Aviation System",
            "# of Delays.Security",
            "# of Delays.Weather",
            "Time.Label",
            "Time.Month",
            "Time.Year",
        ]);
        for record in records {
            wtr.serialize(record).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    Csv::new(
        data_store.lock().unwrap().records().cloned().collect(),
        convertor,
    )
}

/// Get the default representation of the record in the data store. This executed if the other routes are not matched.
#[get("/", rank = 3)]
pub fn get_statistics_default(data_store: State<DataStore>) -> Json<Vec<Record>> {
    get_statistics_json(data_store)
}
