//! A module containing the airport structures returned by the API.

use serde::Serialize;

/// A carrier as returned by the API.
#[derive(Serialize)]
pub struct Carrier {
    /// The id for a carrier.
    pub id: i64,

    /// A non-unique 2 letter code for the carrier as assigned by the
    /// International Air Transport Association.
    pub code: String,

    /// The full name of the carrier.
    pub name: String,
}

impl From<crate::database::models::Carrier> for Carrier {
    fn from(source: crate::database::models::Carrier) -> Self {
        Self {
            id: source.id,
            code: source.code,
            name: source.name,
        }
    }
}
