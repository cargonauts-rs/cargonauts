use heck::{KebabCase};
use quote::{ToTokens, Tokens, Ident};

use ast::*;

pub struct Routes {
    routes: Vec<Route>,
    assets: Tokens,
}

impl Routes {
    pub fn new(assets: Tokens, resources: &[RouteItem]) -> Routes {
        Routes {
            routes: Route::build(resources),
            assets
        }
    }
}

impl ToTokens for Routes {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let routes = &self.routes;
        let assets = &self.assets;
        tokens.append(quote! ({
            use ::cargonauts::server::NewService;
            let mut routes = ::cargonauts::routing::RouteBuilder::default();
            #(#routes)*
            #assets
            routes.build(env)
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
    rel_endpoint: Option<String>,
    middleware: Option<String>,
}

impl Route {
    fn build(routes: &[RouteItem]) -> Vec<Route> {
        fn routes_for_resource(vec: &mut Vec<Route>, resource: &Resource, module: Option<&str>) {
            let endpoint = match module {
                Some(module)    => {
                    match resource.header.endpoint.as_ref() {
                        Some(endpoint)  => [module, endpoint].join("/"),
                        None            => {
                            let endpoint = resource.header.ty.to_kebab_case();
                            [module, &endpoint].join("/")
                        }
                    }
                }
                None            => {
                    resource.header.endpoint.clone().unwrap_or_else(|| {
                        resource.header.ty.to_kebab_case()
                    })
                }
            };

            let resource_ty = &resource.header.ty;

            for (method, attrs) in resource.members.iter().filter_map(|m| m.as_method()) {
                let middleware = attrs.iter().filter_map(|attr| attr.as_middleware()).next();
                for m in &method.methods {
                    vec.push(Route {
                        resource: resource_ty.clone(),
                        endpoint: endpoint.clone(),
                        method: m.clone(),
                        format: method.format.clone(),
                        rel: None,
                        rel_endpoint: None,
                        middleware: middleware.clone(),
                    })
                }
            }

            for relation in resource.members.iter().filter_map(|m| m.as_relation()) {
                for (method, attrs) in relation.members.iter().filter_map(|m| m.as_method()) {
                    let middleware = attrs.iter().filter_map(|attr| attr.as_middleware()).next();
                    for m in &method.methods {
                        vec.push(Route {
                            resource: resource_ty.clone(),
                            endpoint: endpoint.clone(),
                            method: m.clone(),
                            format: method.format.clone(),
                            rel: Some(relation.rel.clone()),
                            rel_endpoint: relation.endpoint.clone().or_else(|| Some(relation.rel.to_kebab_case())),
                            middleware: middleware.clone(),
                        })
                    }
                }
            }
        }

        fn routes_for_item(vec: &mut Vec<Route>, item: &RouteItem, module: Option<&str>) {
            match *item {
                RouteItem::Resource(ref resource) => {
                    routes_for_resource(vec, resource, module);
                }
                RouteItem::Module(ref inner, ref items) => {
                    let module = match module {
                        Some(module) => [module, &inner.to_kebab_case()].join("/"),
                        None         => inner.to_owned()
                    };
                    
                    for item in items {
                        routes_for_item(vec, item, Some(&module))
                    }
                }
            }
        }

        let mut vec = vec![];

        for item in routes {
            routes_for_item(&mut vec, item, None);
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

        let http_method = quote!(<#method as ::cargonauts::methods::def::Method<#resource>>::ROUTE.method);

        let path = if let Some(ref rel) = self.rel_endpoint {
            quote!(::cargonauts::routing::path(<#method as ::cargonauts::methods::def::Method<#resource>>::ROUTE.kind, #endpoint, Some(#rel)))
        } else {
            quote!(::cargonauts::routing::path(<#method as ::cargonauts::methods::def::Method<#resource>>::ROUTE.kind, #endpoint, None))
        };

        let template_key = {
            let r = &self.resource;
            let m = &self.method;
            match self.rel {
                Some(ref rel)   => quote!(::cargonauts::formats::def::TemplateKey::new_rel(#r, #rel, #m)),
                None            => quote!(::cargonauts::formats::def::TemplateKey::new(#r, #m)),
            }
        };

        if let Some(ref middleware) = self.middleware {
            let middleware = Ident::new(&middleware[..]);
            tokens.append(quote!({
                let service = ::cargonauts::routing::EndpointService::<_, _, #resource, #format, #method>::new(#template_key, formats.get::<#format>());
                let service = ::cargonauts::middleware::Middleware::wrap(<#middleware as Default>::default(), service);
                routes.add(#http_method, #path, Box::new(service) as ::cargonauts::routing::Handler);
            }));
        } else {
            tokens.append(quote!({
                let service = ::cargonauts::routing::EndpointService::<_, _, #resource, #format, #method>::new(#template_key, formats.get::<#format>());
                routes.add(#http_method, #path, Box::new(service) as ::cargonauts::routing::Handler);
            }));
        }

    }
}

fn method_for(method: &str, rel: Option<&String>, resource: &Ident) -> Tokens {
    // Special cases to allow mainsail methods to have associated types
    match method {
        "Post"  => {
            quote!(Post<Identifier = <#resource as ::cargonauts::resources::Resource>::Identifier, Post = <#resource as ::cargonauts::methods::Post>::Post>)
        }
        "Patch" => {
            quote!(Patch<Identifier = <#resource as ::cargonauts::resources::Resource>::Identifier, Patch = <#resource as ::cargonauts::methods::Patch>::Patch>)
        }
        "PostRelated" => {
            let rel = Ident::new(&rel.unwrap()[..]);
            quote!(PostRelated<#rel, Identifier = <#resource as ::cargonauts::resources::Resource>::Identifier, Post = <#resource as ::cargonauts::methods::PostRelated<#rel>::Post>)
        }
        "UpdateRelated" => {
            let rel = Ident::new(&rel.unwrap()[..]);
            quote!(UpdateRelated<#rel, Identifier = <#resource as ::cargonauts::resources::Resource>::Identifier, Update = <#resource as ::cargonauts::methods::UpdateRelated<#rel>::Update>)
        }
        _       => {
            let method = Ident::new(method);
            match rel {
                Some(rel)   => {
                    let rel = Ident::new(&rel[..]);
                    quote!(#method<#rel, Identifier = <#resource as ::cargonauts::resources::Resource>::Identifier>)
                }
                None        => {
                    quote!(#method<Identifier = <#resource as ::cargonauts::resources::Resource>::Identifier>)
                }
            }
        }
    }
}
