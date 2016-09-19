use api;
use Serialize;
use Serializer;

pub struct Identifier<T: api::Resource> {
    resource: T,
}

impl<T: api::Resource> Identifier<T> {
    pub fn new(resource: T) -> Identifier<T> {
        Identifier {
            resource: resource,
        }
    }
}

impl<T: api::Resource> Serialize for Identifier<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(2)));
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, T::resource()));
        try!(serializer.serialize_map_key(&mut state, "id"));
        try!(serializer.serialize_map_value(&mut state, self.resource.id().to_string()));
        serializer.serialize_map_end(state)
    }
}
