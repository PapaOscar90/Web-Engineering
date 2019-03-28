//! A module providing a Csv wrapper that implements the Rocket Responder
//! traits.

use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};
use std::io::Cursor;

/// A wrapper for a type represented as CSV.
pub struct Csv<T>(pub T, pub fn(&T) -> String);

impl<T> Csv<T> {
    /// Consumes the CSV wrapper and returns the wrapped item.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Convert the type wrapped by Csv via the supplied `convertor` function.
    pub fn convert(&self) -> String {
        (self.1)(&self.0)
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
