use BASE_URL;
use api::raw::Relationship;
use links::{make_link, LinkObject};
use super::JsonApi;
use presenter::{Represent, Presenter};

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

impl Represent for RelDocument {
    fn repr<P: Presenter>(&self, presenter: &mut P) -> Result<(), P::Error> {
        let mut state = presenter.serialize_map(Some(3))?;
        presenter.serialize_map_key(&mut state, "links")?;
        presenter.serialize_map_value(&mut state, LinkObject {
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
        presenter.serialize_map_key(&mut state, "data")?;
        match self.rel {
            Relationship::One(ref identifier)   => {
                presenter.serialize_map_value(&mut state, identifier)?;
            }
            Relationship::Many(ref identifiers) => {
                presenter.serialize_map_value(&mut state, identifiers)?;
            }
        }
        presenter.serialize_map_key(&mut state, "jsonapi")?;
        presenter.serialize_map_value(&mut state, JsonApi)?;
        presenter.serialize_map_end(state)
    }
}
