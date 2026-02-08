/// Pattern parsing tests
/// Mirrors tests from Frontend/ParsingPass/test/dev/vale/parsing/patterns/

use crate::parsing::tests::test_parse_utils::*;
use crate::parsing::ast::*;

// === CaptureAndDestructureTests.scala ===

// Mirrors CaptureAndDestructureTests.scala line 16
#[test]
fn test_capture_with_destructure_with_type_inside() {
    let result = compile_pattern_expect("a [a int, b bool]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a2_str, .. }),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
            ..
        } if a2_str.str == "a" && int_str.str == "int"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref bool_str, .. } })),
            ..
        } if b_str.str == "b" && bool_str.str == "bool"));
    } else {
        panic!("Expected pattern with capture and typed destructure");
    }
}

// Mirrors CaptureAndDestructureTests.scala line 28
#[test]
fn test_capture_with_empty_sequence_type() {
    let result = compile_pattern_expect("a ()");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            ..
        }),
        templex: Some(ITemplexPT::Tuple(TuplePT { elements: ref elems, .. })),
        destructure: None,
        ..
    } if a_str.str == "a" && elems.is_empty()));
}

// Mirrors CaptureAndDestructureTests.scala line 33
#[test]
fn test_empty_destructure() {
    let result = compile_pattern_expect("[]");
    
    assert!(matches!(result, PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } if patterns.is_empty()));
}

// Mirrors CaptureAndDestructureTests.scala line 37
// Needs the space between the braces, see https://github.com/ValeLang/Vale/issues/434
#[test]
fn test_capture_with_empty_destructure() {
    let result = compile_pattern_expect("a [ ]");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } if a_str.str == "a" && patterns.is_empty()));
}

// Mirrors CaptureAndDestructureTests.scala line 43
#[test]
fn test_destructure_with_nested_atom() {
    let result = compile_pattern_expect("a [b int]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
            ..
        } if b_str.str == "b" && int_str.str == "int"));
    } else {
        panic!("Expected pattern with capture and typed destructure");
    }
}

// === CaptureAndTypeTests.scala ===

// Mirrors CaptureAndTypeTests.scala line 30
#[test]
fn test_no_capture_with_type() {
    let result = compile_pattern_expect("_ int");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
        destructure: None,
        ..
    } if int_str.str == "int"));
}

// Mirrors CaptureAndTypeTests.scala line 35
#[test]
fn test_capture_with_type() {
    let result = compile_pattern_expect("a int");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
        destructure: None,
        ..
    } if a_str.str == "a" && int_str.str == "int"));
}

// Mirrors CaptureAndTypeTests.scala line 40
#[test]
fn test_simple_capture_with_tame() {
    let result = compile_pattern_expect("a T");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref t_str, .. } })),
        destructure: None,
        ..
    } if a_str.str == "a" && t_str.str == "T"));
}

// Mirrors CaptureAndTypeTests.scala line 45
#[test]
fn test_capture_with_borrow_tame() {
    let result = compile_pattern_expect("arr &R");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref arr_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Borrow, .. }),
            maybe_region: None,
            inner: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref r_str, .. } }),
            ..
        })),
        destructure: None,
        ..
    } if arr_str.str == "arr" && r_str.str == "R"));
}

// Mirrors CaptureAndTypeTests.scala line 53
#[test]
fn test_capture_with_self_in_front() {
    let result = compile_pattern_expect("self.arr &&R");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::ConstructingMemberNameDeclaration(NameP { str: ref arr_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Weak, .. }),
            maybe_region: None,
            inner: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref r_str, .. } }),
            ..
        })),
        destructure: None,
        ..
    } if arr_str.str == "arr" && r_str.str == "R"));
}

// === DestructureParserTests.scala ===

// Mirrors DestructureParserTests.scala line 16
#[test]
fn test_only_empty_destructure() {
    let result = compile_pattern_expect("[]");
    
    assert!(matches!(result, PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } if patterns.is_empty()));
}

