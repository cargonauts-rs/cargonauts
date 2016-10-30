use api::raw::{Include, Includes, RawFetch, ResourceRepr};
use BASE_URL;
use SerializeTo;
use Serializer;
use links::{make_link, LinkObject};

pub struct ResourceDocument<S: Serializer, T: RawFetch> {
    resource: ResourceRepr<T>,
    included: Includes<S>,
    self_link: String,
}

impl<S, T> ResourceDocument<S, T> where S: Serializer, T: RawFetch {
    pub fn new(resource: ResourceRepr<T>, included: Vec<Include<S>>) -> ResourceDocument<S, T> {
        let self_link = make_link(&[BASE_URL, T::resource_plural(), &resource.id.to_string()]);
        ResourceDocument {
            resource: resource,
            included: included.into(),
            self_link: self_link,
        }
    }
}

impl<S, T> SerializeTo<S> for ResourceDocument<S, T> where S: Serializer, T: RawFetch {
    fn serialize_to(&self, serializer: &mut S) -> Result<(), S::Error> {
        if self.included.is_empty() {
            let mut state = serializer.serialize_map(Some(2))?;
            serializer.serialize_map_key(&mut state, "data")?;
            serializer.serialize_map_value(&mut state, &self.resource)?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(3))?;
            serializer.serialize_map_key(&mut state, "data")?;
            serializer.serialize_map_value(&mut state, &self.resource)?;
            serializer.serialize_map_key(&mut state, "included")?;
            serializer.serialize_map_value(&mut state, &self.included)?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        }
    }
}
