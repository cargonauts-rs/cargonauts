use api::{Entity, Error};
use api::raw::{RawFetch, ResourceResponse, CollectionResponse, RawGet};
use api::rel::{ToOne, ToMany, HasOne, HasMany};
use router::IncludeQuery;
use IntoFuture;
use futures::Future;

pub trait GetOne<I, T: ToOne>: HasOne<T> where T::Resource: RawFetch {
    type GetOneFut: Future<Item = ResourceResponse<I, T::Resource>, Error = Error>;
    fn get_one(entity: &Entity<Self>, includes: Vec<IncludeQuery>) -> Self::GetOneFut;
}

impl<I, T, Rel> GetOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              ToOne,
      Rel::Resource:    RawGet<I>,
      I:                'static {
    type GetOneFut = Box<Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error>>;
    fn get_one(entity: &Entity<Self>, includes: Vec<IncludeQuery>) -> Self::GetOneFut {
        Box::new(<T as HasOne<Rel>>::has_one(entity).into_future().and_then(|id| {
            if let Some(id) = id {
                Box::new(<Rel::Resource as RawGet<I>>::get(id, includes).into_future()) as Box<Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error>>
            } else { Box::new(Err(Error::NotFound).into_future()) }
        }))
    }
}

pub trait IndexMany<I, T: ToMany>: HasMany<T> where T::Resource: RawFetch {
    type IndexManyFut: Future<Item = CollectionResponse<I, T::Resource>, Error = Error>;
    fn index_many(entity: &Entity<Self>, includes: Vec<IncludeQuery>) -> Self::IndexManyFut;
}

impl<I, T, Rel> IndexMany<I, Rel> for T
where T:                HasMany<Rel>,
      Rel:              ToMany,
      Rel::Resource:    RawGet<I>,
      I:                'static {
    type IndexManyFut = Box<Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error>>;
    fn index_many(entity: &Entity<Self>, includes: Vec<IncludeQuery>) -> Self::IndexManyFut {
        Box::new(<T as HasMany<Rel>>::has_many(entity).into_future().and_then(move |ids| {
            let mut resources = vec![];
            let mut include_objects = vec![];
            for id in ids {
                let response = <Rel::Resource as RawGet<I>>::get(id, includes.clone()).into_future().wait()?;
                resources.push(response.resource);
                include_objects.extend(response.includes);
            }
            Ok(CollectionResponse {
                resources: resources,
                includes: include_objects,
            })
        }))
    }
}
