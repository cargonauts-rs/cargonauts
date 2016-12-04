#![allow(unused_variables)]

use api;
use repr;

routes! {
    resource User {
        has many Photo;
        alias get as "me";
    }
    resource Photo {
        has one User;
    }
}

impl api::GetAliased for User {
    type GetAliasedFut = Result<Self, api::Error>;
    fn get(request: api::AliasRequest) -> Self::GetAliasedFut {
        unimplemented!()
    }
}

struct User;

impl repr::Represent for User {
    fn repr<S: ::Serializer>(&self, _: &mut S, _: Option<&[String]>) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl api::Resource for User {
    type Id = u32;

    fn id(&self) -> u32 { unimplemented!() }

    fn resource() -> &'static str { "user" }
    fn resource_plural() -> &'static str { "users" }
}

impl api::Get for User {
    type GetFut = Result<User, api::Error>;
    fn get(id: &u32) -> Self::GetFut {
        unimplemented!()
    }
}

impl api::Patch for User {
    type Patch = ();
    type PatchFut = Result<User, api::Error>;
    fn patch(id: &u32, patch: Self::Patch) -> Self::PatchFut {
        unimplemented!()
    }
}

impl api::rel::HasMany<Photo> for User {
    type HasManyFut = Result<Vec<u32>, api::Error>;
    fn has_many(entity: api::Entity<User>) -> Self::HasManyFut {
        unimplemented!()
    }
}

impl api::rel::PostLinks<Photo> for User {
    type PostLinksFut = Result<(), api::Error>;
    fn post_link(entity: api::Entity<User>, rel_ids: u32) -> Self::PostLinksFut {
        unimplemented!()
    }
}

struct Photo;

impl repr::Represent for Photo {
    fn repr<S: ::Serializer>(&self, _: &mut S, _: Option<&[String]>) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl ::Deserialize for Photo {
    fn deserialize<D: ::Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        unimplemented!()
    }
}

impl api::Resource for Photo {
    type Id = u32;

    fn id(&self) -> u32 { unimplemented!() }

    fn resource() -> &'static str { "photo" }
    fn resource_plural() -> &'static str { "photos" }
}

impl api::Delete for Photo {
    type DeleteFut = Result<(), api::Error>;
    fn delete(id: &u32) -> Self::DeleteFut {
        unimplemented!()
    }
}

impl api::Get for Photo {
    type GetFut = Result<Photo, api::Error>;
    fn get(id: &u32) -> Self::GetFut {
        unimplemented!()
    }
}

impl api::Index for Photo {
    type IndexFut = Result<Vec<Photo>, api::Error>;
    fn index() -> Self::IndexFut {
        unimplemented!()
    }
}

impl api::Post for Photo {
    type PostFut = Result<Photo, api::Error>;
    fn post(_: Self) -> Self::PostFut {
        unimplemented!()
    }
}

impl api::rel::HasOne<User> for Photo {
    type HasOneFut = Result<Option<u32>, api::Error>;
    fn has_one(entity: api::Entity<Photo>) -> Self::HasOneFut {
        unimplemented!()
    }
}

#[test]
fn it_compiles() { }

#[test]
fn it_has_attached_routes() {
    use router::mock::MockRouter;
    
    const ME_ROUTES: &'static [&'static str] = &["alias-read"];
    const USERS_ROUTES: &'static [&'static str] = &["get", "patch"];
    const PHOTOS_ROUTES: &'static [&'static str] = &["get", "index", "delete", "post"];
    const USERS_PHOTOS_ROUTES: &'static [&'static str] = &["read-rel", "create-rel", "read-related", "create-related"];
    const PHOTOS_USER_ROUTES: &'static [&'static str] = &["read-rel", "read-related", "update-related"];

    let mut router = MockRouter::new();
    attach_routes(&mut router);


    assert_eq!(&router.methods_for("me")[..], ME_ROUTES);
    assert_eq!(&router.methods_for("users")[..], USERS_ROUTES);
    assert_eq!(&router.methods_for("photos")[..], PHOTOS_ROUTES);
    assert_eq!(&router.methods_for_rel("users", "photos")[..], USERS_PHOTOS_ROUTES);
    assert_eq!(&router.methods_for_rel("photos", "user")[..], PHOTOS_USER_ROUTES);
}
