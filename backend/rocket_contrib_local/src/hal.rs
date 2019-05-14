//! A module providing a HalResource wrapper that implements the Rocket Responder
//! traits and maintains a nicely typed form.

use rocket::{http::Status, response::Responder, Request, Response};
use rocket_contrib::json::Json;
use std::marker::PhantomData;

/// A wrapper for a type represented as a HalResource.
pub struct HalResource<T: serde::ser::Serialize>(pub T, pub fn(&T) -> Json<T>);

impl<T> HalResource<T>
where
    T: serde::ser::Serialize,
{
    /// Consumes the HalResource wrapper and returns the wrapped item.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Convert the type wrapped by HalResource via the supplied `convertor` function.
    pub fn convert(&self) -> Json<T> {
        (self.1)(&self.0)
    }
}

impl<T> Responder<'static> for HalResource<T>
where
    T: serde::ser::Serialize,
{
    fn respond_to(self, req: &Request) -> Result<Response<'static>, Status> {
        self.convert().respond_to(req)
    }
}
