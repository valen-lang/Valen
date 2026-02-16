// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::rules::rules_enums_tests

/*
package dev.vale.parsing.rules

import dev.vale.{Collector, StrI, parsing}
import dev.vale.parsing.TestParseUtils
import dev.vale.parsing.ast.{BorrowP, BuiltinCallPR, EqualsPR, IRulexPR, ImmutableP, InlineP, LocationPT, LocationTypePR, MutabilityPT, MutabilityTypePR, MutableP, NameOrRunePT, NameP, OwnP, OwnershipPT, OwnershipTypePR, ShareP, TemplexPR, TypedPR, WeakP, YonderP}
import dev.vale.parsing.templex.TemplexParser
import dev.vale.parsing._
import dev.vale.parsing.ast._
import org.scalatest._

class RulesEnumsTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): IRulexPR = {
    compileRulex(code)
//    compile(new TemplexParser().parseRule(_), code)
  }
*/
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

fn compile<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IRulexPR<'a>
where
  'a: 'ctx,
{
  compile_rulex_expect(interner, keywords, code)
}

#[test]
fn ownership() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  {
    let rule = compile(&interner, &keywords, "X");
    let templex = cast!(rule, IRulexPR::Templex);
    assert_templex_name(&templex, "X");
  }
  {
    let rule = compile(&interner, &keywords, "X Ownership");
    let typed = cast!(rule, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
  }
  {
    let rule = compile(&interner, &keywords, "X = own");
    let equals = cast!(rule, IRulexPR::Equals);
    assert_templex_name(cast!(equals.left.as_ref(), IRulexPR::Templex), "X");
    let ownership = cast!(cast!(equals.right.as_ref(), IRulexPR::Templex), ITemplexPT::Ownership);
    assert_eq!(ownership.ownership, OwnershipP::Own);
  }
  {
    let rule = compile(&interner, &keywords, "X Ownership = any(own, borrow, weak)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
    let any_ = cast!(equals.right.as_ref(), IRulexPR::BuiltinCall);
    assert_eq!(any_.name.as_str(), "any");
    let (own_, borrow_, weak_) = expect_3(&any_.args);
    assert_eq!(
      cast!(cast!(own_, IRulexPR::Templex), ITemplexPT::Ownership).ownership,
      OwnershipP::Own
    );
    assert_eq!(
      cast!(cast!(borrow_, IRulexPR::Templex), ITemplexPT::Ownership).ownership,
      OwnershipP::Borrow
    );
    assert_eq!(
      cast!(cast!(weak_, IRulexPR::Templex), ITemplexPT::Ownership).ownership,
      OwnershipP::Weak
    );
  }
  {
    let rule = compile(&interner, &keywords, "_ Ownership");
    let typed = cast!(rule, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
  }
  {
    let rule = compile(&interner, &keywords, "own");
    let ownership = cast!(cast!(rule, IRulexPR::Templex), ITemplexPT::Ownership);
    assert_eq!(ownership.ownership, OwnershipP::Own);
  }
  {
    let rule = compile(&interner, &keywords, "_ Ownership = any(own, share)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::OwnershipType);
    let any_ = cast!(equals.right.as_ref(), IRulexPR::BuiltinCall);
    assert_eq!(any_.name.as_str(), "any");
    let (own_, share_) = expect_2(&any_.args);
    assert_eq!(
      cast!(cast!(own_, IRulexPR::Templex), ITemplexPT::Ownership).ownership,
      OwnershipP::Own
    );
    assert_eq!(
      cast!(cast!(share_, IRulexPR::Templex), ITemplexPT::Ownership).ownership,
      OwnershipP::Share
    );
  }
}
/*
  test("Ownership") {
    compile("X") shouldHave { case TemplexPR(NameOrRunePT(NameP(_, StrI("X")))) => }
    compile("X Ownership") shouldHave { case TypedPR(_,Some(NameP(_, StrI("X"))),OwnershipTypePR) => }
    compile("X = own") shouldHave { case EqualsPR(_,TemplexPR(NameOrRunePT(NameP(_, StrI("X")))),TemplexPR(OwnershipPT(_,OwnP))) => }
    compile("X Ownership = any(own, borrow, weak)") shouldHave {
      case EqualsPR(_,
          TypedPR(_,Some(NameP(_, StrI("X"))),OwnershipTypePR),
          BuiltinCallPR(_,NameP(_, StrI("any")),Vector(TemplexPR(OwnershipPT(_,OwnP)), TemplexPR(OwnershipPT(_,BorrowP)), TemplexPR(OwnershipPT(_,WeakP))))) =>
    }
    compile("_ Ownership") shouldHave { case TypedPR(_,None,OwnershipTypePR) => }
    compile("own") shouldHave { case TemplexPR(OwnershipPT(_,OwnP)) => }
    compile("_ Ownership = any(own, share)") shouldHave {
      case EqualsPR(_,
          TypedPR(_,None,OwnershipTypePR),
          BuiltinCallPR(_,NameP(_, StrI("any")),Vector(TemplexPR(OwnershipPT(_,OwnP)), TemplexPR(OwnershipPT(_,ShareP))))) =>
    }
  }
*/
#[test]
fn mutability() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  {
    let rule = compile(&interner, &keywords, "X");
    let templex = cast!(rule, IRulexPR::Templex);
    assert_templex_name(&templex, "X");
  }
  {
    let rule = compile(&interner, &keywords, "X Mutability");
    let typed = cast!(rule, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::MutabilityType);
  }
  {
    let rule = compile(&interner, &keywords, "X = mut");
    let equals = cast!(rule, IRulexPR::Equals);
    assert_templex_name(cast!(equals.left.as_ref(), IRulexPR::Templex), "X");
    let mutability = cast!(cast!(equals.right.as_ref(), IRulexPR::Templex), ITemplexPT::Mutability);
    assert_eq!(mutability.mutability, MutabilityP::Mutable);
  }
  {
    let rule = compile(&interner, &keywords, "X Mutability = mut");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::MutabilityType);
    let mutability = cast!(cast!(equals.right.as_ref(), IRulexPR::Templex), ITemplexPT::Mutability);
    assert_eq!(mutability.mutability, MutabilityP::Mutable);
  }
  {
    let rule = compile(&interner, &keywords, "_ Mutability");
    let typed = cast!(rule, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::MutabilityType);
  }
  {
    let rule = compile(&interner, &keywords, "mut");
    let mutability = cast!(cast!(rule, IRulexPR::Templex), ITemplexPT::Mutability);
    assert_eq!(mutability.mutability, MutabilityP::Mutable);
  }
  {
    let rule = compile(&interner, &keywords, "_ Mutability = any(mut, imm)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::MutabilityType);
    let any_ = cast!(equals.right.as_ref(), IRulexPR::BuiltinCall);
    assert_eq!(any_.name.as_str(), "any");
    let (mut_, imm_) = expect_2(&any_.args);
    assert_eq!(
      cast!(cast!(mut_, IRulexPR::Templex), ITemplexPT::Mutability).mutability,
      MutabilityP::Mutable
    );
    assert_eq!(
      cast!(cast!(imm_, IRulexPR::Templex), ITemplexPT::Mutability).mutability,
      MutabilityP::Immutable
    );
  }
}
/*
  test("Mutability") {
    compile("X") shouldHave { case TemplexPR(NameOrRunePT(NameP(_, StrI("X")))) => }
    compile("X Mutability") shouldHave { case TypedPR(_,Some(NameP(_, StrI("X"))),MutabilityTypePR) => }
    compile("X = mut") shouldHave { case EqualsPR(_,TemplexPR(NameOrRunePT(NameP(_, StrI("X")))),TemplexPR(MutabilityPT(_,MutableP))) => }
    compile("X Mutability = mut") shouldHave {
      case EqualsPR(_,
          TypedPR(_,Some(NameP(_, StrI("X"))),MutabilityTypePR),
          TemplexPR(MutabilityPT(_,MutableP))) =>
    }
    compile("_ Mutability") shouldHave { case TypedPR(_,None,MutabilityTypePR) => }
    compile("mut") shouldHave { case TemplexPR(MutabilityPT(_,MutableP)) => }
    compile("_ Mutability = any(mut, imm)") shouldHave {
      case EqualsPR(_,
          TypedPR(_,None,MutabilityTypePR),
          BuiltinCallPR(_,NameP(_, StrI("any")),Vector(TemplexPR(MutabilityPT(_,MutableP)), TemplexPR(MutabilityPT(_,ImmutableP))))) =>
    }
  }
*/
#[test]
fn location() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  {
    let rule = compile(&interner, &keywords, "X");
    let templex = cast!(rule, IRulexPR::Templex);
    assert_templex_name(&templex, "X");
  }
  {
    let rule = compile(&interner, &keywords, "X Location");
    let typed = cast!(rule, IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::LocationType);
  }
  {
    let rule = compile(&interner, &keywords, "X = inl");
    let equals = cast!(rule, IRulexPR::Equals);
    assert_templex_name(cast!(equals.left.as_ref(), IRulexPR::Templex), "X");
    let location = cast!(cast!(equals.right.as_ref(), IRulexPR::Templex), ITemplexPT::Location);
    assert_eq!(location.location, LocationP::Inline);
  }
  {
    let rule = compile(&interner, &keywords, "X Location = inl");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
    assert_eq!(typed.rune.as_ref().unwrap().as_str(), "X");
    assert_eq!(typed.tyype, ITypePR::LocationType);
    let location = cast!(cast!(equals.right.as_ref(), IRulexPR::Templex), ITemplexPT::Location);
    assert_eq!(location.location, LocationP::Inline);
  }
  {
    let rule = compile(&interner, &keywords, "_ Location");
    let typed = cast!(rule, IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::LocationType);
  }
  {
    let rule = compile(&interner, &keywords, "inl");
    let location = cast!(cast!(rule, IRulexPR::Templex), ITemplexPT::Location);
    assert_eq!(location.location, LocationP::Inline);
  }
  {
    let rule = compile(&interner, &keywords, "_ Location = any(inl, heap)");
    let equals = cast!(rule, IRulexPR::Equals);
    let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
    assert!(typed.rune.is_none());
    assert_eq!(typed.tyype, ITypePR::LocationType);
    let any_ = cast!(equals.right.as_ref(), IRulexPR::BuiltinCall);
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
/*
  test("Location") {
    compile("X") shouldHave { case TemplexPR(NameOrRunePT(NameP(_, StrI("X")))) => }
    compile("X Location") shouldHave { case TypedPR(_,Some(NameP(_, StrI("X"))),LocationTypePR) => }
    compile("X = inl") shouldHave { case EqualsPR(_,TemplexPR(NameOrRunePT(NameP(_, StrI("X")))),TemplexPR(LocationPT(_,InlineP))) => }
    compile("X Location = inl") shouldHave {
      case EqualsPR(_,
          TypedPR(_,Some(NameP(_, StrI("X"))),LocationTypePR),
          TemplexPR(LocationPT(_,InlineP))) =>
    }
    compile("_ Location") shouldHave { case TypedPR(_,None,LocationTypePR) => }
    compile("inl") shouldHave { case TemplexPR(LocationPT(_,InlineP)) => }
    compile("_ Location = any(inl, heap)") shouldHave {
      case EqualsPR(_,
          TypedPR(_,None,LocationTypePR),
          BuiltinCallPR(_,NameP(_, StrI("any")),Vector(TemplexPR(LocationPT(_,InlineP)), TemplexPR(LocationPT(_,YonderP))))) =>
    }
  }
}
*/