//! A module dealing with the "/statistics" routes.

use crate::DataStore;
use corgis::airlines::{Airport, Carrier, Flights, Record, Statistics, Time};
use derive_more::*;
use rocket::get;
use rocket::response::status::NotFound;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use serde::Serialize;
use std::collections::HashMap;

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

/// Get the default representation of the record in the data store. This is executed if the other routes are not matched.
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

/// Get the default representation of the flights statistics in the data store. This is executed if the other routes are not matched.
#[get("/flights?<carrier>&<airport>&<month>", rank = 3)]
pub fn get_statistics_flights_default(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    data_store: State<DataStore>,
) -> Json<Vec<FlightsStatistics>> {
    get_statistics_flights_json(carrier, airport, month, data_store)
}

/// An intermediary type to eject specifically the minutes delayed statistics.
#[derive(Serialize)]
pub struct MinutesDelayedStatistics {
    airport: Airport,
    carrier: Carrier,
    minutes_delayed: MinutesDelayedReasons,
    time: Time,
}

/// An intermediary type allowing for optional serialization of its fields. If the values are set None, they are not serialized.
#[derive(Serialize)]
pub struct MinutesDelayedReasons {
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "late aircraft")]
    late_aircraft: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "national aviation system")]
    national_aviation_system: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weather: Option<i32>,
}

impl From<corgis::airlines::MinutesDelayed> for MinutesDelayedReasons {
    fn from(source: corgis::airlines::MinutesDelayed) -> Self {
        Self {
            carrier: Some(*source.carrier()),
            late_aircraft: Some(*source.carrier()),
            national_aviation_system: Some(*source.carrier()),
            security: Some(*source.carrier()),
            total: Some(*source.carrier()),
            weather: Some(*source.weather()),
        }
    }
}

/// Get the JSON representation of the minutes-delayed statistics. The precise
/// statistics returned are contingent on the value of the optional `reason`
/// string:
///     "carrier_specific" -> carrier, late_aircraft
///     _                  -> all minutes delayed statistics
#[get(
    "/minutes-delayed?<carrier>&<airport>&<month>&<reason>",
    format = "application/json",
    rank = 1
)]
pub fn get_statistics_minutes_delayed_json(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    reason: Option<String>,
    data_store: State<DataStore>,
) -> Json<Vec<MinutesDelayedStatistics>> {
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
            .map(|record| {
                let mut result = MinutesDelayedStatistics {
                    airport: record.airport().clone(),
                    carrier: record.carrier().clone(),
                    minutes_delayed: MinutesDelayedReasons::from(
                        record.statistics().minutes_delayed().clone(),
                    ),
                    time: record.time().clone(),
                };
                if let Some(reason) = reason.clone() {
                    match reason.as_str() {
                        "carrier_specific" => {
                            result.minutes_delayed.national_aviation_system = None;
                            result.minutes_delayed.security = None;
                            result.minutes_delayed.total = None;
                            result.minutes_delayed.weather = None;
                        }
                        _ => (),
                    }
                }
                result
            })
            .collect(),
    )
}

