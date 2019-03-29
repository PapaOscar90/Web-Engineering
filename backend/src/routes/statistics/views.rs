//! A module containing the statistics structures returned by the API.

use serde::Deserialize;
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

/// A set of statistics specifically on flights as returned by the API.
#[derive(Serialize)]
pub struct StatisticsFlights {
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

    /// The time.
    pub time: chrono::NaiveDate,

    /// The id of the [carrier](Carrier).
    pub carrier_id: i64,

    /// The id of the [airport](Airport).
    pub airport_id: i64,
}

impl From<crate::database::models::Statistics> for StatisticsFlights {
    fn from(source: crate::database::models::Statistics) -> Self {
        Self {
            id: source.id,
            flights_cancelled: source.flights_cancelled,
            flights_delayed: source.flights_delayed,
            flights_diverted: source.flights_diverted,
            flights_on_time: source.flights_on_time,
            flights_total: source.flights_total,
            time: source.time,
            carrier_id: source.carrier_id,
            airport_id: source.airport_id,
        }
    }
}

/// A set of minutes delayed statistics as returned by the API.
#[derive(Serialize)]
pub struct StatisticsMinutesDelayed {
    /// The id for a set of statistics.
    pub id: i64,

    /// The minutes delayed due to the carrier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_delayed_carrier: Option<i64>,

    /// The minutes delayed due to late aircraft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_delayed_late_aircraft: Option<i64>,

    /// The minutes delayed due to the national aviation system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_delayed_national_aviation_system: Option<i64>,

    /// The minutes delayed due to the security.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_delayed_security: Option<i64>,

    /// The minutes delayed due to weather.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_delayed_weather: Option<i64>,

    /// The total minutes delayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_delayed_total: Option<i64>,

    /// The time.
    pub time: chrono::NaiveDate,

    /// The id of the [carrier](Carrier).
    pub carrier_id: i64,

    /// The id of the [airport](Airport).
    pub airport_id: i64,
}

impl From<crate::database::models::Statistics> for StatisticsMinutesDelayed {
    fn from(source: crate::database::models::Statistics) -> Self {
        Self {
            id: source.id,
            minutes_delayed_carrier: Some(source.minutes_delayed_carrier),
            minutes_delayed_late_aircraft: Some(source.minutes_delayed_late_aircraft),
            minutes_delayed_national_aviation_system: Some(
                source.minutes_delayed_national_aviation_system,
            ),
            minutes_delayed_security: Some(source.minutes_delayed_security),
            minutes_delayed_weather: Some(source.minutes_delayed_weather),
            minutes_delayed_total: Some(source.minutes_delayed_total),
            time: source.time,
            carrier_id: source.carrier_id,
            airport_id: source.airport_id,
        }
    }
}

/// A set of statistics with all the information needed for insertion. In this
/// instance the `id` for the statistics is generated upon insertion, and the
/// `flights_total` and `minutes_delayed_total` are derived values.
#[derive(Deserialize)]
pub struct NewStatistics {
    /// See [Statistics::flights_cancelled].
    pub flights_cancelled: i64,

    /// See [Statistics::flights_delayed].
    pub flights_delayed: i64,

    /// See [Statistics::flights_diverted].
    pub flights_diverted: i64,

    /// See [Statistics::flights_on_time].
    pub flights_on_time: i64,

    /// See [Statistics::minutes_delayed_carrier].
    pub minutes_delayed_carrier: i64,

    /// See [Statistics::minutes_delayed_late_aircraft].
    pub minutes_delayed_late_aircraft: i64,

    /// See [Statistics::minutes_delayed_national_aviation_system].
    pub minutes_delayed_national_aviation_system: i64,

    /// See [Statistics::minutes_delayed_security].
    pub minutes_delayed_security: i64,

    /// See [Statistics::minutes_delayed_weather].
    pub minutes_delayed_weather: i64,

    /// See [Statistics::number_of_delays_carrier].
    pub number_of_delays_carrier: i64,

    /// See [Statistics::number_of_delays_late_aircraft].
    pub number_of_delays_late_aircraft: i64,

    /// See [Statistics::number_of_delays_national_aviation_system].
    pub number_of_delays_national_aviation_system: i64,

    /// See [Statistics::number_of_delays_security].
    pub number_of_delays_security: i64,

    /// See [Statistics::number_of_delays_weather].
    pub number_of_delays_weather: i64,

    /// See [Statistics::time].
    pub time: chrono::NaiveDate,

    /// See [Statistics::carrier_id].
    pub carrier_id: i64,

    /// See [Statistics::airport_id].
    pub airport_id: i64,
}

// impl<'a> Into<crate::database::models::NewStatistics<'a>> for NewStatistics {
//     fn into(self) -> crate::database::models::NewStatistics<'a> {
//         crate::database::models::NewStatistics {
//             flights_cancelled: &self.flights_cancelled,
//             flights_delayed: &self.flights_delayed,
//             flights_diverted: &self.flights_diverted,
//             flights_on_time: &self.flights_on_time,
//             minutes_delayed_carrier: &self.minutes_delayed_carrier,
//             minutes_delayed_late_aircraft: &self.minutes_delayed_late_aircraft,
//             minutes_delayed_national_aviation_system: &self
//                 .minutes_delayed_national_aviation_system,
//             minutes_delayed_security: &self.minutes_delayed_security,
//             minutes_delayed_weather: &self.minutes_delayed_weather,
//             number_of_delays_carrier: &self.number_of_delays_carrier,
//             number_of_delays_late_aircraft: &self.number_of_delays_late_aircraft,
//             number_of_delays_national_aviation_system: &self
//                 .number_of_delays_national_aviation_system,
//             number_of_delays_security: &self.number_of_delays_security,
//             number_of_delays_weather: &self.number_of_delays_weather,
//             time: &self.time,
//             carrier_id: &self.carrier_id,
//             airport_id: &self.airport_id,
//         }
//     }
// }
