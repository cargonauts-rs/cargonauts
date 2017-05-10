use futures::Future;

use rigging::{Error, http};
use rigging::resource::{Relationship, Resource};
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

pub trait DeleteRelated<R: Relationship>: Resource {
    fn delete_related(id: Self::Identifier, env: &Environment) -> Box<Future<Item = (), Error = Error>> where Self: Sized;
}

impl<T: DeleteRelated<R>, R: Relationship> Method<T> for DeleteRelated<R, Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Relationship,
        method: http::Method::Delete,
    };

    type Request = ();
    type Response = ();
    type Future = Box<Future<Item = (), Error = Error>>;
}

impl<T: DeleteRelated<R>, R: Relationship> ResourceMethod<T> for DeleteRelated<R, Identifier = T::Identifier> {
    fn call(id: T::Identifier, _: Self::Request, env: &mut Environment) -> Self::Future {
        T::delete_related(id, env)
    }
}
