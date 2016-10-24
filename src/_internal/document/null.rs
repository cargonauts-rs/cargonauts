use Serialize;
use Serializer;
use links::LinkObject;
use super::JsonApi;

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

impl Serialize for NullDocument {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(3)));
        try!(serializer.serialize_map_key(&mut state, "links"));
        try!(serializer.serialize_map_value(&mut state, LinkObject {
            self_link: Some(&self.self_link),
            related_link: None,
        }));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, ()));
        try!(serializer.serialize_map_key(&mut state, "jsonapi"));
        try!(serializer.serialize_map_value(&mut state, JsonApi));
        serializer.serialize_map_end(state)
    }
}
