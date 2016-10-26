use api::{Entity, Error};
use api::raw::{RawFetch, GetResponse, IndexResponse, RawGet};
use api::rel::{Relation, HasOne, HasMany};

pub trait FetchOne<T: Relation>: HasOne<T> where T::Resource: RawFetch {
    fn fetch_one(entity: &Entity<Self>, includes: &[String]) -> Result<Option<GetResponse<T::Resource>>, Error>;
}

impl<T, Rel> FetchOne<Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet {
    fn fetch_one(entity: &Entity<Self>, includes: &[String]) -> Result<Option<GetResponse<Rel::Resource>>, Error> {
        if let Some(id) = try!(<T as HasOne<Rel>>::has_one(entity)) {
            <Rel::Resource as RawGet>::get(id, includes).map(Some)
        } else { Ok(None) }
        
    }
}

pub trait FetchMany<T: Relation>: HasMany<T> where T::Resource: RawFetch {
    fn fetch_many(entity: &Entity<Self>, includes: &[String]) -> Result<IndexResponse<T::Resource>, Error>;
}

impl<T, Rel> FetchMany<Rel> for T
where T:                HasMany<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet {
    fn fetch_many(entity: &Entity<Self>, includes: &[String]) -> Result<IndexResponse<Rel::Resource>, Error> {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for id in try!(<T as HasMany<Rel>>::has_many(entity)) {
            let response = try!(<Rel::Resource as RawGet>::get(id, includes));
            resources.push(response.resource);
            include_objects.extend(response.includes);
        }
        Ok(IndexResponse {
            resources: resources,
            includes: include_objects,
        })
    }
}
