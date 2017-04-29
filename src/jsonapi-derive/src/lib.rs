extern crate proc_macro;
extern crate syn;

#[macro_use] extern crate quote;

mod ser;

use proc_macro::TokenStream;

#[proc_macro_derive(ApiDeserialize)]
pub fn api_deserialize(tokens: TokenStream) -> TokenStream {
    panic!()
}

#[proc_macro_derive(ApiSerialize, attributes(api_id))]
pub fn api_serialize(tokens: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&tokens.to_string()).unwrap();
    ser::serialize(ast).to_string().parse().unwrap()
}
