use std::io::{self, Read, Write};
use router::*;
use api::AliasRequest;
use Future;

pub struct MockRouter {
    routes: Vec<MockRoute>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MockRoute {
    method: &'static str,
    resource: &'static str,
    rel: Option<&'static str>,
}

impl MockRouter {
    pub fn new() -> MockRouter {
        MockRouter { routes: vec![] }
    }

    pub fn methods_for(&self, resource: &'static str) -> Vec<&'static str> {
        self.routes.iter()
                   .filter(|route| route.resource == resource && route.rel == None)
                   .map(|route| route.method)
                   .collect()
    }

    pub fn methods_for_rel(&self, resource: &'static str, rel: &'static str) -> Vec<&'static str> {
        self.routes.iter()
                   .filter(|route| route.resource == resource && route.rel == Some(rel))
                   .map(|route| route.method)
                   .collect()
    }

    pub fn has_resource_route(&self, method: &'static str, resource: &'static str) -> bool {
        self.routes.contains(&MockRoute {
            method: method,
            resource: resource,
            rel: None,
        })
    }

    pub fn has_relation_route(&self, method: &'static str, resource: &'static str, rel: &'static str) -> bool {
        self.routes.contains(&MockRoute {
            method: method,
            resource: resource,
            rel: Some(rel),
        })
    }

    fn register(&mut self, method: &'static str, resource: &'static str) {
        self.routes.push(MockRoute {
            method: method,
            resource: resource,
            rel: None,
        })
    }

    fn register_rel(&mut self, method: &'static str, resource: &'static str, rel: &'static str) {
        self.routes.push(MockRoute {
            method: method,
            resource: resource,
            rel: Some(rel),
        })
    }
}

impl Router for MockRouter {
    type Request = MockRequest;
    type Response = MockResponse;
    type LinkMaker = MockLinker;

    fn attach_resource(&mut self,
        route: ResourceRoute<'static>,
        _: fn(Self::Request, Self::LinkMaker) -> Box<Future<Item = Self::Response, Error = ()>>
    ) {
        match (route.method, route.component) {
            (Method::Read,    Component::Resource)          => self.register("get", route.name),
            (Method::Destroy, Component::Resource)          => self.register("delete", route.name),
            (Method::Update,  Component::Resource)          => self.register("patch", route.name),
            (Method::Create,  Component::Resource)          => unreachable!(),
            (Method::Read,    Component::Collection)        => self.register("index", route.name),
            (Method::Destroy, Component::Collection)        => self.register("remove", route.name),
            (Method::Create,  Component::Collection)        => self.register("post", route.name),
            (Method::Update,  Component::Collection)        => self.register("replace", route.name),
            (Method::Read,    Component::Related(rel))      => self.register_rel("read-related", route.name, rel),
            (Method::Destroy, Component::Related(rel))      => self.register_rel("destroy-related", route.name, rel),
            (Method::Create,  Component::Related(rel))      => self.register_rel("create-related", route.name, rel),
            (Method::Update,  Component::Related(rel))      => self.register_rel("update-related", route.name, rel),
            (Method::Read,    Component::Relationship(rel)) => self.register_rel("read-rel", route.name, rel),
            (Method::Destroy, Component::Relationship(rel)) => self.register_rel("destroy-rel", route.name, rel),
            (Method::Create,  Component::Relationship(rel)) => self.register_rel("create-rel", route.name, rel),
            (Method::Update,  Component::Relationship(rel)) => self.register_rel("update-rel", route.name, rel),
        }
    }

    fn attach_alias(&mut self,
        alias: &'static str,
        method: Method,
        _: fn(Self::Request, Self::LinkMaker) -> Box<Future<Item = Self::Response, Error = ()>>
    ) {
        match method {
            Method::Read    => self.register("alias-read", alias),
            Method::Destroy => self.register("alias-destroy", alias),
            Method::Update  => self.register("alias-update", alias),
            Method::Create  => self.register("alias-create", alias),
        }
    }
}

#[derive(Copy, Clone)]
pub struct MockLinker;

impl MakeLinks for MockLinker {
    fn collection(&self, resource: &str) -> String {
        ["https://example.org", resource].join("/")
    }

    fn resource(&self, resource: &str, id: &str) -> String {
        ["https://example.org", resource, id].join("/")
    }

    fn relationship(&self, resource: &str, id: &str, relation: &str) -> String {
        ["https://example.org", resource, id, "relationships", relation].join("/")
    }
    fn related_resource(&self, resource: &str, id: &str, relation: &str) -> String {
        ["https://example.org", resource, id, relation].join("/")
    }
}

pub struct MockRequest;

impl Request for MockRequest {
    type Body = Box<Read>;
    fn endpoint(&self) -> &str { unimplemented!() }
    fn id(&self) -> Option<&str> { unimplemented!() }
    fn component(&self) -> Component { unimplemented!() }
    fn method(&self) -> Method { unimplemented!() }
    fn resource_options(&self) -> ResourceOptions { unimplemented!() }
    fn collection_options(&self) -> CollectionOptions { unimplemented!() }
    fn alias_info(&self) -> AliasRequest { unimplemented!() }
    fn body(self) -> Self::Body { unimplemented!() }
}

#[derive(Default)]
pub struct MockResponse;

impl Response for MockResponse {
    fn set_status(&mut self, _: Status) { unimplemented!() }
    fn set_content(&mut self, _: &str) { unimplemented!() }
}

impl Write for MockResponse {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> { unimplemented!() }
    fn flush(&mut self) -> io::Result<()> { unimplemented!() }
}
