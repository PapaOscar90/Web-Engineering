//! A module containing the statistics structures returned by the API.

use serde::Serialize;

/// A set of statistics as returned by the API.
#[derive(Serialize)]
pub struct Statistics {
    /// The id for a set of statistics.
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

impl From<crate::database::models::Statistics> for Statistics {
    fn from(source: crate::database::models::Statistics) -> Self {
        Self {
            id: source.id,
            flights_cancelled: source.flights_cancelled,
            flights_delayed: source.flights_delayed,
            flights_diverted: source.flights_diverted,
            flights_on_time: source.flights_on_time,
            flights_total: source.flights_total,
            minutes_delayed_carrier: source.minutes_delayed_carrier,
            minutes_delayed_late_aircraft: source.minutes_delayed_late_aircraft,
            minutes_delayed_national_aviation_system: source
                .minutes_delayed_national_aviation_system,
            minutes_delayed_security: source.minutes_delayed_security,
            minutes_delayed_weather: source.minutes_delayed_weather,
            minutes_delayed_total: source.minutes_delayed_total,
            number_of_delays_carrier: source.number_of_delays_carrier,
            number_of_delays_late_aircraft: source.number_of_delays_late_aircraft,
            number_of_delays_national_aviation_system: source
                .number_of_delays_national_aviation_system,
            number_of_delays_security: source.number_of_delays_security,
            number_of_delays_weather: source.number_of_delays_weather,
            time: source.time,
            carrier_id: source.carrier_id,
            airport_id: source.airport_id,
        }
    }
}
