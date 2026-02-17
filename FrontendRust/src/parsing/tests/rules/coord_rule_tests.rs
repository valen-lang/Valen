// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::rules::coord_rule_tests

/*
package dev.vale.parsing.rules

import dev.vale.{Collector, Interner, StrI}
import dev.vale.parsing.ast.{AnonymousRunePT, ComponentsPR, CoordTypePR, EqualsPR, IRulexPR, KindTypePR, MutabilityPT, MutableP, NameOrRunePT, NameP, OwnP, OwnershipPT, TemplexPR, TuplePT, TypedPR}
import dev.vale.parsing.templex.TemplexParser
import dev.vale.parsing._
import dev.vale.parsing.ast.PatternPP
import org.scalatest._

class CoordRuleTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): IRulexPR = {
    compileRulex(code)
//    compile(new TemplexParser(interner).parseRule(_), code)
  }
*/
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

fn compile<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p bumpalo::Bump,
  code: &str,
) -> IRulexPR<'a, 'p>
where
  'a: 'ctx,
  'a: 'p,
{
  compile_rulex_expect(interner, keywords, arena, code)
}

#[test]
fn empty_coord_rule() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "_ Ref");
  let typed = cast!(rule, IRulexPR::Typed);
  assert!(typed.rune.is_none());
  assert_eq!(typed.tyype, ITypePR::CoordType);
}
/*
  test("Empty Coord rule") {
    // MIGALLOW: Not searching deeply
    compile("_ Ref") shouldHave {
      case TypedPR(_,None,CoordTypePR) =>
    }
  }
*/

