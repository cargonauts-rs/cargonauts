mod assets;
mod formats;
mod resource;
mod routes;
mod setup;

use cfg::CargonautsConfig;
use ast::*;
use quote::Tokens;

use self::routes::Routes as _Routes;

pub fn code_gen(routes: Routes, cfg: CargonautsConfig) -> String {
    let load_env_vars = load_env_vars(&cfg);
    let resources = routes.all_resources();
    let assets = assets::assets(&cfg, &routes);
    let build_routing_table = _Routes::new(assets, &routes.route_items);
    let addr = cfg.host().to_string();
    let formats = formats::formats(&routes, &cfg);
    let timeout = cfg.timeout().as_secs();

    let tokens = if let Some(ref setup) = routes.setup {
        let setup_environment = setup::setup(setup, &cfg);

        quote! {
            #[allow(unused_variables)]
            pub fn routes(handle: &::cargonauts::server::Handle)
                ->  (
                        ::std::net::SocketAddr,
                        Box<::cargonauts::futures::Future<Item = ::cargonauts::routing::RoutingTable, Error = ::std::io::Error>>
                    )
            {
                fn formats() -> ::std::io::Result<::cargonauts::routing::FormatLender> {
                    #formats
                }
                use ::cargonauts::futures::{Future, Stream};
                let timer = ::cargonauts::routing::Timer::new(::std::time::Duration::from_secs(#timeout), handle.clone());
                #load_env_vars
                let future: ::cargonauts::futures::future::Map<_, _> =
                    {#setup_environment}.and_then(move |env| formats().map(|formats| (env, formats)))
                        .map(move |(env, formats)| {#build_routing_table});
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
                fn formats() -> ::std::io::Result<::cargonauts::routing::FormatLender> {
                    #formats
                }
                use ::cargonauts::futures::{Future, IntoFuture, Stream};
                let timer = ::cargonauts::routing::Timer::new(::std::time::Duration::from_secs(#timeout), handle.clone());
                let env = ::cargonauts::routing::EnvBuilder::new().build();
                let future: ::cargonauts::futures::future::Map<_, _> =
                    formats().into_future().map(move |formats| {#build_routing_table});
                (#addr.parse().unwrap(), Box::new(future))
            }

            #(#resources)*
        }
    };

    tokens.to_string()
}

fn load_env_vars(cfg: &CargonautsConfig) -> Tokens {
    if let Some(vars) = cfg.env("dev") {
        let vars = vars.iter().map(|(k, v)| quote!(::std::env::set_var(#k, #v);));
        quote!({#(#vars)*})
    } else { quote!({}) }
}
