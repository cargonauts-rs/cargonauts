use std::str::FromStr;

pub trait Resource: Send + 'static {
    type Identifier: Eq + ToString + FromStr + Send + 'static;
}

pub trait ResourceEndpoint: Sized + Resource {
    const ENDPOINT: &'static str;
    const RESOURCE: &'static str;
    const REL_LINKS: &'static [RelationshipLink];
    type RelIds: RelIds<Self>;
}

pub trait RelIds<T>: Default {
    fn try_set_rel_id(&mut self, rel: &str, id: String) -> bool;
    fn set_rel_id<R: Relationship>(&mut self, id: String)
        where T: RelationEndpoint<R>, R::Related: ResourceEndpoint;
    fn rel_id<R: Relationship>(&self) -> Option<&str>
        where T: RelationEndpoint<R>, R::Related: ResourceEndpoint;
}

pub struct RelationshipLink {
    pub relation: &'static str,
    pub endpoint: &'static str,
}

pub trait Relationship: 'static {
    type Related: Resource;
}

impl<T: Resource> Relationship for T {
    type Related = T;
}

pub trait RelationEndpoint<R>
where
    R: Relationship,
    R::Related: ResourceEndpoint,
    Self: ResourceEndpoint,
{
    const REL_ENDPOINT: &'static str;
    const RELATION: &'static str;
}

pub struct WithRels<T: ResourceEndpoint, P> {
    pub inner: P,
    rel_ids: T::RelIds,
}

impl<T: ResourceEndpoint, P> WithRels<T, P> {
    pub fn new(inner: P) -> WithRels<T, P> {
        WithRels { inner, rel_ids: T::RelIds::default(), }
    }

    pub fn from_parts(inner: P, rel_ids: T::RelIds) -> WithRels<T, P> {
        WithRels { inner, rel_ids }
    }

    pub fn set_rel_id<R>(&mut self, id: <R::Related as Resource>::Identifier)
    where
        T: RelationEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint,
    {
        self.rel_ids.set_rel_id(id.to_string());
    }

    pub fn rel_id<R>(&self) -> Option<<R::Related as Resource>::Identifier>
    where
        T: RelationEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint,
    {
        // TODO better error handling
        self.rel_ids.rel_id::<R>().and_then(|id| id.parse().ok())
    }
}
