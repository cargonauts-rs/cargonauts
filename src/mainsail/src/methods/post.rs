use futures::Future;
use rigging::{Resource, Error, http};
use rigging::environment::Environment;
use rigging::method::Method;
use rigging::routes::{Route, Kind};
use rigging::request::*;

pub trait Post: Resource {
    type Post;
    fn post(post: Self::Post, env: &Environment) -> Box<Future<Item = Self, Error = Error>> where Self: Sized;
}

pub struct PostRequest<T: Post> {
    post: T::Post,
}

impl<T: Post> Request<T> for PostRequest<T> {
    type BodyParts = T::Post;
}

impl<T: Post> CollectionRequest<T> for PostRequest<T> {
    fn new(post: Self::BodyParts, _: &Environment) -> Self {
        PostRequest { post }
    }
}

impl<T: Post> Method<T> for Post<Identifier = T::Identifier, Post = T::Post> {
    const ROUTE: Route<'static> = Route {
        kind: Kind::Collection,
        method: http::Method::Post,
    };

    type Request = PostRequest<T>;
    type Response = T;
    type Outcome = Box<Future<Item = T, Error = Error>>;

    fn call(req: Self::Request, env: &Environment) -> Self::Outcome {
        T::post(req.post, env)
    }
}
