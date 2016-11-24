use std::io::{self, Read, Write};
use router::*;
use api::AliasRequest;

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
        resource: &'static str,
        route: ResourceRoute<'static>,
        _: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    ) {
        match (route.method, route.relation) {
            (Method::Get, None)                     => self.register("get", resource),
            (Method::Index, None)                   => self.register("index", resource),
            (Method::Delete, None)                  => self.register("delete", resource),
            (Method::Clear, None)                   => self.register("clear", resource),
            (Method::Remove, None)                  => self.register("remove", resource),
            (Method::Patch, None)                   => self.register("patch", resource),
            (Method::Post, None)                    => self.register("post", resource),
            (Method::Append, None)                  => self.register("append", resource),
            (Method::Replace, None)                 => self.register("replace", resource),
            (Method::Get, Some((rel, false)))       => self.register_rel("get-one", resource, rel),
            (Method::Index, Some((rel, false)))     => self.register_rel("index-many", resource, rel),
            (Method::Delete, Some((rel, false)))    => self.register_rel("delete-one", resource, rel),
            (Method::Clear, Some((rel, false)))     => self.register_rel("clear-many", resource, rel),
            (Method::Remove, Some((rel, false)))    => self.register_rel("remove-many", resource, rel),
            (Method::Post, Some((rel, false)))      => self.register_rel("post-one", resource, rel),
            (Method::Patch, Some((rel, false)))     => self.register_rel("patch-one", resource, rel),
            (Method::Append, Some((rel, false)))    => self.register_rel("append-many", resource, rel),
            (Method::Replace, Some((rel, false)))   => self.register_rel("replace-many", resource, rel),
            (Method::Get, Some((rel, true)))        => self.register_rel("get-rel", resource, rel),
            (Method::Index, Some((rel, true)))      => self.register_rel("index-rel", resource, rel),
            (Method::Delete, Some((rel, true)))     => self.register_rel("delete-rel", resource, rel),
            (Method::Clear, Some((rel, true)))      => self.register_rel("clear-rel", resource, rel),
            (Method::Remove, Some((rel, true)))     => self.register_rel("remove-rel", resource, rel),
            (Method::Post, Some((rel, true)))       => self.register_rel("post-rel", resource, rel),
            (Method::Patch, Some((rel, true)))      => self.register_rel("patch-rel", resource, rel),
            (Method::Append, Some((rel, true)))     => self.register_rel("append-rel", resource, rel),
            (Method::Replace, Some((rel, true)))    => self.register_rel("replace-rel", resource, rel),
        }
    }

    fn attach_alias(&mut self,
        alias: &'static str,
        method: Method,
        _: fn(Self::Request, Self::LinkMaker) -> Self::Response,
    ) {
        match method {
            Method::Get     => self.register("alias-get", alias),
            Method::Index   => self.register("alias-index", alias),
            Method::Delete  => self.register("alias-delete", alias),
            Method::Clear   => self.register("alias-clear", alias),
            Method::Remove  => self.register("alias-remove", alias),
            Method::Patch   => self.register("alias-patch", alias),
            Method::Post    => self.register("alias-post", alias),
            Method::Append  => self.register("alias-append", alias),
            Method::Replace => self.register("alias-replace", alias),
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
    fn endpoint(&self) -> &str { unimplemented!() }
    fn id(&self) -> &str { unimplemented!() }
    fn relation(&self) -> Option<(&str, bool)> { unimplemented!() }
    fn method(&self) -> Method { unimplemented!() }
    fn resource_options(&self) -> ResourceOptions { unimplemented!() }
    fn collection_options(&self) -> CollectionOptions { unimplemented!() }
    fn alias_info(&self) -> AliasRequest { unimplemented!() }
}

impl Read for MockRequest {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> { unimplemented!() }
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
