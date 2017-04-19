#![feature(associated_consts)]

pub extern crate futures;
extern crate mainsail;
extern crate rigging;
extern crate simple_debug;

#[allow(unused_imports)]
#[macro_use] extern crate compass_rose;
#[macro_use] extern crate proc_macro_hack;

#[doc(hidden)]
pub use compass_rose::*;

proc_macro_item_decl! {
    /// The routes DSL
    routes! => routes_impl
}

#[macro_use]
pub mod api {
    pub use rigging::{Resource, Error};
    pub use rigging::environment::Environment;
    pub use mainsail::{Get, Index};

    #[macro_use]
    pub mod relations {
        pub use rigging::Relationship;
        pub use mainsail::{GetOne, GetMany};

        #[macro_export]
        macro_rules! relation {
            ($rel:ident => $resource:ident) => {
                pub struct $rel;

                impl $crate::api::relations::Relationship for $rel {
                    type Related = $resource;
                }
            }
        }
    }
}

pub mod routing {
    pub use rigging::{ResourceEndpoint, RelationEndpoint, RelationshipLink};
    pub use rigging::endpoint::EndpointService;
    pub use rigging::routes::{Kind, Routes, RoutingTable, Handler, not_found};
}

pub mod server {
    pub use rigging::http::{Request, Response, Http, Server, Error, Service, NewService};
}

pub mod method {
    pub use rigging::Method;
    pub use rigging::request::{Request, ResourceRequest, CollectionRequest};
    pub use rigging::routes::Route;
    pub use mainsail::{GetRequest, IndexRequest};
}

pub mod format {

    pub use simple_debug::SimpleDebug as Debug;

    pub mod presenter {
        pub use rigging::format::{Present, PresentResource, PresentCollection, Template};
    }

    pub mod receiver {
        pub use rigging::format::Receive;
    }
}
