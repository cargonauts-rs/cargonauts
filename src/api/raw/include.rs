use std::collections::BTreeMap;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, Relationship};
use api::raw::relationship::SerializeRelationships;
use BASE_URL;
use json;
use links::{LinkObject, make_link};
use Serialize;
use Serializer;

pub struct Include {
    pub id: String,
    pub resource: &'static str,
    pub attributes: json::Value,
    pub relationships: BTreeMap<String, Relationship>,
}

impl<T: RawFetch> From<ResourceObject<T>> for Include {
    fn from(resource: ResourceObject<T>) -> Include {
        Include {
            id: resource.id.to_string(),
            resource: T::resource(),
            attributes: json::to_value(resource.attributes.repr()),
            relationships: resource.relationships.iter().map(|(k, v)| (k.to_owned(), v.clone())).collect(),
        }
    }
}

impl Serialize for Include {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if self.relationships.is_empty() {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.resource)?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, &self.attributes)?;
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
            serializer.serialize_map_value(&mut state, &self.attributes)?;
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
