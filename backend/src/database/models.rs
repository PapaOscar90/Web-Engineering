//! Data structures used to interact with, retrieve data from, and insert data
//! into the database.

use super::schema::{airports, carriers, statistics};

/// An airport as represented in the database.
#[derive(Identifiable, Queryable)]
pub struct Airport {
    /// The primary key for an airport.
    pub id: i64,

    /// A unique 3 letter code for the airport as assigned by the International
    /// Air Transport Association.
    pub code: String,

    /// The full name of the airport.
    pub name: String,
}

/// An airport with all the information needed for insertion. In this instance
/// the `id` for the airport is generated upon insertion.
#[derive(Insertable)]
#[table_name = "airports"]
pub struct NewAirport<'a> {
    /// See [Airport::code].
    pub code: &'a str,

    /// See [Airport::name].
    pub name: &'a str,
}

/// A carrier as represented in the database.
#[derive(Identifiable, Queryable)]
pub struct Carrier {
    /// The primary key for a carrier.
    pub id: i64,

    /// A non-unique 2 letter code for the carrier as assigned by the
    /// International Air Transport Association.
    pub code: String,

    /// The full name of the carrier.
    pub name: String,
}

/// A carrier with all the information needed for insertion. In this instance
/// the `id` for the carrier is generated upon insertion.
#[derive(Insertable)]
#[table_name = "carriers"]
pub struct NewCarrier<'a> {
    /// See [Carrier::code].
    pub code: &'a str,

    /// See [Carrier::name].
    pub name: &'a str,
}

/// The statistics as represented in the database. Each set of statistics is for
/// a carrier and an airport at a specific time.
#[derive(Debug, Identifiable, Queryable)]
#[table_name = "statistics"]
pub struct Statistics {
    /// The primary key for the statistics
    pub id: i64,

    /// The number of flights cancelled.
    pub flights_cancelled: i64,

    /// The number of flights delayed.
    pub flights_delayed: i64,

    /// The number of flights diverted.
    pub flights_diverted: i64,

    /// The number of flights on time.
    pub flights_on_time: i64,

    /// The total number of flights.
    pub flights_total: i64,

    /// The minutes delayed due to the carrier.
    pub minutes_delayed_carrier: i64,

    /// The minutes delayed due to late aircraft.
    pub minutes_delayed_late_aircraft: i64,

    /// The minutes delayed due to the national aviation system.
    pub minutes_delayed_national_aviation_system: i64,

    /// The minutes delayed due to the security.
    pub minutes_delayed_security: i64,

    /// The minutes delayed due to weather.
    pub minutes_delayed_weather: i64,

    /// The total minutes delayed due.
    pub minutes_delayed_total: i64,

    /// The number of delays due to the carrier.
    pub number_of_delays_carrier: i64,

    /// The number of delays due to late aircraft.
    pub number_of_delays_late_aircraft: i64,

    /// The number of delays due to the national aviation system.
    pub number_of_delays_national_aviation_system: i64,

    /// The number of delays due to security.
    pub number_of_delays_security: i64,

    /// The number of delays due to the weather.
    pub number_of_delays_weather: i64,

    /// The time.
    pub time: chrono::NaiveDate,

    /// The id of the [carrier](Carrier).
    pub carrier_id: i64,

    /// The id of the [airport](Airport).
    pub airport_id: i64,
}

/// A set of statistics with all the information needed for insertion. In this
/// instance the `id` for the statistics is generated upon insertion, and the
/// `flights_total` and `minutes_delayed_total` are derived values.
#[derive(Insertable)]
#[table_name = "statistics"]
pub struct NewStatistics<'a> {
    /// See [Statistics::flights_cancelled].
    pub flights_cancelled: &'a i64,

    /// See [Statistics::flights_delayed].
    pub flights_delayed: &'a i64,

    /// See [Statistics::flights_diverted].
    pub flights_diverted: &'a i64,

    /// See [Statistics::flights_on_time].
    pub flights_on_time: &'a i64,

    /// See [Statistics::minutes_delayed_carrier].
    pub minutes_delayed_carrier: &'a i64,

    /// See [Statistics::minutes_delayed_late_aircraft].
    pub minutes_delayed_late_aircraft: &'a i64,

    /// See [Statistics::minutes_delayed_national_aviation_system].
    pub minutes_delayed_national_aviation_system: &'a i64,

    /// See [Statistics::minutes_delayed_security].
    pub minutes_delayed_security: &'a i64,

    /// See [Statistics::minutes_delayed_weather].
    pub minutes_delayed_weather: &'a i64,

    /// See [Statistics::number_of_delays_carrier].
    pub number_of_delays_carrier: &'a i64,

    /// See [Statistics::number_of_delays_late_aircraft].
    pub number_of_delays_late_aircraft: &'a i64,

    /// See [Statistics::number_of_delays_national_aviation_system].
    pub number_of_delays_national_aviation_system: &'a i64,

    /// See [Statistics::number_of_delays_security].
    pub number_of_delays_security: &'a i64,

    /// See [Statistics::number_of_delays_weather].
    pub number_of_delays_weather: &'a i64,

    /// See [Statistics::time].
    pub time: &'a chrono::NaiveDate,

    /// See [Statistics::carrier_id].
    pub carrier_id: &'a i64,

    /// See [Statistics::airport_id].
    pub airport_id: &'a i64,
}

/// A set of statistics with all the information needed for updating statistics.
#[derive(AsChangeset)]
#[table_name = "statistics"]
pub struct UpdateStatistics<'a> {
    /// See [Statistics::flights_cancelled].
    pub flights_cancelled: Option<&'a i64>,

    /// See [Statistics::flights_delayed].
    pub flights_delayed: Option<&'a i64>,

    /// See [Statistics::flights_diverted].
    pub flights_diverted: Option<&'a i64>,

    /// See [Statistics::flights_on_time].
    pub flights_on_time: Option<&'a i64>,

    /// See [Statistics::minutes_delayed_carrier].
    pub minutes_delayed_carrier: Option<&'a i64>,

    /// See [Statistics::minutes_delayed_late_aircraft].
    pub minutes_delayed_late_aircraft: Option<&'a i64>,

    /// See [Statistics::minutes_delayed_national_aviation_system].
    pub minutes_delayed_national_aviation_system: Option<&'a i64>,

    /// See [Statistics::minutes_delayed_security].
    pub minutes_delayed_security: Option<&'a i64>,

    /// See [Statistics::minutes_delayed_weather].
    pub minutes_delayed_weather: Option<&'a i64>,

    /// See [Statistics::number_of_delays_carrier].
    pub number_of_delays_carrier: Option<&'a i64>,

    /// See [Statistics::number_of_delays_late_aircraft].
    pub number_of_delays_late_aircraft: Option<&'a i64>,

    /// See [Statistics::number_of_delays_national_aviation_system].
    pub number_of_delays_national_aviation_system: Option<&'a i64>,

    /// See [Statistics::number_of_delays_security].
    pub number_of_delays_security: Option<&'a i64>,

    /// See [Statistics::number_of_delays_weather].
    pub number_of_delays_weather: Option<&'a i64>,
}
