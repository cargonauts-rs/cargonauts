mod debug;
mod display;
pub mod jsonapi;

pub use self::debug::{SimpleDebug as Debug};
pub use self::display::{SimpleDisplay as Display};
pub use self::jsonapi::JsonApi;

use futures::future::{self, FutureResult};

use rigging::{ResourceEndpoint, Error};
use rigging::environment::Environment;
use rigging::format::Receive;
use rigging::http;
use rigging::request::Request;

pub struct BasicReceiver;

impl<T, R> Receive<T, R> for BasicReceiver
where
    T: ResourceEndpoint,
    R: Request<T, BodyParts = ()>,
{
    type Future = FutureResult<R::BodyParts, Error>;
    fn receive(_: http::Request, _: &Environment) -> Self::Future {
        future::ok(())
    }
}