// Mirrors DestructureParserTests.scala line 21
#[test]
fn test_one_element_destructure() {
    let result = compile_pattern_expect("[a]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            ..
        } if a_str.str == "a"));
    } else {
        panic!("Expected destructure pattern");
    }
}

// Mirrors DestructureParserTests.scala line 26
#[test]
fn test_one_typed_element_destructure() {
    let result = compile_pattern_expect("[ _ A ]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref a_str, .. } })),
            ..
        } if a_str.str == "A"));
    } else {
        panic!("Expected destructure pattern");
    }
}

// Mirrors DestructureParserTests.scala line 31
#[test]
fn test_only_two_element_destructure() {
    let result = compile_pattern_expect("[a, b]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            ..
        } if a_str.str == "a"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected destructure pattern");
    }
}

// Mirrors DestructureParserTests.scala line 36
#[test]
fn test_two_element_destructure_with_ignore() {
    let result = compile_pattern_expect("[_, b]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
                mutate: None,
            }),
            templex: None,
            destructure: None,
            ..
        }));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected destructure pattern");
    }
}

// Mirrors DestructureParserTests.scala line 43
#[test]
fn test_capture_with_destructure() {
    let result = compile_pattern_expect("a [x, y]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x_str, .. }),
                ..
            }),
            ..
        } if x_str.str == "x"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref y_str, .. }),
                ..
            }),
            ..
        } if y_str.str == "y"));
    } else {
        panic!("Expected pattern with capture and destructure");
    }
}

// Mirrors DestructureParserTests.scala line 51
#[test]
fn test_type_with_destructure() {
    let result = compile_pattern_expect("A[a, b]");
    
    if let PatternPP {
        destination: None,
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref a_type_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_type_str.str, "A");
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            ..
        } if a_str.str == "a"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected pattern with type and destructure");
    }
}

// Mirrors DestructureParserTests.scala line 59
#[test]
fn test_capture_and_type_with_destructure() {
    let result = compile_pattern_expect("a A[x, y]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref a_type_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(a_type_str.str, "A");
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x_str, .. }),
                ..
            }),
            ..
        } if x_str.str == "x"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref y_str, .. }),
                ..
            }),
            ..
        } if y_str.str == "y"));
    } else {
        panic!("Expected pattern with capture, type, and destructure");
    }
}

// Mirrors DestructureParserTests.scala line 67
#[test]
fn test_capture_with_types_inside() {
    let result = compile_pattern_expect("a [_ int, _ bool]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
            ..
        } if int_str.str == "int"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref bool_str, .. } })),
            ..
        } if bool_str.str == "bool"));
    } else {
        panic!("Expected pattern with capture and typed destructure");
    }
}

// Mirrors DestructureParserTests.scala line 75
#[test]
fn test_destructure_with_type_inside() {
    let result = compile_pattern_expect("[a int, b bool]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
            ..
        } if a_str.str == "a" && int_str.str == "int"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref bool_str, .. } })),
            ..
        } if b_str.str == "b" && bool_str.str == "bool"));
    } else {
        panic!("Expected destructure with typed elements");
    }
}

// Mirrors DestructureParserTests.scala line 83
#[test]
fn test_nested_destructures_a() {
    let result = compile_pattern_expect("[a, [b, c]]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            ..
        } if a_str.str == "a"));
        
        if let PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref inner_patterns, .. }),
            ..
        } = &patterns[1] {
            assert_eq!(inner_patterns.len(), 2);
            assert!(matches!(inner_patterns[0], PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                    ..
                }),
                ..
            } if b_str.str == "b"));
            assert!(matches!(inner_patterns[1], PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref c_str, .. }),
                    ..
                }),
                ..
            } if c_str.str == "c"));
        } else {
            panic!("Expected nested destructure");
        }
    } else {
        panic!("Expected outer destructure");
    }
}

