#![recursion_limit="256"]

#[macro_use] extern crate proc_macro_hack;
#[macro_use] extern crate quote;

extern crate cargonauts_config as cfg;
extern crate heck;
extern crate itertools;
extern crate serde_json as json;
extern crate walkdir;

mod ast;
mod gen;
mod parser;

proc_macro_item_impl! {
    pub fn routes_impl(input: &str) -> String {
        let cfg = cfg::CargonautsConfig::find_and_parse().ok();
        gen::code_gen(parser::parse_routes(input).unwrap(), cfg)
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
