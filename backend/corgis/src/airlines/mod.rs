//! A module wrapping the [CORGIS Airlines JSON](https://think.cs.vt.edu/corgis/json/airlines/airlines.html) data-set.

#[cfg(test)]
mod tests;

use derive_new::new;
use getset::Getters;
use indexmap::IndexSet;
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

    /// Return an iterator across the records of the data-set.
    pub fn records(&self) -> std::slice::Iter<Record> {
        self.records.iter()
    }

    /// Return an iterator across the unique airports in the data-set.
    pub fn airports(&self) -> AirportIter {
        AirportIter::new(self)
    }

    /// Return an iterator across the unique carriers in the data-set.
    pub fn carriers(&self) -> CarrierIter {
        CarrierIter::new(self)
    }
}

/// An entry from the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[get = "pub"]
pub struct Record {
    airport: Airport,
    carrier: Carrier,
    statistics: Statistics,
    time: Time,
}

/// An airport as represented in the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[get = "pub"]
pub struct Airport {
    code: String,
    name: String,
}

/// An airline carrier as represented in the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[get = "pub"]
pub struct Carrier {
    code: String,
    name: String,
}

/// A set of statistics associated with an entry in the CORGIS Airlines dataset.
/// A statistics is identified uniquely by an airport, carrier, and a time.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[get = "pub"]
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
#[get = "pub"]
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
#[get = "pub"]
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
#[get = "pub"]
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
#[get = "pub"]
pub struct Time {
    label: String,
    month: u32,
    year: u32,
}

/// An iterator across the airports in a data-set.
pub struct AirportIter {
    airports: IndexSet<Airport>,
    idx: usize,
}

impl AirportIter {
    /// Create an AirportIter containing a de-duplicated of airports from the data-set.
    fn new(data_set: &DataSet) -> Self
where {
        let airports: IndexSet<_> = data_set
            .records
            .iter()
            .map(|record| record.airport.clone())
            .collect();
        Self { airports, idx: 0 }
    }
}

impl Iterator for AirportIter {
    type Item = Airport;

    fn next(&mut self) -> Option<Airport> {
        let airport = self.airports.get_index(self.idx).cloned();
        self.idx += 1;
        airport
    }
}

/// An iterator across the carriers in a data-set.
pub struct CarrierIter {
    carriers: IndexSet<Carrier>,
    idx: usize,
}

impl CarrierIter {
    /// Create a CarrierIter containing a de-duplicated vector of carriers from the data-set.
    fn new(data_set: &DataSet) -> Self
where {
        let carriers: IndexSet<Carrier> = data_set
            .records
            .iter()
            .map(|record| record.carrier.clone())
            .collect();
        Self { carriers, idx: 0 }
    }
}

impl Iterator for CarrierIter {
    type Item = Carrier;

    fn next(&mut self) -> Option<Carrier> {
        let carrier = self.carriers.get_index(self.idx).cloned();
        self.idx += 1;
        carrier
    }
}