// Mirrors DestructureParserTests.scala line 94
#[test]
fn test_nested_destructures_b() {
    let result = compile_pattern_expect("[[a], b]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(patterns.len(), 2);
        
        if let PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref inner_patterns, .. }),
            ..
        } = &patterns[0] {
            assert_eq!(inner_patterns.len(), 1);
            assert!(matches!(inner_patterns[0], PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                ..
            } if a_str.str == "a"));
        } else {
            panic!("Expected nested destructure");
        }
        
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected outer destructure");
    }
}

// Mirrors DestructureParserTests.scala line 104
#[test]
fn test_nested_destructures_c() {
    let result = compile_pattern_expect("[[[a]]]");
    
    if let PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP { patterns: ref patterns1, .. }),
        ..
    } = result {
        assert_eq!(patterns1.len(), 1);
        
        if let PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref patterns2, .. }),
            ..
        } = &patterns1[0] {
            assert_eq!(patterns2.len(), 1);
            
            if let PatternPP {
                destination: None,
                templex: None,
                destructure: Some(DestructureP { patterns: ref patterns3, .. }),
                ..
            } = &patterns2[0] {
                assert_eq!(patterns3.len(), 1);
                assert!(matches!(patterns3[0], PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                        ..
                    }),
                    ..
                } if a_str.str == "a"));
            } else {
                panic!("Expected third level destructure");
            }
        } else {
            panic!("Expected second level destructure");
        }
    } else {
        panic!("Expected first level destructure");
    }
}

// === PatternParserTests.scala ===

// Mirrors PatternParserTests.scala line 27
#[test]
fn test_simple_int() {
    let result = compile_pattern_expect("_ int");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
        destructure: None,
        ..
    } if int_str.str == "int"));
}

// Mirrors PatternParserTests.scala line 34
#[test]
fn test_name_only_capture() {
    let result = compile_pattern_expect("a");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: None,
        destructure: None,
        ..
    } if a_str.str == "a"));
}

// Mirrors PatternParserTests.scala line 39
#[test]
fn test_empty_pattern() {
    let result = compile_pattern_expect("_");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: None,
        destructure: None,
        ..
    }));
}

// Mirrors PatternParserTests.scala line 43
#[test]
fn test_capture_with_type_with_destructure() {
    let result = compile_pattern_expect("a Moo[a, b]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(patterns.len(), 2);
        
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a2_str, .. }),
                ..
            }),
            ..
        } if a2_str.str == "a"));
        
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected pattern with capture, type, and destructure");
    }
}

// Mirrors PatternParserTests.scala line 54
// This tests us handling an ambiguity properly, see CSTODTS in docs.
#[test]
fn test_cstodts() {
    let result = compile_pattern_expect("moo T[a int]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref moo_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref t_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(moo_str.str, "moo");
        assert_eq!(t_str.str, "T");
        assert_eq!(patterns.len(), 1);
        
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
            destructure: None,
            ..
        } if a_str.str == "a" && int_str.str == "int"));
    } else {
        panic!("Expected pattern with capture, type, and destructure");
    }
}

// Mirrors PatternParserTests.scala line 65
#[test]
fn test_capture_with_destructure_with_type_outside() {
    let result = compile_pattern_expect("a (int, bool)[a, b]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Tuple(TuplePT { elements: ref tuple_elements, .. })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(a_str.str, "a");
        assert_eq!(tuple_elements.len(), 2);
        
        assert!(matches!(tuple_elements[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }) if int_str.str == "int"));
        assert!(matches!(tuple_elements[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref bool_str, .. } }) if bool_str.str == "bool"));
        
        assert_eq!(patterns.len(), 2);
        
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a2_str, .. }),
                ..
            }),
            ..
        } if a2_str.str == "a"));
        
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected pattern with capture, tuple type, and destructure");
    }
}

// === TypeAndDestructureTests.scala ===

