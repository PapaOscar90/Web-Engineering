//! A module dealing with the `get_carriers` routes.

use super::views::Carrier;

use crate::CorgisDbConn;
use diesel::{dsl::*, prelude::*, result::Error};
use rayon::prelude::*;
use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn get_carriers_data(conn: &diesel::PgConnection, id: Option<i64>) -> Result<Vec<Carrier>, Error> {
    use crate::database::schema::carriers::dsl::carriers;
    use crate::database::{models, schema};

    match id {
        None => match carriers.load::<models::Carrier>(conn).optional()? {
            Some(carrier_data) => Ok(carrier_data.into_par_iter().map(Carrier::from).collect()),
            None => Ok(Vec::new()),
        },
        Some(id) => {
            // Find the airports that match the provided id.
            let airport_id: i64 = match schema::airports::table
                .find(id)
                .select(schema::airports::id)
                .first(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(airport_id) => airport_id,
            };

            let carrier_ids: Vec<i64> = match schema::statistics::table
                .filter(schema::statistics::airport_id.eq(airport_id))
                .inner_join(carriers)
                .select(schema::carriers::id)
                .load(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(carrier_ids) => carrier_ids,
            };

            match carriers
                .filter(schema::carriers::id.eq(any(carrier_ids)))
                .load::<models::Carrier>(conn)
                .optional()?
            {
                Some(carrier_data) => Ok(carrier_data.into_par_iter().map(Carrier::from).collect()),
                None => Ok(Vec::new()),
            }
        }
    }
}

/// Get the JSON representation of the carriers in the database.
#[get("/?<airport>", format = "application/json", rank = 1)]
pub fn get_carriers_json(
    conn: CorgisDbConn,
    airport: Option<i64>,
) -> Result<Json<Vec<Carrier>>, Error> {
    get_carriers_data(&conn, airport).map(Json)
}

/// Get the CSV representation of the carriers in the database.
#[get("/?<airport>", format = "text/csv", rank = 2)]
pub fn get_carriers_csv(
    conn: CorgisDbConn,
    airport: Option<i64>,
) -> Result<Csv<Vec<Carrier>>, Error> {
    fn convertor(carriers: &Vec<Carrier>) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        for carrier in carriers {
            wtr.serialize(carrier).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    get_carriers_data(&conn, airport).map(|data| Csv(data, convertor))
}

/// Get the HAL representation of the carriers in the database.
#[get("/?<airport>", format = "application/hal+json", rank = 3)]
pub fn get_carriers_hal(
    conn: CorgisDbConn,
    airport: Option<i64>,
) -> Result<Json<Vec<HalResource>>, Error> {
    let result = get_carriers_data(&conn, airport)?
        .into_par_iter()
        .map(|data| match airport {
            Some(airport) => HalResource::new(&data)
                .with_link("self", format!("/carriers/{}", data.id))
                .with_link("carriers", format!("/airports?carrier={}", data.id))
                .with_link(
                    "statistics",
                    format!("/statistics?airport={}&carrier={}", data.id, airport),
                ),
            None => HalResource::new(&data)
                .with_link("self", format!("/carriers/{}", data.id))
                .with_link("carriers", format!("/airports?carrier={}", data.id))
                .with_link("statistics", format!("/statistics?carrier={}", data.id)),
        })
        .collect();

    Ok(Json(result))
}

/// Get the default representation of the carriers in the data store. This is
/// executed if the other routes are not matched.
#[get("/?<airport>", rank = 4)]
pub fn get_carriers_default(
    conn: CorgisDbConn,
    airport: Option<i64>,
) -> Result<Json<Vec<Carrier>>, diesel::result::Error> {
    get_carriers_json(conn, airport)
}
