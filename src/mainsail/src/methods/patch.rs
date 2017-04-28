use futures::Future;
use rigging::{Resource, Error, http};
use rigging::environment::Environment;
use rigging::method::Method;
use rigging::routes::{Route, Kind};
use rigging::request::*;

pub trait Patch: Resource {
    type Patch;
    fn patch(id: Self::Identifier, patch: Self::Patch, env: &Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

pub struct PatchRequest<T: Patch> {
    id: T::Identifier,
    patch: T::Patch,
}

impl<T: Patch> Request<T> for PatchRequest<T> {
    type BodyParts = T::Patch;
}

impl<T: Patch> Method<T> for Patch<Identifier = T::Identifier, Patch = T::Patch> {
    const ROUTE: Route<'static> = Route {
        kind: Kind::Resource,
        method: http::Method::Patch,
    };

    type Request = PatchRequest<T>;
    type Response = T;
    type Outcome = Box<Future<Item = T, Error = Error>>;

    fn call(req: Self::Request, env: &Environment) -> Self::Outcome {
        T::patch(req.id, req.patch, env)
    }
}