/// Get the CSV representation of the minutes-delayed statistics. The precise
/// statistics returned are contingent on the value of the optional `reason`
/// string:
///     "carrier_specific" -> carrier, late_aircraft
///     _                  -> all minutes delayed statistics
#[get(
    "/minutes-delayed?<carrier>&<airport>&<month>&<reason>",
    format = "text/csv",
    rank = 2
)]
pub fn get_statistics_minutes_delayed_csv(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    reason: Option<String>,
    data_store: State<DataStore>,
) -> Csv<Vec<MinutesDelayedStatistics>> {
    fn convertor(records: &Vec<MinutesDelayedStatistics>) -> String {
        let mut wtr = csv::WriterBuilder::default()
            .has_headers(false)
            .from_writer(Vec::new());
        // Prepare the headers
        let mut headers = vec![
            "Airport.Code",
            "Airport.Name",
            "Carrier.Code",
            "Carrier.Name",
        ];

        // Check the first element to determine which optional headers are set.
        // FIXME this isn't robust.
        if let Some(record) = records.get(0) {
            if record.minutes_delayed.carrier.is_some() {
                headers.push("Minutes Delayed.Carrier");
            }
            if record.minutes_delayed.late_aircraft.is_some() {
                headers.push("Minutes Delayed.Late Aircraft");
            }
            if record.minutes_delayed.national_aviation_system.is_some() {
                headers.push("Minutes Delayed.National Aviation System");
            }
            if record.minutes_delayed.security.is_some() {
                headers.push("Minutes Delayed.Security");
            }
            if record.minutes_delayed.total.is_some() {
                headers.push("Minutes Delayed.Total");
            }
            if record.minutes_delayed.weather.is_some() {
                headers.push("Minutes Delayed.Weather");
            }
        }
        headers.append(&mut vec!["Time.Label", "Time.Month", "Time.Year"]);
        wtr.write_record(&headers).unwrap();
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
            .map(|record| {
                let mut result = MinutesDelayedStatistics {
                    airport: record.airport().clone(),
                    carrier: record.carrier().clone(),
                    minutes_delayed: MinutesDelayedReasons::from(
                        record.statistics().minutes_delayed().clone(),
                    ),
                    time: record.time().clone(),
                };
                if let Some(reason) = reason.clone() {
                    match reason.as_str() {
                        "carrier_specific" => {
                            result.minutes_delayed.national_aviation_system = None;
                            result.minutes_delayed.security = None;
                            result.minutes_delayed.total = None;
                            result.minutes_delayed.weather = None;
                        }
                        _ => (),
                    }
                }
                result
            })
            .collect(),
        convertor,
    )
}

/// Get the default representation of the minutes-delayed statistics in the data store. This is executed if the other routes are not matched.
#[get("/minutes-delayed?<carrier>&<airport>&<month>&<reason>", rank = 3)]
pub fn get_statistics_minutes_delayed_default(
    carrier: Option<String>,
    airport: Option<String>,
    month: Option<u32>,
    reason: Option<String>,
    data_store: State<DataStore>,
) -> Json<Vec<MinutesDelayedStatistics>> {
    get_statistics_minutes_delayed_json(carrier, airport, month, reason, data_store)
}

/// An intermediary type for the ejection of connection statistics.
/// It contains the descriptive statistics for carrier specific delays:
///   i.e. `carrier_delays` and `late_aircraft_delays`
#[derive(Serialize)]
pub struct ConnectionStatistics {
    airport1: Airport,
    airport2: Airport,
    carrier_delays: DescriptiveStatistics,
    late_aircraft_delays: DescriptiveStatistics,
    carrier: Carrier,
}

/// An intermediary type containing the computed descriptive statistics.
#[derive(Serialize, Add, Div)]
pub struct DescriptiveStatistics {
    mean: i32,
    mode: i32,
}

