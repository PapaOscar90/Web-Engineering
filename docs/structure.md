```backend/src
├── bin
│   └── dump_db.rs
├── database
│   ├── models.rs
│   ├── mod.rs
│   └── schema.rs
├── lib.rs
├── main.rs
└── routes
    ├── airports
    │   ├── get_airport.rs
    │   ├── get_airports.rs
    │   ├── mod.rs
    │   └── views.rs
    ├── carriers
    │   ├── get_carrier.rs
    │   ├── get_carriers.rs
    │   ├── mod.rs
    │   └── views.rs
    ├── mod.rs
    └── statistics
        ├── delete_statistic.rs
        ├── get_statistic.rs
        ├── get_statistics_connection.rs
        ├── get_statistics_flights.rs
        ├── get_statistics_minutes_delayed.rs
        ├── get_statistics.rs
        ├── mod.rs
        ├── patch_statistic.rs
        ├── post_statistics.rs
        ├── put_statistic.rs
        └── views.rs
backend/corgis_dataset
├── Cargo.lock
├── Cargo.toml
├── examples
│   └── airlines_print_airlines_and_carriers.rs
└── src
    ├── airlines
    │   ├── airlines.json
    │   └── mod.rs
    └── lib.rs
backend/rocket_contrib_local
├── Cargo.lock
├── Cargo.toml
└── src
    ├── csv.rs
    ├── hal.rs
    └── lib.rs
backend/migrations
├── 00000000000000_diesel_initial_setup
│   ├── down.sql
│   └── up.sql
├── 2019-03-27-102415_create_airports
│   ├── down.sql
│   └── up.sql
├── 2019-03-27-102534_create_carriers
│   ├── down.sql
│   └── up.sql
└── 2019-03-27-102629_create_statistics
    ├── down.sql
    └── up.sql
    ```