#![allow(unused_variables)]

use api;
use repr;

routes! {
    resource Object: [get, index, post, patch, delete] { }
}

pub struct Object;

impl repr::Represent for Object {
    fn repr<P: repr::Presenter>(&self, _: &mut P) -> Result<(), P::Error> {
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
    type Repr = Object;
    fn id(&self) -> u32 { unimplemented!() }
    fn repr(self) -> Object { self }
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
    fn post(repr: Object) -> api::Result<Object> {
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
    use router::mock::MockRouter;
    
    const ROUTES: &'static [&'static str] = &["get", "index", "post", "patch", "delete"];

    let mut router = MockRouter::new();
    attach_routes(&mut router);

    assert_eq!(&router.methods_for("objects")[..], ROUTES);
}