// Mirrors TypeAndDestructureTests.scala line 16
#[test]
fn test_empty_destructure_with_type() {
    let result = compile_pattern_expect("_ Muta[]");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } if muta_str.str == "Muta" && patterns.is_empty()));
}

// Mirrors TypeAndDestructureTests.scala line 25
#[test]
fn test_templated_destructure() {
    let result1 = compile_pattern_expect("_ Muta<int>[]");
    
    assert!(matches!(result1, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Call(CallPT {
            template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } }),
            args: ref args,
            ..
        })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } if muta_str.str == "Muta" && args.len() == 1 && patterns.is_empty() &&
         matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }) if int_str.str == "int")));
    
    let result2 = compile_pattern_expect("_ Muta<R>[]");
    
    assert!(matches!(result2, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Call(CallPT {
            template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } }),
            args: ref args,
            ..
        })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } if muta_str.str == "Muta" && args.len() == 1 && patterns.is_empty() &&
         matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref r_str, .. } }) if r_str.str == "R")));
}

// Mirrors TypeAndDestructureTests.scala line 47
#[test]
fn test_destructure_with_type_outside() {
    let result = compile_pattern_expect("_ (int, bool)[a, b]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Tuple(TuplePT { elements: ref tuple_elements, .. })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(tuple_elements.len(), 2);
        assert!(matches!(tuple_elements[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }) if int_str.str == "int"));
        assert!(matches!(tuple_elements[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref bool_str, .. } }) if bool_str.str == "bool"));
        
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                ..
            }),
            ..
        } if a_str.str == "a"));
        assert!(matches!(patterns[1], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                ..
            }),
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected pattern with tuple type and destructure");
    }
}

// Mirrors TypeAndDestructureTests.scala line 59
#[test]
fn test_destructure_with_typeless_capture() {
    let result = compile_pattern_expect("_ Muta[b]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(muta_str.str, "Muta");
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                mutate: None,
            }),
            templex: None,
            destructure: None,
            ..
        } if b_str.str == "b"));
    } else {
        panic!("Expected pattern with type and typeless capture");
    }
}

// Mirrors TypeAndDestructureTests.scala line 67
#[test]
fn test_destructure_with_typed_capture() {
    let result = compile_pattern_expect("_ Muta[b Marine]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(muta_str.str, "Muta");
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                mutate: None,
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref marine_str, .. } })),
            destructure: None,
            ..
        } if b_str.str == "b" && marine_str.str == "Marine"));
    } else {
        panic!("Expected pattern with type and typed capture");
    }
}

// Mirrors TypeAndDestructureTests.scala line 75
#[test]
fn test_destructure_with_unnamed_capture() {
    let result = compile_pattern_expect("_ Muta[_ Marine]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(muta_str.str, "Muta");
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
                mutate: None,
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref marine_str, .. } })),
            destructure: None,
            ..
        } if marine_str.str == "Marine"));
    } else {
        panic!("Expected pattern with type and unnamed capture");
    }
}

// Mirrors TypeAndDestructureTests.scala line 83
#[test]
fn test_destructure_with_runed_capture() {
    let result = compile_pattern_expect("_ Muta[_ R]");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref muta_str, .. } })),
        destructure: Some(DestructureP { patterns: ref patterns, .. }),
        ..
    } = result {
        assert_eq!(muta_str.str, "Muta");
        assert_eq!(patterns.len(), 1);
        assert!(matches!(patterns[0], PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
                mutate: None,
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref r_str, .. } })),
            destructure: None,
            ..
        } if r_str.str == "R"));
    } else {
        panic!("Expected pattern with type and runed capture");
    }
}

// === TypeTests.scala ===

// Mirrors TypeTests.scala line 17
#[test]
fn test_ignoring_name() {
    let result = compile_pattern_expect("_ int");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
        destructure: None,
        ..
    } if int_str.str == "int"));
}

