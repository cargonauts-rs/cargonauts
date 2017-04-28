mod debug;
mod display;

pub use self::debug::{SimpleDebug as Debug};
pub use self::display::{SimpleDisplay as Display};

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
    fn receive(_: http::Request, _: &Environment) -> Result<R::BodyParts, Error> {
        Ok(())
    }
}
