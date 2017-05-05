use futures::{Future, Stream};

use rigging::{Error, http};
use rigging::resource::{RelationEndpoint, ResourceEndpoint, Relationship, Resource};
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

pub trait Get: Resource {
    fn get(id: Self::Identifier, env: &Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

impl<T: Get> Method<T> for Get<Identifier = T::Identifier> {
    const ROUTE: Route<'static> = Route {
        kind: Kind::Resource,
        method: http::Method::Get,
    };

    type Request = ();
    type Response = T;
    type Outcome = Box<Future<Item = T, Error = Error>>;
}

impl<T: Get> ResourceMethod<T> for Get<Identifier = T::Identifier> {
    fn call(id: T::Identifier, _: Self::Request, env: &mut Environment) -> Self::Outcome {
        T::get(id, env)
    }
}

pub trait GetOne<R: Relationship>: Resource {
    fn get_one(id: Self::Identifier, env: &Environment) -> Box<Future<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: GetOne<R> + RelationEndpoint<R>, R: Relationship> Method<T> for GetOne<R, Identifier = T::Identifier>
    where R::Related: ResourceEndpoint,
{
    const ROUTE: Route<'static> = Route {
        kind: Kind::Relationship(T::REL_ENDPOINT),
        method: http::Method::Get,
    };

    type Request = ();
    type Response = R::Related;
    type Outcome = Box<Future<Item = R::Related, Error = Error>>;
}

impl<T: GetOne<R> + RelationEndpoint<R>, R: Relationship> ResourceMethod<T> for GetOne<R, Identifier = T::Identifier>
    where R::Related: ResourceEndpoint,
{
    fn call(id: T::Identifier, _: Self::Request, env: &mut Environment) -> Self::Outcome {
        T::get_one(id, env)
    }
}

pub trait GetMany<R: Relationship>: Resource {
    fn get_many(id: Self::Identifier, env: &Environment) -> Box<Stream<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: GetMany<R> + RelationEndpoint<R>, R: Relationship> Method<T> for GetMany<R, Identifier = T::Identifier>
    where R::Related: ResourceEndpoint,
{
    const ROUTE: Route<'static> = Route {
        kind: Kind::Relationship(T::REL_ENDPOINT),
        method: http::Method::Get,
    };

    type Request = ();
    type Response = R::Related;
    type Outcome = Box<Stream<Item = R::Related, Error = Error>>;
}

impl<T: GetMany<R> + RelationEndpoint<R>, R: Relationship> ResourceMethod<T> for GetMany<R, Identifier = T::Identifier>
    where R::Related: ResourceEndpoint,
{
    fn call(id: T::Identifier, _: Self::Request, env: &mut Environment) -> Self::Outcome {
        T::get_many(id, env)
    }
}