#[test]
fn coord_with_rune() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "T Ref");
  let typed = cast!(rule, IRulexPR::Typed);
  assert_eq!(typed.rune.unwrap().as_str(), "T");
  assert_eq!(typed.tyype, ITypePR::CoordType);
}
/*
  test("Coord with rune") {
    // MIGALLOW: Not searching deeply
    compile("T Ref") shouldHave {
      case TypedPR(_,Some(NameP(_, StrI("T"))),CoordTypePR) =>
    }
  }
*/
#[test]
fn coord_with_destructure_only() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "Ref[_, _, _]");
  let components = cast!(rule, IRulexPR::Components);
  assert_eq!(components.container, ITypePR::CoordType);
  let (first, second, third) = expect_3(&components.components);
  assert!(matches!(cast!(first, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(second, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(third, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
}
/*
  test("Coord with destructure only") {
    // MIGALLOW: Not searching deeply
    compile("Ref[_, _, _]") shouldHave {
      case ComponentsPR(_,CoordTypePR,Vector(TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)))) =>
    }
  }
*/

#[test]
fn coord_with_rune_and_destructure() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "T = Ref[_, _, _]");
  let equals = cast!(rule, IRulexPR::Equals);
  assert_templex_name(cast!(equals.left.as_ref(), IRulexPR::Templex), "T");
  let components = cast!(equals.right.as_ref(), IRulexPR::Components);
  assert_eq!(components.container, ITypePR::CoordType);
  let (first, second, third) = expect_3(&components.components);
  assert!(matches!(cast!(first, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(second, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(third, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));

  let rule = compile(&interner, &keywords, &parse_arena, "T = Ref[own, _, _]");
  let equals = cast!(rule, IRulexPR::Equals);
  assert_templex_name(cast!(equals.left.as_ref(), IRulexPR::Templex), "T");
  let components = cast!(equals.right.as_ref(), IRulexPR::Components);
  assert_eq!(components.container, ITypePR::CoordType);
  let (first, second, third) = expect_3(&components.components);
  let first_templex = cast!(first, IRulexPR::Templex);
  let ownership = cast!(first_templex, ITemplexPT::Ownership);
  assert_eq!(ownership.ownership, OwnershipP::Own);
  assert!(matches!(cast!(second, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(third, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
}
/*
  test("Coord with rune and destructure") {
    // MIGALLOW: Not searching deeply
    compile("T = Ref[_, _, _]") shouldHave {
      case ComponentsPR(_,CoordTypePR,Vector(TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)))) =>
    }
    // MIGALLOW: Not searching deeply
    compile("T = Ref[own, _, _]") shouldHave {
        case ComponentsPR(_,
          CoordTypePR,
          Vector(TemplexPR(OwnershipPT(_,OwnP)), TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)))) =>
    }
  }
*/
#[test]
fn coord_matches_plain_int() {
  // Coord can do this because I want to be able to say:
  //   func moo
  //   where #T = (Int):Void
  //   (a: #T)
  // instead of:
  //   func moo
  //   rules(
  //     Ref#T[_, Ref[_, Int]]:Ref[_, Void]))
  //   (a: #T)
  // Note from later: I think this is an anachronism, this doesn't test
  // anything with coords.
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "int");
  let templex = cast!(rule, IRulexPR::Templex);
  assert_templex_name(&templex, "int");
}
/*
  test("Coord matches plain Int") {
    // Coord can do this because I want to be able to say:
    //   func moo
    //   where #T = (Int):Void
    //   (a: #T)
    // instead of:
    //   func moo
    //   rules(
    //     Ref#T[_, Ref[_, Int]]:Ref[_, Void]))
    //   (a: #T)
    // MIGALLOW: Different comment
    compile("int") shouldHave {
      case TemplexPR(NameOrRunePT(NameP(_, StrI("int")))) =>
    }
    // MIGALLOW: Not searching deeply
  }
*/
#[test]
fn coord_with_int_in_kind_rule() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "Ref[_, _, int]");
  let components = cast!(rule, IRulexPR::Components);
  assert_eq!(components.container, ITypePR::CoordType);
  let (first, second, third) = expect_3(&components.components);
  assert!(matches!(cast!(first, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(second, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert_templex_name(cast!(third, IRulexPR::Templex), "int");
}
/*
  test("Coord with Int in kind rule") {
    // MIGALLOW: Not searching deeply
    compile("Ref[_, _, int]") shouldHave {
      case ComponentsPR(_,
          CoordTypePR,
          Vector(TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)), TemplexPR(NameOrRunePT(NameP(_, StrI("int")))))) =>
    }
  }
*/

#[test]
fn coord_with_specific_kind_rule() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "Ref[_, _, Kind[mut]]");
  let components = cast!(rule, IRulexPR::Components);
  assert_eq!(components.container, ITypePR::CoordType);
  let (first, second, third) = expect_3(&components.components);
  assert!(matches!(cast!(first, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(second, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  let kind_components = cast!(third, IRulexPR::Components);
  assert_eq!(kind_components.container, ITypePR::KindType);
  let mutability_rule = expect_1(&kind_components.components);
  let mutability = cast!(cast!(mutability_rule, IRulexPR::Templex), ITemplexPT::Mutability);
  assert_eq!(mutability.mutability, MutabilityP::Mutable);
}
/*
  test("Coord with specific Kind rule") {
    // MIGALLOW: Not searching deeply
    compile("Ref[_, _, Kind[mut]]") shouldHave {
      case ComponentsPR(_,
          CoordTypePR,
          Vector(
            TemplexPR(AnonymousRunePT(_)),
            TemplexPR(AnonymousRunePT(_)),
            ComponentsPR(_,
              KindTypePR,Vector(TemplexPR(MutabilityPT(_,MutableP)))))) =>
    }
  }
*/
#[test]
fn coord_with_value() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "T Ref = int");
  let equals = cast!(rule, IRulexPR::Equals);
  let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
  assert_eq!(typed.rune.as_ref().unwrap().as_str(), "T");
  assert_eq!(typed.tyype, ITypePR::CoordType);
  assert_templex_name(cast!(equals.right.as_ref(), IRulexPR::Templex), "int");
}
/*
  test("Coord with value") {
    // MIGALLOW: Not searching deeply
    compile("T Ref = int") shouldHave {
      case EqualsPR(_,
          TypedPR(_,Some(NameP(_, StrI("T"))),CoordTypePR),
          TemplexPR(NameOrRunePT(NameP(_, StrI("int"))))) =>
    }
  }
*/
#[test]
fn coord_with_destructure_and_value() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "Ref[_, _, _] = int");
  let equals = cast!(rule, IRulexPR::Equals);
  let components = cast!(equals.left.as_ref(), IRulexPR::Components);
  assert_eq!(components.container, ITypePR::CoordType);
  let (first, second, third) = expect_3(&components.components);
  assert!(matches!(cast!(first, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(second, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert!(matches!(cast!(third, IRulexPR::Templex), ITemplexPT::AnonymousRune(_)));
  assert_templex_name(cast!(equals.right.as_ref(), IRulexPR::Templex), "int");
}
/*
  test("Coord with destructure and value") {
    // MIGALLOW: Not searching deeply
    compile("Ref[_, _, _] = int") shouldHave {
      case EqualsPR(_,
          ComponentsPR(_,CoordTypePR,Vector(TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)), TemplexPR(AnonymousRunePT(_)))),
          TemplexPR(NameOrRunePT(NameP(_, StrI("int"))))) =>
    }
  }
*/
#[test]
fn coord_with_sequence_in_value_spot() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "T Ref = (int, bool)");
  let equals = cast!(rule, IRulexPR::Equals);
  let typed = cast!(equals.left.as_ref(), IRulexPR::Typed);
  assert_eq!(typed.rune.as_ref().unwrap().as_str(), "T");
  assert_eq!(typed.tyype, ITypePR::CoordType);
  let tuple = cast!(cast!(equals.right.as_ref(), IRulexPR::Templex), ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
}
/*
  test("Coord with sequence in value spot") {
    // MIGALLOW: Not searching deeply
    compile("T Ref = (int, bool)") shouldHave {
      case EqualsPR(_,
          TypedPR(_,Some(NameP(_, StrI("T"))),CoordTypePR),
          TemplexPR(
            TuplePT(_,
              Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("bool"))))))) =>
    }
  }
*/
#[test]
fn lone_tuple_is_sequence() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let rule = compile(&interner, &keywords, &parse_arena, "(int, bool)");
  let tuple = cast!(cast!(&rule, IRulexPR::Templex), ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
}
/*
  test("Lone tuple is sequence") {
    // MIGALLOW: Not searching deeply
    compile("(int, bool)") shouldHave {
      case TemplexPR(
          TuplePT(_,
            Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("bool")))))) =>
        }
  }
}
*/
