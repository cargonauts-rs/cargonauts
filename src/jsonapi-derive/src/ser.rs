use syn::*;
use quote::Tokens;

pub fn serialize(ast: DeriveInput) -> Tokens {
    let ty = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = match ast.body {
        Body::Struct(VariantData::Struct(ref fields))   => fields,
        Body::Struct(VariantData::Tuple(_))             => panic!("ApiSerialize cannot be derived for tuple structs."),
        Body::Struct(VariantData::Unit)                 => panic!("ApiSerialize cannot be derived for unit structs."),
        Body::Enum(_)                                   => panic!("ApiSerialize cannot be derived for enums."),
    };

    let id_body = id_body(&fields[..]);
    let ser_body = serialize_body(&fields[..]);
    quote! {
        impl #impl_generics ::cargonauts::formats::jsonapi::ApiSerialize for #ty #ty_generics
            #where_clause
        {
            fn identifier(&self) -> String {
                #id_body
            }

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            fn serialize<S: ::cargonauts::serde::Serializer>(
                &self,
                fields: Option<&::cargonauts::formats::jsonapi::Fields>,
                serializer: S
            ) -> Result<S::Ok, S::Error> {
                use ::cargonauts::serde::ser::{Serializer, SerializeMap};
                #ser_body
            }
        }
    }
}

fn id_body(fields: &[Field]) -> Tokens {
    let id_fields = fields.iter().filter(|field| ::is_id(field)).collect::<Vec<_>>();

    if id_fields.is_empty() {
        panic!("Could not derive ApiSerialize: must tag one field #[api_id]")
    }
    if id_fields.len() > 1 {
        panic!("Could not derive ApiSerialize: cannot tag more than one field #[api_id]")
    }

    let id = &id_fields[0].ident;

    quote! {
        self.#id.to_string()
    }
}

fn serialize_body(fields: &[Field]) -> Tokens {
    let len = fields.len() - 1;

    let attrs = fields.iter().filter(|field| !::is_id(field)).map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let key: &str = ident.as_ref();
        quote! {
            map.serialize_entry(#key, &self.#ident)?;
        }
    });

    let fieldset_attrs = fields.iter().filter(|field| !::is_id(field)).map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let key: &str = ident.as_ref();
        quote! {
            if fieldset.contains(#key) {
                map.serialize_entry(#key, &self.#ident)?;
            }
        }
    });

    quote! {
        match fields {
            Some(fieldset)  => {
                let mut map = serializer.serialize_map(None)?;
                #( #fieldset_attrs )*
                map.end()
            }
            None            => {
                let mut map = serializer.serialize_map(Some(#len))?;
                #( #attrs )*
                map.end()
            }
        }
    }
}
