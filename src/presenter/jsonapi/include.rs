use api::raw::Include;
use BASE_URL;
use links::{LinkObject, make_link};
use presenter::jsonapi::rels::IncludeRelsObject;
use repr::{RepresentWith, SerializeRepr};
use Serializer;

struct IncludeObject<'a, S: Serializer + 'a>(&'a Include<S>);

impl<'a, S: Serializer> RepresentWith<S> for IncludeObject<'a, S> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        if let Some(relationships) = self.0.relationships.as_ref() {
            let mut state = serializer.serialize_map(Some(5))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.0.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.0.resource)?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.0.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "relationships")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &IncludeRelsObject {
                    resource: self.0.resource,
                    id: &self.0.id,
                    relationships: &*relationships
                },
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, self.0.resource, &self.0.id.to_string()])),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.0.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.0.resource)?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.0.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, self.0.resource, &self.0.id.to_string()])),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        }
    }
}

pub struct IncludesObject<S: Serializer>(pub Vec<Include<S>>);

impl<S: Serializer> RepresentWith<S> for IncludesObject<S> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.0.len()))?;
        for include in &self.0 {
            serializer.serialize_seq_elt(&mut state, SerializeRepr {
                repr: &IncludeObject(include),
                field_set: field_set,
            })?;
        }
        serializer.serialize_seq_end(state)
    }
}
