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
    type GetIdFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    type GetResourceFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn get_id(id: Self::Id, includes: &[IncludeQuery]) -> Self::GetIdFut;
    fn get_resource(resource: Self, includes: &[IncludeQuery]) -> Self::GetResourceFut;
}

impl<I, T> RawGet<I> for T where T: Get + _FetchRels<I> {
    type GetIdFut = Result<ResourceResponse<I, T>, Error>;
    type GetResourceFut = Result<ResourceResponse<I, T>, Error>;
    fn get_id(id: Self::Id, includes: &[IncludeQuery]) -> Self::GetIdFut {
        let entity = Entity::Resource(<T as Get>::get(&id).into_future().wait()?);
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
    fn get_resource(resource: Self, includes: &[IncludeQuery]) -> Self::GetResourceFut {
        let id = resource.id();
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
}
