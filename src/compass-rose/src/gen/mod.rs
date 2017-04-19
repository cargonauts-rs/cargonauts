mod resource;
mod routes;

use cfg::CargonautsConfig;
use ast::*;

use self::routes::Routes as _Routes;

pub fn code_gen(routes: Routes, cfg: Option<CargonautsConfig>) -> String {
    let resources = &routes.resources;
    let build_routing_table = _Routes::new(resources);

    let tokens = quote! {
        pub fn routes() -> ::std::io::Result<::cargonauts::routing::RoutingTable> {
            Ok({#build_routing_table})
        }

        #(#resources)*
    };

    tokens.to_string()
}