// Mirrors TypeTests.scala line 21
#[test]
fn test_15a() {
    let result = compile_pattern_expect("_ [#3]MutableStruct");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
            variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
            size: box ITemplexPT::Int(IntPT { value: 3, .. }),
            element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref struct_str, .. } }),
            ..
        })),
        destructure: None,
        ..
    } if struct_str.str == "MutableStruct"));
}

// Mirrors TypeTests.scala line 32
#[test]
fn test_15b() {
    let result = compile_pattern_expect("_ [#3]<imm>MutableStruct");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Immutable, .. }),
            variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
            size: box ITemplexPT::Int(IntPT { value: 3, .. }),
            element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref struct_str, .. } }),
            ..
        })),
        destructure: None,
        ..
    } if struct_str.str == "MutableStruct"));
}

// Mirrors TypeTests.scala line 43
#[test]
fn test_15c() {
    let result = compile_pattern_expect("_ [#3]<imm, vary>MutableStruct");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Immutable, .. }),
            variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Varying, .. }),
            size: box ITemplexPT::Int(IntPT { value: 3, .. }),
            element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref struct_str, .. } }),
            ..
        })),
        destructure: None,
        ..
    } if struct_str.str == "MutableStruct"));
}

// Mirrors TypeTests.scala line 54
#[test]
fn test_15d() {
    let result = compile_pattern_expect("_ #[]int");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
            mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Immutable, .. }),
            element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }),
            ..
        })),
        destructure: None,
        ..
    } if int_str.str == "int"));
}

// Mirrors TypeTests.scala line 65
#[test]
fn test_sequence_type() {
    let result = compile_pattern_expect("_ (int, bool)");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            ..
        }),
        templex: Some(ITemplexPT::Tuple(TuplePT { elements: ref elems, .. })),
        destructure: None,
        ..
    } if elems.len() == 2 &&
         matches!(&elems[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }) if int_str.str == "int") &&
         matches!(&elems[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref bool_str, .. } }) if bool_str.str == "bool")));
}

// Mirrors TypeTests.scala line 74
#[test]
fn test_15() {
    let result = compile_pattern_expect("_ &[#3]MutableStruct");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Borrow, .. }),
            maybe_region: None,
            inner: box ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
                mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
                variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
                size: box ITemplexPT::Int(IntPT { value: 3, .. }),
                element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref struct_str, .. } }),
                ..
            }),
            ..
        })),
        destructure: None,
        ..
    } if struct_str.str == "MutableStruct"));
}

// Mirrors TypeTests.scala line 90
#[test]
fn test_15m() {
    let result = compile_pattern_expect("_ &&[#3]<_, _>MutableStruct");
    
    assert!(matches!(result, PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Weak, .. }),
            maybe_region: None,
            inner: box ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
                mutability: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
                variability: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
                size: box ITemplexPT::Int(IntPT { value: 3, .. }),
                element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref struct_str, .. } }),
                ..
            }),
            ..
        })),
        destructure: None,
        ..
    } if struct_str.str == "MutableStruct"));
}

// Mirrors TypeTests.scala line 106
#[test]
fn test_15z() {
    let result = compile_pattern_expect("_ MyOption<MyList<int>>");
    
    if let PatternPP {
        destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IgnoredLocalNameDeclaration(_),
            mutate: None,
        }),
        templex: Some(ITemplexPT::Call(CallPT {
            template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref option_str, .. } }),
            args: ref args,
            ..
        })),
        destructure: None,
        ..
    } = result {
        assert_eq!(option_str.str, "MyOption");
        assert_eq!(args.len(), 1);
        
        assert!(matches!(&args[0], ITemplexPT::Call(CallPT {
            template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref list_str, .. } }),
            args: ref inner_args,
            ..
        }) if list_str.str == "MyList" && inner_args.len() == 1 &&
             matches!(&inner_args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }) if int_str.str == "int")));
    } else {
        panic!("Expected nested generic pattern");
    }
}

