use api::Resource;
use api::raw::{FetchRelationships, UpdateRelationships};
use api::raw::relationship::SerializeRelationships;
use BASE_URL;
use links::{LinkObject, make_link};
use Serialize;
use Serializer;

pub trait RawFetch: Resource {
    type Relationships: for<'a> FetchRelationships<'a>;
}

pub trait RawUpdate: RawFetch {
    type Relationships: UpdateRelationships;
}

pub struct ResourceObject<T: RawFetch> {
    pub id: <T as Resource>::Id,
    pub attributes: T,
    pub relationships: <T as RawFetch>::Relationships,
}

impl<T: RawFetch> Serialize for ResourceObject<T> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let id = self.id.to_string();
        if self.relationships.count() == 0 {
            let mut state = try!(serializer.serialize_map(Some(4)));
            try!(serializer.serialize_map_key(&mut state, "id"));
            try!(serializer.serialize_map_value(&mut state, &id));
            try!(serializer.serialize_map_key(&mut state, "type"));
            try!(serializer.serialize_map_value(&mut state, T::resource()));
            try!(serializer.serialize_map_key(&mut state, "attributes"));
            try!(serializer.serialize_map_value(&mut state, &self.attributes));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, T::resource_plural(), &id])),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        } else {
            let mut state = try!(serializer.serialize_map(Some(5)));
            try!(serializer.serialize_map_key(&mut state, "id"));
            try!(serializer.serialize_map_value(&mut state, &id));
            try!(serializer.serialize_map_key(&mut state, "type"));
            try!(serializer.serialize_map_value(&mut state, T::resource()));
            try!(serializer.serialize_map_key(&mut state, "attributes"));
            try!(serializer.serialize_map_value(&mut state, &self.attributes));
            try!(serializer.serialize_map_key(&mut state, "relationships"));
            try!(serializer.serialize_map_value(&mut state, SerializeRelationships {
                resource: T::resource_plural(),
                id: &id,
                relationships: &self.relationships
            }));
            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, LinkObject {
                self_link: Some(&make_link(&[BASE_URL, T::resource_plural(), &id])),
                related_link: None,
            }));
            serializer.serialize_map_end(state)
        }
    }
}
