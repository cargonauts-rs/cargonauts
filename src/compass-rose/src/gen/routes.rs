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
        let routes = self.routes.clone();
        let routes2 = self.routes.clone();
        let handlers = self.handlers.clone();
        tokens.append(quote! {
            const ROUTES: ::cargonauts::routing::Routes = ::cargonauts::routing::Routes {
                endpoints: &[
                    #(#routes,)*
                ]
            };
            let mut hash_map = ::std::collections::HashMap::new();
            #( hash_map.insert(#routes2, #handlers); )*
            ::cargonauts::routing::RoutingTable {
                routes: ROUTES, handlers: hash_map,
            }
        });
    }
}

#[derive(Clone)]
struct Route {
    endpoint: String,
    method: Option<MethodKind>,
    rel_method: Option<(RelMethodKind, String)>,
}

impl Route {
    fn build(resources: &[Resource]) -> Vec<Route> {
        let mut vec = vec![];

        for resource in resources {
            let endpoint = resource.header.endpoint.clone().unwrap_or_else(|| {
                resource.header.ty.to_kebab_case()
            });
            
            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                vec.push(Route {
                    endpoint: endpoint.clone(),
                    method: Some(method.method),
                    rel_method: None
                });
            }

            for relation in resource.members.iter().filter_map(|m| m.as_relation()) {
                for method in relation.members.iter().filter_map(|m| m.as_method()) {
                    let rel_endpoint = relation.endpoint.clone().unwrap_or_else(|| relation.rel.to_kebab_case());
                    vec.push(Route {
                        endpoint: endpoint.clone(),
                        method: None,
                        rel_method: Some((method.method, rel_endpoint)),
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
        if let Some(method) = self.method {
            tokens.append(quote! { (#endpoint, #method) });
        } else if let Some((_, ref rel)) = self.rel_method {
            tokens.append(quote! {
                (#endpoint, ::cargonauts::routing::MethodKind::GetRel(#rel))
            });
        }
    }
}

impl ToTokens for MethodKind {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append(match *self {
            MethodKind::Get     => quote!(::cargonauts::routing::MethodKind::Get),
            MethodKind::Index   => quote!(::cargonauts::routing::MethodKind::Index),
        })
    }
}

#[derive(Clone)]
struct Handler {
    format: String,
    resource: String,
    method: Option<MethodKind>,
    rel_method: Option<(RelMethodKind, String)>,
}

impl Handler {
    fn build(resources: &[Resource]) -> Vec<Handler> {
        let mut vec = vec![];

        for resource in resources {
            let resource_ty = &resource.header.ty;
            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                vec.push(Handler {
                    format: method.format.clone(),
                    resource: resource_ty.clone(),
                    method: Some(method.method),
                    rel_method: None,
                })
            }

            for relation in resource.members.iter().filter_map(|m| m.as_relation()) {
                for method in relation.members.iter().filter_map(|m| m.as_method()) {
                    vec.push(Handler {
                        format: method.format.clone(),
                        resource: resource_ty.clone(),
                        method: None,
                        rel_method: Some((method.method, relation.rel.clone())),
                    });
                }
            }
        }

        vec
    }
}

impl ToTokens for Handler {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let format = Ident::new(&self.format[..]);
        let resource = Ident::new(&self.resource[..]);;
        let method = if let Some(method) = self.method {
            match method {
                MethodKind::Get => quote! {
                    ::cargonauts::api::Get<Identifier = <#resource as ::cargonauts::api::Resource>::Identifier>
                },
                MethodKind::Index => quote! {
                    ::cargonauts::api::Index<Identifier = <#resource as ::cargonauts::api::Resource>::Identifier>
                },
            }
        } else if let Some((method, ref rel)) = self.rel_method {
            let rel = Ident::new(&rel[..]);
            match method {
                RelMethodKind::GetOne => quote! {
                    ::cargonauts::api::relations::GetOne<#rel, Identifier = <#resource as ::cargonauts::api::Resource>::Identifier>
                },
                RelMethodKind::GetMany => quote! {
                    ::cargonauts::api::relations::GetMany<#rel, Identifier = <#resource as ::cargonauts::api::Resource>::Identifier>
                },
            }
        } else { panic!() };
        tokens.append(quote!({
            use ::cargonauts::server::NewService;
            Box::new(<(#resource, #format, #method) as ::cargonauts::routing::Method>::new_service().new_service()?)
                as ::cargonauts::routing::Handler
        }))
    }
}
