use api::raw::{Include, RawFetch, ResourceObject};
use BASE_URL;
use Serialize;
use Serializer;
use links::{make_link, LinkObject};

pub struct ResourceDocument<T: RawFetch> {
    resource: ResourceObject<T>,
    included: Vec<Include>,
    self_link: String,
}

impl<T> ResourceDocument<T> where T: RawFetch {
    pub fn new(resource: ResourceObject<T>, included: Vec<Include>) -> ResourceDocument<T> {
        let self_link = make_link(&[BASE_URL, T::resource_plural(), &resource.id.to_string()]);
        ResourceDocument {
            resource: resource,
            included: included,
            self_link: self_link,
        }
    }
}

impl<T> Serialize for ResourceDocument<T> where T: RawFetch {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if self.included.is_empty() {
            let mut state = try!(serializer.serialize_map(Some(2)));
            try!(serializer.serialize_map_key(&mut state, "data"));
            try!(serializer.serialize_map_value(&mut state, &self.resource));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        } else {
            let mut state = try!(serializer.serialize_map(Some(3)));
            try!(serializer.serialize_map_key(&mut state, "data"));
            try!(serializer.serialize_map_value(&mut state, &self.resource));
            try!(serializer.serialize_map_key(&mut state, "included"));
            try!(serializer.serialize_map_value(&mut state, &self.included));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        }
    }
}
