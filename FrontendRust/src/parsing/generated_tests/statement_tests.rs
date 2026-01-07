/// Statement parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/StatementTests.scala

use crate::parsing::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::lexing::errors::ParseError;

// Mirrors StatementTests.scala line 12
#[test]
fn test_simple_let() {
    let result = compile_statement_expect("x = 4;");
    
    assert!(matches!(result, IExpressionPE::Let(LetPE {
        pattern: PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref name_str, .. }),
                mutate: None,
            }),
            templex: None,
            destructure: None,
            ..
        },
        source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        ..
    }) if name_str.str == "x"));
}

// TODO: Port test "multiple statements" from StatementTests.scala line 18
// Mirrors StatementTests.scala line 18
#[test]
fn test_multiple_statements() {
    let result1 = compile_block_contents_expect("4");
    assert!(matches!(result1, IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })));

    let result2 = compile_block_contents_expect("4;");
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result2 {
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })));
        assert!(matches!(elems[1], IExpressionPE::Void(_)));
    } else {
        panic!("Expected Consecutor");
    }

    let result3 = compile_block_contents_expect("4; 3");
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result3 {
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })));
        assert!(matches!(elems[1], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. })));
    } else {
        panic!("Expected Consecutor");
    }

    let result4 = compile_block_contents_expect("4; 3;");
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result4 {
        assert_eq!(elems.len(), 3);
        assert!(matches!(elems[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })));
        assert!(matches!(elems[1], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. })));
        assert!(matches!(elems[2], IExpressionPE::Void(_)));
    } else {
        panic!("Expected Consecutor");
    }
}

// Mirrors StatementTests.scala line 40
#[test]
fn test_8() {
    let result = compile_statement_expect("[x, y] = (4, 5);");
    
    if let IExpressionPE::Let(LetPE {
        pattern: PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref patterns, .. }),
            ..
        },
        source: box IExpressionPE::Tuple(TuplePE { elements: ref tuple_elems, .. }),
        ..
    }) = result {
        assert_eq!(patterns.len(), 2);
        assert_eq!(tuple_elems.len(), 2);
        
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
        
        assert!(matches!(tuple_elems[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })));
        assert!(matches!(tuple_elems[1], IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. })));
    } else {
        panic!("Expected Let with destructure and tuple");
    }
}

// TODO: Port test "9" from StatementTests.scala line 55
// Mirrors StatementTests.scala line 55
#[test]
fn test_9() {
    let result = compile_statement_expect("set x.a = 5;");
    
    assert!(matches!(result, IExpressionPE::Mutate(MutatePE {
        mutatee: box IExpressionPE::Dot(DotPE {
            left: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref x_str, .. }),
                ..
            }),
            member: NameP { str: ref a_str, .. },
            ..
        }),
        source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
        ..
    }) if x_str.str == "x" && a_str.str == "a"));
}

// Mirrors StatementTests.scala line 61
#[test]
fn test_1pe() {
    let result = compile_statement_expect(r#"set board.PE.PE.symbol = "v";"#);
    
    assert!(matches!(result,
        IExpressionPE::Mutate(MutatePE {
            mutatee: box IExpressionPE::Dot(DotPE {
                left: box IExpressionPE::Dot(DotPE {
                    left: box IExpressionPE::Dot(DotPE {
                        left: box IExpressionPE::Lookup(LookupPE {
                            name: IImpreciseNameP::LookupName(NameP { str: ref board_str, .. }),
                            ..
                        }),
                        member: NameP { str: ref pe1_str, .. },
                        ..
                    }),
                    member: NameP { str: ref pe2_str, .. },
                    ..
                }),
                member: NameP { str: ref symbol_str, .. },
                ..
            }),
            source: box IExpressionPE::ConstantStr(ConstantStrPE { value: ref v_str, .. }),
            ..
        })
    if board_str.str == "board" && pe1_str.str == "PE" && pe2_str.str == "PE" && symbol_str.str == "symbol" && v_str == "v"));
}

// Mirrors StatementTests.scala line 67
#[test]
fn test_simple_let_2() {
    let result = compile_statement_expect("x = 3;");
    
    assert!(matches!(result,
        IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x_str, .. }),
                    mutate: None,
                }),
                templex: None,
                destructure: None,
                ..
            },
            source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
        })
    if x_str.str == "x"));
}

