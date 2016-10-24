#![allow(unused_variables)]

#[macro_use]
extern crate cargonauts;

use cargonauts::api;

routes! {
    resource Object: [get, index, post, patch, delete] { }
}

pub struct Object;

impl cargonauts::Serialize for Object {
    fn serialize<S: cargonauts::Serializer>(&self, _: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl cargonauts::Deserialize for Object {
    fn deserialize<D: cargonauts::Deserializer>(_: &mut D) -> Result<Self, D::Error> {
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
    fn get(id: &u32) -> api::Result<Object> {
        unimplemented!()
    }
}

impl api::Index for Object {
    fn index() -> api::Result<Vec<Object>> {
        unimplemented!()
    }
}

impl api::Post for Object {
    fn post(self) -> api::Result<Object> {
        unimplemented!()
    }
}

impl api::Delete for Object {
    fn delete(id: &u32) -> api::Result<()> {
        unimplemented!()
    }
}

impl api::Patch for Object {
    type Patch = ();
    fn patch(id: &u32, patch: ()) -> api::Result<Object> {
        unimplemented!()
    }
}

#[test]
fn it_compiles() { }

#[test]
fn it_has_attached_routes() {
    use cargonauts::router::mock::MockRouter;
    
    const ROUTES: &'static [&'static str] = &["get", "index", "post", "patch", "delete"];

    let mut router = MockRouter::new();
    attach_routes(&mut router);

    assert_eq!(&router.methods_for("objects")[..], ROUTES);
}
