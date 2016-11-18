use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, RawReceived, CollectionResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Replace: Resource {
    type ReplaceFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn replace(data: Vec<Self>) -> Self::ReplaceFut;
}

pub trait RawReplace<I>: RawUpdate {
    type RawReplaceFut: IntoFuture<Item = CollectionResponse<I, Self>, Error = Error>;
    fn replace(received: Vec<RawReceived<Self, Self>>) -> Self::RawReplaceFut;
}

impl<I, T> RawReplace<I> for T where T: Replace + _UpdateRels {
    type RawReplaceFut = Result<CollectionResponse<I, T>, Error>;
    fn replace(received: Vec<RawReceived<Self, Self>>) -> Self::RawReplaceFut {
        let (data, rels) = split(received.into_iter().map(|r| (r.attributes, r.relationships)));
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

fn split<I: IntoIterator<Item = (T, U)>, T, U>(iter: I) -> (Vec<T>, Vec<U>) {
    let (mut vec1, mut vec2) = (vec![], vec![]);
    for (left, right) in iter {
        vec1.push(left);
        vec2.push(right);
    }
    (vec1, vec2)
}

