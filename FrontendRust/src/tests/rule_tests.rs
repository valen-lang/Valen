/// Rule parsing tests
/// Mirrors tests from Frontend/ParsingPass/test/dev/vale/parsing/rules/

use crate::tests::test_parse_utils::*;
use crate::parsing::ast::*;

// === RulesEnumsTests.scala ===

// Mirrors RulesEnumsTests.scala line 17
#[test]
fn test_ownership() {
    let result1 = compile_rulex_expect("X");
    assert!(matches!(result1, IRulexPR::Templex( ITemplexPT::NameOrRune(NameP { str: ref x_str, .. })) if x_str.str == "X"));
    
    let result2 = compile_rulex_expect("X Ownership");
    assert!(matches!(result2, IRulexPR::Typed { rune: Some(NameP { str: ref x_str, .. }), tyype: ITypePR::OwnershipType, .. } if x_str.str == "X"));
    
    let result3 = compile_rulex_expect("X = own");
    assert!(matches!(result3, IRulexPR::Equals {
        left: box IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref x_str, .. })),
        right: box IRulexPR::Templex(ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Own, .. })),
        ..
    } if x_str.str == "X"));
    
    let result4 = compile_rulex_expect("X Ownership = any(own, borrow, weak)");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed { rune: Some(NameP { str: ref x_str, .. }), tyype: ITypePR::OwnershipType, .. },
        right: box IRulexPR::BuiltinCall { name: NameP { str: ref any_str, .. }, args: ref args, .. },
        ..
    } = result4 {
        assert_eq!(x_str.str, "X");
        assert_eq!(any_str.str, "any");
        assert_eq!(args.len(), 3);
        assert!(matches!(&args[0], IRulexPR::Templex( ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Own, .. }))));
        assert!(matches!(&args[1], IRulexPR::Templex( ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Borrow, .. }))));
        assert!(matches!(&args[2], IRulexPR::Templex( ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Weak, .. }))));
    } else {
        panic!("Expected Equals with Typed and BuiltinCall");
    }
    
    let result5 = compile_rulex_expect("_ Ownership");
    assert!(matches!(result5, IRulexPR::Typed { rune: None, tyype: ITypePR::OwnershipType, .. }));
    
    let result6 = compile_rulex_expect("own");
    assert!(matches!(result6, IRulexPR::Templex( ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Own, .. }))));
    
    let result7 = compile_rulex_expect("_ Ownership = any(own, share)");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed { rune: None, tyype: ITypePR::OwnershipType, .. },
        right: box IRulexPR::BuiltinCall { name: NameP { str: ref any_str, .. }, args: ref args, .. },
        ..
    } = result7 {
        assert_eq!(any_str.str, "any");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], IRulexPR::Templex( ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Own, .. }))));
        assert!(matches!(&args[1], IRulexPR::Templex( ITemplexPT::Ownership(OwnershipPT { ownership: OwnershipP::Share, .. }))));
    } else {
        panic!("Expected Equals with Typed and BuiltinCall");
    }
}

