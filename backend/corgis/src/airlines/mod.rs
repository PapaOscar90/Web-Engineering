//! A module wrapping the [CORGIS Airlines JSON](https://think.cs.vt.edu/corgis/json/airlines/airlines.html) data-set.

#[cfg(test)]
mod tests;

use derive_more::*;
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

impl Default for DataSet {
    /// Create a new data-set containing the CORGIS Airlines data.
    fn default() -> Self {
        let json_string = include_str!("airlines.json");
        serde_json::from_str(json_string).unwrap()
    }
}

impl DataSet {
    /// Create a new data-set containing the CORGIS Airlines data.
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert a record into the dataset
    pub fn add_record(&mut self, record: Record) {
        self.records.push(record);
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
pub struct Record {
    /// Get a reference to the `airport` of a `Record`.
    #[get = "pub"]
    airport: Airport,

    /// Get a reference to the `carrier` of a `Record`.
    #[get = "pub"]
    carrier: Carrier,

    /// Get a reference to the `statistics` of a `Record`.
    #[get = "pub"]
    statistics: Statistics,

    /// Get a reference to the `time` of a `Record`.
    #[get = "pub"]
    time: Time,
}

/// An airport as represented in the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Airport {
    /// Get a reference to the `code` of an `Airport`.
    #[get = "pub"]
    code: String,

    /// Get a reference to the `name` of an `Airport`.
    #[get = "pub"]
    name: String,
}

/// An airline carrier as represented in the CORGIS Airlines dataset.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Carrier {
    /// Get a reference to the `code` of a `Carrier`.
    #[get = "pub"]
    code: String,

    /// Get a reference to the `name` of a `Carrier`.
    #[get = "pub"]
    name: String,
}

/// A set of statistics associated with an entry in the CORGIS Airlines dataset.
/// A statistics is identified uniquely by an airport, carrier, and a time.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Statistics {
    /// Get a reference to the `flights` of a `Statistics`.
    #[get = "pub"]
    flights: Flights,

    /// Get a reference to the `minutes_delayed` of a `Statistics`.
    #[get = "pub"]
    #[serde(rename = "minutes delayed")]
    minutes_delayed: MinutesDelayed,

    /// Get a reference to the `number_of_delays` of a `Statistics`.
    #[get = "pub"]
    #[serde(rename = "# of delays")]
    number_of_delays: NumberOfDelays,
}

/// The statistics providing details on the totals of the number of flights
/// canceled, delayed, diverted, and on time.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Flights {
    /// Get a reference to the number of `canceled` `Flights`.
    #[get = "pub"]
    cancelled: i32,

    /// Get a reference to the number of `delayed` `Flights`.
    #[get = "pub"]
    delayed: i32,

    /// Get a reference to the number of `diverted` `Flights`.
    #[get = "pub"]
    diverted: i32,

    /// Get a reference to the number of `on_time` `Flights`.
    #[get = "pub"]
    #[serde(rename = "on time")]
    on_time: i32,

    /// Get a reference to the `total` number of `Flights`.
    #[get = "pub"]
    total: i32,
}

/// The statistics of the minutes delayed due to a particular reason.
#[derive(
    new,
    Getters,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Hash,
    Eq,
    Add,
    Mul,
    Div,
    Sub,
    Ord,
    PartialOrd,
)]
pub struct MinutesDelayed {
    /// Get a reference to the number of `MinutesDelayed` due to the `carrier`.
    #[get = "pub"]
    carrier: i32,

    /// Get a reference to the number of `MinutesDelayed` due to `late_aircraft`.
    #[get = "pub"]
    #[serde(rename = "late aircraft")]
    late_aircraft: i32,

    /// Get a reference to the number of `MinutesDelayed` due to the `national_aviation_system`.
    #[get = "pub"]
    #[serde(rename = "national aviation system")]
    national_aviation_system: i32,

    /// Get a reference to the number of `MinutesDelayed` due to `security`.
    #[get = "pub"]
    security: i32,

    /// Get a reference to the `total` number of `MinutesDelayed`.
    #[get = "pub"]
    total: i32,

    /// Get a reference to the number of `MinutesDelayed` due to `weather`.
    #[get = "pub"]
    weather: i32,
}

impl std::iter::Sum for MinutesDelayed {
    fn sum<I: Iterator<Item = MinutesDelayed>>(iter: I) -> Self {
        let zero = MinutesDelayed {
            carrier: 0,
            late_aircraft: 0,
            national_aviation_system: 0,
            security: 0,
            total: 0,
            weather: 0,
        };
        iter.fold(zero, std::ops::Add::add)
    }
}

/// The statistics of the number of delays due to a particular reason.
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[get = "pub"]
pub struct NumberOfDelays {
    /// Get a reference to the `NumberOfDelays` due to the `carrier`.
    #[get = "pub"]
    carrier: i32,

    /// Get a reference to the `NumberOfDelays` due to `late_aircraft`.
    #[get = "pub"]
    #[serde(rename = "late aircraft")]
    late_aircraft: i32,

    /// Get a reference to the `NumberOfDelays` due to the `national_aviation_system`.
    #[get = "pub"]
    #[serde(rename = "national aviation system")]
    national_aviation_system: i32,

    /// Get a reference to the `NumberOfDelays` due to `security`.
    #[get = "pub"]
    security: i32,

    /// Get a reference to the `NumberOfDelays` due to the `weather`.
    #[get = "pub"]
    weather: i32,
}

/// A time unit as provided in the Airlines dataset.
// TODO replace this with the `chrono::NaiveDate` type?
#[derive(new, Getters, Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[get = "pub"]
pub struct Time {
    /// Get a reference to the `label` of a `Time`.
    #[get = "pub"]
    label: String,

    /// Get a reference to the `month` of a `Time`.
    #[get = "pub"]
    month: u32,

    /// Get a reference to the `year` of a `Time`.
    #[get = "pub"]
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
