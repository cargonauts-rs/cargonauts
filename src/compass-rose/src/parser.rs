use ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__resources {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_23_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_40_22(&'input str),
        Term_22Get_22(&'input str),
        Term_22Index_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22relation_22(&'input str),
        Term_22resource_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(&'input str),
        Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9_5d_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_3cIdent_3e_20_22_2c_22_29(String),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<String>),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<String>),
        Nt_28_3cMethod_3e_20_22_2b_22_29(Method),
        Nt_28_3cMethod_3e_20_22_2b_22_29_2a(::std::vec::Vec<Method>),
        Nt_28_3cMethod_3e_20_22_2b_22_29_2b(::std::vec::Vec<Method>),
        NtAttribute(Attribute),
        NtAttribute_2a(::std::vec::Vec<Attribute>),
        NtAttribute_2b(::std::vec::Vec<Attribute>),
        NtEndpoint(String),
        NtEndpoint_3f(::std::option::Option<String>),
        NtIdent(String),
        NtIdent_3f(::std::option::Option<String>),
        NtMethod(Method),
        NtMethod_3f(::std::option::Option<Method>),
        NtMethods(Vec<Method>),
        NtMethods_3f(::std::option::Option<Vec<Method>>),
        NtRelation(Relation),
        NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(Vec<String>),
        NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(Vec<Method>),
        NtResource(Resource),
        NtResource_2a(::std::vec::Vec<Resource>),
        NtResource_2b(::std::vec::Vec<Resource>),
        NtResourceHeader(ResourceHeader),
        NtResourceMember(ResourceMember),
        NtResourceMember_2a(::std::vec::Vec<ResourceMember>),
        NtResourceMember_2b(::std::vec::Vec<ResourceMember>),
        NtString(String),
        Nt____resources(Vec<Resource>),
        Ntresources(Vec<Resource>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0,
        // State 1
        -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0,
        // State 2
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0,
        // State 3
        -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0,
        // State 4
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 9
        -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0,
        // State 11
        -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0,
        // State 12
        -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 23, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0,
        // State 15
        0, 0, 0, 0, 0, 28, -55, 29, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, -20, -20, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 17
        -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 31, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, -59, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 33, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0,
        // State 22
        -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0,
        // State 23
        0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 28, -53, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, -39, 0, 43, 44, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 47, 0, 0, 0,
        // State 30
        -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, -60, 0, 0, 0,
        // State 32
        -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 50, -33, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, -20, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0,
        // State 37
        -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -41, 0, 43, 44, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 58, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0,
        // State 42
        0, 0, 0, -23, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0,
        // State 43
        0, 0, 0, -24, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, -61, -61, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0,
        // State 46
        -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 50, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, -39, 0, 63, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, -56, 0, 0, 0,
        // State 52
        0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0,
        // State 53
        0, 0, -34, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 70, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, -9, 0, -9, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, -41, 0, 63, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 72, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, -23, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, -24, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, -36, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -10, 0, -10, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 75, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, -9, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 73
        -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, -10, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -63,
        0,
        0,
        -50,
        -64,
        0,
        -62,
        0,
        0,
        0,
        0,
        -51,
        -42,
        0,
        0,
        0,
        0,
        -43,
        0,
        0,
        0,
        0,
        -44,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -46,
        0,
        -45,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -47,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 5, 6, 0, 0, 0, 0, 0, 7,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 6, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 21, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 30, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""#""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###"":""###,
            r###"";""###,
            r###""@""###,
            r###""Get""###,
            r###""Index""###,
            r###""[""###,
            r###""]""###,
            r###""relation""###,
            r###""resource""###,
            r###""{""###,
            r###""}""###,
            r###"r#"\"(?:[^\"\\\\]|\\\\.)*\""#"###,
            r###"r#"[A-Za-z][A-Za-z0-9]*"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_resources<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Resource>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (17, _) if true => 17,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_23_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_40_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22Get_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22Index_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22relation_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22resource_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9_5d_2a_22_23(__tok0),
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
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
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
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
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
    ) -> Option<Result<Vec<Resource>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Ident> ",") = Ident, "," => ActionFn(42);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Ident> ",")* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Ident> ",")* = (<Ident> ",")+ => ActionFn(41);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Ident> ",")+ = Ident, "," => ActionFn(47);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action47::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Ident> ",")+ = (<Ident> ",")+, Ident, "," => ActionFn(48);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Method> "+") = Method, "+" => ActionFn(37);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Method> "+")* =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Method> "+")* = (<Method> "+")+ => ActionFn(36);
                let __sym0 = __pop_Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Method> "+")+ = Method, "+" => ActionFn(51);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Method> "+")+ = (<Method> "+")+, Method, "+" => ActionFn(52);
                let __sym2 = __pop_Term_22_2b_22(__symbols);
                let __sym1 = __pop_NtMethod(__symbols);
                let __sym0 = __pop_Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Attribute = "#", "[", Ident, "(", RepeatWith<Ident, ",">, ")", "]" => ActionFn(13);
                let __sym6 = __pop_Term_22_5d_22(__symbols);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(__symbols);
                let __sym3 = __pop_Term_22_28_22(__symbols);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_Term_22_23_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtAttribute(__nt), __end));
                6
            }
            12 => {
                // Attribute = "#", "[", Ident, "]" => ActionFn(14);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_Term_22_23_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtAttribute(__nt), __end));
                6
            }
            13 => {
                // Attribute* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAttribute_2a(__nt), __end));
                7
            }
            14 => {
                // Attribute* = Attribute+ => ActionFn(24);
                let __sym0 = __pop_NtAttribute_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAttribute_2a(__nt), __end));
                7
            }
            15 => {
                // Attribute+ = Attribute => ActionFn(29);
                let __sym0 = __pop_NtAttribute(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAttribute_2b(__nt), __end));
                8
            }
            16 => {
                // Attribute+ = Attribute+, Attribute => ActionFn(30);
                let __sym1 = __pop_NtAttribute(__symbols);
                let __sym0 = __pop_NtAttribute_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAttribute_2b(__nt), __end));
                8
            }
            17 => {
                // Endpoint = "@", String => ActionFn(5);
                let __sym1 = __pop_NtString(__symbols);
                let __sym0 = __pop_Term_22_40_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtEndpoint(__nt), __end));
                9
            }
            18 => {
                // Endpoint? = Endpoint => ActionFn(19);
                let __sym0 = __pop_NtEndpoint(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtEndpoint_3f(__nt), __end));
                10
            }
            19 => {
                // Endpoint? =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtEndpoint_3f(__nt), __end));
                10
            }
            20 => {
                // Ident = r#"[A-Za-z][A-Za-z0-9]*"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                11
            }
            21 => {
                // Ident? = Ident => ActionFn(38);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                12
            }
            22 => {
                // Ident? =  => ActionFn(39);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action39::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                12
            }
            23 => {
                // Method = "Get" => ActionFn(9);
                let __sym0 = __pop_Term_22Get_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                13
            }
            24 => {
                // Method = "Index" => ActionFn(10);
                let __sym0 = __pop_Term_22Index_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                13
            }
            25 => {
                // Method? = Method => ActionFn(33);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethod_3f(__nt), __end));
                14
            }
            26 => {
                // Method? =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMethod_3f(__nt), __end));
                14
            }
            27 => {
                // Methods = ":", RepeatWith<Method, "+"> => ActionFn(8);
                let __sym1 = __pop_NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(__symbols);
                let __sym0 = __pop_Term_22_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMethods(__nt), __end));
                15
            }
            28 => {
                // Methods? = Methods => ActionFn(17);
                let __sym0 = __pop_NtMethods(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMethods_3f(__nt), __end));
                16
            }
            29 => {
                // Methods? =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMethods_3f(__nt), __end));
                16
            }
            30 => {
                // Relation = Ident, Endpoint, Methods => ActionFn(71);
                let __sym2 = __pop_NtMethods(__symbols);
                let __sym1 = __pop_NtEndpoint(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action71::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                17
            }
            31 => {
                // Relation = Ident, Endpoint => ActionFn(72);
                let __sym1 = __pop_NtEndpoint(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action72::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                17
            }
            32 => {
                // Relation = Ident, Methods => ActionFn(73);
                let __sym1 = __pop_NtMethods(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action73::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                17
            }
            33 => {
                // Relation = Ident => ActionFn(74);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action74::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRelation(__nt), __end));
                17
            }
            34 => {
                // RepeatWith<Ident, ","> = Ident => ActionFn(63);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(__nt), __end));
                18
            }
            35 => {
                // RepeatWith<Ident, ","> =  => ActionFn(64);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action64::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(__nt), __end));
                18
            }
            36 => {
                // RepeatWith<Ident, ","> = (<Ident> ",")+, Ident => ActionFn(65);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action65::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(__nt), __end));
                18
            }
            37 => {
                // RepeatWith<Ident, ","> = (<Ident> ",")+ => ActionFn(66);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action66::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(__nt), __end));
                18
            }
            38 => {
                // RepeatWith<Method, "+"> = Method => ActionFn(67);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action67::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(__nt), __end));
                19
            }
            39 => {
                // RepeatWith<Method, "+"> =  => ActionFn(68);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action68::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(__nt), __end));
                19
            }
            40 => {
                // RepeatWith<Method, "+"> = (<Method> "+")+, Method => ActionFn(69);
                let __sym1 = __pop_NtMethod(__symbols);
                let __sym0 = __pop_Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(__nt), __end));
                19
            }
            41 => {
                // RepeatWith<Method, "+"> = (<Method> "+")+ => ActionFn(70);
                let __sym0 = __pop_Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action70::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(__nt), __end));
                19
            }
            42 => {
                // Resource = ResourceHeader, ";" => ActionFn(55);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtResourceHeader(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                20
            }
            43 => {
                // Resource = Attribute+, ResourceHeader, ";" => ActionFn(56);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtResourceHeader(__symbols);
                let __sym0 = __pop_NtAttribute_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                20
            }
            44 => {
                // Resource = ResourceHeader, "{", "}" => ActionFn(81);
                let __sym2 = __pop_Term_22_7d_22(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_NtResourceHeader(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action81::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                20
            }
            45 => {
                // Resource = ResourceHeader, "{", ResourceMember+, "}" => ActionFn(82);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_NtResourceMember_2b(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_NtResourceHeader(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action82::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                20
            }
            46 => {
                // Resource = Attribute+, ResourceHeader, "{", "}" => ActionFn(83);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtResourceHeader(__symbols);
                let __sym0 = __pop_NtAttribute_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action83::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                20
            }
            47 => {
                // Resource = Attribute+, ResourceHeader, "{", ResourceMember+, "}" => ActionFn(84);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_NtResourceMember_2b(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtResourceHeader(__symbols);
                let __sym0 = __pop_NtAttribute_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action84::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtResource(__nt), __end));
                20
            }
            48 => {
                // Resource* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtResource_2a(__nt), __end));
                21
            }
            49 => {
                // Resource* = Resource+ => ActionFn(26);
                let __sym0 = __pop_NtResource_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResource_2a(__nt), __end));
                21
            }
            50 => {
                // Resource+ = Resource => ActionFn(27);
                let __sym0 = __pop_NtResource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResource_2b(__nt), __end));
                22
            }
            51 => {
                // Resource+ = Resource+, Resource => ActionFn(28);
                let __sym1 = __pop_NtResource(__symbols);
                let __sym0 = __pop_NtResource_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtResource_2b(__nt), __end));
                22
            }
            52 => {
                // ResourceHeader = "resource", Ident, Endpoint, Methods => ActionFn(75);
                let __sym3 = __pop_NtMethods(__symbols);
                let __sym2 = __pop_NtEndpoint(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22resource_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action75::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtResourceHeader(__nt), __end));
                23
            }
            53 => {
                // ResourceHeader = "resource", Ident, Endpoint => ActionFn(76);
                let __sym2 = __pop_NtEndpoint(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22resource_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action76::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtResourceHeader(__nt), __end));
                23
            }
            54 => {
                // ResourceHeader = "resource", Ident, Methods => ActionFn(77);
                let __sym2 = __pop_NtMethods(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22resource_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action77::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtResourceHeader(__nt), __end));
                23
            }
            55 => {
                // ResourceHeader = "resource", Ident => ActionFn(78);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22resource_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action78::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtResourceHeader(__nt), __end));
                23
            }
            56 => {
                // ResourceMember = "relation", Relation, ";" => ActionFn(6);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtRelation(__symbols);
                let __sym0 = __pop_Term_22relation_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtResourceMember(__nt), __end));
                24
            }
            57 => {
                // ResourceMember* =  => ActionFn(21);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action21::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtResourceMember_2a(__nt), __end));
                25
            }
            58 => {
                // ResourceMember* = ResourceMember+ => ActionFn(22);
                let __sym0 = __pop_NtResourceMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResourceMember_2a(__nt), __end));
                25
            }
            59 => {
                // ResourceMember+ = ResourceMember => ActionFn(31);
                let __sym0 = __pop_NtResourceMember(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtResourceMember_2b(__nt), __end));
                26
            }
            60 => {
                // ResourceMember+ = ResourceMember+, ResourceMember => ActionFn(32);
                let __sym1 = __pop_NtResourceMember(__symbols);
                let __sym0 = __pop_NtResourceMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtResourceMember_2b(__nt), __end));
                26
            }
            61 => {
                // String = r#"\"(?:[^\"\\\\]|\\\\.)*\""# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtString(__nt), __end));
                27
            }
            62 => {
                // __resources = resources => ActionFn(0);
                let __sym0 = __pop_Ntresources(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            63 => {
                // resources =  => ActionFn(79);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action79::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntresources(__nt), __end));
                29
            }
            64 => {
                // resources = Resource+ => ActionFn(80);
                let __sym0 = __pop_NtResource_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action80::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntresources(__nt), __end));
                29
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 30 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_23_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_23_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_40_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_40_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Get_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Get_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Index_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Index_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22relation_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22relation_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_28_3f_3a_5b_5e_5c_22_5c_5c_5c_5c_5d_7c_5c_5c_5c_5c_2e_29_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZa_2dz_5d_5bA_2dZa_2dz0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cMethod_3e_20_22_2b_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Method, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cMethod_3e_20_22_2b_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Method>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cMethod_3e_20_22_2b_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Method>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cMethod_3e_20_22_2b_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAttribute<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Attribute, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAttribute(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAttribute_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Attribute>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAttribute_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAttribute_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Attribute>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAttribute_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtEndpoint<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtEndpoint(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtEndpoint_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtEndpoint_3f(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent_3f(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtMethod_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Method>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethod_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMethods<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Method>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethods(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMethods_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Method>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethods_3f(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtRepeatWith_3cIdent_2c_20_22_2c_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRepeatWith_3cIdent_2c_20_22_2c_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRepeatWith_3cMethod_2c_20_22_2b_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Method>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRepeatWith_3cMethod_2c_20_22_2b_22_3e(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtResource_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Resource>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResource_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResource_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Resource>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResource_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResourceHeader<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ResourceHeader, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResourceHeader(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResourceMember<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ResourceMember, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResourceMember(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResourceMember_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ResourceMember>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResourceMember_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtResourceMember_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ResourceMember>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtResourceMember_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtString<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtString(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____resources<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Resource>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____resources(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntresources<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Resource>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntresources(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__resources::parse_resources;
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
                        34 => /* '\"' */ {
                            __current_state = 1;
                            continue;
                        }
                        35 => /* '#' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        64 => /* '@' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        65 ... 70 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        71 => /* 'G' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        73 => /* 'I' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        74 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 17;
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
                        0 ... 33 => {
                            __current_state = 19;
                            continue;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        35 ... 91 => {
                            __current_state = 19;
                            continue;
                        }
                        92 => /* '\\' */ {
                            __current_state = 21;
                            continue;
                        }
                        93 ... 1114111 => {
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        0 ... 33 => {
                            __current_state = 19;
                            continue;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        35 ... 91 => {
                            __current_state = 19;
                            continue;
                        }
                        92 => /* '\\' */ {
                            __current_state = 21;
                            continue;
                        }
                        93 ... 1114111 => {
                            __current_state = 19;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        _ => {
                            __current_state = 19;
                            continue;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        109 ... 114 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 119 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        121 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 22;
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
    (_, __0, _): (usize, Vec<Resource>, usize),
) -> Vec<Resource>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Resource>, usize),
) -> Vec<Resource>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, attrs, _): (usize, ::std::vec::Vec<Attribute>, usize),
    (_, header, _): (usize, ResourceHeader, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Resource
{
    Resource {
        attrs, header, members: vec![],
    }
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, attrs, _): (usize, ::std::vec::Vec<Attribute>, usize),
    (_, header, _): (usize, ResourceHeader, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, members, _): (usize, ::std::vec::Vec<ResourceMember>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Resource
{
    Resource {
        attrs, header, members,
    }
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, String, usize),
    (_, endpoint, _): (usize, ::std::option::Option<String>, usize),
    (_, methods, _): (usize, ::std::option::Option<Vec<Method>>, usize),
) -> ResourceHeader
{
    ResourceHeader { ty, endpoint, methods }
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, rel, _): (usize, Relation, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ResourceMember
{
    ResourceMember::Relation(rel)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, rel, _): (usize, String, usize),
    (_, endpoint, _): (usize, ::std::option::Option<String>, usize),
    (_, methods, _): (usize, ::std::option::Option<Vec<Method>>, usize),
) -> Relation
{
    Relation { rel, methods, endpoint }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Method>, usize),
) -> Vec<Method>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Get
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Method
{
    Method::Index
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, idents, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Attribute
{
    Attribute::Arg(id, idents)
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Attribute
{
    Attribute::Ident(__0)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    match e {
       None => v,
       Some(e) => {
           let mut v = v;
           v.push(e);
           v
       }
   }
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Method>, usize),
    (_, e, _): (usize, ::std::option::Option<Method>, usize),
) -> Vec<Method>
{
    match e {
       None => v,
       Some(e) => {
           let mut v = v;
           v.push(e);
           v
       }
   }
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Method>, usize),
) -> ::std::option::Option<Vec<Method>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<Method>>
{
    None
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ResourceMember>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ResourceMember>, usize),
) -> ::std::vec::Vec<ResourceMember>
{
    v
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Attribute>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Attribute>, usize),
) -> ::std::vec::Vec<Attribute>
{
    v
}

#[allow(unused_variables)]
pub fn __action25<
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
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Resource>, usize),
) -> ::std::vec::Vec<Resource>
{
    v
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Resource, usize),
) -> ::std::vec::Vec<Resource>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action28<
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
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Attribute, usize),
) -> ::std::vec::Vec<Attribute>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Attribute>, usize),
    (_, e, _): (usize, Attribute, usize),
) -> ::std::vec::Vec<Attribute>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ResourceMember, usize),
) -> ::std::vec::Vec<ResourceMember>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ResourceMember>, usize),
    (_, e, _): (usize, ResourceMember, usize),
) -> ::std::vec::Vec<ResourceMember>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Method, usize),
) -> ::std::option::Option<Method>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Method>
{
    None
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Method>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Method>, usize),
) -> ::std::vec::Vec<Method>
{
    v
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Method, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Method
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
) -> ::std::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Method, usize),
) -> ::std::vec::Vec<Method>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Method>, usize),
    (_, e, _): (usize, Method, usize),
) -> ::std::vec::Vec<Method>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action42(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action40(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action41(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, Method, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Method>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action37(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Method>, usize),
    __1: (usize, Method, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Method>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action37(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Method>, usize),
) -> Vec<Method>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Method>, usize),
    __1: (usize, ::std::option::Option<Method>, usize),
) -> Vec<Method>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, ResourceHeader, usize),
    __1: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Attribute>, usize),
    __1: (usize, ResourceHeader, usize),
    __2: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, ResourceHeader, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<ResourceMember>, usize),
    __3: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Attribute>, usize),
    __1: (usize, ResourceHeader, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<ResourceMember>, usize),
    __4: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action59<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, String, usize),
    __2: (usize, ::std::option::Option<Vec<Method>>, usize),
) -> Relation
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action19(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, ::std::option::Option<Vec<Method>>, usize),
) -> Relation
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, String, usize),
    __3: (usize, ::std::option::Option<Vec<Method>>, usize),
) -> ResourceHeader
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action19(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, ::std::option::Option<Vec<Method>>, usize),
) -> ResourceHeader
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action38(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action64<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action39(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action66<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, Method, usize),
) -> Vec<Method>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action33(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action68<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Method>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action34(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Method>, usize),
    __1: (usize, Method, usize),
) -> Vec<Method>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Method>, usize),
) -> Vec<Method>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action71<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, String, usize),
    __2: (usize, Vec<Method>, usize),
) -> Relation
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action17(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action72<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, String, usize),
) -> Relation
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action73<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, Vec<Method>, usize),
) -> Relation
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action17(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action74<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Relation
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action75<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, String, usize),
    __3: (usize, Vec<Method>, usize),
) -> ResourceHeader
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action17(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action76<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, String, usize),
) -> ResourceHeader
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action77<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, Vec<Method>, usize),
) -> ResourceHeader
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action17(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action78<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> ResourceHeader
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action79<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Resource>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action25(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Resource>, usize),
) -> Vec<Resource>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action81<
    'input,
>(
    input: &'input str,
    __0: (usize, ResourceHeader, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action21(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action82<
    'input,
>(
    input: &'input str,
    __0: (usize, ResourceHeader, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<ResourceMember>, usize),
    __3: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action83<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Attribute>, usize),
    __1: (usize, ResourceHeader, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action21(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action84<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Attribute>, usize),
    __1: (usize, ResourceHeader, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<ResourceMember>, usize),
    __4: (usize, &'input str, usize),
) -> Resource
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action22(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __4,
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
