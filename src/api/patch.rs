use api::{Resource, Error, Entity};
use api::raw::{ResourceResponse, RawReceived, RawUpdate, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Patch: Resource {
    type Patch;
    type PatchFut: IntoFuture<Item = Self, Error = Error>;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Self::PatchFut;
}

pub trait RawHasPatch<Synchronicity>: RawUpdate {
    type Patch;
}

pub trait RawPatch<I>: RawHasPatch<Synchronous> {
    type RawPatchFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn patch(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchFut;
}

pub struct Synchronous;

impl<T> RawHasPatch<Synchronous> for T where T: Patch + _UpdateRels {
    type Patch = <T as Patch>::Patch;
}

impl<I, T> RawPatch<I> for T where T: Patch + _UpdateRels {
    type RawPatchFut = Result<ResourceResponse<I, Self>, Error>;
    fn patch(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchFut {
        let entity = Entity::Resource(<T as Patch>::patch(&id, received.attributes).into_future().wait()?);
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
