use heck::KebabCase;
use quote::{ToTokens, Tokens, Ident};

use ast::*;

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new(resources: &[Resource]) -> Routes {
        Routes {
            routes: Route::build(resources),
        }
    }
}

impl ToTokens for Routes {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let routes = &self.routes;
        tokens.append(quote! ({
            use ::cargonauts::server::NewService;
            let routes = vec!#routes.into_iter().collect();
            ::cargonauts::routing::RoutingTable::new(routes, env)
        }));
    }
}

#[derive(Clone)]
struct Route {
    resource: String,
    endpoint: String,
    method: String,
    format: String,
    rel: Option<String>,
    middleware: Option<String>,
}

impl Route {
    fn build(resources: &[Resource]) -> Vec<Route> {
        let mut vec = vec![];

        for resource in resources {
            let endpoint = resource.header.endpoint.clone().unwrap_or_else(|| {
                resource.header.ty.to_kebab_case()
            });

            let resource_ty = &resource.header.ty;

            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                let middleware = method.attrs.iter().filter_map(|attr| attr.as_middleware()).next();
                vec.push(Route {
                    resource: resource_ty.clone(),
                    endpoint: endpoint.clone(),
                    method: method.method.clone(),
                    format: method.format.clone(),
                    rel: None,
                    middleware,
                })
            }

            for relation in resource.members.iter().filter_map(|m| m.as_relation()) {
                for method in relation.members.iter().filter_map(|m| m.as_method()) {
                    let middleware = method.attrs.iter().filter_map(|attr| attr.as_middleware()).next();
                    vec.push(Route {
                        resource: resource_ty.clone(),
                        endpoint: endpoint.clone(),
                        method: method.method.clone(),
                        format: method.format.clone(),
                        rel: Some(relation.rel.clone()),
                        middleware,
                    })
                }
            }
        }

        vec
    }
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let endpoint = &self.endpoint;
        let resource = Ident::new(&self.resource[..]);
        let format = Ident::new(&self.format[..]);
        let method = method_for(&self.method, self.rel.as_ref(), &resource);

        tokens.append(quote!({
            let route = <#method as ::cargonauts::method::Method<#resource>>::ROUTE;
            let route_key = ::cargonauts::routing::RouteKey::new(#endpoint, route);
            let endpoint = ::cargonauts::routing::endpoint::<_, _, (#resource, #format, #method)>;
            (route_key, endpoint as ::cargonauts::routing::Handler)
        }))
    }
}

fn method_for(method: &str, rel: Option<&String>, resource: &Ident) -> Tokens {
    // Special cases to allow mainsail methods to have associated types
    match method {
        "Post"  => {
            quote!(Post<Identifier = <#resource as ::cargonauts::api::Resource>::Identifier, Post = <#resource as ::cargonauts::api::Post>::Post>)
        }
        "Patch" => {
            quote!(Patch<Identifier = <#resource as ::cargonauts::api::Resource>::Identifier, Patch = <#resource as ::cargonauts::api::Patch>::Patch>)
        }
        _       => {
            let method = Ident::new(method);
            match rel {
                Some(rel)   => {
                    let rel = Ident::new(&rel[..]);
                    quote!(#method<#rel, Identifier = <#resource as ::cargonauts::api::Resource>::Identifier>)
                }
                None        => {
                    quote!(#method<Identifier = <#resource as ::cargonauts::api::Resource>::Identifier>)
                }
            }
        }
    }
}
