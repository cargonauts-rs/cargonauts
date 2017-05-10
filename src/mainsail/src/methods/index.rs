use futures::Future;

use rigging::{Error, http};
use rigging::resource::Resource;
use rigging::method::{Method, CollectionMethod};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

pub trait Index: Resource {
    fn index(env: &Environment) -> Box<Future<Item = Vec<Self>, Error = Error>> where Self: Sized;
}

impl<T: Index> Method<T> for Index<Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        kind: Kind::Collection,
        method: http::Method::Get,
    };

    type Request = ();
    type Response = Vec<T>;
    type Future = Box<Future<Item = Vec<T>, Error = Error>>;
}

impl<T: Index> CollectionMethod<T> for Index<Identifier = T::Identifier> {
    fn call(_: Self::Request, env: &mut Environment) -> Self::Future {
        T::index(env)
    }
}

