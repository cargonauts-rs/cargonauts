use api::Resource;
use api::raw::{FetchRelationships, UpdateRelationships};
use api::raw::relationship::SerializeRelationships;
use BASE_URL;
use links::{LinkObject, make_link};
use repr::{Represent, Presenter, SerializeRepr};

pub trait RawFetch: Resource {
    type Relationships: for<'a> FetchRelationships<'a>;
}

pub trait RawUpdate: RawFetch {
    type Relationships: UpdateRelationships;
}

pub struct ResourceObject<T: RawFetch> {
    pub id: <T as Resource>::Id,
    pub attributes: T,
    pub relationships: <T as RawFetch>::Relationships,
}

impl<T: RawFetch> ResourceObject<T> {
    pub fn repr(self) -> ResourceRepr<T> {
        ResourceRepr {
            id: self.id,
            attributes: self.attributes.repr(),
            relationships: self.relationships,
        }
    }
}

pub struct ResourceRepr<T: RawFetch> {
    pub id: <T as Resource>::Id,
    pub attributes: <T as Resource>::Repr,
    pub relationships: <T as RawFetch>::Relationships,
}

impl<T: RawFetch> Represent for ResourceRepr<T> {
    fn repr<P: Presenter>(&self, presenter: &mut P) -> Result<(), P::Error> {
        let id = self.id.to_string();
        if self.relationships.count() == 0 {
            let mut state = presenter.serialize_map(Some(4))?;
            presenter.serialize_map_key(&mut state, "id")?;
            presenter.serialize_map_value(&mut state, &id)?;
            presenter.serialize_map_key(&mut state, "type")?;
            presenter.serialize_map_value(&mut state, T::resource())?;
            presenter.serialize_map_key(&mut state, "attributes")?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.attributes,
            })?;
            presenter.serialize_map_key(&mut state, "links")?;
            presenter.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, T::resource_plural(), &id])),
                related_link: None,
            })?;
            presenter.serialize_map_end(state)
        } else {
            let mut state = presenter.serialize_map(Some(5))?;
            presenter.serialize_map_key(&mut state, "id")?;
            presenter.serialize_map_value(&mut state, &id)?;
            presenter.serialize_map_key(&mut state, "type")?;
            presenter.serialize_map_value(&mut state, T::resource())?;
            presenter.serialize_map_key(&mut state, "attributes")?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.attributes,
            })?;
            presenter.serialize_map_key(&mut state, "relationships")?;
            presenter.serialize_map_value(&mut state, SerializeRelationships {
                resource: T::resource_plural(),
                id: &id,
                relationships: &self.relationships
            })?;
            presenter.serialize_map_key(&mut state, "links")?;
            presenter.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, T::resource_plural(), &id])),
                related_link: None,
            })?;
            presenter.serialize_map_end(state)
        }
    }
}

impl<T: RawFetch> Represent for Vec<ResourceRepr<T>> {
    fn repr<P: Presenter>(&self, presenter: &mut P) -> Result<(), P::Error> {
        let mut state = presenter.serialize_seq(Some(self.len()))?;
        for resource in self {
            presenter.serialize_seq_elt(&mut state, SerializeRepr {
                repr: resource,
            })?;
        }
        presenter.serialize_seq_end(state)
    }
}
