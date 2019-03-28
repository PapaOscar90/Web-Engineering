//! A module containing the airport structures returned by the API.

use serde::Serialize;

/// An airport as returned by the API.
#[derive(Serialize)]
pub struct Airport {
    /// The id for an airport.
    pub id: i64,

    /// A unique 3 letter code for the airport as assigned by the International
    /// Air Transport Association.
    pub code: String,

    /// The full name of the airport.
    pub name: String,
}

impl From<crate::database::models::Airport> for Airport {
    fn from(source: crate::database::models::Airport) -> Self {
        Self {
            id: source.id,
            code: source.code,
            name: source.name,
        }
    }
}
