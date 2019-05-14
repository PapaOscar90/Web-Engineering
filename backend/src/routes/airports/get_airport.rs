//! A module defining the `get_airport` routes.

use super::views::Airport;

use crate::CorgisDbConn;
use diesel::{prelude::*, result::Error};
use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn get_airport_data(conn: &diesel::PgConnection, id: i64) -> Result<Option<Airport>, Error> {
    use crate::database::models;
    use crate::database::schema::airports::dsl::airports;

    Ok(airports
        .find(id)
        .first::<models::Airport>(conn)
        .optional()?
        .map(Airport::from))
}

/// Get the JSON representation of an airport in the database.
#[get("/<airport>", format = "application/json", rank = 1)]
pub fn get_airport_json(conn: CorgisDbConn, airport: i64) -> Result<Option<Json<Airport>>, Error> {
    Ok(get_airport_data(&conn, airport)?.map(Json))
}

/// Get the CSV representation of an airport in the database.
#[get("/<airport>", format = "text/csv", rank = 2)]
pub fn get_airport_csv(conn: CorgisDbConn, airport: i64) -> Result<Option<Csv<Airport>>, Error> {
    fn convertor(airport: &Airport) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        wtr.serialize(airport).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    Ok(get_airport_data(&conn, airport)?.map(|data| Csv(data, convertor)))
}

/// Get the HAL representation of an airport in the database.
#[get("/<airport>", format = "application/hal+json", rank = 3)]
pub fn get_airport_hal(
    conn: CorgisDbConn,
    airport: i64,
) -> Result<Option<Json<HalResource>>, Error> {
    let data = get_airport_data(&conn, airport)?;
    match data {
        None => Ok(None),
        Some(data) => {
            let result = HalResource::new(&data)
                .with_link("self", format!("/airports/{}", data.id))
                .with_link("carriers", format!("/carriers?airport={}", data.id))
                .with_link("statistics", format!("/statistics?airport={}", data.id));

            Ok(Some(Json(result)))
        }
    }
}

/// Get the default representation of an airport in the data store. This is
/// executed if the other routes are not matched.
#[get("/<airport>", rank = 4)]
pub fn get_airport_default(
    conn: CorgisDbConn,
    airport: i64,
) -> Result<Option<Json<Airport>>, diesel::result::Error> {
    get_airport_json(conn, airport)
}
