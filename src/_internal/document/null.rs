use links::LinkObject;
use super::JsonApi;
use repr::{Represent, Presenter};

pub struct NullDocument {
    self_link: String,
}

impl NullDocument {
    pub fn new(self_link: String) -> NullDocument {
        NullDocument {
            self_link: self_link,
        }
    }
}

impl Represent for NullDocument {
    fn repr<P: Presenter>(&self, presenter: &mut P) -> Result<(), P::Error> {
        let mut state = presenter.serialize_map(Some(3))?;
        presenter.serialize_map_key(&mut state, "links")?;
        presenter.serialize_map_value(&mut state, LinkObject {
            self_link: Some(&self.self_link),
            related_link: None,
        })?;
        presenter.serialize_map_key(&mut state, "data")?;
        presenter.serialize_map_value(&mut state, ())?;
        presenter.serialize_map_key(&mut state, "jsonapi")?;
        presenter.serialize_map_value(&mut state, JsonApi)?;
        presenter.serialize_map_end(state)
    }
}
