use std::fs;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use quote::{ToTokens, Tokens, Ident};

use cfg::CargonautsConfig;

use ast::*;

pub fn formats(routes: &Routes, cfg: &CargonautsConfig) -> Tokens {
    let root = cfg.templates();
    let method_refs = &MethodRef::build(routes).into_iter().group_by(|m| m.format);
    let builders = method_refs.into_iter().map(|(format, methods)| {
        let format = Ident::new(format);
        let templates = methods.filter_map(|method| method.template(&root)).map(|path| Template { path, root: &root });

        quote!({
            <#format as ::cargonauts::format::BuildFormat>::build(&[
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
        let mut root = root.join(self.resource);
        if let Some(rel) = self.relation { root.push(rel); }
        let read_dir = if let Some(rd) = fs::read_dir(root).ok() { rd } else { return None };
        for entry in read_dir {
            let entry = entry.unwrap();
            if entry.file_name().to_str().unwrap().starts_with(self.method) {
                return Some(entry.path())
            }
        }
        None
    }
}

struct Template<'a> {
    path: PathBuf,
    root: &'a Path,
}

impl<'a> ToTokens for Template<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let full_path = self.path.to_string_lossy();
        let path = self.path.strip_prefix(&self.root).unwrap().to_string_lossy();
        tokens.append(quote! {
            ::cargonauts::format::Template {
                path: #path,
                template: include_str!(#full_path),
            }
        })
    }
}
