use api::{Entity, Error};
use api::raw::{RawFetch, GetResponse, IndexResponse, RawGet};
use api::rel::{Relation, HasOne, HasMany};
use router::IncludeQuery;

pub trait FetchOne<I, T: Relation>: HasOne<T> where T::Resource: RawFetch {
    fn fetch_one(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<Option<GetResponse<I, T::Resource>>, Error>;
}

impl<I, T, Rel> FetchOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet<I> {
    fn fetch_one(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<Option<GetResponse<I, Rel::Resource>>, Error> {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity)? {
            <Rel::Resource as RawGet<I>>::get(Entity::Id(id), includes).map(Some)
        } else { Ok(None) }
        
    }
}

pub trait FetchMany<I, T: Relation>: HasMany<T> where T::Resource: RawFetch {
    fn fetch_many(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<IndexResponse<I, T::Resource>, Error>;
}

impl<I, T, Rel> FetchMany<I, Rel> for T
where T:                HasMany<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet<I> {
    fn fetch_many(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<IndexResponse<I, Rel::Resource>, Error> {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for id in <T as HasMany<Rel>>::has_many(entity)? {
            let response = <Rel::Resource as RawGet<I>>::get(Entity::Id(id), includes)?;
            resources.push(response.resource);
            include_objects.extend(response.includes);
        }
        Ok(IndexResponse {
            resources: resources,
            includes: include_objects,
        })
    }
}
