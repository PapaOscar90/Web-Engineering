//! A module defining the `delete_statistic` routes.

use super::views::Statistics;

use crate::CorgisDbConn;
use diesel::{prelude::*, result::Error};
use rocket::delete;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

fn delete_statistics_data(
    conn: &diesel::PgConnection,
    id: i64,
) -> Result<Option<Statistics>, Error> {
    use crate::database::models;
    use crate::database::schema::statistics::dsl::statistics;

    Ok(diesel::delete(statistics.find(id))
        .get_result::<models::Statistics>(conn)
        .optional()?
        .map(Statistics::from))
}

/// Get the JSON representation of the deleted set of statistics in the
/// database.
#[delete("/<statistics>", format = "application/json", rank = 1)]
pub fn delete_statistics_json(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Json<Statistics>>, Error> {
    Ok(delete_statistics_data(&conn, statistics)?.map(Json))
}

/// Get the CSV representation of the deleted set of statistics in the database.
#[delete("/<statistics>", format = "text/csv", rank = 2)]
pub fn delete_statistics_csv(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Csv<Statistics>>, Error> {
    fn convertor(statistics: &Statistics) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        wtr.serialize(statistics).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    Ok(delete_statistics_data(&conn, statistics)?.map(|data| Csv(data, convertor)))
}

/// Get the HAL representation of the deleted set of statistics in the database.
#[delete("/<statistics>", format = "application/hal+json", rank = 3)]
pub fn delete_statistics_hal(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Json<HalResource>>, Error> {
    let data = delete_statistics_data(&conn, statistics)?;
    match data {
        None => Ok(None),
        Some(data) => {
            let result = HalResource::new(&data).with_link("super", "/statistics");

            Ok(Some(Json(result)))
        }
    }
}

/// Get the default representation of the deleted set of statistics in the data
/// store. This is executed if the other routes are not matched.
#[delete("/<statistics>", rank = 4)]
pub fn delete_statistics_default(
    conn: CorgisDbConn,
    statistics: i64,
) -> Result<Option<Json<Statistics>>, diesel::result::Error> {
    delete_statistics_json(conn, statistics)
}
