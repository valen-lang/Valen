/// Kind rule parsing tests
/// Mirrors tests from Frontend/ParsingPass/test/dev/vale/parsing/rules/KindRuleTests.scala

use crate::parsing::tests::test_parse_utils::*;
use crate::parsing::ast::*;

// Mirrors KindRuleTests.scala line 16
#[test]
fn test_empty_kind_rule() {
    let result = compile_rulex_expect("_ Kind");
    assert!(matches!(result, IRulexPR::Typed { rune: None, tyype: ITypePR::KindType, .. }));
}

// Mirrors KindRuleTests.scala line 22
#[test]
fn test_kind_with_rune() {
    let result = compile_rulex_expect("T Kind");
    assert!(matches!(result, IRulexPR::Typed {
        rune: Some(NameP { str: ref t_str, .. }),
        tyype: ITypePR::KindType,
        ..
    } if t_str.str == "T"));
}

// Mirrors KindRuleTests.scala line 29
#[test]
fn test_kind_with_destructure_only() {
    let result = compile_rulex_expect("Kind[_]");
    if let IRulexPR::Components {
        container: ITypePR::KindType,
        components: ref comps,
        ..
    } = result {
        assert_eq!(comps.len(), 1);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(AnonymousRunePT { .. }))));
    } else {
        panic!("Expected Components with KindType");
    }
}

// Mirrors KindRuleTests.scala line 36
#[test]
fn test_kind_matches_plain_int() {
    let result = compile_rulex_expect("int");
    assert!(matches!(result, IRulexPR::Templex(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } })) if s.str == "int"));
}

// Mirrors KindRuleTests.scala line 42
#[test]
fn test_kind_with_value() {
    let result = compile_rulex_expect("T Kind = int");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed {
            rune: Some(NameP { str: ref t_str, .. }),
            tyype: ITypePR::KindType,
            ..
        },
        right: box IRulexPR::Templex(ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } })),
        ..
    } = result {
        assert_eq!(t_str.str, "T");
        assert_eq!(int_str.str, "int");
    } else {
        panic!("Expected Equals with Typed and Templex");
    }
}

// Mirrors KindRuleTests.scala line 48
#[test]
fn test_kind_with_sequence_in_value_spot() {
    let result = compile_rulex_expect("T Kind = (int, bool)");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed {
            rune: Some(NameP { str: ref t_str, .. }),
            tyype: ITypePR::KindType,
            ..
        },
        right: box IRulexPR::Templex(ITemplexPT::Tuple(TuplePT { elements: ref elems, .. })),
        ..
    } = result {
        assert_eq!(t_str.str, "T");
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
        assert!(matches!(&elems[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "bool"));
    } else {
        panic!("Expected Equals with Typed and Tuple");
    }
}

// Mirrors KindRuleTests.scala line 58
#[test]
fn test_lone_sequence() {
    let result = compile_rulex_expect("(int, bool)");
    if let IRulexPR::Templex(ITemplexPT::Tuple(TuplePT { elements: ref elems, .. })) = result {
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
        assert!(matches!(&elems[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "bool"));
    } else {
        panic!("Expected Templex with Tuple");
    }
}

// Mirrors KindRuleTests.scala line 66
#[test]
fn test_templated_struct_one_arg() {
    let result1 = compile_rulex_expect("Moo<int>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } }),
        args: ref args,
        ..
    })) = result1 {
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
    } else {
        panic!("Expected Templex with Call");
    }
    
    let result2 = compile_rulex_expect("Moo<@int>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } }),
        args: ref args,
        ..
    })) = result2 {
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Share, .. }),
            maybe_region: None,
            inner: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }),
            ..
        }) if s.str == "int"));
    } else {
        panic!("Expected Templex with Call");
    }
}

