use Resource;

pub trait RelationEndpoint<R: Relationship>: Send + Sync + Sized + 'static {
    const LINK: RelationshipLink;
}

pub struct RelationshipLink {
    pub endpoint: &'static str,
    pub relation: &'static str,
}

pub trait Relationship: Sized {
    type Related;
}

pub trait ToOne: Relationship<Related = <Self as ToOne>::One> {
    type One: Resource;
}

pub trait ToMany: Relationship<Related = Vec<<Self as ToMany>::Many>> {
    type Many: Resource;
}

impl<T: Resource> Relationship for T {
    type Related = T;
}

impl<T: Resource> ToOne for T {
    type One = T;
}
