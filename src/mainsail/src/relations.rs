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
