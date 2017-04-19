use heck::KebabCase;
use quote::{ToTokens, Tokens, Ident};

use ast::*;

pub struct Routes {
    routes: Vec<Route>,
    handlers: Vec<Handler>,
}

impl Routes {
    pub fn new(resources: &[Resource]) -> Routes {
        Routes {
            routes: Route::build(resources),
            handlers: Handler::build(resources),
        }
    }
}

impl ToTokens for Routes {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let route_key: Vec<_> = self.routes.iter().flat_map(|route| &route.subroutes).collect();

        let routes = &self.routes;
        let handlers = self.handlers.clone();
        tokens.append(quote! {
            use ::cargonauts::server::NewService;
            let routes = ::cargonauts::routing::Routes {
                endpoints: vec! #routes
            };
            let mut hash_map = ::std::collections::HashMap::new();
            #( hash_map.insert(#route_key, #handlers); )*
            ::cargonauts::routing::RoutingTable {
                routes, handlers: hash_map,
            }
        });
    }
}

#[derive(Clone)]
struct Route {
    endpoint: String,
    subroutes: Vec<SubRoute>,
}

impl Route {
    fn build(resources: &[Resource]) -> Vec<Route> {
        let mut vec = vec![];

        for resource in resources {
            let endpoint = resource.header.endpoint.clone().unwrap_or_else(|| {
                resource.header.ty.to_kebab_case()
            });

            let resource_ty = &resource.header.ty;

            let mut subroutes = vec![];
            
            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                subroutes.push(SubRoute {
                    resource: resource_ty.clone(),
                    method: method.method.clone(),
                    rel: None,
                });
            }

            for relation in resource.members.iter().filter_map(|m| m.as_relation()) {
                for method in relation.members.iter().filter_map(|m| m.as_method()) {
                    subroutes.push(SubRoute {
                        resource: resource_ty.clone(),
                        method: method.method.clone(),
                        rel: Some(relation.rel.clone()),
                    });
                }
            }

            vec.push(Route {
                endpoint,
                subroutes,
            })
        }

        vec.sort_by_key(|r| r.endpoint.clone());
        vec
    }
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let endpoint = &self.endpoint;
        let routes = &self.subroutes;
        tokens.append(quote! {
            (#endpoint, vec! #routes)
        })
    }
}

#[derive(Clone)]
struct SubRoute {
    resource: String,
    method: String,
    rel: Option<String>,
}

impl ToTokens for SubRoute {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let resource = Ident::new(&self.resource[..]);
        let method = Ident::new(&self.method[..]);
        if let Some(ref rel) = self.rel {
            let rel = Ident::new(&rel[..]);
            tokens.append(quote! {
                <#method<#rel, Identifier = <#resource as ::cargonauts::api::Resource>::Identifier> as ::cargonauts::method::Method<#resource>>::ROUTE
            })
        } else {
            tokens.append(quote! {
                <#method<Identifier = <#resource as ::cargonauts::api::Resource>::Identifier> as ::cargonauts::method::Method<#resource>>::ROUTE
            })
        }
    }
}

#[derive(Clone)]
struct Handler {
    endpoint: String,
    format: String,
    resource: String,
    method: String,
    rel: Option<String>,
    middleware: Option<String>,
}

impl Handler {
    fn build(resources: &[Resource]) -> Vec<Handler> {
        let mut vec = vec![];

        for resource in resources {
            let resource_ty = &resource.header.ty;
            let endpoint = resource.header.endpoint.clone().unwrap_or_else(|| {
                resource.header.ty.to_kebab_case()
            });
            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                let middleware = method.attrs.iter().filter_map(|attr| attr.as_middleware()).next();
                vec.push(Handler {
                    endpoint: endpoint.clone(),
                    format: method.format.clone(),
                    resource: resource_ty.clone(),
                    method: method.method.clone(),
                    rel: None,
                    middleware,
                })
            }

            for relation in resource.members.iter().filter_map(|m| m.as_relation()) {
                for method in relation.members.iter().filter_map(|m| m.as_method()) {
                    let middleware = method.attrs.iter().filter_map(|attr| attr.as_middleware()).next();
                    vec.push(Handler {
                        endpoint: endpoint.clone(),
                        format: method.format.clone(),
                        resource: resource_ty.clone(),
                        method: method.method.clone(),
                        rel: Some(relation.rel.clone()),
                        middleware,
                    });
                }
            }
        }

        vec.sort_by_key(|r| r.endpoint.clone());
        vec
    }
}

impl ToTokens for Handler {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let format = Ident::new(&self.format[..]);
        let resource = Ident::new(&self.resource[..]);
        let method = if let Some(ref rel) = self.rel {
            let method = Ident::new(&self.method[..]);
            let rel = Ident::new(&rel[..]);
            quote!(#method<#rel, Identifier = <#resource as cargonauts::api::Resource>::Identifier>)
        } else {
            let method = Ident::new(&self.method[..]);
            quote!(#method<Identifier = <#resource as cargonauts::api::Resource>::Identifier>)
        };
        let service = if let Some(_) = self.middleware {
            panic!()
        } else {
            quote!(<::cargonauts::routing::EndpointService<_, _, #resource, (#format, #method)>>)
        };
        tokens.append(quote!({
            Box::new(#service::default().new_service()?) as ::cargonauts::routing::Handler
        }));
    }
}