// Mirrors RulesEnumsTests.scala line 35
#[test]
fn test_mutability() {
    let result1 = compile_rulex_expect("X");
    assert!(matches!(result1, IRulexPR::Templex( ITemplexPT::NameOrRune(NameP { str: ref x_str, .. })) if x_str.str == "X"));
    
    let result2 = compile_rulex_expect("X Mutability");
    assert!(matches!(result2, IRulexPR::Typed { rune: Some(NameP { str: ref x_str, .. }), tyype: ITypePR::MutabilityType, .. } if x_str.str == "X"));
    
    let result3 = compile_rulex_expect("X = mut");
    assert!(matches!(result3, IRulexPR::Equals {
        left: box IRulexPR::Templex( ITemplexPT::NameOrRune(NameP { str: ref x_str, .. })),
        right: box IRulexPR::Templex( ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. }),
        ..
    } if x_str.str == "X"));
    
    let result4 = compile_rulex_expect("X Mutability = mut");
    assert!(matches!(result4, IRulexPR::Equals {
        left: box IRulexPR::Typed { rune: Some(NameP { str: ref x_str, .. }), tyype: ITypePR::MutabilityType, .. },
        right: box IRulexPR::Templex( ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. }),
        ..
    } if x_str.str == "X"));
    
    let result5 = compile_rulex_expect("_ Mutability");
    assert!(matches!(result5, IRulexPR::Typed { rune: None, tyype: ITypePR::MutabilityType, .. }));
    
    let result6 = compile_rulex_expect("mut");
    assert!(matches!(result6, IRulexPR::Templex( ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. })));
    
    let result7 = compile_rulex_expect("_ Mutability = any(mut, imm)");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed { rune: None, tyype: ITypePR::MutabilityType, .. },
        right: box IRulexPR::BuiltinCall { name: NameP { str: ref any_str, .. }, args: ref args, .. },
        ..
    } = result7 {
        assert_eq!(any_str.str, "any");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], IRulexPR::Templex( ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. })));
        assert!(matches!(&args[1], IRulexPR::Templex( ITemplexPT::Mutability { mutability: MutabilityP::Immutable, .. })));
    } else {
        panic!("Expected Equals with Typed and BuiltinCall");
    }
}

// Mirrors RulesEnumsTests.scala line 53
#[test]
fn test_location() {
    let result1 = compile_rulex_expect("X");
    assert!(matches!(result1, IRulexPR::Templex( ITemplexPT::NameOrRune(NameP { str: ref x_str, .. })) if x_str.str == "X"));
    
    let result2 = compile_rulex_expect("X Location");
    assert!(matches!(result2, IRulexPR::Typed { rune: Some(NameP { str: ref x_str, .. }), tyype: ITypePR::LocationType, .. } if x_str.str == "X"));
    
    let result3 = compile_rulex_expect("X = inl");
    assert!(matches!(result3, IRulexPR::Equals {
        left: box IRulexPR::Templex( ITemplexPT::NameOrRune(NameP { str: ref x_str, .. })),
        right: box IRulexPR::Templex( ITemplexPT::Location { location: LocationP::Inline, .. }),
        ..
    } if x_str.str == "X"));
    
    let result4 = compile_rulex_expect("X Location = inl");
    assert!(matches!(result4, IRulexPR::Equals {
        left: box IRulexPR::Typed { rune: Some(NameP { str: ref x_str, .. }), tyype: ITypePR::LocationType, .. },
        right: box IRulexPR::Templex( ITemplexPT::Location { location: LocationP::Inline, .. }),
        ..
    } if x_str.str == "X"));
    
    let result5 = compile_rulex_expect("_ Location");
    assert!(matches!(result5, IRulexPR::Typed { rune: None, tyype: ITypePR::LocationType, .. }));
    
    let result6 = compile_rulex_expect("inl");
    assert!(matches!(result6, IRulexPR::Templex( ITemplexPT::Location { location: LocationP::Inline, .. })));
    
    let result7 = compile_rulex_expect("_ Location = any(inl, heap)");
    if let IRulexPR::Equals {
        left: box IRulexPR::Typed { rune: None, tyype: ITypePR::LocationType, .. },
        right: box IRulexPR::BuiltinCall { name: NameP { str: ref any_str, .. }, args: ref args, .. },
        ..
    } = result7 {
        assert_eq!(any_str.str, "any");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], IRulexPR::Templex( ITemplexPT::Location { location: LocationP::Inline, .. })));
        assert!(matches!(&args[1], IRulexPR::Templex( ITemplexPT::Location { location: LocationP::Yonder, .. })));
    } else {
        panic!("Expected Equals with Typed and BuiltinCall");
    }
}

// === RuleTests.scala ===

