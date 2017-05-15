use syn::*;
use quote::Tokens;

pub fn deserialize(ast: DeriveInput) -> Tokens {
    let ty = &ast.ident;

    let fields = match ast.body {
        Body::Struct(VariantData::Struct(ref fields))   => fields,
        Body::Struct(VariantData::Tuple(_))             => panic!("ApiDeserialize cannot be derived for tuple structs."),
        Body::Struct(VariantData::Unit)                 => panic!("ApiDeserialize cannot be derived for unit structs."),
        Body::Enum(_)                                   => panic!("ApiDeserialize cannot be derived for enums."),
    };

    let ids = fields.iter().filter(|field| ::is_id(field)).collect::<Vec<_>>();

    match ids.len() {
        0                           => {
            let allow_id = ast.attrs.iter().any(|attr| match attr.value {
                MetaItem::Word(ref ident)   => ident.as_ref() == "ignore_api_id",
                _                           => false,
            });
            deserialize_no_id(ty, fields, allow_id)
        }
        1 if is_option(&ids[0].ty)  => deserialize_with_id(ty, fields),
        1                           => deserialize_with_id_required(ty, fields),
        _                           => panic!("Could not derive ApiDeserailize: cannot tag more than one field #[api_id]"),
    }
}

fn deserialize_no_id(ty: &Ident, fields: &[Field], allow_id: bool) -> Tokens {
    let client_id_policy = match allow_id {
        true    => quote!(::cargonauts::format::jsonapi::ClientIdPolicy::Ignored),
        false   => quote!(::cargonauts::format::jsonapi::ClientIdPolicy::NotAccepted),
    };

    let field_decls = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! { let mut #ident: Option<#ty> = None; }
    });

    let field_assignments = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let name: &str = ident.as_ref();
        let ty = &field.ty;
        quote! {
            #name => {
                if #ident.is_none() {
                    #ident = Some(map.next_value::<#ty>()?);
                } else {
                    return Err(A::Error::duplicate_field(#name))
                }
            }
        }
    });

    let field_unwraps = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let name: &str = ident.as_ref();
        let ty=  &field.ty;
        quote! { let #ident: #ty = #ident.ok_or(A::Error::missing_field(#name))?; }
    });

    let field_uses = fields.iter().map(|field| field.ident.as_ref().unwrap());

    quote! {
        impl<'__derive_d> ::cargonauts::format::jsonapi::ApiDeserialize<'__derive_d> for #ty {
            const CLIENT_ID_POLICY: ::cargonauts::format::jsonapi::ClientIdPolicy = #client_id_policy;

            type Identifier = String;
            type Attributes = ();

            fn from_parts(_: Option<Self::Identifier>, _: Self::Attributes) -> Self {
                panic!("ApiDeserialize::from_parts called on a type which does not accept client ids.")
            }

            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::cargonauts::serde::Deserializer<'__derive_d>,
            {
                #[allow(bad_style)]
                struct __derive_Visitor;

                impl<'__derive_d> ::cargonauts::serde::de::Visitor<'__derive_d> for __derive_Visitor {
                    type Value = #ty;
                    
                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "an attributes object")
                    }
                    
                    #[allow(unused_mut)]
                    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
                        where A: ::cargonauts::serde::de::MapAccess<'__derive_d>,
                    {
                        use cargonauts::serde::de::Error;
                        #(#field_decls)*
                        while let Some(key) = map.next_key()? {
                            match key {
                                #(#field_assignments)*
                                _ => continue,
                            }
                        }
                        #(#field_unwraps)*
                        Ok(#ty { #(#field_uses, )* })
                    }
                }

                deserializer.deserialize_map(__derive_Visitor)
            }
        }
    }
}

fn deserialize_with_id(ty: &Ident, fields: &[Field]) -> Tokens {
    panic!("deserializing with ids not implemented")
}

fn deserialize_with_id_required(ty: &Ident, fields: &[Field]) -> Tokens {
    panic!("deserializing with ids not implemented")
}

fn is_option(ty: &Ty) -> bool {
    if let Ty::Path(_, ref path) = *ty {
        path.segments.iter().last().unwrap().ident.as_ref() == "Option"
    } else {
        false
    }
}
