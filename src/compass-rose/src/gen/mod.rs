mod resource;
mod routes;
mod setup;

use cfg::CargonautsConfig;
use ast::*;
use quote::Tokens;

use self::routes::Routes as _Routes;

pub fn code_gen(routes: Routes, cfg: Option<CargonautsConfig>) -> String {
    let load_env_vars = load_env_vars(cfg.as_ref());
    let resources = &routes.resources;
    let build_routing_table = _Routes::new(resources);
    let addr = cfg.as_ref().and_then(|cfg| cfg.host()).map_or("127.0.0.1:7878".to_string(), |addr| addr.to_string());
    let tokens = if let Some(ref setup) = routes.setup {
        let setup_environment = setup::setup(setup, cfg.as_ref());

        quote! {
            #[allow(unused_variables)]
            pub fn routes(handle: &::cargonauts::server::Handle)
                ->  (
                        ::std::net::SocketAddr,
                        Box<::cargonauts::futures::Future<Item = ::cargonauts::routing::RoutingTable, Error = ::std::io::Error>>
                    )
            {
                use ::cargonauts::futures::{Future, Stream};
                #load_env_vars
                let future: ::cargonauts::futures::future::Map<_, _> = {#setup_environment}.map(|env| {#build_routing_table});
                (#addr.parse().unwrap(), Box::new(future))
            }

            #(#resources)*
        }
    } else {
        quote! {
            #[allow(unused_variables)]
            pub fn routes(handle: &::cargonauts::server::Handle)
                ->  (
                        ::std::net::SocketAddr,
                        Box<::cargonauts::futures::Future<Item = ::cargonauts::routing::RoutingTable, Error = ::std::io::Error>>
                    )
            {
                use ::cargonauts::futures::{Future, Stream};
                let env = ::cargonauts::routing::EnvBuilder::new().build();
                (#addr.parse().unwrap(), Box::new(::cargonauts::futures::future::ok({#build_routing_table})))
            }

            #(#resources)*
        }
    };

    tokens.to_string()
}

fn load_env_vars(cfg: Option<&CargonautsConfig>) -> Tokens {
    if let Some(cfg) = cfg {
        if let Some(vars) = cfg.env("dev") {
            let vars = vars.iter().map(|(k, v)| quote!(::std::env::set_var(#k, #v);));
            quote!({#(#vars)*})
        } else { quote!({}) }
    } else { quote!({}) }
}
