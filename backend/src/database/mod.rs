//! Functions for interacting with the database containing the CORGIS airlines
//! data.

pub mod models;
#[allow(missing_docs)]
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{Airport, Carrier, Statistics};
use models::{NewAirport, NewCarrier, NewStatistics};
use std::env;

/// Create a connection to the database.
/// Variables in the `.env` are loaded by this function prior to checking for
/// the `DATABASE_URL`.
///
/// # Panics
///   1. If the `DATABASE_URL` is not set in the environment variables.
///   2. If there is an error in establishing a connecting to the database.
pub fn establish_connection() -> PgConnection {
    // Load the variables from the `.env` file.
    dotenv().ok();

    // Extract the url from the environment variables.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// Insert an airport into the database if possible and return the result.
pub fn create_airport<'a>(
    conn: &PgConnection,
    new_airport: NewAirport,
) -> Result<Airport, diesel::result::Error> {
    use schema::airports;

    diesel::insert_into(airports::table)
        .values(&new_airport)
        .get_result(conn)
}

/// Insert a carrier into the database if possible and return the result.
pub fn create_carrier<'a>(
    conn: &PgConnection,
    new_carrier: NewCarrier,
) -> Result<Carrier, diesel::result::Error> {
    use schema::carriers;

    diesel::insert_into(carriers::table)
        .values(&new_carrier)
        .get_result(conn)
}

/// Insert a set of statistics into the database if possible and return the result.
pub fn create_statistics<'a>(
    conn: &PgConnection,
    new_statistics: NewStatistics,
) -> Result<Statistics, diesel::result::Error> {
    use schema::statistics;

    diesel::insert_into(statistics::table)
        .values(&new_statistics)
        .get_result(conn)
}
