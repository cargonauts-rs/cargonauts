use std::fs;
use std::path::{Path, PathBuf};

use heck::SnekCase;
use itertools::Itertools;
use quote::{ToTokens, Tokens, Ident};

use cfg::CargonautsConfig;

use ast::*;

pub fn formats(routes: &Routes, cfg: &CargonautsConfig) -> Tokens {
    let root = cfg.templates();
    let method_refs = &MethodRef::build(routes).into_iter().group_by(|m| m.format);
    let builders = method_refs.into_iter().map(|(format, methods)| {
        let format = Ident::new(format);

        let templates = methods.filter_map(|method| {
            method.template(&root).map(|path| (path, method))
        }).map(|(path, method)| {
            let r = method.resource;
            let m = method.method;
            let key = match method.relation {
                Some(rel)   => quote!(::cargonauts::formats::TemplateKey::new_rel(#r, #rel, #m)),
                None        => quote!(::cargonauts::formats::TemplateKey::new(#r, #m)),
            };
            Template { key, path }
        });

        quote!({
            <#format as ::cargonauts::formats::BuildFormat>::build(&[
                #(#templates,)*
            ])?
        })
    });
    quote!({
        let mut formats = ::cargonauts::routing::FormatLender::default();
        #(formats.store(#builders);)*
        Ok(formats)
    })
}

#[derive(Copy, Clone)]
struct MethodRef<'a> {
    format: &'a str,
    method: &'a str,
    resource: &'a str,
    relation: Option<&'a str>,
}

impl<'a> MethodRef<'a> {
    fn build(routes: &'a Routes) -> Vec<MethodRef<'a>> {
        let mut vec = vec![];
        routes.visit_resources(&mut vec, |vec, resource| {
            for member in &resource.members {
                match *member {
                    ResourceMember::Method(ref m, _)    => {
                        for method in &m.methods {
                            vec.push(MethodRef {
                                format: &m.format[..],
                                method: &method[..],
                                resource: &resource.header.ty[..],
                                relation: None,
                            });
                        }
                    }
                    ResourceMember::Relation(ref rel, _) => {
                        for member in &rel.members {
                            match *member {
                                RelationMember::Method(ref m, _) => {
                                    for method in &m.methods {
                                        vec.push(MethodRef {
                                            format: &m.format[..],
                                            method: &method[..],
                                            resource: &resource.header.ty[..],
                                            relation: Some(&rel.rel[..]),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        vec
    }

    fn template(self, root: &Path) -> Option<PathBuf> {
        let mut root = root.join(self.resource.to_snek_case());
        if let Some(rel) = self.relation { root.push(rel.to_snek_case()); }
        let read_dir = if let Some(rd) = fs::read_dir(root).ok() { rd } else { return None };
        for entry in read_dir {
            let entry = entry.unwrap();
            let filename = entry.file_name();
            if self.method.to_snek_case() == filename.to_str().unwrap().split('.').next().unwrap() {
                return Some(entry.path())
            }
        }
        None
    }
}

struct Template {
    path: PathBuf,
    key: Tokens,
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let path = self.path.to_string_lossy();
        let key = &self.key;
        tokens.append(quote! {
            ::cargonauts::formats::Template {
                key: #key,
                template: include_str!(#path),
            }
        })
    }
}
