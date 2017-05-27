use futures::Future;

use rigging::{Error, http};
use rigging::resource::Resource;
use rigging::environment::Environment;
use rigging::method::{Method, ResourceMethod};
use rigging::routes::{Route, Kind};

/// Delete an instance of this resource.
///
/// This method corresponds to `DELETE /$resource-type/$identifier`.
pub trait Delete: Resource {
    fn delete(id: Self::Identifier, env: Environment) -> Box<Future<Item = (), Error = Error>> where Self: Sized;
}

impl<T: Delete> Method<T> for Delete<Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Resource,
        method: http::Method::Delete,
    };

    type Request = ();
    type Response = ();
    type Future = Box<Future<Item = (), Error = Error>>;
}

impl<T: Delete> ResourceMethod<T> for Delete<Identifier = T::Identifier> {
    fn call(id: T::Identifier, _: Self::Request, env: Environment) -> Self::Future {
        T::delete(id, env)
    }
}
