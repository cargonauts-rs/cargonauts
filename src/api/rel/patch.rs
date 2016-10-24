use api::Error;
use api::raw::{RawUpdate, RawPatch, PatchResponse};
use api::rel::{Relation, HasOne};
use Deserialize;

pub trait PatchOne<T: Relation>: HasOne<T> where T::Resource: RawUpdate {
    type Patch: Deserialize;
    fn patch_one(id: &Self::Id, patch: Self::Patch, rels: <T::Resource as RawUpdate>::Relationships) -> Result<Option<PatchResponse<T::Resource>>, Error>;
}

impl<T, Rel> PatchOne<Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPatch {
    type Patch = <<Rel as Relation>::Resource as RawPatch>::Patch;
    fn patch_one(id: &Self::Id, patch: Self::Patch, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<Option<PatchResponse<Rel::Resource>>, Error> {
        if let Some(id) = try!(<T as HasOne<Rel>>::has_one(id)) {
            <Rel::Resource as RawPatch>::patch(id, patch, rels).map(Some)
        } else { Ok(None) }
    }
}
