use crate::cast;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::parsing::ast::{
  BorrowRefPT, GroupP, INameDeclarationP, ITemplexPT, NameOrRunePT, NameP, OwnRefPT, PatternPP,
  RegionP, SharednessP, WeakRefPT,
};
use crate::parsing::tests::utils::{
  assert_templex_name, compile_pattern_expect, expect_1, expect_2,
};
use bumpalo::Bump;

fn compile<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  code: &str,
) -> PatternPP<'p>
where
  'p: 'ctx,
{
  compile_pattern_expect(parse_arena, keywords, code)
}

#[test]
fn ignoring_name() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ int");
  let destination = pattern.destination.unwrap();
  assert!(matches!(destination.decl, INameDeclarationP::IgnoredLocalNameDeclaration(_)));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}

#[test]
fn runtime_sized_array() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ []int");
  let destination = pattern.destination.unwrap();
  assert!(matches!(destination.decl, INameDeclarationP::IgnoredLocalNameDeclaration(_)));
  assert!(destination.mutate.is_none());
  let rsa = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::RuntimeSizedArray);
  assert_templex_name(rsa.element, "int");
  assert!(pattern.destructure.is_none());
}

#[test]
fn sequence_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ (int, bool)");
  let destination = pattern.destination.unwrap();
  assert!(matches!(destination.decl, INameDeclarationP::IgnoredLocalNameDeclaration(_)));
  assert!(destination.mutate.is_none());
  let tuple = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Tuple);
  let (int_t, bool_t) = expect_2(&tuple.elements);
  assert_templex_name(int_t, "int");
  assert_templex_name(bool_t, "bool");
  assert!(pattern.destructure.is_none());
}

#[test]
fn caret_type_is_error() {
  // `^` is a value-level Move operator only; it's not a templex prefix.
  use crate::parsing::tests::utils::compile_templex;
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let result = compile_templex(&parse_arena, &keywords, "^T");
  assert!(result.is_err(), "expected `^T` at templex level to be a parse error");
}

#[test]
fn weak_prefix_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ weak T");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::WeakRef(WeakRefPT {
      inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("T")), .. }),
      ..
    }) => {}
    other => panic!("expected `weak T` → WeakRef(T), got {:?}", other),
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn own_prefix_type() {
  // `own T` parses as an OwnRef wrap around T.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ own T");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::OwnRef(OwnRefPT {
      inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("T")), .. }),
      ..
    }) => {}
    other => panic!("expected `own T` → OwnRef(T), got {:?}", other),
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn borrow_without_region() {
  // A bare `&MyStruct` has no group: its region parses to `Unspecified`. A borrow that names a
  // group with a trailing `in g` is covered by `borrow_with_group`.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Unspecified,
      inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("MyStruct")), .. }),
      ..
    }) => {}
    other => panic!("expected `&MyStruct` → BorrowRef(Unspecified, MyStruct), got {:?}", other),
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn borrow_with_group() {
  // A trailing `in g` on a borrow parses to a `Group` region naming the group g.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct in g");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Group(GroupP::Name(NameP(_, StrI("g")))),
      inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("MyStruct")), .. }),
      ..
    }) => {}
    other => {
      panic!("expected `&MyStruct in g` → BorrowRef(Group(Name g), MyStruct), got {:?}", other)
    }
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn borrow_with_element_group() {
  // A trailing `in g[]` parses to an `Elements` group over `g` — a reference into an element of g.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct in g[]");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Group(GroupP::Elements { base: GroupP::Name(NameP(_, StrI("g"))) }),
      ..
    }) => {}
    other => panic!("expected `&MyStruct in g[]` → Group(Elements(Name g)), got {:?}", other),
  }
}

#[test]
fn borrow_with_member_group() {
  // A trailing `in g.items` parses to a `Member` group naming member `items` of `g`.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct in g.items");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region:
        RegionP::Group(GroupP::Member {
          base: GroupP::Name(NameP(_, StrI("g"))),
          member: NameP(_, StrI("items")),
        }),
      ..
    }) => {}
    other => panic!("expected `&MyStruct in g.items` → Group(Member(Name g, items)), got {:?}", other),
  }
}

#[test]
fn borrow_with_member_element_group() {
  // `in g.items[]` parses to `Elements` over `Member(g, items)` — an element of g's `items`.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct in g.items[]");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region:
        RegionP::Group(GroupP::Elements {
          base: GroupP::Member { base: GroupP::Name(NameP(_, StrI("g"))), member: NameP(_, StrI("items")) },
        }),
      ..
    }) => {}
    other => panic!("expected `&MyStruct in g.items[]` → Group(Elements(Member(g, items))), got {:?}", other),
  }
}

#[test]
fn borrow_with_descendant_group() {
  // A trailing `in g...` parses to an `Ellipsis` group over `g` — a reference somewhere within g.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct in g...");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Group(GroupP::Ellipsis { base: GroupP::Name(NameP(_, StrI("g"))) }),
      ..
    }) => {}
    other => panic!("expected `&MyStruct in g...` → Group(Ellipsis(Name g)), got {:?}", other),
  }
}

#[test]
fn borrow_with_member_descendant_group() {
  // `in g.items...` parses to `Ellipsis` over `Member(g, items)` — somewhere within g's items.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &MyStruct in g.items...");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region:
        RegionP::Group(GroupP::Ellipsis {
          base: GroupP::Member { base: GroupP::Name(NameP(_, StrI("g"))), member: NameP(_, StrI("items")) },
        }),
      ..
    }) => {}
    other => {
      panic!("expected `&MyStruct in g.items...` → Group(Ellipsis(Member(g, items))), got {:?}", other)
    }
  }
}

#[test]
fn held_ref_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ held MyStruct");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Held,
      inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("MyStruct")), .. }),
      ..
    }) => {}
    other => panic!("expected `held MyStruct` → BorrowRef(Held, MyStruct), got {:?}", other),
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn held_and_borrow_ref_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ held &MyStruct");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Held,
      inner:
        ITemplexPT::BorrowRef(BorrowRefPT {
          region: RegionP::Unspecified,
          inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("MyStruct")), .. }),
          ..
        }),
      ..
    }) => {}
    other => panic!(
      "expected `held &MyStruct` → BorrowRef(Held, BorrowRef(Unspecified, MyStruct)), got {:?}",
      other
    ),
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn call_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ MyOption<MyList<int>>");
  let destination = pattern.destination.unwrap();
  assert!(matches!(destination.decl, INameDeclarationP::IgnoredLocalNameDeclaration(_)));
  assert!(destination.mutate.is_none());
  let myoption_call = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Call);
  assert_templex_name(myoption_call.template, "MyOption");
  let mylist_type = expect_1(&myoption_call.args);
  let mylist_call = cast!(mylist_type, ITemplexPT::Call);
  assert_templex_name(mylist_call.template, "MyList");
  let int_type = expect_1(&mylist_call.args);
  assert_templex_name(int_type, "int");
  assert!(pattern.destructure.is_none());
}
