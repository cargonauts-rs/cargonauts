use futures::Future;

use rigging::{Error, http};
use rigging::resource::{Relationship, Resource};
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

pub trait UpdateRelated<R: Relationship>: Resource {
    type Update;
    fn update_related(id: Self::Identifier, update: Self::Update, env: &Environment) -> Box<Future<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: UpdateRelated<R>, R: Relationship> Method<T> for UpdateRelated<R, Identifier = T::Identifier, Update = T::Update> {
    const ROUTE: Route = Route {
        kind: Kind::Relationship,
        method: http::Method::Patch,
    };

    type Request = T::Update;
    type Response = R::Related;
    type Outcome = Box<Future<Item = R::Related, Error = Error>>;
}

impl<T: UpdateRelated<R>, R: Relationship> ResourceMethod<T> for UpdateRelated<R, Identifier = T::Identifier, Update = T::Update> {
    fn call(id: T::Identifier, update: Self::Request, env: &mut Environment) -> Self::Outcome {
        T::update_related(id, update, env)
    }
}
