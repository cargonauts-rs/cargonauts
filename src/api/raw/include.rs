use std::cell::RefCell;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, RelationshipLinkage};
use presenter::ConvertInclude;

pub struct Include<I> {
    pub id: String,
    pub resource: &'static str,
    pub attributes: I,
    pub relationships: Option<Box<RefCell<Iterator<Item = (&'static str, RelationshipLinkage)>>>>,
}

impl<T, I> From<ResourceObject<T>> for Include<I>
where T: RawFetch,
      I: ConvertInclude<T> {
    fn from(resource: ResourceObject<T>) -> Include<I> {
        if resource.relationships.count() == 0 {
            Include {
                id: resource.id.to_string(),
                resource: T::resource(),
                attributes: I::convert(resource.attributes),
                relationships: None,
            }
        } else {
            Include {
                id: resource.id.to_string(),
                resource: T::resource(),
                attributes: I::convert(resource.attributes),
                relationships: Some(Box::new(RefCell::new(resource.relationships.into_iter()))),
            }
        }
    }
}