// Mirrors KindRuleTests.scala line 75
#[test]
fn test_rwkilc() {
    let result1 = compile_rulex_expect("List<int>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref list_str, .. } }),
        args: ref args,
        ..
    })) = result1 {
        assert_eq!(list_str.str, "List");
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
    } else {
        panic!("Expected Templex with Call");
    }
    
    let result2 = compile_rulex_expect("K Int");
    assert!(matches!(result2, IRulexPR::Typed {
        rune: Some(NameP { str: ref k_str, .. }),
        tyype: ITypePR::IntType,
        ..
    } if k_str.str == "K"));
    
    let result3 = compile_rulex_expect("K<int>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref k_str, .. } }),
        args: ref args,
        ..
    })) = result3 {
        assert_eq!(k_str.str, "K");
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
    } else {
        panic!("Expected Templex with Call");
    }
}

// Mirrors KindRuleTests.scala line 87
#[test]
fn test_templated_struct_rune_arg() {
    let result = compile_rulex_expect("Moo<R>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } }),
        args: ref args,
        ..
    })) = result {
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "R"));
    } else {
        panic!("Expected Templex with Call");
    }
}

// Mirrors KindRuleTests.scala line 93
#[test]
fn test_templated_struct_multiple_args() {
    let result = compile_rulex_expect("Moo<int, str>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } }),
        args: ref args,
        ..
    })) = result {
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
        assert!(matches!(&args[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "str"));
    } else {
        panic!("Expected Templex with Call");
    }
}

// Mirrors KindRuleTests.scala line 99
#[test]
fn test_templated_struct_arg_is_another_templated_struct_with_one_arg() {
    let result = compile_rulex_expect("Moo<Blarg<int>>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } }),
        args: ref args,
        ..
    })) = result {
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(args.len(), 1);
        
        if let ITemplexPT::Call(CallPT {
            template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref blarg_str, .. } }),
            args: ref inner_args,
            ..
        }) = &args[0] {
            assert_eq!(blarg_str.str, "Blarg");
            assert_eq!(inner_args.len(), 1);
            assert!(matches!(&inner_args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
        } else {
            panic!("Expected inner Call");
        }
    } else {
        panic!("Expected Templex with Call");
    }
}

// Mirrors KindRuleTests.scala line 111
#[test]
fn test_templated_struct_arg_is_another_templated_struct_with_multiple_arg() {
    let result = compile_rulex_expect("Moo<Blarg<int, str>>");
    if let IRulexPR::Templex(ITemplexPT::Call(CallPT {
        template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref moo_str, .. } }),
        args: ref args,
        ..
    })) = result {
        assert_eq!(moo_str.str, "Moo");
        assert_eq!(args.len(), 1);
        
        if let ITemplexPT::Call(CallPT {
            template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref blarg_str, .. } }),
            args: ref inner_args,
            ..
        }) = &args[0] {
            assert_eq!(blarg_str.str, "Blarg");
            assert_eq!(inner_args.len(), 2);
            assert!(matches!(&inner_args[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
            assert!(matches!(&inner_args[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "str"));
        } else {
            panic!("Expected inner Call");
        }
    } else {
        panic!("Expected Templex with Call");
    }
}

// Mirrors KindRuleTests.scala line 124
#[test]
fn test_static_sized_array() {
    let result1 = compile_templex_expect("[#_]_");
    if let ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
        variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
        size: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
        element: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
        ..
    }) = result1 {
        // Success
    } else {
        panic!("Expected StaticSizedArray");
    }
    
    let result2 = compile_templex_expect("[#_]<imm>_");
    if let ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Immutable, .. }),
        variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
        size: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
        element: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
        ..
    }) = result2 {
        // Success
    } else {
        panic!("Expected StaticSizedArray with imm");
    }
    
    let result3 = compile_templex_expect("[#3]int");
    if let ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
        variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
        size: box ITemplexPT::Int(IntPT { value: 3, .. }),
        element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }),
        ..
    }) = result3 {
        assert_eq!(s.str, "int");
    } else {
        panic!("Expected StaticSizedArray with size 3");
    }
    
    let result4 = compile_templex_expect("[#N]int");
    if let ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
        variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
        size: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref n_str, .. } }),
        element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_str, .. } }),
        ..
    }) = result4 {
        assert_eq!(n_str.str, "N");
        assert_eq!(int_str.str, "int");
    } else {
        panic!("Expected StaticSizedArray with N");
    }
    
    let result5 = compile_templex_expect("[#_]int");
    if let ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
        variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
        size: box ITemplexPT::AnonymousRune(AnonymousRunePT { .. }),
        element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }),
        ..
    }) = result5 {
        assert_eq!(s.str, "int");
    } else {
        panic!("Expected StaticSizedArray");
    }
    
    let result6 = compile_templex_expect("[#N]T");
    if let ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        mutability: box ITemplexPT::Mutability(MutabilityPT { mutability: MutabilityP::Mutable, .. }),
        variability: box ITemplexPT::Variability(VariabilityPT { variability: VariabilityP::Final, .. }),
        size: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref n_str, .. } }),
        element: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref t_str, .. } }),
        ..
    }) = result6 {
        assert_eq!(n_str.str, "N");
        assert_eq!(t_str.str, "T");
    } else {
        panic!("Expected StaticSizedArray with N and T");
    }
}

