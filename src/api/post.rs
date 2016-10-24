use Deserialize;
use api::{Resource, Error};
use api::raw::{RawUpdate, ResourceObject, RawFetch};
use _internal::_UpdateRels;

pub trait Post: Resource + Deserialize {
    fn post(self) -> Result<Self, Error>;
}

pub trait RawPost: RawUpdate + Deserialize {
    fn post(self, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self>, Error>;
}

impl<T> RawPost for T where T: Post + _UpdateRels {
    fn post(self, rels: <T as RawUpdate>::Relationships) -> Result<PostResponse<T>, Error> {
        let attributes = try!(self.post());
        let id = attributes.id();
        let relationships = try!(<T as _UpdateRels>::update_rels(&id, rels));
        Ok(PostResponse {
            resource: ResourceObject {
                id: id,
                attributes: attributes,
                relationships: relationships,
            }
        })
    }
}

pub struct PostResponse<T> where T: RawFetch {
    pub resource: ResourceObject<T>,
}
