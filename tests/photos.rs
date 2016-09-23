#![allow(unused_variables)]
#[macro_use]
extern crate cargonauts;


use cargonauts::api;

routes! {
    resource User => ["get", "patch"] {
        related Photo: "has-many";
    }
    resource Photo => ["get", "index", "post", "delete"] {
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

impl api::Patch for User {
    type Patch = ();
    fn patch(id: Self::Id, patch: Self::Patch) -> Result<Option<User>, api::PatchError> {
        unimplemented!()
    }
}

impl api::HasMany<Photo> for User {
    fn has_many(id: &Self::Id) -> Vec<Photo> {
        unimplemented!()
    }
    fn link_many(id: &Self::Id, rel_ids: &[<Photo as api::Resource>::Id]) -> Result<(), api::LinkError> {
        unimplemented!()
    }
}

struct Photo;

impl cargonauts::Serialize for Photo {
    fn serialize<S: cargonauts::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl cargonauts::Deserialize for Photo {
    fn deserialize<D: cargonauts::Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
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

impl api::Delete for Photo {
    fn delete(id: Self::Id) -> Result<(), api::DeleteError> {
        unimplemented!()
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

impl api::Post for Photo {
    fn post(self) -> Result<Option<Photo>, api::PostError> {
        unimplemented!()
    }
}

impl api::HasOne<User> for Photo {
    fn has_one(id: &Self::Id) -> Option<User> {
        unimplemented!()
    }
    fn link_one(id: &Self::Id, rel_id: &<User as api::Resource>::Id) -> Result<(), api::LinkError> {
        unimplemented!()
    }
}

#[test]
fn it_compiles() { }
