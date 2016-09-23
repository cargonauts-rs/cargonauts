use api;
use Serialize;
use Serializer;
use _internal::{Resource, Wrapper};
use _internal::identifier::Identifier;
use _internal::links::{make_link, LinkObject};

pub struct HasManyDocument<T: api::Resource> where Resource<T>: Wrapper<T> {
    identifiers: Vec<Identifier<T>>,
    self_link: String,
}

impl<T: api::Resource> HasManyDocument<T> where Resource<T>: Wrapper<T> {
    pub fn new(resources: Vec<T>, base_url: &str, base_resource: &str, id: &str) -> HasManyDocument<T> {
        let identifiers = resources.into_iter().map(Identifier::new).collect();
        HasManyDocument {
            identifiers: identifiers,
            self_link: make_link(&[base_url, base_resource, id, T::resource()]),
        }
    }
}

impl<T: api::Resource> Serialize for HasManyDocument<T> where Resource<T>: Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(2)));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, &self.identifiers));
        try!(serializer.serialize_map_key(&mut state, "links"));
        try!(serializer.serialize_map_value(&mut state, LinkObject {
            self_link: Some(&self.self_link),
            related_link: None,
        }));
        serializer.serialize_map_end(state)
    }
}