// Mirrors StatementTests.scala line 73
#[test]
fn test_simple_mut() {
    let result = compile_statement_expect("set x = 5;");
    
    assert!(matches!(result,
        IExpressionPE::Mutate(MutatePE {
            mutatee: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref x_str, .. }),
                ..
            }),
            source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
            ..
        })
    if x_str.str == "x"));
}

// Mirrors StatementTests.scala line 79
#[test]
fn test_expr_starting_with_return() {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    let result = compile_statement_expect("retcode()");
    
    assert!(matches!(result,
        IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref name_str, .. }),
                ..
            }),
            arg_exprs: ref args,
            ..
        })
    if name_str.str == "retcode" && args.is_empty()));
}

// Mirrors StatementTests.scala line 87
#[test]
fn test_inner_set() {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    let result = compile_statement_expect("oldArray = set list.array = newArray;");
    
    assert!(matches!(result,
        IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref old_str, .. }),
                    ..
                }),
                ..
            },
            source: box IExpressionPE::Mutate(MutatePE {
                mutatee: box IExpressionPE::Dot(DotPE {
                    left: box IExpressionPE::Lookup(LookupPE {
                        name: IImpreciseNameP::LookupName(NameP { str: ref list_str, .. }),
                        ..
                    }),
                    member: NameP { str: ref array_str, .. },
                    ..
                }),
                source: box IExpressionPE::Lookup(LookupPE {
                    name: IImpreciseNameP::LookupName(NameP { str: ref new_str, .. }),
                    ..
                }),
                ..
            }),
            ..
        })
    if old_str.str == "oldArray" && list_str.str == "list" && array_str.str == "array" && new_str.str == "newArray"));
}

// TODO: Port test "Test if-statement producing" from StatementTests.scala line 100
// Mirrors StatementTests.scala line 100
#[test]
fn test_if_statement_producing() {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    let result = compile_statement_expect("if true { 3 } else { 4 }");
    
    assert!(matches!(result,
        IExpressionPE::If(IfPE {
            condition: box IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. }),
            then_body: box BlockPE {
                inner: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
                ..
            },
            else_body: box BlockPE {
                inner: box IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
                ..
            },
            ..
        })
   ));
}

// Mirrors StatementTests.scala line 112
#[test]
fn test_destruct() {
    let result = compile_statement_expect("destruct x;");
    
    assert!(matches!(result,
        IExpressionPE::Destruct(DestructPE {
            inner: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref x_str, .. }),
                ..
            }),
            ..
        })
    if x_str.str == "x"));
}

// Mirrors StatementTests.scala line 118
#[test]
fn test_unlet() {
    let result = compile_statement_expect("unlet x");
    
    assert!(matches!(result,
        IExpressionPE::Unlet(UnletPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref x_str, .. }),
            ..
        })
    if x_str.str == "x"));
}

// Mirrors StatementTests.scala line 124
#[test]
fn test_dot_on_function_call_result() {
    let result = compile_statement_expect("Wizard(8).charges");
    
    if let IExpressionPE::Dot(DotPE {
        left: box IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref wizard_str, .. }),
                ..
            }),
            arg_exprs: ref args,
            ..
        }),
        member: NameP { str: ref charges_str, .. },
        ..
    }) = result {
        assert_eq!(wizard_str.str, "Wizard");
        assert_eq!(args.len(), 1);
        assert_eq!(charges_str.str, "charges");
        assert!(matches!(args[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 8, .. })));
    } else {
        panic!("Expected Dot expression");
    }
}

// Mirrors StatementTests.scala line 135
#[test]
fn test_let_with_pattern_with_only_a_capture() {
    let result = compile_statement_expect("a = m;");
    
    assert!(matches!(result,
        IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                templex: None,
                destructure: None,
                ..
            },
            source: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref m_str, .. }),
                ..
            }),
            ..
        })
    if a_str.str == "a" && m_str.str == "m"));
}

