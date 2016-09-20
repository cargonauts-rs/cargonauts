use std::marker::PhantomData;

use api;
use Serialize;
use Serializer;
use _internal::identifier::Identifier;
use _internal::links::{make_link, LinkObject};

pub struct HasOne<'a, T: api::HasOne<R>, R: api::Resource> where T::Id: 'a {
    id: &'a T::Id,
    related_link: String,
    _spoopy: PhantomData<R>,
}

impl<'a, T: api::HasOne<R>, R: api::Resource> HasOne<'a, T, R> {
    pub fn new(id: &'a T::Id, base_url: &str) -> HasOne<'a, T, R> {
        let link = make_link(&[base_url, T::resource(), &id.to_string(), &R::resource()]);
        HasOne {
            id: id,
            related_link: link,
            _spoopy: PhantomData,
        }
    }
}

impl<'a, T: api::HasOne<R>, R: api::Resource> Serialize for HasOne<'a, T, R> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some(related) = T::has_one(&self.id) {
            let mut state = try!(serializer.serialize_map(Some(2)));

            // TODO do not include identifiers unless those links were included in this document
            try!(serializer.serialize_map_key(&mut state, "data"));
            try!(serializer.serialize_map_value(&mut state, &Identifier::new(related)));

            try!(serializer.serialize_map_key(&mut state, "links"));
            try!(serializer.serialize_map_value(&mut state, &LinkObject {
                self_link: None,
                related_link: Some(&self.related_link),
            }));

            serializer.serialize_map_end(state)
        } else { Ok(()) }
    }
}

pub struct HasMany<'a, T: api::HasMany<R>, R: api::Resource> where T::Id: 'a {
    id: &'a T::Id,
    related_link: String,
    _spoopy: PhantomData<R>,
}

impl<'a, T: api::HasMany<R>, R: api::Resource> HasMany<'a, T, R> {
    pub fn new(id: &'a T::Id, base_url: &str) -> HasMany<'a, T, R> {
        let link = make_link(&[base_url, T::resource(), &id.to_string(), &R::resource()]);
        HasMany {
            id: id,
            related_link: link,
            _spoopy: PhantomData,
        }
    }
}

impl<'a, T: api::HasMany<R>, R: api::Resource> Serialize for HasMany<'a, T, R> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(2)));
        let resources = T::has_many(&self.id);

        // TODO do not include identifiers unless those links were included in this document
        try!(serializer.serialize_map_key(&mut state, "data"));
        let identifiers = resources.into_iter().map(Identifier::new).collect::<Vec<_>>();
        try!(serializer.serialize_map_value(&mut state, identifiers));

        try!(serializer.serialize_map_key(&mut state, "links"));
        try!(serializer.serialize_map_value(&mut state, &LinkObject {
            self_link: None,
            related_link: Some(&self.related_link),
        }));

        serializer.serialize_map_end(state)
    }
}
