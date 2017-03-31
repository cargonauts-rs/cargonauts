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
    pub use mainsail::{AnyMap, Get, Index, Resource, Error, Environment};

    #[macro_use]
    pub mod relations {
        pub use mainsail::relations::{Relationship, ToOne, ToMany};

        #[macro_export]
        macro_rules! to_one {
            ($rel:ident => $resource:ident) => {
                pub struct $rel;

                impl $crate::api::relations::Relationship for $rel {
                    type Related = $resource;
                }

                impl $crate::api::relations::ToOne for $rel {
                    type One = $resource;
                }
            }
        }

        #[macro_export]
        macro_rules! to_many {
            ($rel:ident => $resource:ident) => {
                pub struct $rel;

                impl $crate::api::relations::Relationship for $rel {
                    type Related = Vec<$resource>;
                }

                impl $crate::api::relations::ToMany for $rel {
                    type Many = $resource;
                }
            }
        }
    }
}

pub mod routing {
    pub use mainsail::ResourceEndpoint;
    pub use mainsail::relations::{RelationEndpoint, RelationshipLink};
    pub use rigging::present::middleware::Presenter;
    pub use rigging::route::{Route, Method, Handler, not_found};
    pub use rigging::route::{new_resource_service, new_collection_service};
}

pub mod server {
    pub use rigging::http::{Request, Response, Http, Server, Error, Service, NewService};
}

pub mod format {
    pub use rigging::request::{Request, ResourceRequest, CollectionRequest, GetRequest, IndexRequest};

    pub use simple_debug::SimpleDebug as Debug;

    pub mod presenter {
        pub use rigging::present::{Present, PresentResource, PresentCollection, Template};
    }

    pub mod receiver {
        pub use rigging::receive::Receive;
    }
}
