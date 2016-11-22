#![allow(unused_variables)]

use api;
use repr;

routes! {
    resource Object { }
}

pub struct Object;

impl repr::Represent for Object {
    fn repr<S: ::Serializer>(&self, _: &mut S, _: Option<&[String]>) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl ::Deserialize for Object {
    fn deserialize<D: ::Deserializer>(_: &mut D) -> Result<Self, D::Error> {
        unimplemented!()
    }
}

impl api::Resource for Object {
    type Id = u32;
    fn id(&self) -> u32 { unimplemented!() }
    fn resource() -> &'static str { "object" }
    fn resource_plural() -> &'static str { "objects" }
}

impl api::Get for Object {
    type GetFut = Result<Object, api::Error>;
    fn get(id: &u32) -> Self::GetFut {
        unimplemented!()
    }
}

impl api::Index for Object {
    type IndexFut = Result<Vec<Object>, api::Error>;
    fn index() -> Self::IndexFut {
        unimplemented!()
    }
}

impl api::Post for Object {
    type PostFut = Result<Object, api::Error>;
    fn post(_: Self) -> Self::PostFut {
        unimplemented!()
    }
}

impl api::Delete for Object {
    type DeleteFut = Result<(), api::Error>;
    fn delete(id: &u32) -> Self::DeleteFut {
        unimplemented!()
    }
}

impl api::Patch for Object {
    type Patch = ();
    type PatchFut = Result<Object, api::Error>;
    fn patch(id: &u32, patch: ()) -> Self::PatchFut {
        unimplemented!()
    }
}

#[test]
fn it_compiles() { }

#[test]
fn it_has_attached_routes() {
    use router::mock::MockRouter;
    
    const ROUTES: &'static [&'static str] = &["get", "index", "delete", "patch", "post"];

    let mut router = MockRouter::new();
    attach_routes(&mut router);

    assert_eq!(&router.methods_for("objects")[..], ROUTES);
}
