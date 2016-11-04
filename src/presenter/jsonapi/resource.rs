use api::raw::{FetchRelationships, RawFetch, ResourceObject};
use repr::{Represent, SerializeRepr};
use presenter::jsonapi::links::LinkObject;
use presenter::jsonapi::rels::RelsObject;
use Serializer;
use router::Linker;

pub struct JsonApiResourceObject<'a, T: RawFetch, L: Linker + 'a> {
    pub resource: &'a ResourceObject<T>,
    pub linker: &'a L,
    pub id: &'a str,
    pub self_link: &'a str,
}

impl<'a, T: RawFetch + Represent, L: Linker> Represent for JsonApiResourceObject<'a, T, L> {
    fn repr<S: Serializer>(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        if self.resource.relationships.count() == 0 {
            let mut state = serializer.serialize_map(Some(4))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, self.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, T::resource())?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.resource.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(self.self_link),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(5))?;
            serializer.serialize_map_key(&mut state, "id")?;
            serializer.serialize_map_value(&mut state, self.id)?;
            serializer.serialize_map_key(&mut state, "type")?;
            serializer.serialize_map_value(&mut state, T::resource())?;
            serializer.serialize_map_key(&mut state, "attributes")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.resource.attributes,
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "relationships")?;
            serializer.serialize_map_value(&mut state, SerializeRepr {
                repr: &RelsObject {
                    resource: T::resource_plural(),
                    id: self.id,
                    relationships: &self.resource.relationships,
                    linker: self.linker,
                },
                field_set: field_set,
            })?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(self.self_link),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        }
    }
}

pub struct JsonApiCollectionObject<'a, T: RawFetch, L: Linker + 'a> {
    pub resources: &'a [ResourceObject<T>],
    pub linker: &'a L,
}

impl<'a, T: RawFetch + Represent, L: Linker> Represent for JsonApiCollectionObject<'a, T, L> {
    fn repr<S: Serializer>(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        let mut state = serializer.serialize_seq(Some(self.resources.len()))?;
        for resource in self.resources {
            let id = resource.id.to_string();
            let self_link = self.linker.resource(T::resource_plural(), &id);
            serializer.serialize_seq_elt(&mut state, SerializeRepr {
                repr: &JsonApiResourceObject {
                    resource: resource,
                    linker: self.linker,
                    id: &id,
                    self_link: &self_link,
                },
                field_set: field_set,
            })?;
        }
        serializer.serialize_seq_end(state)
    }
}
