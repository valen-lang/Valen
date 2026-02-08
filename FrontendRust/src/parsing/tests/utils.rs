use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::generated_tests::test_parse_utils::{compile_file, compile};
use crate::parsing::tests::traverse::NodeRefP;
use crate::cast;

/// Returns the function with the given name.
/// See test_find_func_named_returns_function for an example.
pub fn find_func_named<'a>(file: &'a FileP, name: &str) -> &'a FunctionP {
    crate::collect_only!(
        file,
        NodeRefP::Function(function @ FunctionP {
            header: FunctionHeaderP {
                name: Some(NameP { str: ref s, .. }),
                ..
            },
            ..
        }) if s.str == name => Some(function)
    )
}
#[test]
fn test_find_func_named_returns_function() {
    let program = compile("exported func main() int {}");
    let main_function = find_func_named(&program, "main");
    assert!(main_function.header.params.as_ref().unwrap().params.is_empty());
}

/// Returns the struct with the given name. See find_func_named's test for a similar example.
pub fn find_struct_named<'a>(file: &'a FileP, name: &str) -> &'a StructP {
    crate::collect_only!(
        file,
        NodeRefP::Struct(struct_ @ StructP {
            name: NameP { str: ref s, .. },
            ..
        }) if s.str == name => Some(struct_)
    )
}

pub fn compile_for_error(code: &str) -> ParseError {
    compile_file(code).expect_err("Should be error")
}

pub fn assert_lookup_name(expr: &IExpressionPE, expected: &str) {
    let lookup = cast!(expr, IExpressionPE::Lookup);
    assert_name(&lookup.name, expected);
    assert!(lookup.template_args.is_none());
}

pub fn assert_templex_name(expr: &ITemplexPT, expected: &str) {
    let templex_lookup = cast!(expr, ITemplexPT::NameOrRune);
    assert_eq!(templex_lookup.name.str.str, expected);
}

pub fn assert_name(name: &IImpreciseNameP, expected: &str) {
    let lookup_name = cast!(name, IImpreciseNameP::LookupName);
    assert_eq!(lookup_name.str.str, expected);
}

/// Unwraps an ESCCD-compliant enum (single field enum) and returns a reference to the thing
/// inside. Intended for use in tests only.
#[macro_export]
macro_rules! cast {
    ($value:expr, $variant:path) => {{
        match $value {
            $variant(inner) => inner,
            _ => panic!("expected {}", stringify!($variant)),
        }
    }};
}
