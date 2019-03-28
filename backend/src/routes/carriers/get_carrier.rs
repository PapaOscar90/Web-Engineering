//! A module defining the `get_carrier` routes.

use super::views::Carrier;

use crate::CorgisDbConn;
use diesel::{prelude::*, result::Error};
use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn get_carrier_data(conn: &diesel::PgConnection, id: i64) -> Result<Option<Carrier>, Error> {
    use crate::database::models;
    use crate::database::schema::carriers::dsl::carriers;

    Ok(carriers
        .find(id)
        .first::<models::Carrier>(conn)
        .optional()?
        .map(Carrier::from))
}

/// Get the JSON representation of a carrier in the database.
#[get("/<carrier>", format = "application/json", rank = 1)]
pub fn get_carrier_json(conn: CorgisDbConn, carrier: i64) -> Result<Option<Json<Carrier>>, Error> {
    Ok(get_carrier_data(&conn, carrier)?.map(Json))
}

/// Get the CSV representation of a carrier in the database.
#[get("/<carrier>", format = "text/csv", rank = 2)]
pub fn get_carrier_csv(conn: CorgisDbConn, carrier: i64) -> Result<Option<Csv<Carrier>>, Error> {
    fn convertor(carrier: &Carrier) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        wtr.serialize(carrier).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    Ok(get_carrier_data(&conn, carrier)?.map(|data| Csv(data, convertor)))
}

/// Get the HAL representation of a carrier in the database.
#[get("/<carrier>", format = "application/hal+json", rank = 3)]
pub fn get_carrier_hal(
    conn: CorgisDbConn,
    carrier: i64,
) -> Result<Option<Json<HalResource>>, Error> {
    let data = get_carrier_data(&conn, carrier)?;
    match data {
        None => Ok(None),
        Some(data) => {
            let result = HalResource::new(&data)
                .with_link("self", format!("/carriers/{}", data.id))
                .with_link("carriers", format!("/airports?carrier={}", data.id))
                .with_link("statistics", format!("/statistics?carrier={}", data.id));

            Ok(Some(Json(result)))
        }
    }
}

/// Get the default representation of a carrier in the data store. This is
/// executed if the other routes are not matched.
#[get("/<carrier>", rank = 4)]
pub fn get_carrier_default(
    conn: CorgisDbConn,
    carrier: i64,
) -> Result<Option<Json<Carrier>>, diesel::result::Error> {
    get_carrier_json(conn, carrier)
}
