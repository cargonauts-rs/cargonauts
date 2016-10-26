use Deserialize;
use api::{Resource, Error, Entity};
use api::raw::{RawUpdate, ResourceObject, RawFetch};
use _internal::_UpdateRels;

pub trait Post: Resource where Self::Repr: Deserialize {
    fn post(repr: Self::Repr) -> Result<Self, Error>;
}

pub trait RawPost: RawUpdate where Self::Repr: Deserialize {
    fn post(repr: Self::Repr, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self>, Error>;
}

impl<T> RawPost for T where T: Post + _UpdateRels, T::Repr: Deserialize {
    fn post(repr: Self::Repr, rels: <T as RawUpdate>::Relationships) -> Result<PostResponse<T>, Error> {
        let entity = Entity::Resource(try!(<Self as Post>::post(repr)));
        let relationships = try!(<T as _UpdateRels>::update_rels(&entity, rels));
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(PostResponse {
            resource: ResourceObject {
                id: resource.id(),
                attributes: resource,
                relationships: relationships,
            }
        })
    }
}

pub struct PostResponse<T> where T: RawFetch {
    pub resource: ResourceObject<T>,
}
