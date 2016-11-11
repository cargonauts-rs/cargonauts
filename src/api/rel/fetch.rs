use api::{Entity, Error};
use api::raw::{RawFetch, GetResponse, IndexResponse, RawGet};
use api::rel::{Relation, HasOne, HasMany};
use router::IncludeQuery;
use IntoFuture;
use futures::Future;

pub trait FetchOne<I, T: Relation>: HasOne<T> where T::Resource: RawFetch {
    type FetchOneFut: IntoFuture<Item = GetResponse<I, T::Resource>, Error = Error>;
    fn fetch_one(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::FetchOneFut;
}

impl<I, T, Rel> FetchOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet<I> {
    type FetchOneFut = Result<GetResponse<I, Rel::Resource>, Error>;
    fn fetch_one(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::FetchOneFut {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity).into_future().wait()? {
            <Rel::Resource as RawGet<I>>::get_id(id, includes).into_future().wait()
        } else { Err(Error::NotFound) }
        
    }
}

pub trait FetchMany<I, T: Relation>: HasMany<T> where T::Resource: RawFetch {
    type FetchManyFut: IntoFuture<Item = IndexResponse<I, T::Resource>, Error = Error>;
    fn fetch_many(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::FetchManyFut;
}

impl<I, T, Rel> FetchMany<I, Rel> for T
where T:                HasMany<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet<I> {
    type FetchManyFut = Result<IndexResponse<I, Rel::Resource>, Error>;
    fn fetch_many(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::FetchManyFut {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for id in <T as HasMany<Rel>>::has_many(entity).into_future().wait()? {
            let response = <Rel::Resource as RawGet<I>>::get_id(id, includes).into_future().wait()?;
            resources.push(response.resource);
            include_objects.extend(response.includes);
        }
        Ok(IndexResponse {
            resources: resources,
            includes: include_objects,
        })
    }
}
