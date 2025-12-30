/// Struct parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/StructTests.scala

use crate::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::{should_have, matches_pattern};

#[test]
fn simple_struct() {
    // Test: Simple struct
    // Lines 25-38 in StructTests.scala
    
    let denizen = &compile_file_expect("struct Moo { }").denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        attributes: ref attrs,
        mutability: None,
        identifying_runes: None,
        template_rules: None,
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "Moo" && attrs.is_empty() && m.is_empty() => {});
}

#[test]
fn test_17a() {
    // Test: 17a
    // Lines 40-53 in StructTests.scala
    
    let denizen = &compile_file_expect(
        r#"
        struct Mork {
          a @ListNode<T>;
        }
        "#
    ).denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        members: StructMembersP { contents: ref m, .. },
        ..
    }) => {
        assert_eq!(m.len(), 1);
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            variability: VariabilityP::Final,
            tyype: ITemplexPT::Interpreted {
                maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Share, .. }),
                maybe_region: None,
                inner: box ITemplexPT::Call {
                    template: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                    args: ref a,
                    ..
                },
                ..
            },
            ..
        } if n.str == "a" && t.str == "ListNode" => {
            assert_eq!(a.len(), 1);
        });
    });
}

#[test]
fn imm_generic_param() {
    // Test: Imm generic param
    // Lines 54-73 in StructTests.scala
    
    let denizen = compile_denizen_expect(
        r#"
        struct MyImmContainer<T Ref imm> imm { value T; }
        "#
    );
    
    let s = should_have!(denizen, IDenizenP::TopLevelStruct(ref s) => s);
    
    // Check generic parameter has ImmutableRuneAttribute
    let params = s.identifying_runes.as_ref().expect("Expected identifying_runes");
    assert_eq!(params.params.len(), 1);
    should_have!(&params.params[0].attributes[0], IRuneAttributeP::ImmutableRuneAttribute(_) => {});
    
    // Check struct member
    should_have!(&s.members.contents[0], IStructContent::NormalStructMember {
        name: NameP { str: ref n, .. },
        variability: VariabilityP::Final,
        tyype: ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
        ..
    } if n.str == "value" && t.str == "T" => {});
}

#[test]
fn test_18() {
    // Test: 18
    // Lines 75-85 in StructTests.scala
    
    let denizen = &compile_file_expect(
        r#"
        struct Mork {
          a []<imm>T;
        }
        "#
    ).denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        members: StructMembersP { contents: ref m, .. },
        ..
    }) => {
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            variability: VariabilityP::Final,
            tyype: ITemplexPT::RuntimeSizedArray {
                mutability: box ITemplexPT::Mutability { mutability: MutabilityP::Immutable, .. },
                element: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                ..
            },
            ..
        } if n.str == "a" && t.str == "T" => {});
    });
}

#[test]
fn variadic_struct() {
    // Test: Variadic struct
    // Lines 87-93 in StructTests.scala
    
    let denizen = &compile_file_expect("struct Moo<T> { _ ..T; }").denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        members: StructMembersP { contents: ref m, .. },
        ..
    }) => {
        should_have!(&m[0], IStructContent::VariadicStructMember {
            variability: VariabilityP::Final,
            tyype: ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
            ..
        } if t.str == "T" => {});
    });
}

#[test]
fn variadic_struct_with_varying() {
    // Test: Variadic struct with varying
    // Lines 95-99 in StructTests.scala
    
    let denizen = &compile_file_expect("struct Moo<T> { _! ..T; }").denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        members: StructMembersP { contents: ref m, .. },
        ..
    }) => {
        should_have!(&m[0], IStructContent::VariadicStructMember {
            variability: VariabilityP::Varying,
            tyype: ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
            ..
        } if t.str == "T" => {});
    });
}

#[test]
fn struct_with_weak() {
    // Test: Struct with weak
    // Lines 101-105 in StructTests.scala
    
    let denizen = &compile_file_expect("struct Moo { x &&int; }").denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "Moo" => {
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            variability: VariabilityP::Final,
            tyype: ITemplexPT::Interpreted {
                maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Weak, .. }),
                inner: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                ..
            },
            ..
        } if n.str == "x" && t.str == "int" => {});
    });
}

#[test]
fn struct_with_heap() {
    // Test: Struct with heap
    // Lines 107-111 in StructTests.scala
    
    let denizen = &compile_file_expect("struct Moo { x ^Marine; }").denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "Moo" => {
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            variability: VariabilityP::Final,
            tyype: ITemplexPT::Interpreted {
                maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Own, .. }),
                inner: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                ..
            },
            ..
        } if n.str == "x" && t.str == "Marine" => {});
    });
}

