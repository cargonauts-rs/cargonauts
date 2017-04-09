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
        let handlers = self.handlers.clone();
        tokens.append(quote! {
            let mut hash_map = ::std::collections::HashMap::new();
            #( hash_map.insert(#routes, #handlers); )*
            hash_map
        });
    }
}

#[derive(Clone)]
struct Route {
    endpoint: String,
    method: MethodKind,
}

impl Route {
    fn build(resources: &[Resource]) -> Vec<Route> {
        let mut vec = vec![];

        for resource in resources {
            let endpoint = resource.header.endpoint.clone().unwrap_or_else(|| {
                resource.header.ty.to_kebab_case()
            });
            
            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                vec.push(Route { method: method.method, endpoint: endpoint.clone(), });
            }
        }

        vec
    }
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let endpoint = &self.endpoint;
        let method = self.method;
        tokens.append(quote! {
            ::cargonauts::routing::Route {
                endpoint: String::from(#endpoint),
                method: #method
            }
        })
    }
}

impl ToTokens for MethodKind {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            MethodKind::Get     => tokens.append(quote!(::cargonauts::routing::Method::Get)),
            MethodKind::Index   => tokens.append(quote!(::cargonauts::routing::Method::Index)),
        }
    }
}

#[derive(Clone)]
struct Handler {
    method: MethodKind,
    format: String,
    resource: String,
}

impl Handler {
    fn build(resources: &[Resource]) -> Vec<Handler> {
        let mut vec = vec![];

        for resource in resources {
            let resource_ty = &resource.header.ty;
            for method in resource.members.iter().filter_map(|m| m.as_method()) {
                vec.push(Handler {
                    method: method.method,
                    format: method.format.clone(),
                    resource: resource_ty.clone(),
                })
            }
        }

        vec
    }
}

impl ToTokens for Handler {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let format = Ident::new(&self.format[..]);
        let resource = Ident::new(&self.resource[..]);;
        let handler = match self.method {
            MethodKind::Get => Ident::new("new_resource_service"),
            MethodKind::Index => Ident::new("new_collection_service"),
        };
        let req = match self.method {
            MethodKind::Get => quote!(::cargonauts::format::GetRequest<#resource>),
            MethodKind::Index => quote!(::cargonauts::format::IndexRequest<#resource>),
        };
        let service = match self.method {
            MethodKind::Get => quote! {
                <::cargonauts::format::GetRequest<#resource> as ::cargonauts::format::Request<#resource>>::Service
            },
            MethodKind::Index => quote! {
                <::cargonauts::format::IndexRequest<#resource> as ::cargonauts::format::Request<#resource>>::Service
            },
        };
        tokens.append(quote!({
            use ::cargonauts::server::NewService;
            Box::new(::cargonauts::routing::#handler::<#format, #req, #resource, #service>().new_service()?)
                as ::cargonauts::routing::Handler
        }))
    }
}
