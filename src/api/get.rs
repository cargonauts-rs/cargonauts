use itertools::Itertools;

use api::{Resource, Error, Entity};
use api::raw::{ResourceResponse, RawFetch, ResourceObject};
use router::IncludeQuery;
use _internal::_FetchRels;
use IntoFuture;
use futures::Future;

pub trait Get: Resource {
    type GetFut: IntoFuture<Item = Self, Error = Error>;
    fn get(id: &Self::Id) -> Self::GetFut;
}

pub trait RawGet<I>: RawFetch {
    type RawGetFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn get(id: Self::Id, includes: &[IncludeQuery]) -> Self::RawGetFut;
}

impl<I, T> RawGet<I> for T where T: Get + _FetchRels<I> {
    type RawGetFut = Result<ResourceResponse<I, T>, Error>;
    fn get(id: Self::Id, includes: &[IncludeQuery]) -> Self::RawGetFut {
        let resource = <T as Get>::get(&id).into_future().wait()?;
        raw_get(id, resource, includes)
    }
}

pub fn raw_get<I, T: _FetchRels<I>>(id: T::Id, resource: T, includes: &[IncludeQuery]) -> Result<ResourceResponse<I, T>, Error> {
    let entity = Entity::Resource(resource);
    let (rels, includes) = <T as _FetchRels<I>>::rels(&entity, &includes)?;
    let includes = includes.into_iter()
        .unique_by(|include| (include.id.clone(), include.resource))
        .collect();
    let resource = match entity {
        Entity::Resource(resource)  => resource,
        _                           => unreachable!()
    };
    Ok(ResourceResponse {
        resource: ResourceObject {
            id: id,
            attributes: resource,
            relationships: rels,
        },
        includes: includes,
    })
}
