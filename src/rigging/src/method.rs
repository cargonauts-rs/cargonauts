use futures::Future;

use Error;
use resource::Resource;
use environment::Environment;
use routes::Route;

pub trait Method<T: Resource> {
    const ROUTE: Route;

    type Request;
    type Response;
    type Future: Future<Item = Self::Response, Error = Error> + 'static;
}

pub trait ResourceMethod<T: Resource>: Method<T> {
    fn call(id: T::Identifier, req: Self::Request, env: &mut Environment) -> Self::Future;
}

pub trait CollectionMethod<T: Resource>: Method<T> {
    fn call(req: Self::Request, env: &mut Environment) -> Self::Future;
}