// Mirrors StatementTests.scala line 141
#[test]
fn test_let_with_simple_pattern() {
    let result = compile_statement_expect("a Moo = m;");
    
    assert!(matches!(result,
        IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                templex: Some(ITemplexPT::NameOrRune(NameP { str: ref moo_str, .. })),
                destructure: None,
                ..
            },
            source: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref m_str, .. }),
                ..
            }),
            ..
        })
    if a_str.str == "a" && moo_str.str == "Moo" && m_str.str == "m"));
}

// TODO: Port test "Let with simple pattern in destructure" from StatementTests.scala line 149
// Mirrors StatementTests.scala line 149
#[test]
fn test_let_with_simple_pattern_in_destructure() {
    let result = compile_statement_expect("[a Moo] = m;");
    
    if let IExpressionPE::Let(LetPE {
        pattern: PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref patterns, .. }),
            ..
        },
        source: box IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref m_str, .. }),
            ..
        }),
        ..
    }) = result {
        assert_eq!(patterns.len(), 1);
        assert_eq!(m_str.str, "m");
        assert!(matches!(patterns[0],
            PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                templex: Some(ITemplexPT::NameOrRune(NameP { str: ref moo_str, .. })),
                ..
            }
        if a_str.str == "a" && moo_str.str == "Moo"));
    } else {
        panic!("Expected Let expression");
    }
}

// Mirrors StatementTests.scala line 159
#[test]
fn test_let_with_destructuring_pattern() {
    let result = compile_statement_expect("Muta[ ] = m;");
    
    assert!(matches!(result,
        IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: None,
                templex: Some(ITemplexPT::NameOrRune(NameP { str: ref muta_str, .. })),
                destructure: Some(DestructureP { patterns: ref patterns, .. }),
                ..
            },
            source: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref m_str, .. }),
                ..
            }),
            ..
        })
    if muta_str.str == "Muta" && patterns.is_empty() && m_str.str == "m"));
}

// Mirrors StatementTests.scala line 165
#[test]
fn test_destructure_pattern_with_let_and_set() {
    let result = compile_statement_expect("[a, set x] = m;");
    
    if let IExpressionPE::Let(LetPE {
        pattern: PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref patterns, .. }),
            ..
        },
        ..
    }) = result {
        assert_eq!(patterns.len(), 2);
        assert!(matches!(patterns[0],
            PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    mutate: None,
                }),
                ..
            }
        if a_str.str == "a"));
        assert!(matches!(patterns[1],
            PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x_str, .. }),
                    mutate: Some(_),
                }),
                ..
            }
        if x_str.str == "x"));
    } else {
        panic!("Expected Let expression");
    }
}

// Mirrors StatementTests.scala line 179
#[test]
fn test_ret() {
    let result = compile_statement_expect("return 3;");
    
    assert!(matches!(result,
        IExpressionPE::Return(ReturnPE {
            expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
        })
   ));
}

// Mirrors StatementTests.scala line 185
#[test]
fn test_foreach() {
    let result = compile_statement_expect("foreach i in myList { }");
    
    assert!(matches!(result,
        IExpressionPE::Each(EachPE {
            entry_pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref i_str, .. }),
                    ..
                }),
                ..
            },
            iterable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref list_str, .. }),
                ..
            }),
            body: box BlockPE { .. },
            ..
        })
    if i_str.str == "i" && list_str.str == "myList"));
}

// Mirrors StatementTests.scala line 196
#[test]
fn test_foreach_with_borrow() {
    let result = compile_statement_expect("foreach i in &myList { }");
    
    assert!(matches!(result,
        IExpressionPE::Each(EachPE {
            entry_pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref i_str, .. }),
                    ..
                }),
                ..
            },
            iterable_expr: box IExpressionPE::Augment(AugmentPE {
                target_ownership: OwnershipP::Borrow,
                inner: box IExpressionPE::Lookup(LookupPE {
                    name: IImpreciseNameP::LookupName(NameP { str: ref list_str, .. }),
                    ..
                }),
                ..
            }),
            body: box BlockPE { .. },
            ..
        })
    if i_str.str == "i" && list_str.str == "myList"));
}

