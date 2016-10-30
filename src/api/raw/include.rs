use std::collections::BTreeMap;

use api::raw::{RawFetch, FetchRelationships, ResourceObject, RelationshipLinkage};
use api::raw::relationship::SerializeRelationships;
use BASE_URL;
use links::{LinkObject, make_link};
use presenter::{Presenter, RepresentWith, SerializeRepr};

pub struct Include<P: Presenter> {
    pub id: String,
    pub resource: &'static str,
    pub attributes: Box<RepresentWith<P>>,
    pub relationships: BTreeMap<String, RelationshipLinkage>,
}

impl<P: Presenter, T: RawFetch> From<ResourceObject<T>> for Include<P> {
    fn from(resource: ResourceObject<T>) -> Include<P> {
        Include {
            id: resource.id.to_string(),
            resource: T::resource(),
            attributes: Box::new(resource.attributes.repr()),
            relationships: resource.relationships.iter().map(|(k, v)| (k.to_owned(), v.clone())).collect(),
        }
    }
}

impl<P: Presenter> RepresentWith<P> for Include<P> {
    fn repr_with(&self, presenter: &mut P) -> Result<(), P::Error> {
        if self.relationships.is_empty() {
            let mut state = presenter.serialize_map(Some(4))?;
            presenter.serialize_map_key(&mut state, "id")?;
            presenter.serialize_map_value(&mut state, &self.id)?;
            presenter.serialize_map_key(&mut state, "type")?;
            presenter.serialize_map_value(&mut state, self.resource)?;
            presenter.serialize_map_key(&mut state, "attributes")?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.attributes,
            })?;
            presenter.serialize_map_key(&mut state, "links")?;
            presenter.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, self.resource, &self.id.to_string()])),
                related_link: None,
            })?;
            presenter.serialize_map_end(state)
        } else {
            let mut state = presenter.serialize_map(Some(5))?;
            presenter.serialize_map_key(&mut state, "id")?;
            presenter.serialize_map_value(&mut state, &self.id)?;
            presenter.serialize_map_key(&mut state, "type")?;
            presenter.serialize_map_value(&mut state, self.resource)?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &*self.attributes,
            })?;
            presenter.serialize_map_key(&mut state, "relationships")?;
            presenter.serialize_map_value(&mut state, SerializeRelationships {
                resource: self.resource,
                id: &self.id,
                relationships: &self.relationships
            })?;
            presenter.serialize_map_key(&mut state, "links")?;
            presenter.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, self.resource, &self.id.to_string()])),
                related_link: None,
            })?;
            presenter.serialize_map_end(state)
        }
    }
}

impl<P: Presenter> RepresentWith<P> for Vec<Include<P>> {
    fn repr_with(&self, presenter: &mut P) -> Result<(), P::Error> {
        let mut state = presenter.serialize_seq(Some(self.len()))?;
        for include in self {
            presenter.serialize_seq_elt(&mut state, SerializeRepr {
                repr: include,
            })?;
        }
        presenter.serialize_seq_end(state)
    }
}
