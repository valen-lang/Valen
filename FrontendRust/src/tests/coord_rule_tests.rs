/// Coordinate rule parsing tests
/// Mirrors tests from Frontend/ParsingPass/test/dev/vale/parsing/rules/CoordRuleTests.scala

use crate::tests::test_parse_utils::*;
use crate::parsing::ast::*;

// Mirrors CoordRuleTests.scala line 16
#[test]
fn test_empty_coord_rule() {
    let result = compile_rulex_expect("_ Ref");
    assert!(matches!(result, IRulexPR::Typed { rune: None, tyype: ITypePR::CoordType, .. }));
}

// Mirrors CoordRuleTests.scala line 22
#[test]
fn test_coord_with_rune() {
    let result = compile_rulex_expect("T Ref");
    assert!(matches!(result, IRulexPR::Typed {
        rune: Some(NameP { str: ref t_str, .. }),
        tyype: ITypePR::CoordType,
        ..
    } if t_str.str == "T"));
}

// Mirrors CoordRuleTests.scala line 28
#[test]
fn test_coord_with_destructure_only() {
    let result = compile_rulex_expect("Ref[_, _, _]");
    if let IRulexPR::Components {
        container: ITypePR::CoordType,
        components: ref comps,
        ..
    } = result {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
    } else {
        panic!("Expected Components with CoordType");
    }
}

// Mirrors CoordRuleTests.scala line 34
#[test]
fn test_coord_with_rune_and_destructure() {
    let result1 = compile_rulex_expect("T = Ref[_, _, _]");
    if let IRulexPR::Equals {
        right: box IRulexPR::Components {
            container: ITypePR::CoordType,
            components: ref comps,
            ..
        },
        ..
    } = result1 {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
    } else {
        panic!("Expected Components with CoordType {:?}", result1);
    }
    
    let result2 = compile_rulex_expect("T = Ref[own, _, _]");
    if let IRulexPR::Equals {
        right: box IRulexPR::Components {
            container: ITypePR::CoordType,
            components: ref comps,
            ..
        },
        ..
    } = result2 {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::Ownership { ownership: OwnershipP::Own, .. })));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
    } else {
        panic!("Expected Components with CoordType {:?}", result2);
    }
}

// Mirrors CoordRuleTests.scala line 45
#[test]
fn test_coord_matches_plain_int() {
    let result = compile_rulex_expect("int");
    assert!(matches!(result, IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "int"));
}

// Mirrors CoordRuleTests.scala line 62
#[test]
fn test_coord_with_int_in_kind_rule() {
    let result = compile_rulex_expect("Ref[_, _, int]");
    if let IRulexPR::Components {
        container: ITypePR::CoordType,
        components: ref comps,
        ..
    } = result {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "int"));
    } else {
        panic!("Expected Components with CoordType");
    }
}

// Mirrors CoordRuleTests.scala line 72
#[test]
fn test_coord_with_specific_kind_rule() {
    let result = compile_rulex_expect("Ref[_, _, Kind[mut]]");
    if let IRulexPR::Components {
        container: ITypePR::CoordType,
        components: ref comps,
        ..
    } = result {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        
        // Check the Kind destructure
        if let IRulexPR::Components {
            container: ITypePR::KindType,
            components: ref kind_comps,
            ..
        } = &comps[2] {
            assert_eq!(kind_comps.len(), 1);
            assert!(matches!(&kind_comps[0], IRulexPR::Templex(ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. })));
        } else {
            panic!("Expected Components with KindType");
        }
    } else {
        panic!("Expected Components with CoordType");
    }
}

// Mirrors CoordRuleTests.scala line 84
#[test]
fn test_coord_with_value() {
    let result = compile_rulex_expect("T Ref = int");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed {
            rune: Some(NameP { str: ref t_str, .. }),
            tyype: ITypePR::CoordType,
            ..
        },
        right: box IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref int_str, .. })),
        ..
    } = result {
        assert_eq!(t_str.str, "T");
        assert_eq!(int_str.str, "int");
    } else {
        panic!("Expected Equals with Typed and Templex");
    }
}

// Mirrors CoordRuleTests.scala line 92
#[test]
fn test_coord_with_destructure_and_value() {
    let result = compile_rulex_expect("Ref[_, _, _] = int");
    if let IRulexPR::Equals {
        left: box IRulexPR::Components {
            container: ITypePR::CoordType,
            components: ref comps,
            ..
        },
        right: box IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref int_str, .. })),
        ..
    } = result {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert_eq!(int_str.str, "int");
    } else {
        panic!("Expected Equals with Components and Templex");
    }
}

// Mirrors CoordRuleTests.scala line 101
#[test]
fn test_coord_with_sequence_in_value_spot() {
    let result = compile_rulex_expect("T Ref = (int, bool)");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed {
            rune: Some(NameP { str: ref t_str, .. }),
            tyype: ITypePR::CoordType,
            ..
        },
        right: box IRulexPR::Templex(ITemplexPT::Tuple { elements: ref elems, .. }),
        ..
    } = result {
        assert_eq!(t_str.str, "T");
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::NameOrRune(NameP { str: ref s, .. }) if s.str == "int"));
        assert!(matches!(&elems[1], ITemplexPT::NameOrRune(NameP { str: ref s, .. }) if s.str == "bool"));
    } else {
        panic!("Expected Equals with Typed and Tuple");
    }
}

// Mirrors CoordRuleTests.scala line 111
#[test]
fn test_lone_tuple_is_sequence() {
    let result = compile_rulex_expect("(int, bool)");
    if let IRulexPR::Templex(ITemplexPT::Tuple { elements: ref elems, .. }) = result {
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::NameOrRune(NameP { str: ref s, .. }) if s.str == "int"));
        assert!(matches!(&elems[1], ITemplexPT::NameOrRune(NameP { str: ref s, .. }) if s.str == "bool"));
    } else {
        panic!("Expected Templex with Tuple");
    }
}

