use api;
use Serialize;
use Serializer;
use Value;
use _internal::{Resource, Wrapper};
use _internal::links::{make_link, LinkObject};

pub struct CollectionDocument<T: api::Resource> where Resource<T>: Wrapper<T> {
    resources: Vec<Resource<T>>,
    included: Vec<Value>,
    self_link: String,
}

impl<T> CollectionDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    pub fn new(resources: Vec<T>, params: &[String], base_url: &str) -> CollectionDocument<T> {
        let wrapped_resources = resources.into_iter().map(|resource| {
            Resource::wrap(resource, base_url)
        }).collect::<Vec<_>>();

        let included = wrapped_resources.iter().flat_map(|resource| {
            resource.include(params, base_url)
        }).collect();

        CollectionDocument {
            resources: wrapped_resources,
            included: included,
            self_link: make_link(&[base_url, T::resource()]),
        }
    }
}

impl<T> Serialize for CollectionDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if self.included.is_empty() {
            let mut state = try!(serializer.serialize_map(Some(2)));
            try!(serializer.serialize_map_key(&mut state, "data"));
            try!(serializer.serialize_map_value(&mut state, &self.resources));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        } else {
            let mut state = try!(serializer.serialize_map(Some(3)));
            try!(serializer.serialize_map_key(&mut state, "data"));
            try!(serializer.serialize_map_value(&mut state, &self.resources));
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
