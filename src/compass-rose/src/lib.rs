#![recursion_limit="256"]

#[macro_use] extern crate proc_macro_hack;
#[macro_use] extern crate quote;

extern crate heck;

mod ast;
mod gen;
mod parser;

proc_macro_item_impl! {
    pub fn routes_impl(input: &str) -> String {
        gen::code_gen(parser::parse_routes(input).unwrap().resources)
    }
}

#[test]
fn parse_resources() {
    assert_eq!(&*parser::parse_routes("resource Foo @ \"foo\";").unwrap().resources[0].header.ty, "Foo")
}

#[test]
fn parse_resources_with_setup() {
    assert_eq!(&*parser::parse_routes("setup {} resource Foo;").unwrap().resources[0].header.ty, "Foo")
}
