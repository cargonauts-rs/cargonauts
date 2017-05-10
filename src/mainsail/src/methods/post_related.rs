use futures::Future;

use rigging::{Error, http};
use rigging::resource::{Relationship, Resource};
use rigging::method::{ResourceMethod, Method};
use rigging::environment::Environment;
use rigging::routes::{Route, Kind};

pub trait PostRelated<R: Relationship>: Resource {
    type Post;
    fn post_related(id: Self::Identifier, post: Self::Post, env: &Environment) -> Box<Future<Item = R::Related, Error = Error>> where Self: Sized;
}

impl<T: PostRelated<R>, R: Relationship> Method<T> for PostRelated<R, Identifier = T::Identifier, Post = T::Post> {
    const ROUTE: Route = Route {
        kind: Kind::Relationship,
        method: http::Method::Post,
    };

    type Request = T::Post;
    type Response = R::Related;
    type Future = Box<Future<Item = R::Related, Error = Error>>;
}

impl<T: PostRelated<R>, R: Relationship> ResourceMethod<T> for PostRelated<R, Identifier = T::Identifier, Post = T::Post> {
    fn call(id: T::Identifier, post: Self::Request, env: &mut Environment) -> Self::Future {
        T::post_related(id, post, env)
    }
}
