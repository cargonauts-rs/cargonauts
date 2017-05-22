use cargonauts::futures::Future;
use cargonauts::methods::def::*;
use cargonauts::resources::*;

pub trait Random: Resource {
    fn random(env: Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

impl<T: Random> Method<T> for Random<Identifier = T::Identifier> {
    const ROUTE: Route = Route {
        method: HttpMethod::Get,
        kind: Kind::Collection,
    };

    type Request = ();
    type Response = T;
    type Future = Box<Future<Item = T, Error = Error>>;
}

impl<T: Random> CollectionMethod<T> for Random<Identifier = T::Identifier> {
    fn call(_: Self::Request, env: Environment) -> Self::Future {
        T::random(env)
    }
}
