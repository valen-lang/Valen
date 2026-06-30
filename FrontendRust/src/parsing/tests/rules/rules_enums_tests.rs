// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::rules::rules_enums_tests


use bumpalo::Bump;
use crate::cast;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

fn compile<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  code: &str,
) -> IRulexPR<'p>
where
  'p: 'ctx,
{
  compile_rulex_expect(parse_arena, keywords, code)
}

#[test]
fn ownership() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  {
    let rule = compile(&parse_arena, &keywords, "X");
    let templex = cast!(rule, IRulexPR::Templex);
    assert_templex_name(&templex, "X");
  }
  {
    let rule = compile(&parse_arena, &keywords, "X Ownership");
    let typed = cast!(rule, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
  }
  {
    let rule = compile(&parse_arena, &keywords, "X = own");
    let equals = cast!(rule, IRulexPR::Equals);
    assert_templex_name(cast!(equals.left, IRulexPR::Templex), "X");
    let ownership = cast!(cast!(equals.right, IRulexPR::Templex), ITemplexPT::Ownership);
    assert_eq!(ownership.1, OwnershipP::Own);
  }
  {
    let rule = compile(&parse_arena, &keywords, "X Ownership = any(own, borrow, weak)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
    let any_ = cast!(equals.right, IRulexPR::BuiltinCall);
    assert_eq!(any_.name.as_str(), "any");
    let (own_, borrow_, weak_) = expect_3(&any_.args);
    assert_eq!(
      cast!(cast!(own_, IRulexPR::Templex), ITemplexPT::Ownership).1,
      OwnershipP::Own
    );
    assert_eq!(
      cast!(cast!(borrow_, IRulexPR::Templex), ITemplexPT::Ownership).1,
      OwnershipP::Borrow
    );
    assert_eq!(
      cast!(cast!(weak_, IRulexPR::Templex), ITemplexPT::Ownership).1,
      OwnershipP::Weak
    );
  }
  {
    let rule = compile(&parse_arena, &keywords, "_ Ownership");
    let typed = cast!(rule, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
  }
  {
    let rule = compile(&parse_arena, &keywords, "own");
    let ownership = cast!(cast!(rule, IRulexPR::Templex), ITemplexPT::Ownership);
    assert_eq!(ownership.1, OwnershipP::Own);
  }
  {
    let rule = compile(&parse_arena, &keywords, "_ Ownership = any(own, share)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
    let any_ = cast!(equals.right, IRulexPR::BuiltinCall);
    assert_eq!(any_.name.as_str(), "any");
    let (own_, share_) = expect_2(&any_.args);
    assert_eq!(
      cast!(cast!(own_, IRulexPR::Templex), ITemplexPT::Ownership).1,
      OwnershipP::Own
    );
    assert_eq!(
      cast!(cast!(share_, IRulexPR::Templex), ITemplexPT::Ownership).1,
      OwnershipP::Share
    );
  }
}


#[test]
fn location() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  {
    let rule = compile(&parse_arena, &keywords, "X");
    let templex = cast!(rule, IRulexPR::Templex);
    assert_templex_name(&templex, "X");
  }
  {
    let rule = compile(&parse_arena, &keywords, "X Location");
    let typed = cast!(rule, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::LocationType);
  }
  {
    let rule = compile(&parse_arena, &keywords, "X = inl");
    let equals = cast!(rule, IRulexPR::Equals);
    assert_templex_name(cast!(equals.left, IRulexPR::Templex), "X");
    let location = cast!(cast!(equals.right, IRulexPR::Templex), ITemplexPT::Location);
    assert_eq!(location.location, LocationP::Inline);
  }
  {
    let rule = compile(&parse_arena, &keywords, "X Location = inl");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::LocationType);
    let location = cast!(cast!(equals.right, IRulexPR::Templex), ITemplexPT::Location);
    assert_eq!(location.location, LocationP::Inline);
  }
  {
    let rule = compile(&parse_arena, &keywords, "_ Location");
    let typed = cast!(rule, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::LocationType);
  }
  {
    let rule = compile(&parse_arena, &keywords, "inl");
    let location = cast!(cast!(rule, IRulexPR::Templex), ITemplexPT::Location);
    assert_eq!(location.location, LocationP::Inline);
  }
  {
    let rule = compile(&parse_arena, &keywords, "_ Location = any(inl, heap)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::LocationType);
    let any_ = cast!(equals.right, IRulexPR::BuiltinCall);
    assert_eq!(any_.name.as_str(), "any");
    let (inl_, heap_) = expect_2(&any_.args);
    assert_eq!(
      cast!(cast!(inl_, IRulexPR::Templex), ITemplexPT::Location).location,
      LocationP::Inline
    );
    assert_eq!(
      cast!(cast!(heap_, IRulexPR::Templex), ITemplexPT::Location).location,
      LocationP::Yonder
    );
  }
}
