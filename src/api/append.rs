use Deserialize;
use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, CollectionResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Append: Resource + Deserialize {
    type AppendFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn append(data: Vec<Self>) -> Self::AppendFut;
}

pub trait RawAppend<I>: RawUpdate + Deserialize {
    type RawAppendFut: IntoFuture<Item = CollectionResponse<I, Self>, Error = Error>;
    fn append(data: Vec<Self>, rels: Vec<<Self as RawUpdate>::Relationships>) -> Self::RawAppendFut;
}

impl<I, T> RawAppend<I> for T where T: Append + _UpdateRels {
    type RawAppendFut = Result<CollectionResponse<I, T>, Error>;
    fn append(data: Vec<Self>, rels: Vec<<Self as RawUpdate>::Relationships>) -> Self::RawAppendFut {
        let mut resources = vec![];
        for (resource, rels) in <Self as Append>::append(data).into_future().wait()?.into_iter().zip(rels) {
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
