#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Airport {
    pub code: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Flights {
    pub cancelled: i32,
    pub delayed: i32,
    pub diverted: i32,
    #[serde(rename = "on time")]
    pub on_time: i32,
    pub total: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct NumberOfDelays {
    pub carrier: i32,
    #[serde(rename = "late aircraft")]
    pub late_aircraft: i32,
    #[serde(rename = "national aviation system")]
    pub national_aviation_system: i32,
    pub security: i32,
    pub weather: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct MinutesDelayed {
    pub carrier: i32,
    #[serde(rename = "late aircraft")]
    pub late_aircraft: i32,
    #[serde(rename = "national aviation system")]
    pub national_aviation_system: i32,
    pub security: i32,
    pub total: i32,
    pub weather: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Statistics {
    pub flights: Flights,
    #[serde(rename = "minutes delayed")]
    pub minutes_delayed: MinutesDelayed,
    #[serde(rename = "# of delays")]
    pub number_of_delays: NumberOfDelays,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Time {
    pub label: String,
    pub month: u32,
    pub year: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Carrier {
    pub code: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub struct Record {
    pub airport: Airport,
    pub carrier: Carrier,
    pub statistics: Statistics,
    pub time: Time,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[serde(transparent)]
pub struct DataStore {
    pub records: Vec<Record>,
}

impl DataStore {
    pub fn new() -> Self {
        let json_string = include_str!("airlines.json");
        serde_json::from_str(json_string).unwrap()
    }
}
