use std::collections::BTreeMap;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, RelationshipLinkage};
use repr::RepresentWith;
use Serializer;

pub struct Include<S: Serializer> {
    pub id: String,
    pub resource: &'static str,
    pub attributes: Box<RepresentWith<S>>,
    pub relationships: BTreeMap<String, RelationshipLinkage>,
}

impl<S: Serializer, T: RawFetch> From<ResourceObject<T>> for Include<S> {
    fn from(resource: ResourceObject<T>) -> Include<S> {
        Include {
            id: resource.id.to_string(),
            resource: T::resource(),
            attributes: Box::new(resource.attributes),
            relationships: resource.relationships.iter().map(|(k, v)| (k.to_owned(), v.clone())).collect(),
        }
    }
}
