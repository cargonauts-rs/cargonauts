use api::Resource;
use api::raw::{ResourceObject, RawFetch};
use Serialize;
use Serializer;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Identifier {
    pub resource: &'static str,
    pub id: String,
}

impl Identifier {
    pub fn new<T: Resource>(id: &T::Id) -> Identifier {
        Identifier {
            resource: T::resource(),
            id: id.to_string(),
        }
    }
}

impl<'a, T: Resource + 'a> From<&'a T> for Identifier {
    fn from(resource: &'a T) -> Identifier {
        Identifier {
            resource: T::resource(),
            id: resource.id().to_string(),
        }
    }
}

impl<'a, T: RawFetch> From<&'a ResourceObject<T>> for Identifier {
    fn from(resource: &'a ResourceObject<T>) -> Identifier {
        Identifier {
            resource: T::resource(),
            id: resource.id.to_string(),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(2)));
        try!(serializer.serialize_map_key(&mut state, "id"));
        try!(serializer.serialize_map_value(&mut state, &self.id));
        try!(serializer.serialize_map_key(&mut state, "type"));
        try!(serializer.serialize_map_value(&mut state, self.resource));
        serializer.serialize_map_end(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use to_value;

    #[test]
    fn serialize_identifier() {
        let identifier = Identifier {
            resource: "identified",
            id: String::from("101"),
        };
        let expected = {
            let mut identifier = BTreeMap::new();
            identifier.insert(String::from("type"), to_value("identified"));
            identifier.insert(String::from("id"), to_value("101"));
            to_value(identifier)
        };
        assert_eq!(to_value(identifier), expected);
    }
}