#[test]
fn export_struct() {
    // Test: Export struct
    // Lines 113-117 in StructTests.scala
    
    let denizen = &compile_file_expect("exported struct Moo { x &int; }").denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        attributes: ref attrs,
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "Moo" => {
        assert_eq!(attrs.len(), 1);
        should_have!(&attrs[0], IAttributeP::ExportAttribute(_) => {});
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            tyype: ITemplexPT::Interpreted {
                maybe_ownership: Some(box OwnershipPT { ownership: OwnershipP::Borrow, .. }),
                inner: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                ..
            },
            ..
        } if n.str == "x" && t.str == "int" => {});
    });
}

#[test]
fn struct_with_rune() {
    // Test: Struct with rune
    // Lines 119-142 in StructTests.scala
    
    let denizen = &compile_file_expect(
        r#"
        struct ListNode<E> {
          value E;
          next ListNode<E>;
        }
        "#
    ).denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        identifying_runes: Some(GenericParametersP { params: ref p, .. }),
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "ListNode" => {
        assert_eq!(p.len(), 1);
        should_have!(&p[0], GenericParameterP { name: NameP { str: ref n, .. }, .. } if n.str == "E" => {});
        
        assert_eq!(m.len(), 2);
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            tyype: ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
            ..
        } if n.str == "value" && t.str == "E" => {});
        
        should_have!(&m[1], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            tyype: ITemplexPT::Call {
                template: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                args: ref a,
                ..
            },
            ..
        } if n.str == "next" && t.str == "ListNode" => {
            assert_eq!(a.len(), 1);
        });
    });
}

#[test]
fn struct_with_int_rune() {
    // Test: Struct with int rune
    // Lines 144-165 in StructTests.scala
    
    let denizen = &compile_file_expect(
        r#"
        struct Vecf<N> where N Int
        {
          values [#N]float;
        }
        "#
    ).denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        identifying_runes: Some(GenericParametersP { params: ref p, .. }),
        template_rules: Some(TemplateRulesP { rules: ref r, .. }),
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "Vecf" => {
        assert_eq!(p.len(), 1);
        should_have!(&p[0], GenericParameterP { name: NameP { str: ref n, .. }, .. } if n.str == "N" => {});
        
        assert_eq!(r.len(), 1);
        should_have!(&r[0], IRulexPR::Typed {
            rune: Some(NameP { str: ref n, .. }),
            tyype: ITypePR::IntType,
            ..
        } if n.str == "N" => {});
        
        assert_eq!(m.len(), 1);
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            tyype: ITemplexPT::StaticSizedArray {
                mutability: box ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. },
                variability: box ITemplexPT::Variability { variability: VariabilityP::Final, .. },
                size: box ITemplexPT::NameOrRune(NameP { str: ref sz, .. }),
                element: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                ..
            },
            ..
        } if n.str == "values" && sz.str == "N" && t.str == "float" => {});
    });
}

#[test]
fn struct_with_int_rune_array_sequence_specifies_mutability() {
    // Test: Struct with int rune, array sequence specifies mutability
    // Lines 167-196 in StructTests.scala
    
    let denizen = &compile_file_expect(
        r#"
        struct Vecf<N> where N Int
        {
          values [#N]float;
        }
        "#
    ).denizens[0];
    
    should_have!(denizen, IDenizenP::TopLevelStruct(StructP {
        name: NameP { str: ref s, .. },
        identifying_runes: Some(GenericParametersP { params: ref p, .. }),
        template_rules: Some(TemplateRulesP { rules: ref r, .. }),
        members: StructMembersP { contents: ref m, .. },
        ..
    }) if s.str == "Vecf" => {
        assert_eq!(p.len(), 1);
        should_have!(&p[0], GenericParameterP { name: NameP { str: ref n, .. }, .. } if n.str == "N" => {});
        
        assert_eq!(r.len(), 1);
        should_have!(&r[0], IRulexPR::Typed {
            rune: Some(NameP { str: ref n, .. }),
            tyype: ITypePR::IntType,
            ..
        } if n.str == "N" => {});
        
        assert_eq!(m.len(), 1);
        should_have!(&m[0], IStructContent::NormalStructMember {
            name: NameP { str: ref n, .. },
            tyype: ITemplexPT::StaticSizedArray {
                mutability: box ITemplexPT::Mutability { mutability: MutabilityP::Mutable, .. },
                variability: box ITemplexPT::Variability { variability: VariabilityP::Final, .. },
                size: box ITemplexPT::NameOrRune(NameP { str: ref sz, .. }),
                element: box ITemplexPT::NameOrRune(NameP { str: ref t, .. }),
                ..
            },
            ..
        } if n.str == "values" && sz.str == "N" && t.str == "float" => {});
    });
}

