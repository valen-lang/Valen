use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::parsing::ast::{
  BorrowRefPT, INameDeclarationP, ITemplexPT, NameOrRunePT, NameP, PatternPP, RegionP,
};
use crate::parsing::tests::utils::{
  assert_destination_local_name, assert_templex_name, compile_pattern_expect,
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
fn no_capture_with_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ int");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}

#[test]
fn capture_with_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "a int");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}

#[test]
fn simple_capture_with_tame() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "a T");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "T");
  assert!(pattern.destructure.is_none());
}

#[test]
fn capture_with_borrow_tame() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "arr &R");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "arr");
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Unspecified,
      inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("R")), .. }),
      ..
    }) => {}
    other => panic!("expected `&R` → BorrowRef(Unspecified, R), got {:?}", other),
  }
  assert!(pattern.destructure.is_none());
}

#[test]
fn capture_with_self_in_front() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "self.arr &&R");
  let destination = pattern.destination.as_ref().unwrap();
  match &destination.decl {
    INameDeclarationP::ConstructingMemberNameDeclaration(member_name) => {
      assert_eq!(member_name.as_str(), "arr");
    }
    other => panic!("expected `self.arr` → ConstructingMemberNameDeclaration, got {:?}", other),
  }
  assert!(destination.mutate.is_none());
  match pattern.templex.as_ref().unwrap() {
    ITemplexPT::BorrowRef(BorrowRefPT {
      region: RegionP::Unspecified,
      inner:
        ITemplexPT::BorrowRef(BorrowRefPT {
          region: RegionP::Unspecified,
          inner: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("R")), .. }),
          ..
        }),
      ..
    }) => {}
    other => {
      panic!("expected `&&R` → BorrowRef(Unspecified, BorrowRef(Unspecified, R)), got {:?}", other)
    }
  }
  assert!(pattern.destructure.is_none());
}
