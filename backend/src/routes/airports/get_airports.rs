//! A module dealing with the `get_airports` routes.

use super::views::Airport;

use crate::CorgisDbConn;
use diesel::{dsl::*, prelude::*, result::Error};
use rayon::prelude::*;
use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn get_airports_data(conn: &diesel::PgConnection, id: Option<i64>) -> Result<Vec<Airport>, Error> {
    use crate::database::schema::airports::dsl::airports;
    use crate::database::{models, schema};

    match id {
        None => match airports.load::<models::Airport>(conn).optional()? {
            Some(airport_data) => Ok(airport_data.into_par_iter().map(Airport::from).collect()),
            None => Ok(Vec::new()),
        },
        Some(id) => {
            // Find the carriers that match the provided id.
            let carrier_id: i64 = match schema::carriers::table
                .find(id)
                .select(schema::carriers::id)
                .first(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(carrier_id) => carrier_id,
            };

            let airport_ids: Vec<i64> = match schema::statistics::table
                .filter(schema::statistics::carrier_id.eq(carrier_id))
                .inner_join(airports)
                .select(schema::airports::id)
                .load(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(airport_ids) => airport_ids,
            };

            match airports
                .filter(schema::airports::id.eq(any(airport_ids)))
                .load::<models::Airport>(conn)
                .optional()?
            {
                Some(airport_data) => Ok(airport_data.into_par_iter().map(Airport::from).collect()),
                None => Ok(Vec::new()),
            }
        }
    }
}

/// Get the JSON representation of the airports in the database.
#[get("/?<carrier>", format = "application/json", rank = 1)]
pub fn get_airports_json(
    conn: CorgisDbConn,
    carrier: Option<i64>,
) -> Result<Json<Vec<Airport>>, Error> {
    get_airports_data(&conn, carrier).map(Json)
}

/// Get the CSV representation of the airports in the database.
#[get("/?<carrier>", format = "text/csv", rank = 2)]
pub fn get_airports_csv(
    conn: CorgisDbConn,
    carrier: Option<i64>,
) -> Result<Csv<Vec<Airport>>, Error> {
    fn convertor(airports: &Vec<Airport>) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        for airport in airports {
            wtr.serialize(airport).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    get_airports_data(&conn, carrier).map(|data| Csv(data, convertor))
}

/// Get the HAL representation of the airports in the database.
#[get("/?<carrier>", format = "application/hal+json", rank = 3)]
pub fn get_airports_hal(
    conn: CorgisDbConn,
    carrier: Option<i64>,
) -> Result<Json<Vec<HalResource>>, Error> {
    let result = get_airports_data(&conn, carrier)?
        .into_par_iter()
        .map(|data| match carrier {
            Some(carrier) => HalResource::new(&data)
                .with_link("self", format!("/airports/{}", data.id))
                .with_link("carriers", format!("/carriers?airport={}", data.id))
                .with_link(
                    "statistics",
                    format!("/statistics?airport={}&carrier={}", data.id, carrier),
                ),
            None => HalResource::new(&data)
                .with_link("self", format!("/airports/{}", data.id))
                .with_link("carriers", format!("/carriers?airport={}", data.id))
                .with_link("statistics", format!("/statistics?airport={}", data.id)),
        })
        .collect();

    Ok(Json(result))
}

/// Get the default representation of the airports in the data store. This is
/// executed if the other routes are not matched.
#[get("/?<carrier>", rank = 4)]
pub fn get_airports_default(
    conn: CorgisDbConn,
    carrier: Option<i64>,
) -> Result<Json<Vec<Airport>>, diesel::result::Error> {
    get_airports_json(conn, carrier)
}
