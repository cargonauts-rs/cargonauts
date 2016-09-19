#![allow(unused_variables)]
#[macro_use]
extern crate cargonauts;

use cargonauts::api;

routes! {
    resource User => ["get"] {
        related Photo: "has-many";
    }
    resource Photo => ["get", "index"] {
        related User: "has-one";
    }
}

struct User;

impl cargonauts::Serialize for User {
    fn serialize<S: cargonauts::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl api::Resource for User {
    type Id = u32;

    fn id(&self) -> u32 {
        unimplemented!()
    }

    fn resource() -> &'static str {
        "user"
    }
}

impl api::Get for User {
    fn get(id: Self::Id) -> Option<User> {
        unimplemented!()
    }
}

impl api::HasMany<Photo> for User {
    fn has_many(id: &Self::Id) -> Vec<Photo> {
        unimplemented!()
    }
}

struct Photo;

impl cargonauts::Serialize for Photo {
    fn serialize<S: cargonauts::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl api::Resource for Photo {
    type Id = u32;

    fn id(&self) -> u32 {
        unimplemented!()
    }

    fn resource() -> &'static str {
        "photo"
    }
}

impl api::Get for Photo {
    fn get(id: Self::Id) -> Option<Photo> {
        unimplemented!()
    }
}

impl api::Index for Photo {
    fn index() -> Vec<Photo> {
        unimplemented!()
    }
}

impl api::HasOne<User> for Photo {
    fn has_one(id: &Self::Id) -> Option<User> {
        unimplemented!()
    }
}

#[test]
fn it_compiles() { }
