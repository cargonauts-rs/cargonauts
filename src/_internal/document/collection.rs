use api::raw::{Include, RawFetch, ResourceRepr};
use BASE_URL;
use presenter::{Presenter, RepresentWith, SerializeRepr};
use links::{make_link, LinkObject};

pub struct CollectionDocument<P: Presenter, T: RawFetch> {
    resources: Vec<ResourceRepr<T>>,
    included: Vec<Include<P>>,
    self_link: String,
}

impl<P, T> CollectionDocument<P, T> where T: RawFetch, P: Presenter {
    pub fn new(resources: Vec<ResourceRepr<T>>, included: Vec<Include<P>>) -> CollectionDocument<P, T> {
        let self_link = make_link(&[BASE_URL, T::resource_plural()]);
        CollectionDocument {
            resources: resources,
            included: included,
            self_link: self_link,
        }
    }
}

impl<P: Presenter, T: RawFetch> RepresentWith<P> for CollectionDocument<P, T> {
    fn repr_with(&self, presenter: &mut P) -> Result<(), P::Error> {
        if self.included.is_empty() {
            let mut state = presenter.serialize_map(Some(2))?;
            presenter.serialize_map_key(&mut state, "data")?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.resources,
            })?;
            presenter.serialize_map_key(&mut state, "links")?;
            presenter.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            })?;
            presenter.serialize_map_end(state)
        } else {
            let mut state = presenter.serialize_map(Some(3))?;
            presenter.serialize_map_key(&mut state, "data")?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.resources,
            })?;
            presenter.serialize_map_key(&mut state, "included")?;
            presenter.serialize_map_value(&mut state, SerializeRepr {
                repr: &self.included,
            })?;
            presenter.serialize_map_key(&mut state, "links")?;
            presenter.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            })?;
            presenter.serialize_map_end(state)
        }
    }
}