// Mirrors KindRuleTests.scala line 145
#[test]
fn test_regular_sequence() {
    let result1 = compile_templex_expect("()");
    if let ITemplexPT::Tuple(TuplePT { elements: ref elems, .. }) = result1 {
        assert_eq!(elems.len(), 0);
    } else {
        panic!("Expected empty Tuple");
    }
    
    let result2 = compile_templex_expect("(int)");
    if let ITemplexPT::Tuple(TuplePT { elements: ref elems, .. }) = result2 {
        assert_eq!(elems.len(), 1);
        assert!(matches!(&elems[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
    } else {
        panic!("Expected Tuple with one element");
    }
    
    let result3 = compile_templex_expect("(int, bool)");
    if let ITemplexPT::Tuple(TuplePT { elements: ref elems, .. }) = result3 {
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
        assert!(matches!(&elems[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "bool"));
    } else {
        panic!("Expected Tuple with two elements");
    }
    
    let result4 = compile_templex_expect("(_, bool)");
    if let ITemplexPT::Tuple(TuplePT { elements: ref elems, .. }) = result4 {
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::AnonymousRune(AnonymousRunePT { .. })));
        assert!(matches!(&elems[1], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "bool"));
    } else {
        panic!("Expected Tuple with anonymous rune and bool");
    }
    
    let result5 = compile_templex_expect("(_, _)");
    if let ITemplexPT::Tuple(TuplePT { elements: ref elems, .. }) = result5 {
        assert_eq!(elems.len(), 2);
        assert!(matches!(&elems[0], ITemplexPT::AnonymousRune(AnonymousRunePT { .. })));
        assert!(matches!(&elems[1], ITemplexPT::AnonymousRune(AnonymousRunePT { .. })));
    } else {
        panic!("Expected Tuple with two anonymous runes");
    }
}

// Mirrors KindRuleTests.scala line 168
#[test]
fn test_prototype_kind_rule() {
    let result1 = compile_templex_expect("func moo(int)void");
    if let ITemplexPT::Func(FuncPT {
        name: NameP { str: ref moo_str, .. },
        parameters: ref params,
        return_type: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref void_str, .. } }),
        ..
    }) = result1 {
        assert_eq!(moo_str.str, "moo");
        assert_eq!(params.len(), 1);
        assert!(matches!(&params[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "int"));
        assert_eq!(void_str.str, "void");
    } else {
        panic!("Expected Func");
    }
    
    let result2 = compile_templex_expect("func moo(T)R");
    if let ITemplexPT::Func(FuncPT {
        name: NameP { str: ref moo_str, .. },
        parameters: ref params,
        return_type: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref r_str, .. } }),
        ..
    }) = result2 {
        assert_eq!(moo_str.str, "moo");
        assert_eq!(params.len(), 1);
        assert!(matches!(&params[0], ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }) if s.str == "T"));
        assert_eq!(r_str.str, "R");
    } else {
        panic!("Expected Func");
    }
}

