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
        tokens.append(quote! {
            impl ::cargonauts::routing::ResourceEndpoint for #resource_ty {
                const ENDPOINT: &'static str = #endpoint;
                const RESOURCE: &'static str = #resource_name;
                const REL_LINKS: &'static [::cargonauts::routing::RelationshipLink] = &#rel_links;
            }

            #relationships
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
        }).collect()
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
            <#resource_ty as ::cargonauts::routing::RelationEndpoint<#relation_ty>>::LINK,
        })
    }
}


struct Relationship<'a> {
    resource: &'a str,
    relation: &'a str,
    endpoint: Option<&'a String>,
}

impl<'a> ToTokens for Relationship<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let relation_ty = Ident::new(self.relation);
        let resource_ty = Ident::new(self.resource);
        let relation = self.relation.to_kebab_case();
        let endpoint = self.endpoint.unwrap_or(&relation);
        tokens.append(quote! {
            impl ::cargonauts::routing::RelationEndpoint<#relation_ty> for #resource_ty {
                const LINK: ::cargonauts::routing::RelationshipLink
                    = ::cargonauts::routing::RelationshipLink {
                        endpoint: #endpoint,
                        relation: #relation,
                    };
            }
        })
    }
}
