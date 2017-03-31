mod resource;
mod routes;

use ast::*;

use self::routes::Routes;

pub fn code_gen(resources: Vec<Resource>) -> String {
    let build_routes_table = Routes::new(&resources);

    let tokens = quote! {
        pub struct Routes {
            routes: ::std::collections::HashMap<::cargonauts::routing::Route, ::cargonauts::routing::Handler>,
        }

        #[allow(unused_mut)]
        pub fn routes() -> ::std::io::Result<Routes> {
            Ok(Routes {
                routes: { #build_routes_table }
            })
        }

        impl::cargonauts::server::Service for Routes {
            type Request = ::cargonauts::server::Request;
            type Response = ::cargonauts::server::Response;
            type Error = ::cargonauts::server::Error;
            type Future = ::cargonauts::futures::BoxFuture<Self::Response, Self::Error>;

            fn call(&self, req: Self::Request) -> Self::Future {
                use ::cargonauts::server::Service;
                let route = ::cargonauts::routing::Route::from_request(&req);
                match self.routes.get(&route) {
                    Some(ref route) => route.call(req),
                    None            => ::cargonauts::routing::not_found(req)
                }
            }
        }

        #(#resources)*
    };

    tokens.to_string()
}
