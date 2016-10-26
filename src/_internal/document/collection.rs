use api::raw::{Include, RawFetch, ResourceRepr};
use BASE_URL;
use Serialize;
use Serializer;
use links::{make_link, LinkObject};

pub struct CollectionDocument<T: RawFetch> {
    resources: Vec<ResourceRepr<T>>,
    included: Vec<Include>,
    self_link: String,
}

impl<T> CollectionDocument<T> where T: RawFetch {
    pub fn new(resources: Vec<ResourceRepr<T>>, included: Vec<Include>) -> CollectionDocument<T> {
        let self_link = make_link(&[BASE_URL, T::resource_plural()]);
        CollectionDocument {
            resources: resources,
            included: included,
            self_link: self_link,
        }
    }
}

impl<T> Serialize for CollectionDocument<T> where T: RawFetch {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if self.included.is_empty() {
            let mut state = serializer.serialize_map(Some(2))?;
            serializer.serialize_map_key(&mut state, "data")?;
            serializer.serialize_map_value(&mut state, &self.resources)?;
            serializer.serialize_map_key(&mut state, "links")?;
            serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            })?;
            serializer.serialize_map_end(state)
        } else {
            let mut state = serializer.serialize_map(Some(3))?;
            serializer.serialize_map_key(&mut state, "data")?;
            serializer.serialize_map_value(&mut state, &self.resources)?;
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
