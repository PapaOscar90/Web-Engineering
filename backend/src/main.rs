#![deny(missing_docs)]
#![feature(proc_macro_hygiene, decl_macro)]

//! # Web Engineering Backend - Documentation
//!
//! This is a binary project that provides access to the [CORGIS Airlines data-set](https://think.cs.vt.edu/corgis/json/airlines/airlines.html). It uses [rocket](https://rocket.rs) to launch a web-server, with routes implementation the required features. It uses the `corgis` sub-crate to get initial access to the airlines data-set.

pub mod routes;

/// Prepare the rocket web-server for launch:
///    1. Mount each end-point to the root.
///    2. Add managed state containing the data-set.
pub fn rocket() -> rocket::Rocket {
    routes::mount(rocket::ignite())
}

/// Rocket launch in T-10...
pub fn main() {
    rocket().launch();
}
