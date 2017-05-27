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

/// Configuration of cargonauts.
pub mod config {
    pub use cargonauts_config::CargonautsConfig;
}

pub use rigging::Error;
pub use rigging::environment::Environment;
pub use rigging::resource::{Resource, Relationship};
pub use server::serve;

/// This macro constructs a new relationship alias. The syntax is
/// `relation!(NewRelation => ResourceType);`, which creates a new type called
/// `NewRelation` that implements Relationship to `ResourceType`.
#[macro_export]
macro_rules! relation {
    ($rel:ident => $resource:ident) => {
        pub struct $rel;

        impl $crate::Relationship for $rel {
            type Related = $resource;
        }
    }
}

/// For implementing methods on resources and defining new ones.
///
/// # Methods table
///
/// Method trait  | Route                                     | Http Method | Relationship
/// --------------|-------------------------------------------|-------------|-------------
/// Get           | /$resource-type/$identifier               | GET         | 
/// Index         | /$resource-type                           | GET         | 
/// Post          | /$resource-type                           | POST        | 
/// Delete        | /$resource-type/$identifier               | DELETE      | 
/// Patch         | /$resource-type/$identifier               | PATCH       |
/// GetOne        | /$resource-type/$identifier/$relationship | GET         | has one
/// GetMany       | /$resource-type/$identifier/$relationship | GET         | has many
/// PostRelated   | /$resource-type/$identifier/$relationship | POST        | has many
/// DeleteRelated | /$resource-type/$identifier/$relationship | DELETE      | has one
/// UpdateRelated | /$resource-type/$identifier/$relationship | PATCH       | has one
pub mod methods {
    pub use mainsail::methods::{Delete, Get, Index, Patch, Post};
    pub use mainsail::methods::{DeleteRelated, GetOne, GetMany, PostRelated, UpdateRelated};

    pub mod def {
        pub use rigging::method::{Method, ResourceMethod, CollectionMethod};
        pub use rigging::routes::{Route, Kind};
        pub use rigging::http::Method as HttpMethod;
    }
}

/// For providing formats for methods and defining new ones.
pub mod formats {
    pub use mainsail::formats::Debug;
    pub use mainsail::formats::jsonapi::JsonApi;
    pub use mainsail::formats::handlebars::Handlebars;

    pub mod jsonapi {
        pub use mainsail::formats::jsonapi::{ApiSerialize, ApiDeserialize, Fields, ClientIdPolicy};
    }

    pub mod def {
        pub use rigging::format::{Format, BuildFormat, Template, TemplateKey};
    }
}

/// Raw HTTP types.
pub mod server {
    pub use rigging::http::{Request, Response, Error, Service, NewService, serve, Handle, Method, StatusCode};

    pub mod pool {
        pub use rigging::connections::Configure;
        pub use c3po::{Pool, Config};
    }
}

/// For defining high level clients to other services.
pub mod clients {
    pub use rigging::connections::{Client, ConnectClient, Configure, NewServiceLike};
    pub use c3po::{Config as PoolConfig, Conn};

    pub mod mock {
        pub use rigging::connections::mock::MockConnection;
    }
}

/// For wrapping your endpoints in middleware.
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

#[doc(hidden)]
pub mod routing {
    pub use rigging::endpoint::{Endpoint, EndpointService};
    pub use rigging::environment::EnvBuilder;
    pub use rigging::http::BoxFuture as HttpFuture;
    pub use rigging::format::FormatLender;
    pub use rigging::resource::{ResourceEndpoint, RelationEndpoint, RelationshipLink, RelIds, HasOneEndpoint, HasManyEndpoint};
    pub use rigging::routes::{RoutingTable, RouteBuilder, Handler, path};
    pub use rigging::routes::{AssetHandler, default_asset_handler};
    pub use rigging::routes::Timer;
}

