use futures::Future;

use rigging::{Error, http};
use rigging::resource::{Relationship, Resource};
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};


pub trait GetMany<R: Relationship>: Resource {
    fn get_many(id: Self::Identifier, env: &Environment) -> Box<Future<Item = Vec<R::Related>, Error = Error>> where Self: Sized;
}

impl<T: GetMany<R>, R: Relationship> Method<T> for GetMany<R, Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Relationship,
        method: http::Method::Get,
    };

    type Request = ();
    type Response = R::Related;
    type Outcome = Box<Future<Item = Vec<R::Related>, Error = Error>>;
}

impl<T: GetMany<R>, R: Relationship> ResourceMethod<T> for GetMany<R, Identifier = T::Identifier> {
    fn call(id: T::Identifier, _: Self::Request, env: &mut Environment) -> Self::Outcome {
        T::get_many(id, env)
    }
}