// Mirrors StatementTests.scala line 207
#[test]
fn test_foreach_with_two_receivers() {
    let result = compile_statement_expect("foreach [a, b] in myList { }");
    
    if let IExpressionPE::Each(EachPE {
        entry_pattern: PatternPP {
            destination: None,
            templex: None,
            destructure: Some(DestructureP { patterns: ref patterns, .. }),
            ..
        },
        iterable_expr: box IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref list_str, .. }),
            ..
        }),
        body: box BlockPE { .. },
        ..
    }) = result {
        assert_eq!(patterns.len(), 2);
        assert_eq!(list_str.str, "myList");
        assert!(matches!(patterns[0],
            PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                ..
            }
        if a_str.str == "a"));
        assert!(matches!(patterns[1],
            PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref b_str, .. }),
                    ..
                }),
                ..
            }
        if b_str.str == "b"));
    } else {
        panic!("Expected Each expression");
    }
}

// Mirrors StatementTests.scala line 224
#[test]
fn test_foreach_complex_iterable() {
    let result = compile_statement_expect("foreach i in myList = 3; myList { }");
    
    if let IExpressionPE::Each(EachPE {
        entry_pattern: PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref i_str, .. }),
                ..
            }),
            ..
        },
        iterable_expr: box IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }),
        body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    }) = result {
        assert_eq!(i_str.str, "i");
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0],
            IExpressionPE::Let(LetPE {
                pattern: PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref list_str, .. }),
                        ..
                    }),
                    ..
                },
                source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
                ..
            }) if list_str.str == "myList"));
        // Check the second element is a lookup of "myList"
        if let IExpressionPE::Lookup(LookupPE { name: IImpreciseNameP::LookupName(NameP { str: ref list2_str, .. }), .. }) = &elems[1] {
            assert_eq!(list2_str.str, "myList");
        } else {
            panic!("Expected Lookup expression");
        }
    } else {
        panic!("Expected Each expression");
    }
}

// TODO: Port test "Multiple statements" from StatementTests.scala line 238
// Mirrors StatementTests.scala line 238
#[test]
fn test_multiple_statements_2() {
    compile_block_contents_expect(
        r#"
42;
43;
"#);
}

// Mirrors StatementTests.scala line 246
#[test]
fn test_if_and_another_statement() {
    compile_block_contents_expect(
        r#"
newCapacity = if (true) { 1 } else { 2 };
newArray = 3;
"#);
}

// Mirrors StatementTests.scala line 254
#[test]
fn test_block_trailing_void_presence() {
    let result1 = compile_block_contents_expect("moo()");
    assert!(matches!(result1, IExpressionPE::FunctionCall(FunctionCallPE {
        callable_expr: box IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref moo_str, .. }),
            ..
        }),
        arg_exprs: ref args,
        ..
    }) if moo_str.str == "moo" && args.is_empty()));

    let result2 = compile_block_contents_expect("moo();");
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result2 {
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0], IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref moo_str, .. }),
                ..
            }),
            ..
        }) if moo_str.str == "moo"));
        assert!(matches!(elems[1], IExpressionPE::Void(_)));
    } else {
        panic!("Expected Consecutor");
    }
}

// Mirrors StatementTests.scala line 267
#[test]
fn test_block_with_statement_and_result() {
    let result = compile_block_contents_expect(
        r#"
b;
a
"#);
    
    // Should return a vector with two elements
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result {
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0], IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref b_str, .. }),
            ..
        }) if b_str.str == "b"));
        assert!(matches!(elems[1], IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref a_str, .. }),
            ..
        }) if a_str.str == "a"));
    } else {
        panic!("Expected Consecutor");
    }
}

// Mirrors StatementTests.scala line 278
#[test]
fn test_block_with_result() {
    let result = compile_statement_expect("3");
    
    assert!(matches!(result, IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. })));
}

// Mirrors StatementTests.scala line 284
#[test]
fn test_block_with_result_that_could_be_an_expr() {
    // = doThings(a); could be misinterpreted as an expression doThings(=, a) if we're
    // not careful.
    let result = compile_block_contents_expect(
        r#"
a = 2;
doThings(a)
"#);
    
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result {
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0], IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                ..
            },
            source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
            ..
        }) if a_str.str == "a"));
        assert!(matches!(elems[1], IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref do_str, .. }),
                ..
            }),
            arg_exprs: ref args,
            ..
        }) if do_str.str == "doThings" && args.len() == 1));
    } else {
        panic!("Expected Consecutor");
    }
}

