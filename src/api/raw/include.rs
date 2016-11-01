use std::collections::BTreeMap;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, RelationshipLinkage};
use api::raw::relationship::ReprRels;
use BASE_URL;
use links::{LinkObject, make_link};
use repr::{RepresentWith, SerializeRepr};
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

impl<S: Serializer> RepresentWith<S> for Include<S> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        if self.relationships.is_empty() {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.resource)?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.attributes,
                field_set: field_set,
            })?;
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
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "relationships")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &ReprRels {
                    resource: self.resource,
                    id: &self.id,
                    relationships: &self.relationships
                },
                field_set: field_set,
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

impl<S: Serializer> RepresentWith<S> for Vec<Include<S>> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.len()))?;
        for include in self {
            serializer.serialize_seq_elt(&mut state, SerializeRepr {
                repr: include,
                field_set: field_set,
            })?;
        }
        serializer.serialize_seq_end(state)
    }
}