// Mirrors RuleTests.scala line 17
#[test]
fn test_relations() {
    // Test implements relation
    let result1 = compile_rulex_expect("implements(MyObject, IObject)");
    if let IRulexPR::BuiltinCall { name: NameP { str: ref name_str, .. }, args: ref args, .. } = result1 {
        assert_eq!(name_str.str, "implements");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "MyObject"));
        assert!(matches!(&args[1], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "IObject"));
    } else {
        panic!("Expected BuiltinCall for implements");
    }
    
    let result2 = compile_rulex_expect("implements(R, IObject)");
    if let IRulexPR::BuiltinCall { name: NameP { str: ref name_str, .. }, args: ref args, .. } = result2 {
        assert_eq!(name_str.str, "implements");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "R"));
        assert!(matches!(&args[1], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "IObject"));
    } else {
        panic!("Expected BuiltinCall for implements");
    }
    
    let result3 = compile_rulex_expect("implements(MyObject, T)");
    if let IRulexPR::BuiltinCall { name: NameP { str: ref name_str, .. }, args: ref args, .. } = result3 {
        assert_eq!(name_str.str, "implements");
        assert_eq!(args.len(), 2);
        assert!(matches!(&args[0], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "MyObject"));
        assert!(matches!(&args[1], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "T"));
    } else {
        panic!("Expected BuiltinCall for implements");
    }
    
    // Test exists with func
    let result4 = compile_rulex_expect("exists(func +(T)int)");
    if let IRulexPR::BuiltinCall { name: NameP { str: ref name_str, .. }, args: ref args, .. } = result4 {
        assert_eq!(name_str.str, "exists");
        assert_eq!(args.len(), 1);
        assert!(matches!(&args[0], IRulexPR::Templex(ITemplexPT::Func {
            name: NameP { str: ref func_name, .. },
            parameters: ref params,
            return_type: box ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. }),
            ..
        }) if func_name.str == "+" && params.len() == 1 && ret_str.str == "int"));
    } else {
        panic!("Expected BuiltinCall for exists");
    }
}

// Mirrors RuleTests.scala line 32
#[test]
fn test_super_complicated() {
    // Just make sure it parses without error
    let _result = compile_rulex_expect("C = any([#I]X, [#N]T)");
}

// Mirrors RuleTests.scala line 36
#[test]
fn test_destructure_prototype() {
    let result = compile_rulex_expect("Prot[_, _, T] = moo");
    if let IRulexPR::Equals {
        left: box IRulexPR::Components {
            container: ITypePR::PrototypeType,
            components: ref comps,
            ..
        },
        right: box IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref moo_str, .. })),
        ..
    } = result {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[1], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref t_str, .. })) if t_str.str == "T"));
        assert_eq!(moo_str.str, "moo");
    } else {
        panic!("Expected Equals with Components and Templex");
    }
}

// Mirrors RuleTests.scala line 46
#[test]
fn test_func() {
    let result = compile_rulex_expect("func moo()T");
    if let IRulexPR::Templex(ITemplexPT::Func {
        name: NameP { str: ref func_name, .. },
        parameters: ref params,
        return_type: box ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. }),
        ..
    }) = result {
        assert_eq!(func_name.str, "moo");
        assert_eq!(params.len(), 0);
        assert_eq!(ret_str.str, "T");
    } else {
        panic!("Expected Templex with Func");
    }
}

// Mirrors RuleTests.scala line 57
#[test]
fn test_prototype_with_coords() {
    let result = compile_rulex_expect("Prot[_, pack(int, bool), _]");
    if let IRulexPR::Components {
        container: ITypePR::PrototypeType,
        components: ref comps,
        ..
    } = result {
        assert_eq!(comps.len(), 3);
        assert!(matches!(&comps[0], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
        
        // Check the pack builtin call
        if let IRulexPR::BuiltinCall { name: NameP { str: ref pack_str, .. }, args: ref args, .. } = &comps[1] {
            assert_eq!(pack_str.str, "pack");
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "int"));
            assert!(matches!(&args[1], IRulexPR::Templex(ITemplexPT::NameOrRune(NameP { str: ref s, .. })) if s.str == "bool"));
        } else {
            panic!("Expected BuiltinCall for pack");
        }
        
        assert!(matches!(&comps[2], IRulexPR::Templex(ITemplexPT::AnonymousRune(_))));
    } else {
        panic!("Expected Components with PrototypeType");
    }
}

