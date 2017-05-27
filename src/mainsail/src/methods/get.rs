use futures::Future;

use rigging::{Error, http};
use rigging::resource::Resource;
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

/// Get a single instance of this resource.
///
/// This method corresponds to `GET /$resource-type/$identifier`.
pub trait Get: Resource {
    fn get(id: Self::Identifier, env: Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

impl<T: Get> Method<T> for Get<Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Resource,
        method: http::Method::Get,
    };

    type Request = ();
    type Response = T;
    type Future = Box<Future<Item = T, Error = Error>>;
}

impl<T: Get> ResourceMethod<T> for Get<Identifier = T::Identifier> {
    fn call(id: T::Identifier, _: Self::Request, env: Environment) -> Self::Future {
        T::get(id, env)
    }
}
