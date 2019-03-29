//! A module dealing with the `get_statistics_minutes_delayed` routes.

use super::views::StatisticsMinutesDelayed;

use crate::CorgisDbConn;
use diesel::{dsl::*, prelude::*, result::Error};
use rayon::prelude::*;
use rocket::get;
use rocket_contrib::json::Json;
use rocket_contrib_local::csv::Csv;
use rustic_hal::HalResource;

const LIMIT: usize = 100;
const OFFSET: usize = 0;

fn get_statistics_minutes_delayed_data(
    conn: &diesel::PgConnection,
    carrier: Option<i64>,
    airport: Option<i64>,
    month: Option<u32>,
    reason: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<StatisticsMinutesDelayed>, Error> {
    use crate::database::{models, schema};
    use chrono::Datelike;

    let offset = offset.unwrap_or(OFFSET) as i64;
    let limit = limit.unwrap_or(LIMIT) as i64;

    let statistics: Vec<models::Statistics> = match (carrier, airport) {
        (None, None) => match schema::statistics::table
            .limit(limit)
            .offset(offset)
            .load(conn)
            .optional()?
        {
            Some(statistics_data) => statistics_data,
            None => return Ok(Vec::new()),
        },
        (None, Some(airport)) => {
            let airport_id: i64 = match schema::airports::table
                .find(airport)
                .select(schema::airports::id)
                .first(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(airport_id) => airport_id,
            };
            match schema::statistics::table
                .filter(schema::statistics::airport_id.eq(airport_id))
                .limit(limit)
                .offset(offset)
                .load::<models::Statistics>(conn)
                .optional()?
            {
                Some(data) => data,
                None => Vec::new(),
            }
        }
        (Some(carrier), None) => {
            let carrier_id: i64 = match schema::carriers::table
                .find(carrier)
                .select(schema::carriers::id)
                .first(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(carrier_id) => carrier_id,
            };
            match schema::statistics::table
                .filter(schema::statistics::carrier_id.eq(carrier_id))
                .limit(limit)
                .offset(offset)
                .load::<models::Statistics>(conn)
                .optional()?
            {
                Some(data) => data,
                None => Vec::new(),
            }
        }
        (Some(carrier), Some(airport)) => {
            let airport_id: i64 = match schema::airports::table
                .find(airport)
                .select(schema::airports::id)
                .first(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(airport_id) => airport_id,
            };

            let carrier_id: i64 = match schema::carriers::table
                .find(carrier)
                .select(schema::carriers::id)
                .first(conn)
                .optional()?
            {
                None => return Ok(Vec::new()),
                Some(carrier_id) => carrier_id,
            };

            match schema::statistics::table
                .filter(
                    schema::statistics::airport_id
                        .eq(airport_id)
                        .and(schema::statistics::carrier_id.eq(carrier_id)),
                )
                .limit(limit)
                .offset(offset)
                .load::<models::Statistics>(conn)
                .optional()?
            {
                Some(data) => data,
                None => Vec::new(),
            }
        }
    };

    let statistics = match month {
        Some(month) => statistics
            .into_par_iter()
            .filter(|record| record.time.month() == month)
            .collect(),
        None => statistics,
    };

    Ok(statistics
        .into_par_iter()
        .map(|statistics| {
            let mut statistics = StatisticsMinutesDelayed::from(statistics);
            if let Some(reason) = reason.clone() {
                match reason.as_str() {
                    "carrier_specific" => {
                        statistics.minutes_delayed_national_aviation_system = None;
                        statistics.minutes_delayed_security = None;
                        statistics.minutes_delayed_weather = None;
                        statistics.minutes_delayed_total = None;
                    }
                    _ => (),
                }
            }
            statistics
        })
        .collect())
}

/// Get the JSON representation of the minutes delayed statistics in the database.
#[get(
    "/minutes-delayed?<carrier>&<airport>&<month>&<reason>&<limit>&<offset>",
    format = "application/json",
    rank = 1
)]
pub fn get_statistics_minutes_delayed_json(
    conn: CorgisDbConn,
    carrier: Option<i64>,
    airport: Option<i64>,
    month: Option<u32>,
    reason: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Json<Vec<StatisticsMinutesDelayed>>, Error> {
    get_statistics_minutes_delayed_data(&conn, carrier, airport, month, reason, limit, offset)
        .map(Json)
}

/// Get the CSV representation of the minutes_delayed statistics in the database.
#[get(
    "/minutes-delayed?<carrier>&<airport>&<month>&<reason>&<limit>&<offset>",
    format = "text/csv",
    rank = 2
)]
pub fn get_statistics_minutes_delayed_csv(
    conn: CorgisDbConn,
    carrier: Option<i64>,
    airport: Option<i64>,
    month: Option<u32>,
    reason: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Csv<Vec<StatisticsMinutesDelayed>>, Error> {
    fn convertor(statistics_set: &Vec<StatisticsMinutesDelayed>) -> String {
        let mut wtr = csv::WriterBuilder::default().from_writer(Vec::new());
        for statistics in statistics_set {
            wtr.serialize(statistics).unwrap();
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    };

    get_statistics_minutes_delayed_data(&conn, carrier, airport, month, reason, limit, offset)
        .map(|data| Csv(data, convertor))
}

/// Get the HAL representation of the minutes delayed statistics in the database.
#[get(
    "/minutes-delayed?<carrier>&<airport>&<month>&<reason>&<limit>&<offset>",
    format = "application/hal+json",
    rank = 3
)]
pub fn get_statistics_minutes_delayed_hal(
    conn: CorgisDbConn,
    carrier: Option<i64>,
    airport: Option<i64>,
    month: Option<u32>,
    reason: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Json<Vec<HalResource>>, Error> {
    let result =
        get_statistics_minutes_delayed_data(&conn, carrier, airport, month, reason, limit, offset)?
            .into_par_iter()
            .map(|data| {
                HalResource::new(&data).with_link("super", format!("/statistics/{}", data.id))
            })
            .collect();

    Ok(Json(result))
}

/// Get the default representation of the minutes delayed statistics in the data store. This is
/// executed if the other routes are not matched.
#[get(
    "/minutes-delayed?<carrier>&<airport>&<month>&<reason>&<limit>&<offset>",
    rank = 4
)]
pub fn get_statistics_minutes_delayed_default(
    conn: CorgisDbConn,
    carrier: Option<i64>,
    airport: Option<i64>,
    month: Option<u32>,
    reason: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Json<Vec<StatisticsMinutesDelayed>>, diesel::result::Error> {
    get_statistics_minutes_delayed_json(conn, carrier, airport, month, reason, limit, offset)
}
