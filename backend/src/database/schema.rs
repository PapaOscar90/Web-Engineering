table! {
    airports (id) {
        id -> BigInt,
        code -> Text,
        name -> Text,
    }
}

table! {
    carriers (id) {
        id -> BigInt,
        code -> Text,
        name -> Text,
    }
}

table! {
    statistics (id) {
        id -> BigInt,
        flights_cancelled -> BigInt,
        flights_delayed -> BigInt,
        flights_diverted -> BigInt,
        flights_on_time -> BigInt,
        flights_total -> BigInt,
        minutes_delayed_carrier -> BigInt,
        minutes_delayed_late_aircraft -> BigInt,
        minutes_delayed_national_aviation_system -> BigInt,
        minutes_delayed_security -> BigInt,
        minutes_delayed_weather -> BigInt,
        minutes_delayed_total -> BigInt,
        number_of_delays_carrier -> BigInt,
        number_of_delays_late_aircraft -> BigInt,
        number_of_delays_national_aviation_system -> BigInt,
        number_of_delays_security -> BigInt,
        number_of_delays_weather -> BigInt,
        time -> Date,
        carrier_id -> BigInt,
        airport_id -> BigInt,
    }
}

joinable!(statistics -> airports (airport_id));
joinable!(statistics -> carriers (carrier_id));

allow_tables_to_appear_in_same_query!(airports, carriers, statistics,);
