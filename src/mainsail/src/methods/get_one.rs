use futures::Future;

use rigging::{Error, http};
use rigging::resource::{Relationship, Resource};
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

pub trait GetOne<R: Relationship>: Resource {
    fn get_one(id: Self::Identifier, env: &Environment) -> Box<Future<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: GetOne<R>, R: Relationship> Method<T> for GetOne<R, Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Relationship,
        method: http::Method::Get,
    };

    type Request = ();
    type Response = R::Related;
    type Future = Box<Future<Item = R::Related, Error = Error>>;
}

impl<T: GetOne<R>, R: Relationship> ResourceMethod<T> for GetOne<R, Identifier = T::Identifier> {
    fn call(id: T::Identifier, _: Self::Request, env: &mut Environment) -> Self::Future {
        T::get_one(id, env)
    }
}
