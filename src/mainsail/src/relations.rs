use futures::BoxFuture;
use futures::stream::BoxStream;

use Environment;
use Error;
use Resource;
use ResourceEndpoint;

pub trait RelationEndpoint<R>
where
    R: Relationship,
    R::Related: ResourceEndpoint,
    Self: ResourceEndpoint,
{
    const LINK: RelationshipLink;
}

pub struct RelationshipLink {
    pub endpoint: &'static str,
    pub relation: &'static str,
}

pub trait Relationship: Sized {
    type Related: Resource;
}

impl<T: Resource> Relationship for T {
    type Related = T;
}

pub trait GetOne<R: Relationship>: Resource {
    fn get_one(identifier: Self::Identifier, env: Environment) -> BoxFuture<R::Related, Error>
    where Self: Sized;
}

pub trait GetMany<R: Relationship>: Resource {
    fn get_many(identifier: Self::Identifier, env: Environment) -> BoxStream<R::Related, Error>
    where Self: Sized;
}
