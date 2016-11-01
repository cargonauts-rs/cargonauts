use api::{Entity, Error};
use api::raw::{RawFetch, GetResponse, IndexResponse, RawGet};
use api::rel::{Relation, HasOne, HasMany};
use router::IncludeQuery;
use Serializer;

pub trait FetchOne<T: Relation>: HasOne<T> where T::Resource: RawFetch {
    fn fetch_one<S: Serializer>(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<Option<GetResponse<S, T::Resource>>, Error>;
}

impl<T, Rel> FetchOne<Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet {
    fn fetch_one<S: Serializer>(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<Option<GetResponse<S, Rel::Resource>>, Error> {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity)? {
            <Rel::Resource as RawGet>::get(id, includes).map(Some)
        } else { Ok(None) }
        
    }
}

pub trait FetchMany<T: Relation>: HasMany<T> where T::Resource: RawFetch {
    fn fetch_many<S: Serializer>(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<IndexResponse<S, T::Resource>, Error>;
}

impl<T, Rel> FetchMany<Rel> for T
where T:                HasMany<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet {
    fn fetch_many<S: Serializer>(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<IndexResponse<S, Rel::Resource>, Error> {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for id in <T as HasMany<Rel>>::has_many(entity)? {
            let response = <Rel::Resource as RawGet>::get(id, includes)?;
            resources.push(response.resource);
            include_objects.extend(response.includes);
        }
        Ok(IndexResponse {
            resources: resources,
            includes: include_objects,
        })
    }
}
