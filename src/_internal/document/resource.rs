use api;
use Serialize;
use Serializer;
use Value;
use _internal::{Resource, Wrapper};
use _internal::links::{make_link, LinkObject};

pub struct ResourceDocument<T: api::Resource> where Resource<T>: Wrapper<T> {
    resource: Resource<T>,
    included: Vec<Value>,
    self_link: String,
}

impl<T> ResourceDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    pub fn new(resource: T, params: &[String], base_url: &str) -> ResourceDocument<T> {
        let wrapped_resource = Resource::wrap(resource, base_url);
        let included = wrapped_resource.include(params, base_url);
        ResourceDocument {
            included: included,
            self_link: make_link(&[base_url, T::resource(), &wrapped_resource.id().to_string()]),
            resource: wrapped_resource,
        }
    }
}

impl<T> Serialize for ResourceDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
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
