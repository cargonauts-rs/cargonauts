use futures::Future;
use rigging::{Resource, Error, http};
use rigging::environment::Environment;
use rigging::method::Method;
use rigging::routes::{Route, Kind};
use rigging::request::*;

pub trait Delete: Resource {
    fn delete(id: Self::Identifier, env: &mut Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

pub struct DeleteRequest<T: Resource> {
    id: T::Identifier,
}

impl<T: Resource> Request<T> for DeleteRequest<T> {
    type BodyParts = ();
}

impl<T: Resource> ResourceRequest<T> for DeleteRequest<T> {
    fn new(_: Self::BodyParts, id: T::Identifier, _: &mut Environment) -> Self {
        DeleteRequest { id }
    }
}

impl<T: Delete> Method<T> for Delete<Identifier = T::Identifier> {
    const ROUTE: Route<'static> = Route {
        kind: Kind::Resource,
        method: http::Method::Delete,
    };

    type Request = DeleteRequest<T>;
    type Response = T;
    type Outcome = Box<Future<Item = T, Error = Error>>;

    fn call(req: Self::Request, env: &mut Environment) -> Self::Outcome {
        T::delete(req.id, env)
    }
}
