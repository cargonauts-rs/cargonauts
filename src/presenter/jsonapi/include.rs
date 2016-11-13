use api::raw::Include;
use presenter::ConvertInclude;
use presenter::jsonapi::links::LinkObject;
use presenter::jsonapi::rels::IncludeRelsObject;
use repr::{RepresentWith, SerializeRepr};
use router::MakeLinks;
use Serializer;

pub struct JsonApiInclude<S: Serializer>(Box<RepresentWith<S>>);

impl<T: RepresentWith<S> + 'static, S: Serializer> ConvertInclude<T> for JsonApiInclude<S> {
    fn convert(attributes: T) -> JsonApiInclude<S> {
        JsonApiInclude(Box::new(attributes))
    }
}

struct IncludeObject<'a, S: Serializer + 'a, L: MakeLinks + 'a> {
    pub include: &'a Include<JsonApiInclude<S>>,
    pub linker: &'a L,
}

impl<'a, S: Serializer, L: MakeLinks> RepresentWith<S> for IncludeObject<'a, S, L> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        if let Some(relationships) = self.include.relationships.as_ref() {
            let mut state = serializer.serialize_map(Some(5))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.include.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.include.resource)?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.include.attributes.0,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "relationships")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &IncludeRelsObject {
                    resource: self.include.resource,
                    id: &self.include.id,
                    relationships: &*relationships,
                    linker: &self.linker,
                },
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.linker.resource(self.include.resource, &self.include.id)),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, &self.include.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, self.include.resource)?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.include.attributes.0,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.linker.resource(self.include.resource, &self.include.id)),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        }
    }
}

pub struct IncludesObject<'a, S: Serializer + 'a, L: MakeLinks + 'a> {
    pub includes: &'a [Include<JsonApiInclude<S>>],
    pub linker: &'a L,
}

impl<'a, S: Serializer, L: MakeLinks> RepresentWith<S> for IncludesObject<'a, S, L> {
    fn repr_with(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.includes.len()))?;
        for include in self.includes {
            serializer.serialize_seq_elt(&mut state, SerializeRepr {
                repr: &IncludeObject {
                    include: include,
                    linker: self.linker,
                },
                field_set: field_set,
            })?;
        }
        serializer.serialize_seq_end(state)
    }
}
