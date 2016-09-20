use api;
use Serialize;
use Serializer;
use Value;
use _internal::{Resource, Wrapper};

pub struct CollectionDocument<T: api::Resource> where Resource<T>: Wrapper<T> {
    resources: Vec<Resource<T>>,
    included: Vec<Value>,
}

impl<T> CollectionDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    pub fn new(resources: Vec<T>, params: &[String]) -> CollectionDocument<T> {
        let wrapped_resources = resources.into_iter().map(Resource::wrap).collect::<Vec<_>>();
        let included = wrapped_resources.iter().flat_map(|resource| resource.include(params)).collect();
        CollectionDocument {
            resources: wrapped_resources,
            included: included,
        }
    }
}

impl<T> Serialize for CollectionDocument<T> where T: api::Resource, Resource<T>: Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(3)));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, &self.resources));
        try!(serializer.serialize_map_key(&mut state, "included"));
        try!(serializer.serialize_map_value(&mut state, &self.included));
        // TODO links
        serializer.serialize_map_end(state)
    }
}

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
