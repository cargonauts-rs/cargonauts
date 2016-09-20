use std::marker::PhantomData;

use api;
use Serialize;
use Serializer;
use _internal::identifier::Identifier;

pub struct HasOne<'a, T: api::HasOne<R>, R: api::Resource> where T::Id: 'a {
    id: &'a T::Id,
    _spoopy: PhantomData<R>,
}

impl<'a, T: api::HasOne<R>, R: api::Resource> HasOne<'a, T, R> {
    pub fn new(id: &'a T::Id) -> HasOne<'a, T, R> {
        HasOne {
            id: id,
            _spoopy: PhantomData,
        }
    }
}

impl<'a, T: api::HasOne<R>, R: api::Resource> Serialize for HasOne<'a, T, R> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some(related) = T::has_one(&self.id) {
            let mut state = try!(serializer.serialize_map(Some(1)));
            try!(serializer.serialize_map_key(&mut state, "data"));
            try!(serializer.serialize_map_value(&mut state, &Identifier::new(related)));
            // TODO links
            serializer.serialize_map_end(state)
        } else { Ok(()) }
    }
}

pub struct HasMany<'a, T: api::HasMany<R>, R: api::Resource> where T::Id: 'a {
    id: &'a T::Id,
    _spoopy: PhantomData<R>,
}

impl<'a, T: api::HasMany<R>, R: api::Resource> HasMany<'a, T, R> {
    pub fn new(id: &'a T::Id) -> HasMany<'a, T, R> {
        HasMany {
            id: id,
            _spoopy: PhantomData,
        }
    }
}

impl<'a, T: api::HasMany<R>, R: api::Resource> Serialize for HasMany<'a, T, R> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_map(Some(1)));
        let resources = T::has_many(&self.id);
        try!(serializer.serialize_map_key(&mut state, "data"));
        let identifiers = resources.into_iter().map(Identifier::new).collect::<Vec<_>>();
        try!(serializer.serialize_map_value(&mut state, identifiers));
        serializer.serialize_map_end(state)
    }
}
