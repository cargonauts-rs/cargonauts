use heck::KebabCase;
use quote::{ToTokens, Tokens, Ident};

use ast::*;

impl ToTokens for Resource {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let resource_ty = Ident::new(&self.header.ty[..]);
        let resource_name = self.header.ty.to_kebab_case();
        let endpoint = self.header.endpoint.as_ref().unwrap_or(&resource_name);
        let rel_links = self.rel_links();
        let relationships = self.relationships();
        let rel_ids = self.rel_ids();
        let rel_ids_ty = Ident::new(&rel_ids.ty[..]);
        tokens.append(quote! {
            impl ::cargonauts::routing::ResourceEndpoint for #resource_ty {
                const ENDPOINT: &'static str = #endpoint;
                const RESOURCE: &'static str = #resource_name;
                const REL_LINKS: &'static [::cargonauts::routing::RelationshipLink] = &#rel_links;
                type RelIds = #rel_ids_ty;
            }

            #(#relationships)*

            #rel_ids; 
        })
    }
}

impl Resource {
    fn rel_links(&self) -> Vec<RelLink> {
        self.members.iter().filter_map(|m| m.as_relation()).map(|rel| RelLink {
            resource: &self.header.ty,
            relation: &rel.rel,
        }).collect()
    }

    fn relationships(&self) -> Vec<Relationship> {
        self.members.iter().filter_map(|m| m.as_relation()).map(|rel| Relationship {
            resource: &self.header.ty,
            relation: &rel.rel,
            endpoint: rel.endpoint.as_ref(),
            kind: rel.kind,
        }).collect()
    }

    fn rel_ids(&self) -> RelIds {
        RelIds {
            ty: self.header.ty.clone() + "RelIds",
            resource: &self.header.ty,
            rels: self.members.iter().filter_map(|m| m.as_relation()).map(|rel| (&rel.rel[..], rel.kind)).collect(),
        }
    }
}

struct RelLink<'a> {
    resource: &'a str,
    relation: &'a str,
}

impl<'a> ToTokens for RelLink<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let relation_ty = Ident::new(self.relation);
        let resource_ty = Ident::new(self.resource);
        tokens.append(quote! {
            ::cargonauts::routing::RelationshipLink {
                endpoint: <#resource_ty as ::cargonauts::routing::RelationEndpoint<#relation_ty>>::REL_ENDPOINT,
                relation: <#resource_ty as ::cargonauts::routing::RelationEndpoint<#relation_ty>>::RELATION,
            }
        })
    }
}


struct Relationship<'a> {
    resource: &'a str,
    relation: &'a str,
    endpoint: Option<&'a String>,
    kind: RelationKind,
}

impl<'a> ToTokens for Relationship<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let relation_ty = Ident::new(self.relation);
        let resource_ty = Ident::new(self.resource);
        let relation = self.relation.to_kebab_case();
        let endpoint = self.endpoint.unwrap_or(&relation);
        tokens.append(quote! {
            impl ::cargonauts::routing::RelationEndpoint<#relation_ty> for #resource_ty {
                const REL_ENDPOINT: &'static str = #endpoint;
                const RELATION: &'static str = #relation;
            }
        });

        tokens.append(match self.kind {
            RelationKind::Single => quote! {
                impl ::cargonauts::routing::HasOneEndpoint<#relation_ty> for #resource_ty {
                }
            },
            RelationKind::Many => quote! {
                impl ::cargonauts::routing::HasManyEndpoint<#relation_ty> for #resource_ty {
                }
            },
        });
    }
}

struct RelIds<'a> {
    ty: String,
    resource: &'a str,
    rels: Vec<(&'a str, RelationKind)>,
}

