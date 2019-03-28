//! A module defining the `get_carrier` routes.

use super::views::Statistics;

use crate::CorgisDbConn;
use diesel::{prelude::*, result::Error};
use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn get_statistics_data(conn: &diesel::PgConnection, id: i64) -> Result<Option<Statistics>, Error> {
    use crate::database::models;
    use crate::database::schema::statistics::dsl::statistics;

    Ok(statistics
        .find(id)
        .first::<models::Statistics>(conn)
        .optional()?
        .map(Statistics::from))
}

/// Get the JSON representation of a set of statistics in the database.
#[get("/<statistics>", format = "application/json", rank = 1)]
pub fn get_statistics_json(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Json<Statistics>>, Error> {
    Ok(get_statistics_data(&conn, statistics)?.map(Json))
}

/// Get the CSV representation of a set of statistics in the database.
#[get("/<statistics>", format = "text/csv", rank = 2)]
pub fn get_statistics_csv(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Csv<Statistics>>, Error> {
    fn convertor(statistics: &Statistics) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        wtr.serialize(statistics).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    Ok(get_statistics_data(&conn, statistics)?.map(|data| Csv(data, convertor)))
}

/// Get the HAL representation of a set of statistics in the database.
#[get("/<statistics>", format = "application/hal+json", rank = 3)]
pub fn get_statistics_hal(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Json<HalResource>>, Error> {
    let data = get_statistics_data(&conn, statistics)?;
    match data {
        None => Ok(None),
        Some(data) => {
            let result = HalResource::new(&data)
                .with_link("self", format!("/statistics/{}", data.id))
                .with_link("delete", format!("/statistics/{}", data.id))
                .with_link("patch", format!("/statistics/{}", data.id))
                .with_link("put", format!("/statistics/{}", data.id));

            Ok(Some(Json(result)))
        }
    }
}

/// Get the default representation of a set of statistics in the data store. This is
/// executed if the other routes are not matched.
#[get("/<statistics>", rank = 4)]
pub fn get_statistics_default(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Json<Statistics>>, diesel::result::Error> {
    get_statistics_json(conn, statistics)
}
