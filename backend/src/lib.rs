#![deny(missing_docs)]
#![feature(custom_attribute)]

//! # Web Engineering Backend - Documentation
//!
//! This is a lib project used by the `backend` that provides access to the [CORGIS Airlines data-set](https://think.cs.vt.edu/corgis/json/airlines/airlines.html). This is primarily to facilitate external access to the database for scripts.

#[macro_use]
pub extern crate diesel;

pub mod database;
