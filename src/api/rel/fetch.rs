use api::{Entity, Error};
use api::raw::{RawFetch, ResourceResponse, CollectionResponse, RawGet};
use api::rel::{Relation, HasOne, HasMany};
use router::IncludeQuery;
use IntoFuture;
use futures::Future;

pub trait GetOne<I, T: Relation>: HasOne<T> where T::Resource: RawFetch {
    type GetOneFut: IntoFuture<Item = ResourceResponse<I, T::Resource>, Error = Error>;
    fn get_one(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::GetOneFut;
}

impl<I, T, Rel> GetOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet<I> {
    type GetOneFut = Result<ResourceResponse<I, Rel::Resource>, Error>;
    fn get_one(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::GetOneFut {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity).into_future().wait()? {
            <Rel::Resource as RawGet<I>>::get_id(id, includes).into_future().wait()
        } else { Err(Error::NotFound) }
        
    }
}

pub trait IndexMany<I, T: Relation>: HasMany<T> where T::Resource: RawFetch {
    type IndexManyFut: IntoFuture<Item = CollectionResponse<I, T::Resource>, Error = Error>;
    fn index_many(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::IndexManyFut;
}

impl<I, T, Rel> IndexMany<I, Rel> for T
where T:                HasMany<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawGet<I> {
    type IndexManyFut = Result<CollectionResponse<I, Rel::Resource>, Error>;
    fn index_many(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Self::IndexManyFut {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for id in <T as HasMany<Rel>>::has_many(entity).into_future().wait()? {
            let response = <Rel::Resource as RawGet<I>>::get_id(id, includes).into_future().wait()?;
            resources.push(response.resource);
            include_objects.extend(response.includes);
        }
        Ok(CollectionResponse {
            resources: resources,
            includes: include_objects,
        })
    }
}
