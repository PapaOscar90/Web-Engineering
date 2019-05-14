#![deny(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

//! # Web Engineering Backend - Documentation
//!
//! This is a binary project that provides access to the [CORGIS Airlines data-set](https://think.cs.vt.edu/corgis/json/airlines/airlines.html). It uses [Rocket](https://rocket.rs) to launch a web-server, with routes implementation the required features. The data has been dumped into a database after being parsed by the `corgis` sub-crate.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

mod database;
mod routes;
mod util;

/// A struct with a database fairing allowing the database to be attached to the rocket.
#[database("corgis_airlines")]
pub struct CorgisDbConn(diesel::pg::PgConnection);

/// Prepare the rocket web-server for launch:
///    1. Mount each end-point to the root.
///    2. Attach the database.
///    3. Attach the CORS fairing.
pub fn rocket() -> rocket::Rocket {
    routes::mount(rocket::ignite())
        .attach(CorgisDbConn::fairing())
        .attach(util::cors())
}

/// Rocket launch in T-10...
fn main() {
    rocket().launch();
}
