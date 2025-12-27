/// Impl parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/ImplTests.scala

use crate::tests::test_utils::*;
use crate::parsing::ast::*;
use crate::{should_have, matches_pattern};

// Mirrors ImplTests.scala line 11
#[test]
fn test_normal_impl() {
    let file = compile_file_expect(
        r#"
        impl MyInterface for SomeStruct;
        "#
    );
    
    assert_eq!(file.denizens.len(), 1);
    should_have!(file.denizens[0], IDenizenP::TopLevelImpl(ImplP {
        generic_params: None,
        template_rules: None,
        struct_: Some(ITemplexPT::NameOrRune(NameP { str: ref struct_str, .. })),
        interface: ITemplexPT::NameOrRune(NameP { str: ref interface_str, .. }),
        ..
    }) if struct_str.str == "SomeStruct" && interface_str.str == "MyInterface" => {});
}

// Mirrors ImplTests.scala line 26
#[test]
fn test_templated_impl() {
    let file = compile_file_expect(
        r#"
        impl<T> MyInterface<T> for SomeStruct<T>;
        "#
    );
    
    assert_eq!(file.denizens.len(), 1);
    should_have!(file.denizens[0], IDenizenP::TopLevelImpl(ImplP {
        generic_params: Some(GenericParametersP { params: ref params, .. }),
        template_rules: None,
        struct_: Some(ITemplexPT::Call { .. }),
        interface: ITemplexPT::Call { .. },
        ..
    }) if params.len() == 1 => {});
}

// Mirrors ImplTests.scala line 41
#[test]
fn test_impling_a_template_call() {
    let file = compile_file_expect(
        r#"
        impl IFunction1<mut, int, int> for MyIntIdentity;
        "#
    );
    
    assert_eq!(file.denizens.len(), 1);
    should_have!(file.denizens[0], IDenizenP::TopLevelImpl(ImplP {
        generic_params: None,
        template_rules: None,
        struct_: Some(ITemplexPT::NameOrRune(NameP { str: ref struct_str, .. })),
        interface: ITemplexPT::Call { args: ref args, .. },
        ..
    }) if struct_str.str == "MyIntIdentity" && args.len() == 3 => {});
}

