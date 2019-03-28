extern crate rocket_corgis;

use corgis_dataset::airlines;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use log::info;
use rocket_corgis::database::*;

fn main() -> Result<(), diesel::result::Error> {
    // Initialize the logger.
    pretty_env_logger::formatted_builder()
        .default_format()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Establishing connection to database");
    // Establish a connection to the database.
    let connection = establish_connection();

    info!("Loading CORGIS data-set into memory");
    // Generate the data-set from the CORGIS data.
    let data_set = airlines::DataSet::new();

    info!(
        "Inserting {} carriers from CORGIS data-set into database",
        data_set.carriers().count()
    );
    // Insert each carrier into the database.
    for carrier in data_set.carriers() {
        let carrier = models::NewCarrier {
            code: &carrier.code,
            name: &carrier.name,
        };
        match create_carrier(&connection, carrier) {
            Ok(_) => (),
            Err(DatabaseError(UniqueViolation, _)) => (),
            Err(e) => panic!("{:#?}", e),
        }
    }

    info!(
        "Inserting {} airports from CORGIS data-set into database",
        data_set.airports().count()
    );
    // Insert each airport into the database.
    for airport in data_set.airports() {
        let airport = models::NewAirport {
            code: &airport.code,
            name: &airport.name,
        };
        match create_airport(&connection, airport) {
            Ok(_) => (),
            Err(DatabaseError(UniqueViolation, _)) => (),
            Err(e) => panic!("{:#?}", e),
        }
    }

    info!(
        "Inserting {} statistics from CORGIS data-set into database",
        data_set.records().count()
    );
    // Insert each statistic into the database.
    for record in data_set.records() {
        use schema::{airports, carriers};

        // Retrieve the id of the airport.
        let airport_id: i64 = airports::table
            .select(airports::id)
            .filter(airports::code.eq(&record.airport.code))
            .filter(airports::name.eq(&record.airport.name))
            .first(&connection)
            .expect("Error retrieving airport");

        // Retrieve the id of the carrier.
        let carrier_id: i64 = carriers::table
            .select(carriers::id)
            .filter(carriers::code.eq(&record.carrier.code))
            .filter(carriers::name.eq(&record.carrier.name))
            .first(&connection)
            .expect("Error retrieving carrier");

        // Covert the time to the `chrono` data type. No day is provided in the data so 1 is chosen.
        let time = chrono::NaiveDate::from_ymd(record.time.year, record.time.month, 1);

        // Extract the flights statistics.
        let corgis_dataset::airlines::Flights {
            cancelled: flights_cancelled,
            delayed: flights_delayed,
            diverted: flights_diverted,
            on_time: flights_on_time,
            ..
        } = record.statistics.flights;

        // Extract the minutes delayed.
        let corgis_dataset::airlines::MinutesDelayed {
            carrier: minutes_delayed_carrier,
            late_aircraft: minutes_delayed_late_aircraft,
            national_aviation_system: minutes_delayed_national_aviation_system,
            security: minutes_delayed_security,
            weather: minutes_delayed_weather,
            ..
        } = record.statistics.minutes_delayed;

        // Extract the number of delays.
        let corgis_dataset::airlines::NumberOfDelays {
            carrier: number_of_delays_carrier,
            late_aircraft: number_of_delays_late_aircraft,
            national_aviation_system: number_of_delays_national_aviation_system,
            security: number_of_delays_security,
            weather: number_of_delays_weather,
        } = record.statistics.number_of_delays;

        let new_statistics = models::NewStatistics {
            flights_cancelled: &flights_cancelled,
            flights_delayed: &flights_delayed,
            flights_diverted: &flights_diverted,
            flights_on_time: &flights_on_time,
            minutes_delayed_carrier: &minutes_delayed_carrier,
            minutes_delayed_late_aircraft: &minutes_delayed_late_aircraft,
            minutes_delayed_national_aviation_system: &minutes_delayed_national_aviation_system,
            minutes_delayed_security: &minutes_delayed_security,
            minutes_delayed_weather: &minutes_delayed_weather,
            number_of_delays_carrier: &number_of_delays_carrier,
            number_of_delays_late_aircraft: &number_of_delays_late_aircraft,
            number_of_delays_national_aviation_system: &number_of_delays_national_aviation_system,
            number_of_delays_security: &number_of_delays_security,
            number_of_delays_weather: &number_of_delays_weather,
            time: &time,
            carrier_id: &carrier_id,
            airport_id: &airport_id,
        };

        // Insert the statistics.
        match create_statistics(&connection, new_statistics) {
            Ok(_) => (),
            Err(DatabaseError(UniqueViolation, _)) => (),
            Err(e) => panic!("{:#?}", e),
        }
    }

    info!("CORGIS data-set inserted successfully into database");
    Ok(())
}
