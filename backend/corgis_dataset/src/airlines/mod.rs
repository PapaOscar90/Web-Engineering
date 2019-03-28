//! A module providing wrappers to the [CORGIS Airlines JSON](https://think.cs.vt.edu/corgis/json/airlines/airlines.html) data-set.

use serde::Deserialize;
use std::collections::HashSet;

/// A collection of [records](Record) providing access to the CORGIS Airlines
/// data-set.
#[derive(Clone, Debug, Deserialize)]
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

    /// Return at iterator across the records of the data-set.
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

/// An entry from the CORGIS Airlines data-set.
#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    /// A record involves a specific airport.
    pub airport: Airport,

    /// A record involves a specific carrier.
    pub carrier: Carrier,

    /// A record contains statistics for an `Carrier` at an `Airport` at a
    /// specific `Time`.
    pub statistics: Statistics,

    /// A record is specified at a particular time.
    pub time: Time,
}

/// An airport as represented in the CORGIS Airlines data-set.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Airport {
    /// A unique 3 letter code for the airport as assigned by the International
    /// Air Transport Association.
    pub code: String,

    /// The full name of the airport.
    pub name: String,
}

/// An airline carrier as represented in the CORGIS Airlines data-set.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Carrier {
    /// A 2 letter code for the carrier as assigned by the International Air
    /// Transport Association. NOTE this code may not be unique, e.g. both
    /// Endeavor Air Inc. and Pinnacle Airlines Inc. have the code 9E.
    pub code: String,

    /// The full name of the carrier.
    pub name: String,
}

/// A collection of statistics associated with a carrier and an airport at a
/// particular time.
#[derive(Clone, Debug, Deserialize)]
pub struct Statistics {
    /// The statistics associated with the number of flights.
    pub flights: Flights,

    /// The statistics associated with the number of minutes delayed due to
    /// different reasons.
    #[serde(rename = "minutes delayed")]
    pub minutes_delayed: MinutesDelayed,

    /// The statistics associated with the number of delays due to different reasons.
    #[serde(rename = "# of delays")]
    pub number_of_delays: NumberOfDelays,
}

/// The number of flights for different properties.
#[derive(Clone, Debug, Deserialize)]
pub struct Flights {
    /// The number of flights cancelled.
    pub cancelled: i64,

    /// The number of flights delayed.
    pub delayed: i64,

    /// The number of flights diverted.
    pub diverted: i64,

    /// The number of flights that were on time.
    #[serde(rename = "on time")]
    pub on_time: i64,

    /// The total number of flights.
    pub total: i64,
}

/// The number of minutes delayed due to different reasons.
#[derive(Clone, Debug, Deserialize)]
pub struct MinutesDelayed {
    /// The number of minutes delayed due to carrier delays.
    pub carrier: i64,

    /// The number of minutes delayed to late aircraft.
    #[serde(rename = "late aircraft")]
    pub late_aircraft: i64,

    /// The number of minutes delayed due to the national aviation system.
    #[serde(rename = "national aviation system")]
    pub national_aviation_system: i64,

    /// The number of minutes delayed due to security.
    pub security: i64,

    /// The total number of minutes delayed.
    pub total: i64,

    /// The number of minutes delayed due to weather.
    pub weather: i64,
}

/// The number of delays due to different reasons.
#[derive(Clone, Debug, Deserialize)]
pub struct NumberOfDelays {
    /// The number of delays due to carrier delays.
    pub carrier: i64,

    /// The number of delays due to late aircraft.
    #[serde(rename = "late aircraft")]
    pub late_aircraft: i64,

    /// The number of delays due to the national aviation system.
    #[serde(rename = "national aviation system")]
    pub national_aviation_system: i64,

    /// The number of delays due to security.
    pub security: i64,

    /// The number of delays due to weather.
    pub weather: i64,
}

/// A time unit as provided in the Airlines dataset.
#[derive(Clone, Debug, Deserialize)]
pub struct Time {
    /// A label of the time unit in the format "Y/M", e.g. "2018/01".
    pub label: String,
    /// A month as represented by an integer. The values in the initial dataset are indexed at 0.
    pub month: u32,
    /// A year as represented as a 4 digit number.
    pub year: i32,
}

/// An iterator across the airports in a data-set.
pub struct AirportIter<'a> {
    airports: HashSet<&'a Airport>,
    idx: usize,
}

impl<'a> AirportIter<'a> {
    /// Create an new iterator containing unique airports from the provided data-set.
    fn new(data_set: &'a DataSet) -> Self {
        let airports: HashSet<_> = data_set
            .records
            .iter()
            .map(|record| &record.airport)
            .collect();
        Self { airports, idx: 0 }
    }
}

impl<'a> Iterator for AirportIter<'a> {
    type Item = &'a Airport;

    fn next(&mut self) -> Option<&'a Airport> {
        let next = self.airports.iter().nth(self.idx).cloned();
        self.idx += 1;
        next
    }
}

impl<'a> ExactSizeIterator for AirportIter<'a> {
    fn len(&self) -> usize {
        self.airports.iter().len()
    }
}

/// An iterator across the carriers in a data-set.
pub struct CarrierIter<'a> {
    carriers: HashSet<&'a Carrier>,
    idx: usize,
}

impl<'a> CarrierIter<'a> {
    /// Create an new iterator containing unique carriers from the provided data-set.
    fn new(data_set: &'a DataSet) -> Self {
        use std::collections::HashSet;
        let carriers: HashSet<_> = data_set
            .records
            .iter()
            .map(|record| &record.carrier)
            .collect();
        Self { carriers, idx: 0 }
    }
}

impl<'a> Iterator for CarrierIter<'a> {
    type Item = &'a Carrier;

    fn next(&mut self) -> Option<&'a Carrier> {
        let next = self.carriers.iter().nth(self.idx).cloned();
        self.idx += 1;
        next
    }
}

impl<'a> ExactSizeIterator for CarrierIter<'a> {
    fn len(&self) -> usize {
        self.carriers.iter().len()
    }
}
