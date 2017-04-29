mod present;
mod receive;

use rigging::ResourceEndpoint;
use rigging::format::Format;
use rigging::method::Method;
use rigging::request::Request;

pub use self::present::{Fields, ApiSerialize};
pub use self::receive::{ParseBody, ApiDeserialize};

pub struct JsonApi;

impl<T, M, P> Format<T, M> for JsonApi
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: Request<T>,
    M::Response: ApiSerialize + ResourceEndpoint,
    M::Request: Request<T, BodyParts = P>,
    P: ParseBody,
{
    type Receiver = Self;
    type Presenter = Self;
}
