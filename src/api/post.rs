use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, RawReceived, ResourceResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Post: Resource {
    type PostFut: IntoFuture<Item = Self, Error = Error>;
    fn post(data: Self) -> Self::PostFut;
}

pub trait RawPost<I>: RawUpdate {
    type RawPostFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn post(received: RawReceived<Self, Self>) -> Self::RawPostFut;
}

impl<I, T> RawPost<I> for T where T: Post + _UpdateRels {
    type RawPostFut = Result<ResourceResponse<I, T>, Error>;
    fn post(received: RawReceived<Self, Self>) -> Self::RawPostFut {
        let entity = Entity::Resource(<Self as Post>::post(received.attributes).into_future().wait()?);
        let relationships = <T as _UpdateRels>::update_rels(&entity, received.relationships)?;
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(ResourceResponse {
            resource: ResourceObject {
                id: resource.id(),
                attributes: resource,
                relationships: relationships,
            },
            includes: vec![],
        })
    }
}
