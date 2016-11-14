use Deserialize;
use api::{Resource, Error, Entity};
use api::raw::{ResourceResponse, RawUpdate, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Patch: Resource {
    type Patch: Deserialize;
    type PatchFut: IntoFuture<Item = Self, Error = Error>;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Self::PatchFut;
}

pub trait RawPatch<I>: RawUpdate {
    type Patch: Deserialize;
    type RawPatchFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchFut;
}

impl<I, T> RawPatch<I> for T where T: Patch + _UpdateRels {
    type Patch = <Self as Patch>::Patch;
    type RawPatchFut = Result<ResourceResponse<I, Self>, Error>;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchFut {
        let entity = Entity::Resource(<T as Patch>::patch(&id, patch).into_future().wait()?);
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
