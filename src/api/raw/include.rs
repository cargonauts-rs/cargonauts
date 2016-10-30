use std::collections::BTreeMap;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, RelationshipLinkage};
use api::raw::relationship::SerializeRelationships;
use BASE_URL;
use links::{LinkObject, make_link};
use SerializeTo;
use Serializer;

pub struct Include<S: Serializer> {
    pub id: String,
    pub resource: &'static str,
    pub attributes: Box<SerializeTo<S>>,
    pub relationships: BTreeMap<String, RelationshipLinkage>,
}

impl<S: Serializer, T: RawFetch> From<ResourceObject<T>> for Include<S> {
    fn from(resource: ResourceObject<T>) -> Include<S> {
        Include {
            id: resource.id.to_string(),
            resource: T::resource(),
            attributes: Box::new(resource.attributes.repr()),
            relationships: resource.relationships.iter().map(|(k, v)| (k.to_owned(), v.clone())).collect(),
        }
    }
}

impl<'a, S: Serializer> SerializeTo<S> for &'a Include<S> {
    fn serialize_to(&self, serializer: &mut S) -> Result<(), S::Error> {
        if self.relationships.is_empty() {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.resource)?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, &*self.attributes)?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, self.resource, &self.id.to_string()])),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(5))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.resource)?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, &*self.attributes)?;
            serializer.serialize_map_key(&mut state, "relationships")?;
            serializer.serialize_map_value(&mut state, SerializeRelationships {
                resource: self.resource,
                id: &self.id,
                relationships: &self.relationships
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, self.resource, &self.id.to_string()])),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        }
    }
}

pub struct Includes<S: Serializer> {
    includes: Vec<Include<S>>,
}

impl<S: Serializer> Includes<S> {
    pub fn is_empty(&self) -> bool {
        self.includes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.includes.len()
    }
}

impl<S: Serializer> From<Vec<Include<S>>> for Includes<S> {
    fn from(includes: Vec<Include<S>>) -> Includes<S> {
        Includes {
            includes: includes,
        }
    }
}

impl<'a, S: Serializer> SerializeTo<S> for &'a Includes<S> {
    fn serialize_to(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.includes.len()))?;
        for include in &self.includes {
            serializer.serialize_seq_elt(&mut state, include)?;
        }
        serializer.serialize_seq_end(state)
    }
}
