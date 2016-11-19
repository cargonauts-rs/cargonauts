use std::io::{self, Write};
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
    type Response = MockResponse;
    type LinkMaker = MockLinker;

    fn attach_get(&mut self,
        resource: &'static str,
        _: fn(GetRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("get", resource); }

    fn attach_index(&mut self,
        resource: &'static str, 
        _: fn(IndexRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("index", resource); }

    fn attach_delete(&mut self,
        resource: &'static str,
        _: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("delete", resource); }

    fn attach_clear(&mut self,
        resource: &'static str,
        _: fn(Self::LinkMaker) -> Self::Response,
    ) { self.register("clear", resource); }

    fn attach_remove(&mut self,
        resource: &'static str,
        _: fn(RemoveRequest, Self::LinkMaker) -> Self::Response
    ) { self.register("remove", resource); }

    fn attach_patch(&mut self,
        resource: &'static str,
        _: fn(PatchRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("patch", resource); }

    fn attach_post(&mut self,
        resource: &'static str,
        _: fn(PostRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("post", resource); }

    fn attach_append(&mut self,
        resource: &'static str,
        _: fn(MultiPostRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("append", resource); }

    fn attach_replace(&mut self,
        resource: &'static str,
        _: fn(MultiPostRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("replace", resource); }

    fn attach_fetch_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(GetRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("fetch-one", resource, relation); }

    fn attach_fetch_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(GetRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("fetch-many", resource, relation); }

    fn attach_fetch_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(FetchRelRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("fetch-rel", resource, relation); }

    fn attach_delete_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("delete-one", resource, relation); }

    fn attach_clear_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("clear-many", resource, relation); }

    fn attach_remove_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(RemoveManyRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("remove-many", resource, relation); }

    fn attach_delete_one_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("delete-one-rel", resource, relation); }

    fn attach_clear_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(DeleteRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("clear-many-rel", resource, relation); }

    fn attach_remove_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(RemoveManyRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("remove-many-rel", resource, relation); }

    fn attach_post_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(PostOneRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("post-one", resource, relation); }

    fn attach_patch_one(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(PatchRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("patch-one", resource, relation); }

    fn attach_append_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(PostManyRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("append-many", resource, relation); }

    fn attach_replace_many(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(PostManyRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("replace-many", resource, relation); }


    fn attach_update_one_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(UpdateRelRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("post-one-rel", resource, relation); }

    fn attach_append_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(UpdateRelRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("append-many-rel", resource, relation); }

    fn attach_replace_many_rel(&mut self,
        resource: &'static str,
        relation: &'static str,
        _: fn(UpdateRelRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register_rel("replace-many-rel", resource, relation); }

    fn attach_get_alias(&mut self,
        alias: &'static str,
        _: fn(AliasRequest, GetRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("alias-get", alias); }

    fn attach_index_alias(&mut self,
        alias: &'static str,
        _: fn(AliasRequest, IndexRequest, Self::LinkMaker) -> Self::Response,
    ) { self.register("alias-index", alias); }
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

#[derive(Default)]
pub struct MockResponse;

impl Response for MockResponse {
    fn set_status(&mut self, _: Status) { unimplemented!() }
    fn set_content(&mut self, _: &str) { unimplemented!() }
}

impl Write for MockResponse {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }
    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}
