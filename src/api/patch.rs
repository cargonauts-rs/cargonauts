use Deserialize;
use api::{Resource, Error, Entity};
use api::raw::{RawUpdate, ResourceObject, RawFetch};
use _internal::_UpdateRels;

pub trait Patch: Resource {
    type Patch: Deserialize;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Result<Self, Error>;
}
pub trait RawPatch: RawUpdate {
    type Patch: Deserialize;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Result<PatchResponse<Self>, Error>;
}

impl<T> RawPatch for T where T: Patch + _UpdateRels {
    type Patch = <Self as Patch>::Patch;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Result<PatchResponse<Self>, Error> {
        let entity = Entity::Resource(<T as Patch>::patch(&id, patch)?);
        let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(PatchResponse {
            resource: ResourceObject {
                id: resource.id(),
                attributes: resource,
                relationships: relationships,
            }
        })
    }
}

pub struct PatchResponse<T: RawFetch> {
    pub resource: ResourceObject<T>,
}
