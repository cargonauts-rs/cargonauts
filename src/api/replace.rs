use Deserialize;
use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, CollectionResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Replace: Resource + Deserialize {
    type ReplaceFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn replace(data: Vec<Self>) -> Self::ReplaceFut;
}

pub trait RawReplace<I>: RawUpdate + Deserialize {
    type RawReplaceFut: IntoFuture<Item = CollectionResponse<I, Self>, Error = Error>;
    fn replace(data: Vec<Self>, rels: Vec<<Self as RawUpdate>::Relationships>) -> Self::RawReplaceFut;
}

impl<I, T> RawReplace<I> for T where T: Replace + _UpdateRels {
    type RawReplaceFut = Result<CollectionResponse<I, T>, Error>;
    fn replace(data: Vec<Self>, rels: Vec<<Self as RawUpdate>::Relationships>) -> Self::RawReplaceFut {
        let mut resources = vec![];
        for (resource, rels) in <Self as Replace>::replace(data).into_future().wait()?.into_iter().zip(rels) {
            let entity = Entity::Resource(resource);
            let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
            match entity {
                Entity::Resource(resource)  => resources.push(ResourceObject {
                    id: resource.id(),
                    attributes: resource,
                    relationships: relationships,
                }),
                _                           => unreachable!()
            }
        }
        Ok(CollectionResponse {
            resources: resources,
            includes: vec![],
        })
    }
}
