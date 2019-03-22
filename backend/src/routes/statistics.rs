//! A module dealing with the "/statistics" routes.

use crate::DataStore;
use corgis::airlines::{Airport, Carrier, Flights, Record, Statistics, Time};
use rocket::get;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use serde::Serialize;

/// Get the JSON representation of the record in the data store.
#[get("/?<carrier>&<airport>&<month>", format = "application/json", rank = 1)]
pub fn get_statistics_json(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<Record>> {
    Json(
        data_store
            .lock()
            .unwrap()
            .records()
            .filter(|&record| {
                carrier
                    .clone()
                    .map_or(true, |carrier| *record.carrier().code() == carrier)
                    && airport
                        .clone()
                        .map_or(true, |airport| *record.airport().code() == airport)
                    && month.map_or(true, |month| *record.time().month() == month)
            })
            .cloned()
            .collect(),
    )
}

/// Get the CSV representation of the record in the data store.
#[get("/?<carrier>&<airport>&<month>", format = "text/csv", rank = 2)]
pub fn get_statistics_csv(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Csv<Vec<Record>> {
    fn convertor(records: &Vec<Record>) -> String {
        let mut wtr = csv::WriterBuilder::default()
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
        ])
        .unwrap();
        for record in records {
            wtr.serialize(record).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    Csv::new(
        data_store
            .lock()
            .unwrap()
            .records()
            .filter(|&record| {
                carrier
                    .clone()
                    .map_or(true, |carrier| *record.carrier().code() == carrier)
                    && airport
                        .clone()
                        .map_or(true, |airport| *record.airport().code() == airport)
                    && month.map_or(true, |month| *record.time().month() == month)
            })
            .cloned()
            .collect(),
        convertor,
    )
}

/// Get the default representation of the record in the data store. This executed if the other routes are not matched.
#[get("/?<carrier>&<airport>&<month>", rank = 3)]
pub fn get_statistics_default(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<Record>> {
    get_statistics_json(carrier, airport, month, data_store)
}

/// An intermediary type to eject specifically the flights statistics
#[derive(Serialize)]
pub struct FlightsStatistics {
    airport: Airport,
    carrier: Carrier,
    flights: Flights,
    time: Time,
}

/// Get the JSON representation of the flights statistics in the data store.
#[get(
    "/flights?<carrier>&<airport>&<month>",
    format = "application/json",
    rank = 1
)]
pub fn get_statistics_flights_json(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<FlightsStatistics>> {
    Json(
        data_store
            .lock()
            .unwrap()
            .records()
            .filter(|&record| {
                carrier
                    .clone()
                    .map_or(true, |carrier| *record.carrier().code() == carrier)
                    && airport
                        .clone()
                        .map_or(true, |airport| *record.airport().code() == airport)
                    && month.map_or(true, |month| *record.time().month() == month)
            })
            .map(|record| FlightsStatistics {
                airport: record.airport().clone(),
                carrier: record.carrier().clone(),
                flights: record.statistics().flights().clone(),
                time: record.time().clone(),
            })
            .collect(),
    )
}

/// Get the CSV representation of the flights statistics in the data store.
#[get("/flights?<carrier>&<airport>&<month>", format = "text/csv", rank = 2)]
pub fn get_statistics_flights_csv(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Csv<Vec<FlightsStatistics>> {
    fn convertor(records: &Vec<FlightsStatistics>) -> String {
        let mut wtr = csv::WriterBuilder::default()
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
            "Time.Label",
            "Time.Month",
            "Time.Year",
        ])
        .unwrap();
        for record in records {
            wtr.serialize(record).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };
    Csv::new(
        data_store
            .lock()
            .unwrap()
            .records()
            .filter(|&record| {
                carrier
                    .clone()
                    .map_or(true, |carrier| *record.carrier().code() == carrier)
                    && airport
                        .clone()
                        .map_or(true, |airport| *record.airport().code() == airport)
                    && month.map_or(true, |month| *record.time().month() == month)
            })
            .map(|record| FlightsStatistics {
                airport: record.airport().clone(),
                carrier: record.carrier().clone(),
                flights: record.statistics().flights().clone(),
                time: record.time().clone(),
            })
            .collect(),
        convertor,
    )
}

/// Get the default representation of the on time flight statistics in the data store. This executed if the other routes are not matched.
#[get("/flights?<carrier>&<airport>&<month>", rank = 3)]
pub fn get_statistics_flights_default(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<FlightsStatistics>> {
    get_statistics_flights_json(carrier, airport, month, data_store)
}
