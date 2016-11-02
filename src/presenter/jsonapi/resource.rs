use api::raw::{FetchRelationships, RawFetch, ResourceObject};
use BASE_URL;
use links::{LinkObject, make_link};
use repr::{Represent, SerializeRepr};
use presenter::jsonapi::rels::RelsObject;
use Serializer;

pub struct JsonApiResourceObject<'a, T: RawFetch>(pub &'a ResourceObject<T>);

impl<'a, T: RawFetch> Represent for JsonApiResourceObject<'a, T> {
    fn repr<S: Serializer>(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        let id = self.0.id.to_string();
        if self.0.relationships.count() == 0 {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, T::resource())?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.0.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, T::resource_plural(), &id])),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(5))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, T::resource())?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.0.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "relationships")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &RelsObject {
                    resource: T::resource_plural(),
                    id: &id,
                    relationships: &self.0.relationships
                },
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, T::resource_plural(), &id])),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        }
    }
}

pub struct JsonApiCollectionObject<T: RawFetch>(pub Vec<ResourceObject<T>>);

impl<T: RawFetch> Represent for JsonApiCollectionObject<T> {
    fn repr<S: Serializer>(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.0.len()))?;
        for resource in &self.0 {
            serializer.serialize_seq_elt(&mut state, SerializeRepr {
                repr: &JsonApiResourceObject(resource),
                field_set: field_set,
            })?;
        }
        serializer.serialize_seq_end(state)
    }
}
