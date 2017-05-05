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
        where T: HasOneEndpoint<R>, R::Related: ResourceEndpoint;
    fn rel_id<R: Relationship>(&self) -> Option<&str>
        where T: HasOneEndpoint<R>, R::Related: ResourceEndpoint;
    fn set_rel_ids<R: Relationship>(&mut self, ids: Vec<String>)
        where T: HasManyEndpoint<R>, R::Related: ResourceEndpoint;
    fn rel_ids<R: Relationship>(&self) -> &[String]
        where T: HasManyEndpoint<R>, R::Related: ResourceEndpoint;
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

pub trait HasOneEndpoint<R>
where
    R: Relationship,
    R::Related: ResourceEndpoint,
    Self: RelationEndpoint<R>,
{ }

pub trait HasManyEndpoint<R>
where
    R: Relationship,
    R::Related: ResourceEndpoint,
    Self: RelationEndpoint<R>,
{ }

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

    pub fn set_rel_id<R>(&mut self, id: &<R::Related as Resource>::Identifier)
    where
        T: HasOneEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint,
    {
        self.rel_ids.set_rel_id::<R>(id.to_string());
    }

    pub fn rel_id<R>(&self) -> Option<<R::Related as Resource>::Identifier>
    where
        T: HasOneEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint,
    {
        // TODO better error handling
        self.rel_ids.rel_id::<R>().and_then(|id| id.parse().ok())
    }

    pub fn set_rel_ids<R>(&mut self, ids: &[<R::Related as Resource>::Identifier])
    where
        T: HasManyEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint,
    {
        self.rel_ids.set_rel_ids::<R>(ids.iter().map(|id| id.to_string()).collect())
    }

    pub fn rel_ids<R>(&mut self) -> Vec<<R::Related as Resource>::Identifier>
    where
        T: HasManyEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint,
    {
        // TODO better error handling
        self.rel_ids.rel_ids::<R>().iter().filter_map(|id| id.parse().ok()).collect()
    }
}
