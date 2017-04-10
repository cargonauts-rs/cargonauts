mod resource;
mod routes;

use ast::*;

use self::routes::Routes;

pub fn code_gen(resources: Vec<Resource>) -> String {
    let build_routing_table = Routes::new(&resources);

    let tokens = quote! {
        pub fn routes() -> ::std::io::Result<::cargonauts::routing::RoutingTable> {
            Ok({#build_routing_table})
        }

        #(#resources)*
    };

    tokens.to_string()
}
