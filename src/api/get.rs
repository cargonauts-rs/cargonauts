use itertools::Itertools;

use api::{Resource, Error, Entity};
use api::raw::{ResourceResponse, RawResource, ResourceObject};
use router::IncludeQuery;
use _internal::_FetchRels;
use IntoFuture;
use futures::Future;

pub trait Get: Resource {
    type GetFut: IntoFuture<Item = Self, Error = Error>;
    fn get(id: &Self::Id) -> Self::GetFut;
}

pub trait RawGet<I>: RawResource {
    type RawGetFut: Future<Item = ResourceResponse<I, Self>, Error = Error>;
    fn get(id: Self::Id, includes: Vec<IncludeQuery>) -> Self::RawGetFut;
}

impl<I, T> RawGet<I> for T where T: Get + _FetchRels<I>, I: 'static {
    type RawGetFut = Box<Future<Item = ResourceResponse<I, T>, Error = Error>>;
    fn get(id: Self::Id, includes: Vec<IncludeQuery>) -> Self::RawGetFut {
        Box::new(<T as Get>::get(&id).into_future().and_then(move |resource| {
            raw_get(id, resource, includes)
        }))
    }
}

pub fn raw_get<I: 'static, T: _FetchRels<I>>(id: T::Id, resource: T, includes: Vec<IncludeQuery>) -> Box<Future<Item = ResourceResponse<I, T>, Error = Error>> {
    let entity = Entity::Resource(resource);
    Box::new(<T as _FetchRels<I>>::rels(&entity, &includes).into_future().map(move |(rels, includes)| {
        let includes = includes.into_iter()
            .unique_by(|include| (include.id.clone(), include.resource))
            .collect();
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        ResourceResponse {
            resource: ResourceObject {
                id: id,
                attributes: resource,
                relationships: rels,
            },
            includes: includes,
        }
    }))
}
