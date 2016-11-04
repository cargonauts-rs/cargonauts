use json;
use router::*;
use api::raw::Relationship;

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
    type Linker = MockLinker;
    fn attach_delete<F>(&mut self, resource: &'static str, _: F)
            where F: Fn(String) -> Self::Response {
        self.register("delete", resource);
    }
    fn attach_get<F>(&mut self, resource: &'static str, _: F)
            where F: Fn(GetRequest) -> Self::Response {
        self.register("get", resource);
    }
    fn attach_index<F>(&mut self, resource: &'static str, _: F)
            where F: Fn(IndexRequest) -> Self::Response {
        self.register("index", resource);
    }
    fn attach_patch<F>(&mut self, resource: &'static str, _: F)
            where F: Fn(PatchRequest) -> Self::Response {
        self.register("patch", resource);
    }
    fn attach_post<F>(&mut self, resource: &'static str, _: F)
            where F: Fn(PostRequest) -> Self::Response {
        self.register("post", resource);
    }
    fn attach_fetch_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(GetRequest) -> Self::Response {
        self.register_rel("fetch-many", resource, relationship);
    }
    fn attach_fetch_many<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(GetRequest) -> Self::Response {
        self.register_rel("fetch-one", resource, relationship);
    }
    fn attach_fetch_rel<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(FetchRelRequest) -> Self::Response {
        self.register_rel("fetch-rel", resource, relationship);
    }
    fn attach_delete_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String) -> Self::Response {
        self.register_rel("delete-one", resource, relationship);
    }
    fn attach_remove_many<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, Vec<String>) -> Self::Response {
        self.register_rel("remove", resource, relationship);
    }
    fn attach_clear_many<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String) -> Self::Response {
        self.register_rel("clear", resource, relationship);
    }
    fn attach_delete_one_rel<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String) -> Self::Response {
        self.register_rel("delete-one-rel", resource, relationship);
    }
    fn attach_remove_many_rel<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, Vec<String>) -> Self::Response {
        self.register_rel("remove-links", resource, relationship);
    }
    fn attach_clear_many_rel<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String) -> Self::Response {
        self.register_rel("clear-links", resource, relationship);
    }
    fn attach_patch_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(PatchRequest) -> Self::Response {
        self.register_rel("patch-one", resource, relationship);
    }
    fn attach_post_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, PostRequest) -> Self::Response {
        self.register_rel("post-one", resource, relationship);
    }

    fn attach_link_one<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, Relationship) -> Self::Response {
        self.register_rel("link-one", resource, relationship);
    }
    fn attach_append_link_many<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, Relationship) -> Self::Response {
        self.register_rel("append-links", resource, relationship);
    }
    fn attach_replace_link_many<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, Relationship) -> Self::Response {
        self.register_rel("replace-links", resource, relationship);
    }
    fn attach_append_many<F>(&mut self, resource: &'static str, relationship: &'static str, _: F)
            where F: Fn(String, PostRequest) -> Self::Response {
        self.register_rel("append", resource, relationship);
    }
}

#[derive(Copy, Clone)]
pub struct MockLinker;

impl Linker for MockLinker {
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
    type Serializer = json::value::Serializer;
    fn set_status(&mut self, _: Status) { unimplemented!() }
    fn set_content(&mut self, _: &str) { unimplemented!() }
    fn serializer(&mut self) -> &mut Self::Serializer { unimplemented!() }
}
