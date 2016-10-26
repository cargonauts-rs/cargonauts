use BASE_URL;
use Serialize;
use Serializer;
use api::raw::Relationship;
use links::{make_link, LinkObject};
use super::JsonApi;

pub struct RelDocument {
    base_resource: &'static str,
    base_id: String,
    relation: &'static str,
    rel: Relationship,
}

impl RelDocument {
    pub fn new(rel: Relationship, base: &'static str, id: String, relation: &'static str) -> RelDocument {
        RelDocument {
            base_resource: base,
            base_id: id,
            relation: relation,
            rel: rel,
        }
    }
}

impl Serialize for RelDocument {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, LinkObject {
            self_link: Some(&make_link(&[
                BASE_URL,
                self.base_resource,
                &self.base_id,
                "relationships",
                self.relation,
            ])),
            related_link: Some(&make_link(&[
                BASE_URL,
                self.base_resource,
                &self.base_id,
                self.relation,
            ])),
        })?;
        serializer.serialize_map_key(&mut state, "data")?;
        match self.rel {
            Relationship::One(ref identifier)   => {
                serializer.serialize_map_value(&mut state, identifier)?;
            }
            Relationship::Many(ref identifiers) => {
                serializer.serialize_map_value(&mut state, identifiers)?;
            }
        }
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApi)?;
        serializer.serialize_map_end(state)
    }
}