impl<'a> ToTokens for RelIds<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let resource = Ident::new(self.resource);
        let ty = Ident::new(&self.ty[..]);
        let rels = self.rels.iter().map(|&(rel, kind)| {
            let rel = Ident::new(rel);
            match kind {
                RelationKind::Single    => quote!(#rel: Option<String>,),
                RelationKind::Many      => quote!(#rel: Vec<String>,),
            }
        });

        let single_try_set_arms = self.rels.iter().filter_map(|&(rel, kind)| {
            if let RelationKind::Single = kind {
                let rel = Ident::new(rel);
                Some(quote!(<#resource as ::cargonauts::routing::RelationEndpoint<#rel>>::RELATION => {
                    self.#rel = Some(id);
                    true
                }))
            } else { None }
        });

        let many_try_set_arms = self.rels.iter().filter_map(|&(rel, kind)| {
            if let RelationKind::Many = kind {
                let rel = Ident::new(rel);
                Some(quote!(<#resource as ::cargonauts::routing::RelationEndpoint<#rel>>::RELATION => {
                    self.#rel = ids;
                    true
                }))
            } else { None }
        });

        let single_set_arms = self.rels.iter().filter_map(|&(rel, kind)| {
            if let RelationKind::Single = kind {
                let rel = Ident::new(rel);
                Some(quote!(<#resource as ::cargonauts::routing::RelationEndpoint<#rel>>::RELATION => self.#rel = Some(id),))
            } else { None }
        });

        let single_get_arms = self.rels.iter().filter_map(|&(rel, kind)| {
            if let RelationKind::Single = kind {
                let rel = Ident::new(rel);
                Some(quote!(<#resource as ::cargonauts::routing::RelationEndpoint<#rel>>::RELATION => self.#rel.as_ref().map(|s| &s[..]),))
            } else { None }
        });

        let many_set_arms = self.rels.iter().filter_map(|&(rel, kind)| {
            if let RelationKind::Many = kind {
                let rel = Ident::new(rel);
                Some(quote!(<#resource as ::cargonauts::routing::RelationEndpoint<#rel>>::RELATION => self.#rel = ids,))
            } else { None }
        });

        let many_get_arms = self.rels.iter().filter_map(|&(rel, kind)| {
            if let RelationKind::Many = kind {
                let rel = Ident::new(rel);
                Some(quote!(<#resource as ::cargonauts::routing::RelationEndpoint<#rel>>::RELATION => &self.#rel[..],))
            } else { None }
        });

        tokens.append(quote! {
            #[allow(non_snake_case)]
            #[derive(Default)]
            pub struct #ty {
                #(#rels)*
            }

            #[allow(unused_variables)]
            impl ::cargonauts::routing::RelIds<#resource> for #ty {
                fn try_set_rel_id(&mut self, rel: &str, id: String) -> bool {
                    match rel {
                        #(#single_try_set_arms)*
                        _ => false,
                    }
                }

                fn try_set_rel_ids(&mut self, rel: &str, ids: Vec<String>) -> bool {
                    match rel {
                        #(#many_try_set_arms)*
                        _ => false,
                    }
                }


                fn set_rel_id<R>(&mut self, id: String)
                where
                    R: ::cargonauts::api::Relationship,
                    #resource: ::cargonauts::routing::HasOneEndpoint<R>,
                    R::Related: ::cargonauts::routing::ResourceEndpoint,
                {
                    match <#resource as ::cargonauts::routing::RelationEndpoint<R>>::RELATION {
                        #(#single_set_arms)*
                        _ => (),
                    }
                }

                fn rel_id<R>(&self) -> Option<&str>
                where
                    R: ::cargonauts::api::Relationship,
                    #resource: ::cargonauts::routing::HasOneEndpoint<R>,
                    R::Related: ::cargonauts::routing::ResourceEndpoint,
                {
                    match <#resource as ::cargonauts::routing::RelationEndpoint<R>>::RELATION {
                        #(#single_get_arms)*
                        _ => None,
                    }
                }

                fn set_rel_ids<R>(&mut self, id: Vec<String>)
                where
                    R: ::cargonauts::api::Relationship,
                    #resource: ::cargonauts::routing::HasManyEndpoint<R>,
                    R::Related: ::cargonauts::routing::ResourceEndpoint,
                {
                    match <#resource as ::cargonauts::routing::RelationEndpoint<R>>::RELATION {
                        #(#many_set_arms)*
                        _ => (),
                    }
                }

                fn rel_ids<R>(&self) -> &[String]
                where
                    R: ::cargonauts::api::Relationship,
                    #resource: ::cargonauts::routing::HasManyEndpoint<R>,
                    R::Related: ::cargonauts::routing::ResourceEndpoint,
                {
                    match <#resource as ::cargonauts::routing::RelationEndpoint<R>>::RELATION {
                        #(#many_get_arms)*
                        _ => &[],
                    }
                }
            }
        })
    }
}
