use front::ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__routes {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use front::ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_3b_22(&'input str),
        Term_22alias_22(&'input str),
        Term_22append_22(&'input str),
        Term_22as_22(&'input str),
        Term_22clear_22(&'input str),
        Term_22delete_22(&'input str),
        Term_22get_22(&'input str),
        Term_22has_22(&'input str),
        Term_22index_22(&'input str),
        Term_22many_22(&'input str),
        Term_22one_22(&'input str),
        Term_22patch_22(&'input str),
        Term_22post_22(&'input str),
        Term_22remove_22(&'input str),
        Term_22replace_22(&'input str),
        Term_22resource_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
        Nt_28_3cResource_3e_29(Resource),
        Nt_28_3cResource_3e_29_2a(::std::vec::Vec<Resource>),
        Nt_28_3cResource_3e_29_2b(::std::vec::Vec<Resource>),
        Nt_28_3cResourceAttr_3e_20_22_3b_22_29(ResourceAttr),
        Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2a(::std::vec::Vec<ResourceAttr>),
        Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(::std::vec::Vec<ResourceAttr>),
        NtAlias(Alias),
        NtIdent(String),
        NtMethod(Method),
        NtRelVariance(RelVariance),
        NtRelation(Relation),
        NtResource(Resource),
        NtResourceAttr(ResourceAttr),
        NtResourceAttrs(Vec<ResourceAttr>),
        Nt____routes(Routes),
        Ntroutes(Routes),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        5, // on "resource", goto 4
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 1
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        5, // on "resource", goto 4
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 2
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        -4, // on "resource", reduce `(<Resource>)+ = Resource => ActionFn(30);`
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 3
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 4
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        8, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 7
        // State 5
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        -5, // on "resource", reduce `(<Resource>)+ = (<Resource>)+, Resource => ActionFn(31);`
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 6
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        9, // on "{", goto 8
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 7
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        -12, // on "{", reduce `Ident = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 8
        0, // on ";", error
        15, // on "alias", goto 14
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        16, // on "has", goto 15
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        -28, // on "}", reduce `ResourceAttrs =  => ActionFn(36);`
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 9
        0, // on ";", error
        15, // on "alias", goto 14
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        16, // on "has", goto 15
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        -29, // on "}", reduce `ResourceAttrs = (<ResourceAttr> ";")+ => ActionFn(37);`
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 10
        -26, // on ";", reduce `ResourceAttr = Alias => ActionFn(15);`
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 11
        -27, // on ";", reduce `ResourceAttr = Relation => ActionFn(16);`
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 12
        18, // on ";", goto 17
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 13
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        19, // on "}", goto 18
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 14
        0, // on ";", error
        0, // on "alias", error
        21, // on "append", goto 20
        0, // on "as", error
        22, // on "clear", goto 21
        23, // on "delete", goto 22
        24, // on "get", goto 23
        0, // on "has", error
        25, // on "index", goto 24
        0, // on "many", error
        0, // on "one", error
        26, // on "patch", goto 25
        27, // on "post", goto 26
        28, // on "remove", goto 27
        29, // on "replace", goto 28
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 15
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        31, // on "many", goto 30
        32, // on "one", goto 31
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 16
        33, // on ";", goto 32
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 17
        0, // on ";", error
        -9, // on "alias", reduce `(<ResourceAttr> ";")+ = ResourceAttr, ";" => ActionFn(34);`
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        -9, // on "has", reduce `(<ResourceAttr> ";")+ = ResourceAttr, ";" => ActionFn(34);`
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        -9, // on "}", reduce `(<ResourceAttr> ";")+ = ResourceAttr, ";" => ActionFn(34);`
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 18
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        -25, // on "resource", reduce `Resource = "resource", Ident, "{", ResourceAttrs, "}" => ActionFn(18);`
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 19
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        34, // on "as", goto 33
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 20
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -18, // on "as", reduce `Method = "append" => ActionFn(6);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 21
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -21, // on "as", reduce `Method = "clear" => ActionFn(9);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 22
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -16, // on "as", reduce `Method = "delete" => ActionFn(4);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 23
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -13, // on "as", reduce `Method = "get" => ActionFn(1);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 24
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -17, // on "as", reduce `Method = "index" => ActionFn(5);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 25
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -14, // on "as", reduce `Method = "patch" => ActionFn(2);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 26
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -15, // on "as", reduce `Method = "post" => ActionFn(3);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 27
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -20, // on "as", reduce `Method = "remove" => ActionFn(8);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 28
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        -19, // on "as", reduce `Method = "replace" => ActionFn(7);`
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 29
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        36, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 35
        // State 30
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        -23, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `RelVariance = "many" => ActionFn(13);`
        // State 31
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        -22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `RelVariance = "one" => ActionFn(12);`
        // State 32
        0, // on ";", error
        -10, // on "alias", reduce `(<ResourceAttr> ";")+ = (<ResourceAttr> ";")+, ResourceAttr, ";" => ActionFn(35);`
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        -10, // on "has", reduce `(<ResourceAttr> ";")+ = (<ResourceAttr> ";")+, ResourceAttr, ";" => ActionFn(35);`
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        -10, // on "}", reduce `(<ResourceAttr> ";")+ = (<ResourceAttr> ";")+, ResourceAttr, ";" => ActionFn(35);`
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 33
        0, // on ";", error
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        36, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 35
        // State 34
        -24, // on ";", reduce `Relation = "has", RelVariance, Ident => ActionFn(14);`
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 35
        -12, // on ";", reduce `Ident = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 36
        -11, // on ";", reduce `Alias = "alias", Method, "as", Ident => ActionFn(10);`
        0, // on "alias", error
        0, // on "append", error
        0, // on "as", error
        0, // on "clear", error
        0, // on "delete", error
        0, // on "get", error
        0, // on "has", error
        0, // on "index", error
        0, // on "many", error
        0, // on "one", error
        0, // on "patch", error
        0, // on "post", error
        0, // on "remove", error
        0, // on "replace", error
        0, // on "resource", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -31, // on EOF, reduce `routes =  => ActionFn(32);`
        -32, // on EOF, reduce `routes = (<Resource>)+ => ActionFn(33);`
        -4, // on EOF, reduce `(<Resource>)+ = Resource => ActionFn(30);`
        -30, // on EOF, reduce `__routes = routes => ActionFn(0);`
        0, // on EOF, error
        -5, // on EOF, reduce `(<Resource>)+ = (<Resource>)+, Resource => ActionFn(31);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -25, // on EOF, reduce `Resource = "resource", Ident, "{", ResourceAttrs, "}" => ActionFn(18);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        2, // on (<Resource>)+, goto 1
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        3, // on Resource, goto 2
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        4, // on routes, goto 3
        // State 1
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        6, // on Resource, goto 5
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 2
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 3
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 4
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        7, // on Ident, goto 6
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 5
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 6
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 7
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 8
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        10, // on (<ResourceAttr> ";")+, goto 9
        11, // on Alias, goto 10
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        12, // on Relation, goto 11
        0, // on Resource, error
        13, // on ResourceAttr, goto 12
        14, // on ResourceAttrs, goto 13
        0, // on __routes, error
        0, // on routes, error
        // State 9
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        11, // on Alias, goto 10
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        12, // on Relation, goto 11
        0, // on Resource, error
        17, // on ResourceAttr, goto 16
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 10
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 11
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 12
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 13
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 14
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        20, // on Method, goto 19
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 15
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        30, // on RelVariance, goto 29
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 16
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 17
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 18
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 19
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 20
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 21
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 22
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 23
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 24
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 25
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 26
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 27
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 28
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 29
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        35, // on Ident, goto 34
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 30
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 31
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 32
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 33
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        37, // on Ident, goto 36
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 34
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 35
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
        // State 36
        0, // on (<Resource>), error
        0, // on (<Resource>)*, error
        0, // on (<Resource>)+, error
        0, // on (<ResourceAttr> ";"), error
        0, // on (<ResourceAttr> ";")*, error
        0, // on (<ResourceAttr> ";")+, error
        0, // on Alias, error
        0, // on Ident, error
        0, // on Method, error
        0, // on RelVariance, error
        0, // on Relation, error
        0, // on Resource, error
        0, // on ResourceAttr, error
        0, // on ResourceAttrs, error
        0, // on __routes, error
        0, // on routes, error
    ];
    pub fn parse_routes<
        'input,
    >(
        input: &'input str,
    ) -> Result<Routes, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                (_, (12, _), _) if true => 12,
                (_, (13, _), _) if true => 13,
                (_, (14, _), _) if true => 14,
                (_, (15, _), _) if true => 15,
                (_, (16, _), _) if true => 16,
                (_, (17, _), _) if true => 17,
                (_, (18, _), _) if true => 18,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22alias_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22append_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22as_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22clear_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22delete_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22get_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22has_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22index_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22many_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22one_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22patch_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22post_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22remove_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22replace_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22resource_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Routes,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Resource>) = Resource => ActionFn(22);
                let __sym0 = __pop_NtResource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cResource_3e_29(__nt), __end));
                0
            }
            2 => {
                // (<Resource>)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cResource_3e_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Resource>)* = (<Resource>)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_3cResource_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cResource_3e_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Resource>)+ = Resource => ActionFn(30);
                let __sym0 = __pop_NtResource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cResource_3e_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Resource>)+ = (<Resource>)+, Resource => ActionFn(31);
                let __sym1 = __pop_NtResource(__symbols);
                let __sym0 = __pop_Nt_28_3cResource_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cResource_3e_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<ResourceAttr> ";") = ResourceAttr, ";" => ActionFn(25);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtResourceAttr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29(__nt), __end));
                3
            }
            7 => {
                // (<ResourceAttr> ";")* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<ResourceAttr> ";")* = (<ResourceAttr> ";")+ => ActionFn(24);
                let __sym0 = __pop_Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<ResourceAttr> ";")+ = ResourceAttr, ";" => ActionFn(34);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtResourceAttr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<ResourceAttr> ";")+ = (<ResourceAttr> ";")+, ResourceAttr, ";" => ActionFn(35);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtResourceAttr(__symbols);
                let __sym0 = __pop_Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Alias = "alias", Method, "as", Ident => ActionFn(10);
                let __sym3 = __pop_NtIdent(__symbols);
                let __sym2 = __pop_Term_22as_22(__symbols);
                let __sym1 = __pop_NtMethod(__symbols);
                let __sym0 = __pop_Term_22alias_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtAlias(__nt), __end));
                6
            }
            12 => {
                // Ident = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                7
            }
            13 => {
                // Method = "get" => ActionFn(1);
                let __sym0 = __pop_Term_22get_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            14 => {
                // Method = "patch" => ActionFn(2);
                let __sym0 = __pop_Term_22patch_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            15 => {
                // Method = "post" => ActionFn(3);
                let __sym0 = __pop_Term_22post_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            16 => {
                // Method = "delete" => ActionFn(4);
                let __sym0 = __pop_Term_22delete_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            17 => {
                // Method = "index" => ActionFn(5);
                let __sym0 = __pop_Term_22index_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            18 => {
                // Method = "append" => ActionFn(6);
                let __sym0 = __pop_Term_22append_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            19 => {
                // Method = "replace" => ActionFn(7);
                let __sym0 = __pop_Term_22replace_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            20 => {
                // Method = "remove" => ActionFn(8);
                let __sym0 = __pop_Term_22remove_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            21 => {
                // Method = "clear" => ActionFn(9);
                let __sym0 = __pop_Term_22clear_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                8
            }
            22 => {
                // RelVariance = "one" => ActionFn(12);
                let __sym0 = __pop_Term_22one_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRelVariance(__nt), __end));
                9
            }
            23 => {
                // RelVariance = "many" => ActionFn(13);
                let __sym0 = __pop_Term_22many_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRelVariance(__nt), __end));
                9
            }
            24 => {
                // Relation = "has", RelVariance, Ident => ActionFn(14);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_NtRelVariance(__symbols);
                let __sym0 = __pop_Term_22has_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                10
            }
            25 => {
                // Resource = "resource", Ident, "{", ResourceAttrs, "}" => ActionFn(18);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_NtResourceAttrs(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22resource_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                11
            }
            26 => {
                // ResourceAttr = Alias => ActionFn(15);
                let __sym0 = __pop_NtAlias(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResourceAttr(__nt), __end));
                12
            }
            27 => {
                // ResourceAttr = Relation => ActionFn(16);
                let __sym0 = __pop_NtRelation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResourceAttr(__nt), __end));
                12
            }
            28 => {
                // ResourceAttrs =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtResourceAttrs(__nt), __end));
                13
            }
            29 => {
                // ResourceAttrs = (<ResourceAttr> ";")+ => ActionFn(37);
                let __sym0 = __pop_Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResourceAttrs(__nt), __end));
                13
            }
            30 => {
                // __routes = routes => ActionFn(0);
                let __sym0 = __pop_Ntroutes(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            31 => {
                // routes =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntroutes(__nt), __end));
                15
            }
            32 => {
                // routes = (<Resource>)+ => ActionFn(33);
                let __sym0 = __pop_Nt_28_3cResource_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntroutes(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22alias_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22alias_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22append_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22append_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22as_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22as_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22clear_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22clear_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22delete_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22delete_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22get_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22get_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22has_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22has_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22index_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22index_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22many_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22many_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22one_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22one_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22patch_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22patch_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22post_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22post_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22remove_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22remove_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22replace_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22replace_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22resource_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22resource_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cResource_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Resource, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cResource_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cResource_3e_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Resource>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cResource_3e_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cResource_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Resource>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cResource_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cResourceAttr_3e_20_22_3b_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ResourceAttr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ResourceAttr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ResourceAttr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cResourceAttr_3e_20_22_3b_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAlias<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Alias, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAlias(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMethod<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Method, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethod(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRelVariance<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RelVariance, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRelVariance(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRelation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Relation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRelation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResource<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Resource, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResource(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResourceAttr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ResourceAttr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResourceAttr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResourceAttrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ResourceAttr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResourceAttrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____routes<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Routes, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____routes(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntroutes<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Routes, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntroutes(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__routes::parse_routes;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        59 => /* ';' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        101 ... 102 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        106 ... 108 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        113 => /* 'q' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        109 ... 111 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        113 ... 114 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        98 ... 110 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        110 ... 111 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        113 ... 114 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 44;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 120 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        122 => /* 'z' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 50;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 51;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 52;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 53;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 54;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 55;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 56;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 58;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 119 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 59;
                            continue;
                        }
                        121 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 103 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 60;
                            continue;
                        }
                        105 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 117 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        118 => /* 'v' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 61;
                            continue;
                        }
                        119 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 62;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 63;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 64;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 65;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                59 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                60 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                61 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 66;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                62 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 67;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                63 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 68;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                64 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                65 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                66 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                67 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 69;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                68 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 70;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                69 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                70 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 71;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                71 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((18, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Routes, usize),
) -> Routes
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Get
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Patch
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Post
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Delete
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Index
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Append
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Replace
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Remove
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Clear
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, Method, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, String, usize),
) -> Alias
{
    Alias {
        method: m,
        route: s.to_owned(),
    }
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    s.to_owned()
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> RelVariance
{
    RelVariance::One
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> RelVariance
{
    RelVariance::Many
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, RelVariance, usize),
    (_, s, _): (usize, String, usize),
) -> Relation
{
    Relation {
        variance: v,
        relation: s,
    }
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, Alias, usize),
) -> ResourceAttr
{
    ResourceAttr::Alias(a)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, r, _): (usize, Relation, usize),
) -> ResourceAttr
{
    ResourceAttr::Relation(r)
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, attrs, _): (usize, ::std::vec::Vec<ResourceAttr>, usize),
) -> Vec<ResourceAttr>
{
    attrs
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<ResourceAttr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Resource
{
    Resource { resource: r, attrs: a }
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, resources, _): (usize, ::std::vec::Vec<Resource>, usize),
) -> Routes
{
    Routes { resources: resources }
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Resource>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Resource>, usize),
) -> ::std::vec::Vec<Resource>
{
    v
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Resource, usize),
) -> Resource
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ResourceAttr>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ResourceAttr>, usize),
) -> ::std::vec::Vec<ResourceAttr>
{
    v
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ResourceAttr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ResourceAttr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ResourceAttr, usize),
) -> ::std::vec::Vec<ResourceAttr>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ResourceAttr>, usize),
    (_, e, _): (usize, ResourceAttr, usize),
) -> ::std::vec::Vec<ResourceAttr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Resource, usize),
) -> ::std::vec::Vec<Resource>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Resource>, usize),
    (_, e, _): (usize, Resource, usize),
) -> ::std::vec::Vec<Resource>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, Resource, usize),
) -> ::std::vec::Vec<Resource>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action22(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Resource>, usize),
    __1: (usize, Resource, usize),
) -> ::std::vec::Vec<Resource>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Routes
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Resource>, usize),
) -> Routes
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action21(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    __0: (usize, ResourceAttr, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<ResourceAttr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action25(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ResourceAttr>, usize),
    __1: (usize, ResourceAttr, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<ResourceAttr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action25(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ResourceAttr>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ResourceAttr>, usize),
) -> Vec<ResourceAttr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
