#![recursion_limit="256"]

#[macro_use] extern crate proc_macro_hack;
#[macro_use] extern crate quote;

extern crate heck;

mod ast;
mod gen;
mod parser;

proc_macro_item_impl! {
    pub fn routes_impl(input: &str) -> String {
        gen::code_gen(parser::parse_resources(input).unwrap())
    }
}

#[test]
fn parse_resources() {
    assert_eq!(&*parser::parse_resources("resource Foo @ \"foo\";").unwrap()[0].header.ty, "Foo")
}
