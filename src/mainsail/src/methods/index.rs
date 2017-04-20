use futures::Stream;
use rigging::{Resource, Method, Error, http};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};
use rigging::request::*;

pub trait Index: Resource {
    fn index(env: Environment) -> Box<Stream<Item = Self, Error = Error>> where Self: Sized;
}

pub struct IndexRequest;

impl<T: Resource> Request<T> for IndexRequest {
    type BodyParts = ();
}

impl<T: Resource> CollectionRequest<T> for IndexRequest {
    fn new(_: Self::BodyParts, _: &mut Environment) -> Self {
        IndexRequest
    }
}

impl<T: Index> Method<T> for Index<Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Collection,
        method: http::Method::Get,
    };

    type Request = IndexRequest;
    type Response = T;
    type Outcome = Box<Stream<Item = T, Error = Error>>;

    fn call(_: Self::Request, env: Environment) -> Self::Outcome {
        T::index(env)
    }
}

