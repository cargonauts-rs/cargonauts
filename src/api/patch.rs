use Deserialize;
use api::{Resource, Error};
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
        let attributes = try!(<T as Patch>::patch(&id, patch));
        let relationships = try!(<T as _UpdateRels>::update_rels(&id, rels));
        Ok(PatchResponse {
            resource: ResourceObject {
                id: id,
                attributes: attributes,
                relationships: relationships,
            }
        })
    }
}

pub struct PatchResponse<T: RawFetch> {
    pub resource: ResourceObject<T>,
}
