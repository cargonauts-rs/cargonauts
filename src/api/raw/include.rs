use std::cell::RefCell;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, RelationshipLinkage};
use repr::RepresentWith;
use Serializer;

pub struct Include<S: Serializer> {
    pub id: String,
    pub resource: &'static str,
    pub attributes: Box<RepresentWith<S>>,
    pub relationships: Option<Box<RefCell<Iterator<Item = (&'static str, RelationshipLinkage)>>>>,
}

impl<S: Serializer, T: RawFetch> From<ResourceObject<T>> for Include<S> {
    fn from(resource: ResourceObject<T>) -> Include<S> {
        if resource.relationships.count() == 0 {
            Include {
                id: resource.id.to_string(),
                resource: T::resource(),
                attributes: Box::new(resource.attributes),
                relationships: None,
            }
        } else {
            Include {
                id: resource.id.to_string(),
                resource: T::resource(),
                attributes: Box::new(resource.attributes),
                relationships: Some(Box::new(RefCell::new(resource.relationships.into_iter()))),
            }
        }
    }
}
