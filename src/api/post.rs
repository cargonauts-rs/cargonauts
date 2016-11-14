use Deserialize;
use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, ResourceResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Post: Resource + Deserialize {
    type PostFut: IntoFuture<Item = Self, Error = Error>;
    fn post(self) -> Self::PostFut;
}

pub trait RawPost<I>: RawUpdate + Deserialize {
    type RawPostFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn post(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostFut;
}

impl<I, T> RawPost<I> for T where T: Post + _UpdateRels {
    type RawPostFut = Result<ResourceResponse<I, T>, Error>;
    fn post(self, rels: <T as RawUpdate>::Relationships) -> Self::RawPostFut {
        let entity = Entity::Resource(<Self as Post>::post(self).into_future().wait()?);
        let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
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