// Mirrors StatementTests.scala line 298
#[test]
fn test_mutating_as_statement() {
    let result = compile_statement_expect("set x = 6;");
    
    assert!(matches!(result, IExpressionPE::Mutate(MutatePE {
        mutatee: box IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref x_str, .. }),
            ..
        }),
        source: box IExpressionPE::ConstantInt(ConstantIntPE { value: 6, .. }),
        ..
    }) if x_str.str == "x"));
}

// Mirrors StatementTests.scala line 307
#[test]
fn test_lone_block() {
    let result = compile_block_contents_expect(
        r#"
block {
  a
}
"#);
    
    assert!(matches!(result, IExpressionPE::Block(BlockPE {
        maybe_pure: None,
        maybe_default_region: None,
        inner: box IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref a_str, .. }),
            ..
        }),
        ..
    }) if a_str.str == "a"));
}

// TODO: Port test "Pure block" from StatementTests.scala line 318
// Mirrors StatementTests.scala line 318
#[test]
fn test_pure_block() {
    // Just make sure it parses, so that we can highlight it.
    // The pure block feature doesn't actually exist yet.
    compile_block_contents_expect(
        r#"
pure block {
  a
}
"#);
}

// Mirrors StatementTests.scala line 329
#[test]
fn test_unsafe_pure_block() {
    // Just make sure it parses, so that we can highlight it.
    // The unsafe pure block feature doesn't actually exist yet.
    compile_block_contents_expect(
        r#"
unsafe pure block {
  a
}
"#);
}

// Mirrors StatementTests.scala line 341
#[test]
fn test_report_leaving_out_semicolon_or_ending_body_after_expression_for_square() {
    let result = compile_statement(
        r#"
block {
  floop() ]
}
"#);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        ParseError::BadStartOfStatementError(_) => {},
        e => panic!("Expected BadStartOfStatement, got {:?}", e),
    }
}

// Mirrors StatementTests.scala line 352
#[test]
fn test_empty_block() {
    let result = compile_block_contents_expect(
        r#"
block {
}
return 3;
"#);
    
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result {
        assert_eq!(elems.len(), 3);
        assert!(matches!(elems[0], IExpressionPE::Block(BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        })));
        assert!(matches!(elems[1], IExpressionPE::Return(ReturnPE {
            expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
        })));
        assert!(matches!(elems[2], IExpressionPE::Void(_)));
    } else {
        panic!("Expected Consecutor");
    }
}

// Mirrors StatementTests.scala line 366
#[test]
fn test_cant_use_set_as_a_local_name() {
    let result = compile_statement(r#"[set] = (6,)"#);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        ParseError::CantUseThatLocalName { name, .. } if name == "set" => {},
        e => panic!("Expected CantUseThatLocalName with 'set', got {:?}", e),
    }
}

// Mirrors StatementTests.scala line 374
#[test]
fn test_foreach_2() {
    let result = compile_block_contents_expect(
        r#"
foreach i in a {
  i
}
"#);
    
    assert!(matches!(result, IExpressionPE::Each(EachPE {
        entry_pattern: PatternPP {
            destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref i_str, .. }),
                ..
            }),
            ..
        },
        iterable_expr: box IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP { str: ref a_str, .. }),
            ..
        }),
        body: box BlockPE {
            inner: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref i2_str, .. }),
                ..
            }),
            ..
        },
        ..
    }) if i_str.str == "i" && a_str.str == "a" && i2_str.str == "i"));
}

// Mirrors StatementTests.scala line 393
#[test]
fn test_foreach_expr() {
    let result = compile_block_contents_expect(
        r#"
a = foreach i in c { i };
"#);
    
    if let IExpressionPE::Consecutor(ConsecutorPE { inners: ref elems, .. }) = result {
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0], IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a_str, .. }),
                    ..
                }),
                ..
            },
            source: box IExpressionPE::Each(EachPE { .. }),
            ..
        }) if a_str.str == "a"));
        assert!(matches!(elems[1], IExpressionPE::Void(_)));
    } else {
        panic!("Expected Consecutor");
    }
}
