use futures::Future;

use rigging::{Error, http};
use rigging::resource::Resource;
use rigging::environment::Environment;
use rigging::method::{Method, CollectionMethod};
use rigging::routes::{Route, Kind};

/// Post a new instance of a resource.
///
/// This method coresponds to `POST /$resource-type`.
pub trait Post: Resource {
    /// The representation of this resouce that will be received by the POST
    /// request.
    ///
    /// This could just be the resource, but it could also be a different type,
    /// if some fields are generated during posting instead of received as a
    /// part of the request.
    type Post;
    fn post(post: Self::Post, env: Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

impl<T: Post> Method<T> for Post<Identifier = T::Identifier, Post = T::Post> {
    const ROUTE: Route = Route {
        kind: Kind::Collection,
        method: http::Method::Post,
    };

    type Request = T::Post;
    type Response = T;
    type Future = Box<Future<Item = T, Error = Error>>;
}

impl<T: Post> CollectionMethod<T> for Post<Identifier = T::Identifier, Post = T::Post> {
    fn call(req: Self::Request, env: Environment) -> Self::Future {
        T::post(req, env)
    }
}
