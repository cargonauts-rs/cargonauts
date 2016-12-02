use std::sync::Arc;

use api::{Resource, Entity, Error};
use api::raw::{RawResource, RawReceived, CollectionResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;
use futures::stream::{self, Stream};

pub trait Replace: Resource {
    type ReplaceFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn replace(data: Vec<Self>) -> Self::ReplaceFut;
}

pub trait RawReplace<I>: RawResource {
    type RawReplaceFut: Future<Item = CollectionResponse<I, Self>, Error = Error>;
    fn replace(received: Vec<RawReceived<Self, Self>>) -> Self::RawReplaceFut;
}

impl<I, T> RawReplace<I> for T where T: Replace + _UpdateRels, I: 'static {
    type RawReplaceFut = Box<Future<Item = CollectionResponse<I, T>, Error = Error>>;
    fn replace(received: Vec<RawReceived<Self, Self>>) -> Self::RawReplaceFut {
        let (data, rels) = split(received.into_iter().map(|r| (r.attributes, r.relationships)));
        Box::new(<Self as Replace>::replace(data).into_future().and_then(move |data| {
            stream::iter(data.into_iter().zip(rels).map(Ok)).and_then(move |(resource, rels)| {
                let entity = Entity::Resource(Arc::new(resource));
                <T as _UpdateRels>::update_rels(entity.clone(), rels).into_future().join(Ok(entity))
            }).fold(CollectionResponse::default(), |mut response, (rels, entity)| {
                match entity {
                    Entity::Resource(resource)  => response.resources.push(ResourceObject {
                        id: resource.id(),
                        attributes: Arc::try_unwrap(resource).ok().unwrap(),
                        relationships: rels,
                    }),
                    _                           => unreachable!()
                }
                Ok(response)
            })
        }))
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

