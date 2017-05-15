#![recursion_limit = "256"]

extern crate proc_macro;
extern crate syn;

#[macro_use] extern crate quote;

mod de;
mod ser;

use proc_macro::TokenStream;

#[proc_macro_derive(ApiDeserialize, attributes(api_id, ignore_api_id))]
pub fn api_deserialize(tokens: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&tokens.to_string()).unwrap();
    de::deserialize(ast).to_string().parse().unwrap()
}

#[proc_macro_derive(ApiSerialize, attributes(api_id))]
pub fn api_serialize(tokens: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&tokens.to_string()).unwrap();
    ser::serialize(ast).to_string().parse().unwrap()
}

fn is_id(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| match attr.value {
        syn::MetaItem::Word(ref ident)  => ident.as_ref() == "api_id",
        _                               => false,
    })
}
