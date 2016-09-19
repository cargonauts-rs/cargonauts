use api;
use Serialize;
use Serializer;
use Value;
use _internal::Wrapper;

pub struct ResourceDocument<T: api::Resource> where Resource<T>: Wrapper<T> {
    resource: Resource<T>,
    included: Vec<Value>,
}

impl<T> ResourceDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    pub fn new(resource: T, params: &[String]) -> ResourceDocument<T> {
        let wrapped_resource = Resource::wrap(resource);
        let included = wrapped_resource.include(params);
        ResourceDocument {
            resource: wrapped_resource,
            included: included,
        }
    }
}

impl<T> Serialize for ResourceDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(3)));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, &self.resource));
        try!(serializer.serialize_map_key(&mut state, "included"));
        try!(serializer.serialize_map_value(&mut state, &self.included));
        // TODO links
        serializer.serialize_map_end(state)
    }
}


pub struct Resource<T: api::Resource> {
    attributes: T,
    links: bool,
}

impl<T: api::Resource> Resource<T> {
    pub fn wrap(resource: T) -> Resource<T> {
        Resource {
            attributes: resource,
            links: false,
        }
    }

    pub fn id(&self) -> T::Id {
        self.attributes.id()
    }
}

impl<T: api::Resource> Serialize for Resource<T> where Resource<T>: Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some(related) = self.related() {
            let mut state = try!(serializer.serialize_map(Some(4)));
            try!(serializer.serialize_map_key(&mut state, "type"));
            try!(serializer.serialize_map_value(&mut state, T::resource()));
            try!(serializer.serialize_map_key(&mut state, "id"));
            try!(serializer.serialize_map_value(&mut state, self.attributes.id().to_string()));
            try!(serializer.serialize_map_key(&mut state, "attributes"));
            try!(serializer.serialize_map_value(&mut state, &self.attributes));
            try!(serializer.serialize_map_key(&mut state, "relationships"));
            try!(serializer.serialize_map_value(&mut state, &related));
            if self.links {
                //TODO links
                unimplemented!()
            }
            serializer.serialize_map_end(state)
        } else {
            let mut state = try!(serializer.serialize_map(Some(3)));
            try!(serializer.serialize_map_key(&mut state, "type"));
            try!(serializer.serialize_map_value(&mut state, T::resource()));
            try!(serializer.serialize_map_key(&mut state, "id"));
            try!(serializer.serialize_map_value(&mut state, self.attributes.id().to_string()));
            try!(serializer.serialize_map_key(&mut state, "attributes"));
            try!(serializer.serialize_map_value(&mut state, &self.attributes));
            // TODO links?
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
            fn related(&self) -> Option<()> {
                None
            }
            fn include(&self, _: &[String]) -> Vec<Value> {
                vec![]
            }
        }

        #[test]
        fn serialize_resource() {
            let mut attributes = BTreeMap::new();
            attributes.insert(String::from("MOCK_KEY"), Value::String(String::from("MOCK_VALUE")));
            let mut resource = BTreeMap::new();
            resource.insert(String::from("attributes"), Value::Object(attributes));
            resource.insert(String::from("type"), Value::String(String::from("MOCK")));
            resource.insert(String::from("id"), Value::String(String::from("ID")));
            assert_eq!(to_value(Resource::wrap(MockResource)), Value::Object(resource));
        }
    }
}
