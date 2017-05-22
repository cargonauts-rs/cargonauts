#![feature(associated_consts)]

pub extern crate futures;
extern crate c3po;
extern crate mainsail;
extern crate rigging;
extern crate cargonauts_config;
pub extern crate serde;
pub extern crate serde_json as json;
pub extern crate tokio_redis as redis;
extern crate tokio_service;

#[allow(unused_imports)]
#[macro_use] extern crate compass_rose;
#[macro_use] extern crate proc_macro_hack;

#[doc(hidden)]
pub use compass_rose::*;

proc_macro_item_decl! {
    /// The routes DSL
    routes! => routes_impl
}

pub mod config {
    pub use cargonauts_config::CargonautsConfig;
}

#[macro_use]
pub mod resources {
    pub use rigging::Error;
    pub use rigging::environment::Environment;

    pub use rigging::resource::{Resource, Relationship};

    #[macro_export]
    macro_rules! relation {
        ($rel:ident => $resource:ident) => {
            pub struct $rel;

            impl $crate::resources::Relationship for $rel {
                type Related = $resource;
            }
        }
    }
}

pub mod methods {
    pub use mainsail::methods::{Delete, Get, Index, Patch, Post};
    pub use mainsail::methods::{DeleteRelated, GetOne, GetMany, PostRelated, UpdateRelated};

    pub mod def {
        pub use rigging::method::{Method, ResourceMethod, CollectionMethod};
        pub use rigging::routes::{Route, Kind};
        pub use rigging::http::Method as HttpMethod;
    }
}

pub mod formats {

    pub use rigging::format::{Format, BuildFormat, Template, TemplateKey};
    pub use mainsail::formats::Debug;
    pub use mainsail::formats::jsonapi::JsonApi;
    pub use mainsail::formats::handlebars::Handlebars;

    pub mod jsonapi {
        pub use mainsail::formats::jsonapi::{ApiSerialize, ApiDeserialize, Fields, ClientIdPolicy};
    }
}



#[doc(hidden)]
pub mod routing {
    pub use rigging::resource::{ResourceEndpoint, RelationEndpoint, RelationshipLink, RelIds, HasOneEndpoint, HasManyEndpoint};
    pub use rigging::endpoint::{Endpoint, EndpointService};
    pub use rigging::routes::{RoutingTable, RouteBuilder, Handler, path};
    pub use rigging::routes::{AssetHandler, default_asset_handler};
    pub use rigging::environment::EnvBuilder;
    pub use rigging::http::BoxFuture as HttpFuture;
    pub use rigging::format::FormatLender;
}

pub use server::serve;

pub mod server {
    pub use rigging::http::{Request, Response, Error, Service, NewService, serve, Handle, Method, StatusCode};

    pub mod pool {
        pub use rigging::connections::Configure;
        pub use c3po::{Pool, Config};
    }
}

pub mod clients {
    pub use rigging::connections::{Client, ConnectClient, Configure, NewServiceLike};
    pub use c3po::{Config as PoolConfig, Conn};

    pub mod mock {
        pub use rigging::connections::mock::MockConnection;
    }
}

pub mod middleware {
    pub use rigging::endpoint::Request;

    pub mod http {
        pub use rigging::http::{Request, Response, Error, BoxFuture};
    }

    pub use tokio_service::Service;

    pub trait Middleware<S>: Default
    where
        S: Service<Request = Request, Response = http::Response, Error = http::Error, Future = http::BoxFuture>
    {
        type WrappedService: Service<Request = Request, Response = http::Response, Error = http::Error, Future = http::BoxFuture>;

        fn wrap(self, service: S) -> Self::WrappedService;
    }
}
