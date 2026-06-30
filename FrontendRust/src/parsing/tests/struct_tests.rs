
use bumpalo::Bump;
use crate::cast;
use crate::interner::StrI;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn simple_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "struct Moo { }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [],
      sharedness: SharednessP::Single,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP { contents: [], .. },
      ..
    }) => {}
    _ => panic!("expected simple struct Moo shape"),
  }
}

#[test]
fn struct_with_list_node() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(
    &parse_arena,
    &keywords,
    "
      struct Mork {
        a @ListNode<T>;
      }
    ",
  );
  match expect_1(&struct_.members.contents) {
    IStructContent::NormalStructMember(NormalStructMemberP {
      name: NameP(_, StrI("a")),
      tyype: ITemplexPT::Interpreted(InterpretedPT {
        maybe_ownership: Some(OwnershipPT(_, OwnershipP::Share)),
        maybe_region: None,
        inner: ITemplexPT::Call(CallPT {
          template: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("ListNode")), .. }),
          args: [ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("T")), .. })],
          ..
        }),
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected struct Mork {{ a @ListNode<T>; }} member structure"),
  }
}

#[test]
fn share_struct_aliases_imm() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(
    &parse_arena,
    &keywords,
    "
      struct MyShared share { }
    ",
  );
  assert_eq!(struct_.sharedness, SharednessP::Shared);
}

#[test]
fn share_interface_aliases_imm() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      interface MyShared share { }
    ",
  );
  let interface = cast!(&denizen, IDenizenP::TopLevelInterface);
  assert_eq!(interface.sharedness, SharednessP::Shared);
}

#[test]
fn struct_with_rsa_member() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(
    &parse_arena,
    &keywords,
    "
      struct Mork {
        a []T;
      }
    ",
  );
  match expect_1(&struct_.members.contents) {
    IStructContent::NormalStructMember(NormalStructMemberP {
      name: NameP(_, StrI("a")),
      tyype: ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
        element: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("T")), .. }),
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected struct Mork {{ a []T; }} member structure"),
  }
}

#[test]
fn variadic_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(&parse_arena, &keywords, "struct Moo<T> { _ ..T; }");
  match expect_1(&struct_.members.contents) {
    IStructContent::VariadicStructMember(VariadicStructMemberP {
      tyype: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("T")), .. }),
      ..
    }) => {}
    _ => panic!("expected variadic struct Moo<T> {{ _ ..T; }} structure"),
  }
}

#[test]
fn struct_with_weak() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "struct Moo { x &&int; }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [],
      sharedness: SharednessP::Single,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("x")),
              tyype: ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(OwnershipPT(_, OwnershipP::Weak)),
            maybe_region: None,
            inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("int")), .. }),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Moo {{ x &&int; }} full structure"),
  }
}

#[test]
fn struct_with_heap() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "struct Moo { x ^Marine; }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [],
      sharedness: SharednessP::Single,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("x")),
              tyype: ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(OwnershipPT(_, OwnershipP::Own)),
            maybe_region: None,
            inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("Marine")), .. }),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Moo {{ x ^Marine; }} full structure"),
  }
}

#[test]
fn export_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "exported struct Moo { x &int; }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [IAttributeP::ExportAttribute(_)],
      sharedness: SharednessP::Single,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("x")),
              tyype: ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(OwnershipPT(_, OwnershipP::Borrow)),
            maybe_region: None,
            inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("int")), .. }),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected exported struct Moo {{ x &int; }} full structure"),
  }
}

#[test]
fn struct_with_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct ListNode<E> {
        value E;
        next ListNode<E>;
      }
    ",
  );
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("ListNode")),
      attributes: [],
      sharedness: SharednessP::Single,
      identifying_runes: Some(GenericParametersP {
        params: [GenericParameterP {
          name: NameP(_, StrI("E")),
          maybe_type: None,
          coord_region: None,
          attributes: [],
          maybe_default: None,
          ..
        }],
        ..
      }),
      template_rules: None,
      members: StructMembersP {
        contents: [
          IStructContent::NormalStructMember(NormalStructMemberP {
            name: NameP(_, StrI("value")),
                  tyype: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("E")), .. }),
            ..
          }),
          IStructContent::NormalStructMember(NormalStructMemberP {
            name: NameP(_, StrI("next")),
                  tyype: ITemplexPT::Call(CallPT {
              template: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("ListNode")), .. }),
              args: [ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("E")), .. })],
              ..
            }),
            ..
          }),
        ],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct ListNode<E> full structure"),
  }
}

#[test]
fn struct_with_int_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct Vecf<N> where N Int
      {
        values [#N]float;
      }
    ",
  );
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Vecf")),
      attributes: [],
      sharedness: SharednessP::Single,
      identifying_runes: Some(GenericParametersP {
        params: [GenericParameterP {
          name: NameP(_, StrI("N")),
          maybe_type: None,
          coord_region: None,
          attributes: [],
          maybe_default: None,
          ..
        }],
        ..
      }),
      template_rules: Some(TemplateRulesP {
        rules: [IRulexPR::Typed(TypedPR {
          rune: Some(NameP(_, StrI("N"))),
          tyype: ITypePR::IntType,
          ..
        })],
        ..
      }),
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("values")),
              tyype: ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            size: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("N")), .. }),
            element: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("float")), .. }),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Vecf<N> full structure"),
  }
}

#[test]
fn struct_with_int_rune_array_sequence_specifies_mutability() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct Vecf<N> where N Int
      {
        values [#N]float;
      }
    ",
  );
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Vecf")),
      attributes: [],
      sharedness: SharednessP::Single,
      identifying_runes: Some(GenericParametersP {
        params: [GenericParameterP {
          name: NameP(_, StrI("N")),
          maybe_type: None,
          coord_region: None,
          attributes: [],
          maybe_default: None,
          ..
        }],
        ..
      }),
      template_rules: Some(TemplateRulesP {
        rules: [IRulexPR::Typed(TypedPR {
          rune: Some(NameP(_, StrI("N"))),
          tyype: ITypePR::IntType,
          ..
        })],
        ..
      }),
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("values")),
              tyype: ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            size: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("N")), .. }),
            element: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("float")), .. }),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Vecf<N> full structure"),
  }
}