fn get_connection_statistics_data(
    airport1: String,
    airport2: String,
    carrier: Option<String>,
    data_store: State<DataStore>,
) -> Vec<ConnectionStatistics> {
    // Map each carrier to the delay information.
    let mut airport1_delays: HashMap<Carrier, Vec<corgis::airlines::MinutesDelayed>> =
        HashMap::new();

    for record in data_store.lock().unwrap().records().filter(|&record| {
        *record.airport().code() == airport1
            && carrier
                .clone()
                .map_or(true, |carrier| *record.carrier().code() == carrier)
    }) {
        let carrier_vec = airport1_delays
            .entry(record.carrier().clone())
            .or_insert_with(Vec::new);
        carrier_vec.push(record.statistics().minutes_delayed().clone());
    }

    let mut airport2_delays: HashMap<Carrier, Vec<corgis::airlines::MinutesDelayed>> =
        HashMap::new();

    for record in data_store.lock().unwrap().records().filter(|&record| {
        *record.airport().code() == airport2
            && carrier
                .clone()
                .map_or(true, |carrier| *record.carrier().code() == carrier)
    }) {
        let carrier_vec = airport2_delays
            .entry(record.carrier().clone())
            .or_insert_with(Vec::new);
        carrier_vec.push(record.statistics().minutes_delayed().clone());
    }

    let mut result: Vec<ConnectionStatistics> = Vec::new();

    use corgis::airlines::MinutesDelayed;
    for carrier in data_store.lock().unwrap().carriers() {
        if let (Some(delays1), Some(delays2)) =
            (airport1_delays.get(&carrier), airport2_delays.get(&carrier))
        {
            let mean: MinutesDelayed = delays1.iter().cloned().sum::<MinutesDelayed>()
                / (delays1.len() as i32)
                + delays2.iter().cloned().sum::<MinutesDelayed>() / (delays2.len() as i32);

            let mode: MinutesDelayed = std::cmp::max(
                delays1.iter().max().unwrap().clone(),
                delays2.iter().max().unwrap().clone(),
            );
            let carrier_delays = DescriptiveStatistics {
                mean: *mean.carrier(),
                mode: *mode.carrier(),
            };
            let late_aircraft_delays = DescriptiveStatistics {
                mean: *mean.late_aircraft(),
                mode: *mode.late_aircraft(),
            };

            let stat = ConnectionStatistics {
                airport1: data_store
                    .lock()
                    .unwrap()
                    .airports()
                    .find(|airport| *airport.code() == airport1)
                    .unwrap(),
                airport2: data_store
                    .lock()
                    .unwrap()
                    .airports()
                    .find(|airport| *airport.code() == airport1)
                    .unwrap(),
                carrier_delays,
                late_aircraft_delays,
                carrier,
            };
            result.push(stat);
        }
    }
    result
}

#[get(
    "/connection?<airport1>&<airport2>&<carrier>",
    format = "application/json",
    rank = 1
)]
pub fn get_statistics_connection_json(
    airport1: String,
    airport2: String,
    carrier: Option<String>,
    data_store: State<DataStore>,
) -> Result<Json<Vec<ConnectionStatistics>>, NotFound<String>> {
    if data_store
        .lock()
        .unwrap()
        .airports()
        .find(|airport| *airport.code() == airport1)
        .is_none()
    {
        return Err(NotFound(format!("Airport {} does not exist!", airport1)));
    }
    if data_store
        .lock()
        .unwrap()
        .airports()
        .find(|airport| *airport.code() == airport2)
        .is_none()
    {
        return Err(NotFound(format!("Airport {} does not exist!", airport2)));
    }
    Ok(Json(get_connection_statistics_data(
        airport1, airport2, carrier, data_store,
    )))
}

#[get(
    "/connection?<airport1>&<airport2>&<carrier>",
    format = "text/csv",
    rank = 2
)]
pub fn get_statistics_connection_csv(
    airport1: String,
    airport2: String,
    carrier: Option<String>,
    data_store: State<DataStore>,
) -> Result<Csv<Vec<ConnectionStatistics>>, NotFound<String>> {
    if data_store
        .lock()
        .unwrap()
        .airports()
        .find(|airport| *airport.code() == airport1)
        .is_none()
    {
        return Err(NotFound(format!("Airport {} does not exist!", airport1)));
    }
    if data_store
        .lock()
        .unwrap()
        .airports()
        .find(|airport| *airport.code() == airport2)
        .is_none()
    {
        return Err(NotFound(format!("Airport {} does not exist!", airport2)));
    }

    fn convertor(statistics: &Vec<ConnectionStatistics>) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        for statistic in statistics {
            wtr.serialize(statistic).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    Ok(Csv::new(
        get_connection_statistics_data(airport1, airport2, carrier, data_store),
        convertor,
    ))
}

/// Get the default representation of the connections statistics between 2 airports. This is executed if the other routes are not matched.
#[get("/connection?<airport1>&<airport2>&<carrier>", rank = 3)]
pub fn get_statistics_connection_default(
    airport1: String,
    airport2: String,
    carrier: Option<String>,
    data_store: State<DataStore>,
) -> Result<Json<Vec<ConnectionStatistics>>, NotFound<String>> {
    get_statistics_connection_json(airport1, airport2, carrier, data_store)
}
