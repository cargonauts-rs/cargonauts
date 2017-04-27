use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use cfg::CargonautsConfig;
use heck::KebabCase;
use quote::{ToTokens, Tokens, Ident};

use ast::*;

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new(resources: &[Resource], cfg: Option<&CargonautsConfig>) -> Routes {
        Routes {
            routes: Route::build(resources, cfg),
        }
    }
}

impl ToTokens for Routes {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let routes = &self.routes;
        tokens.append(quote! ({
            use ::cargonauts::server::NewService;
            let routes = vec!#routes.into_iter().collect();
            ::cargonauts::routing::RoutingTable::new(routes, assets, asset_handler, env)
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
    template_root: PathBuf,
}

impl Route {
    fn build(resources: &[Resource], cfg: Option<&CargonautsConfig>) -> Vec<Route> {
        let mut vec = vec![];

        let root = {
            let root: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
            let sub = cfg.and_then(|cfg| cfg.templates()).unwrap_or(Path::new("src/templates"));
            root.join(sub)
        };
        

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
                    template_root: root.clone(),
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
                        template_root: root.clone(),
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
        let template = load_template(self.template_root.clone(),
                                     &self.resource,
                                     &self.method,
                                     self.rel.as_ref());
        let resource = Ident::new(&self.resource[..]);
        let format = Ident::new(&self.format[..]);
        let method = method_for(&self.method, self.rel.as_ref(), &resource);

        tokens.append(quote!({
            fn handle<H, R, E>(req: ::cargonauts::server::Request, env: ::cargonauts::api::Environment) -> E::Future
            where
                E: ?Sized + ::cargonauts::routing::Endpoint<H, R>,
            {
                ::cargonauts::routing::endpoint::<_, _, E>(req, #template, env)
            }
            let route = <#method as ::cargonauts::method::Method<#resource>>::ROUTE;
            let route_key = ::cargonauts::routing::RouteKey::new(#endpoint, route);
            (route_key, handle::<_, _, (#resource, #format, #method)> as ::cargonauts::routing::Handler)
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

fn load_template(mut path: PathBuf, resource: &str, method: &str, rel: Option<&String>) -> Tokens {
    path.push(resource);
    if let Some(rel) = rel { path.push(rel); }

    let match_method = |entry: &fs::DirEntry| {
        entry.file_name().into_string().unwrap().starts_with(&method.to_kebab_case())
    };

    let mut walk_dir = fs::read_dir(path).into_iter().flat_map(|ls| ls.into_iter().map(|e| e.unwrap()));

    if let Some(entry) = walk_dir.find(match_method) {
        let path = entry.path();
        let path = path.to_string_lossy();
        quote!(Some(::cargonauts::format::Template::static_prepare(include_str!(#path))))
    } else {
        quote!(None)
    }
}
