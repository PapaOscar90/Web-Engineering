//! A module providing a Csv wrapper that implements the Rocket Responder
//! traits.

use derive_new::new;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;
use std::str::FromStr;

/// A wrapper for a type represented as CSV
#[derive(new)]
pub struct Csv<T> {
    /// The data contained within a Csv wrapper
    pub data: T,
    convertor: fn(&T) -> String,
}

impl<T> Csv<T> {
    // pub fn new(data: T, convertor: fn(&T) -> String) -> Self {
    //     Self { data, convertor }
    // }

    /// Consumes the CSV wrapper and returns the wrapped item.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Convert the type wrapped by Csv via the supplied `convertor` function.
    pub fn convert(&self) -> String {
        (self.convertor)(&self.data)
    }
}

impl<T> Responder<'static> for Csv<T> {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::CSV)
            .sized_body(Cursor::new(self.convert()))
            .ok()
    }
}
