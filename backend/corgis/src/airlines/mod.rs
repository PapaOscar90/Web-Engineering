//! A module wrapping the [CORGIS Airlines JSON](https://think.cs.vt.edu/corgis/json/airlines/airlines.html) data-set.

#[cfg(test)]
mod tests;

use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// A collection of [records](Record) providing access to the CORGIS Airlines
/// data-set.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[serde(transparent)]
pub struct DataSet {
    records: Vec<Record>,
}

impl DataSet {
    /// Create a new data-set containing the CORGIS Airlines data.
    pub fn new() -> Self {
        let json_string = include_str!("airlines.json");
        serde_json::from_str(json_string).unwrap()
    }
}

/// An entry from the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Record {
    airport: Airport,
    carrier: Carrier,
    statistics: Statistics,
    time: Time,
}

/// An airport as represented in the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Airport {
    code: String,
    name: String,
}

/// An airline carrier as represented in the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Carrier {
    code: String,
    name: String,
}

/// A set of statistics associated with an entry in the CORGIS Airlines dataset.
/// A statistics is identified uniquely by an airport, carrier, and a time.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Statistics {
    flights: Flights,
    #[serde(rename = "minutes delayed")]
    minutes_delayed: MinutesDelayed,
    #[serde(rename = "# of delays")]
    number_of_delays: NumberOfDelays,
}

/// The statistics providing details on the totals of the number of flights
/// canceled, delayed, diverted, and on time.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Flights {
    cancelled: i32,
    delayed: i32,
    diverted: i32,
    #[serde(rename = "on time")]
    on_time: i32,
    total: i32,
}

/// The statistics of the minutes delayed due to a particular reason.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct MinutesDelayed {
    carrier: i32,
    #[serde(rename = "late aircraft")]
    late_aircraft: i32,
    #[serde(rename = "national aviation system")]
    national_aviation_system: i32,
    security: i32,
    total: i32,
    weather: i32,
}

/// The statistics of the number of delays due to a particular reason.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct NumberOfDelays {
    carrier: i32,
    #[serde(rename = "late aircraft")]
    late_aircraft: i32,
    #[serde(rename = "national aviation system")]
    national_aviation_system: i32,
    security: i32,
    weather: i32,
}

/// A time unit as provided in the Airlines dataset.
// TODO replace this with the `chrono::NaiveDate` type?
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Time {
    label: String,
    month: u32,
    year: u32,
}
