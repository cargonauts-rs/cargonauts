use api;
use Serialize;
use Serializer;
use _internal::Wrapper;
use _internal::links::{make_link, LinkObject};

pub struct Resource<T: api::Resource> {
    attributes: T,
    base_url: String,
    self_link: String,
}

impl<T: api::Resource> Resource<T> {
    pub fn wrap(resource: T, base_url: &str) -> Resource<T> {
        Resource {
            self_link: make_link(&[base_url, T::resource(), &resource.id().to_string()]),
            base_url: base_url.to_string(),
            attributes: resource,
        }
    }

    pub fn id(&self) -> T::Id {
        self.attributes.id()
    }
}

impl<T: api::Resource + Serialize> Serialize for Resource<T> where Resource<T>: Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some(related) = self.related(&self.base_url) {
            let mut state = try!(serializer.serialize_map(Some(5)));
            try!(serializer.serialize_map_key(&mut state, "type"));
            try!(serializer.serialize_map_value(&mut state, T::resource()));
            try!(serializer.serialize_map_key(&mut state, "id"));
            try!(serializer.serialize_map_value(&mut state, self.id().to_string()));
            try!(serializer.serialize_map_key(&mut state, "attributes"));
            try!(serializer.serialize_map_value(&mut state, &self.attributes));
            try!(serializer.serialize_map_key(&mut state, "relationships"));
            try!(serializer.serialize_map_value(&mut state, &related));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, &LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        } else {
            let mut state = try!(serializer.serialize_map(Some(4)));
            try!(serializer.serialize_map_key(&mut state, "type"));
            try!(serializer.serialize_map_value(&mut state, T::resource()));
            try!(serializer.serialize_map_key(&mut state, "id"));
            try!(serializer.serialize_map_value(&mut state, self.id().to_string()));
            try!(serializer.serialize_map_key(&mut state, "attributes"));
            try!(serializer.serialize_map_value(&mut state, &self.attributes));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, &LinkObject {
                self_link: Some(&self.self_link),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod no_rels {
        use std::collections::BTreeMap;

        use api;
        use Serialize;
        use Serializer;
        use to_value;
        use Value;
        use _internal::Wrapper;

        use super::*;

        struct MockResource;

        impl api::Resource for MockResource {
            type Id = String;
            fn id(&self) -> String {
                String::from("ID")
            }

            fn resource() -> &'static str {
                "MOCK"
            }
        }

        impl Serialize for MockResource {
            fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
                let mut state = try!(serializer.serialize_map(Some(1)));
                try!(serializer.serialize_map_key(&mut state, "MOCK_KEY"));
                try!(serializer.serialize_map_value(&mut state, "MOCK_VALUE"));
                serializer.serialize_map_end(state)
            }
        }

        impl Wrapper<MockResource> for Resource<MockResource> {
            type Relationships = ();
            fn related(&self, _: &str) -> Option<()> {
                None
            }
            fn include(&self, _: &[String], _: &str) -> Vec<Value> {
                vec![]
            }
        }

        #[test]
        fn serialize_resource() {
            let mut attributes = BTreeMap::new();
            attributes.insert(String::from("MOCK_KEY"), Value::String(String::from("MOCK_VALUE")));
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), Value::String(String::from("BASE_URL/MOCK/ID")));
            let mut resource = BTreeMap::new();
            resource.insert(String::from("attributes"), Value::Object(attributes));
            resource.insert(String::from("links"), Value::Object(links));
            resource.insert(String::from("type"), Value::String(String::from("MOCK")));
            resource.insert(String::from("id"), Value::String(String::from("ID")));
            assert_eq!(to_value(Resource::wrap(MockResource, "BASE_URL")), Value::Object(resource));
        }
    }
}
