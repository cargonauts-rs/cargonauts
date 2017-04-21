use futures::{Future, Stream};
use rigging::{RelationEndpoint, ResourceEndpoint, Relationship, Resource, Method, Error, http};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};
use rigging::request::*;

pub trait Get: Resource {
    fn get(id: Self::Identifier, env: Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

pub struct GetRequest<T: Resource> {
    id: T::Identifier,
}

impl<T: Resource> Request<T> for GetRequest<T> {
    type BodyParts = ();
}

impl<T: Resource> ResourceRequest<T> for GetRequest<T> {
    fn new(_: Self::BodyParts, id: T::Identifier, _: &mut Environment) -> Self {
        GetRequest { id }
    }
}

impl<T: Get> Method<T> for Get<Identifier = T::Identifier> {
    const ROUTE: Route<'static> = Route {
        kind: Kind::Resource,
        method: http::Method::Get,
    };

    type Request = GetRequest<T>;
    type Response = T;
    type Outcome = Box<Future<Item = T, Error = Error>>;

    fn call(req: Self::Request, env: Environment) -> Self::Outcome {
        T::get(req.id, env)
    }
}

pub trait GetOne<R: Relationship>: Resource {
    fn get_one(id: Self::Identifier, env: Environment) -> Box<Future<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: GetOne<R> + RelationEndpoint<R>, R: Relationship> Method<T> for GetOne<R, Identifier = T::Identifier>
    where R::Related: ResourceEndpoint,
{
    const ROUTE: Route<'static> = Route {
        kind: Kind::Relationship(T::LINK.endpoint),
        method: http::Method::Get,
    };

    type Request = GetRequest<T>;
    type Response = R::Related;
    type Outcome = Box<Future<Item = R::Related, Error = Error>>;

    fn call(req: Self::Request, env: Environment) -> Self::Outcome {
        T::get_one(req.id, env)
    }
}

pub trait GetMany<R: Relationship>: Resource {
    fn get_many(id: Self::Identifier, env: Environment) -> Box<Stream<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: GetMany<R> + RelationEndpoint<R>, R: Relationship> Method<T> for GetMany<R, Identifier = T::Identifier>
    where R::Related: ResourceEndpoint,
{
    const ROUTE: Route<'static> = Route {
        kind: Kind::Relationship(T::LINK.endpoint),
        method: http::Method::Get,
    };

    type Request = GetRequest<T>;
    type Response = R::Related;
    type Outcome = Box<Stream<Item = R::Related, Error = Error>>;

    fn call(req: Self::Request, env: Environment) -> Self::Outcome {
        T::get_many(req.id, env)
    }
}
