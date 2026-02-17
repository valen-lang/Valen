// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::patterns::type_and_destructure_tests

/*
package dev.vale.parsing.patterns

import dev.vale.{Collector, StrI}
import dev.vale.parsing.ast.{CallPT, DestinationLocalP, DestructureP, IgnoredLocalNameDeclarationP, LocalNameDeclarationP, NameOrRunePT, NameP, PatternPP, TuplePT}
import dev.vale.parsing.ast.Patterns._
import dev.vale.parsing._
import dev.vale.Collector
import org.scalatest._

class TypeAndDestructureTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): PatternPP = {
    compilePattern(code)
//    compile(new PatternParser().parsePattern(_), code)
  }
*/
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::{INameDeclarationP, ITemplexPT, PatternPP};
use crate::parsing::tests::utils::{
  assert_destination_local_name, assert_templex_name, compile_pattern_expect, expect_1, expect_2,
};

fn compile<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p bumpalo::Bump,
  code: &str,
) -> PatternPP<'a, 'p>
where
  'a: 'ctx,
  'a: 'p,
{
  compile_pattern_expect(interner, keywords, arena, code)
}
#[test]
fn empty_destructure() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta[]");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Muta");
  let destructure = pattern.destructure.as_ref().unwrap();
  assert!(destructure.patterns.is_empty());
}
/*
  test("Empty destructure") {
    compile("_ Muta[]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(NameOrRunePT(NameP(_, StrI("Muta")))),
          Some(DestructureP(_,Vector()))) =>
    }
  }
*/
#[test]
fn templated_destructure() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  {
    let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta<int>[]");
    let destination = pattern.destination.as_ref().unwrap();
    assert!(matches!(
      destination.decl,
      INameDeclarationP::IgnoredLocalNameDeclaration(_)
    ));
    assert!(destination.mutate.is_none());
    let call = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Call);
    assert_templex_name(call.template.as_ref(), "Muta");
    let first_arg = expect_1(&call.args);
    assert_templex_name(first_arg, "int");
    let destructure = pattern.destructure.as_ref().unwrap();
    assert!(destructure.patterns.is_empty());
  }

  {
    let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta<R>[]");
    let destination = pattern.destination.as_ref().unwrap();
    assert!(matches!(
      destination.decl,
      INameDeclarationP::IgnoredLocalNameDeclaration(_)
    ));
    assert!(destination.mutate.is_none());
    let call = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Call);
    assert_templex_name(call.template.as_ref(), "Muta");
    let first_arg = expect_1(&call.args);
    assert_templex_name(first_arg, "R");
    let destructure = pattern.destructure.as_ref().unwrap();
    assert!(destructure.patterns.is_empty());
  }
}
/*
  test("Templated destructure") {
    compile("_ Muta<int>[]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(
            CallPT(_,
              NameOrRunePT(NameP(_, StrI("Muta"))),
              Vector(NameOrRunePT(NameP(_, StrI("int")))))),
          Some(DestructureP(_,Vector()))) =>
    }
    compile("_ Muta<R>[]") shouldHave {
        case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(
            CallPT(_,
              NameOrRunePT(NameP(_, StrI("Muta"))),
              Vector(NameOrRunePT(NameP(_, StrI("R")))))),
          Some(DestructureP(_,Vector()))) =>
    }
  }

*/
#[test]
fn destructure_with_type_outside() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let pattern = compile(&interner, &keywords, &parse_arena, "_ (int, bool)[a, b]");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let tuple = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
  let destructure = pattern.destructure.as_ref().unwrap();
  let (a_pattern, b_pattern) = expect_2(&destructure.patterns);
  let a_destination = a_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(a_destination, "a");
  assert!(a_destination.mutate.is_none());
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  let b_destination = b_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(b_destination, "b");
  assert!(b_destination.mutate.is_none());
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Destructure with type outside") {
    compile("_ (int, bool)[a, b]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(
            TuplePT(_,
                Vector(
                  NameOrRunePT(NameP(_, StrI("int"))),
                  NameOrRunePT(NameP(_, StrI("bool")))))),
          Some(DestructureP(_,Vector(capture("a"), capture("b"))))) =>
    }
  }
*/
#[test]
fn destructure_with_typeless_capture() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta[b]");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Muta");
  let destructure = pattern.destructure.as_ref().unwrap();
  let b_pattern = expect_1(&destructure.patterns);
  let b_destination = b_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(b_destination, "b");
  assert!(b_destination.mutate.is_none());
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Destructure with typeless capture") {
    compile("_ Muta[b]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(NameOrRunePT(NameP(_, StrI("Muta")))),
          Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("b"))), None)),None,None))))) =>
    }
  }
*/
#[test]
fn destructure_with_typed_capture() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta[b Marine]");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Muta");
  let destructure = pattern.destructure.as_ref().unwrap();
  let b_pattern = expect_1(&destructure.patterns);
  let b_destination = b_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(b_destination, "b");
  assert!(b_destination.mutate.is_none());
  assert_templex_name(b_pattern.templex.as_ref().unwrap(), "Marine");
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Destructure with typed capture") {
    compile("_ Muta[b Marine]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(NameOrRunePT(NameP(_, StrI("Muta")))),
          Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("b"))), None)),Some(NameOrRunePT(NameP(_, StrI("Marine")))),None))))) =>
    }
  }
*/
#[test]
fn destructure_with_unnamed_capture() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta[_ Marine]");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Muta");
  let destructure = pattern.destructure.as_ref().unwrap();
  let unnamed_pattern = expect_1(&destructure.patterns);
  let unnamed_destination = unnamed_pattern.destination.as_ref().unwrap();
  assert!(matches!(
    unnamed_destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(unnamed_destination.mutate.is_none());
  assert_templex_name(unnamed_pattern.templex.as_ref().unwrap(), "Marine");
  assert!(unnamed_pattern.destructure.is_none());
}
/*
  test("Destructure with unnamed capture") {
    compile("_ Muta[_ Marine]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(NameOrRunePT(NameP(_, StrI("Muta")))),
          Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),Some(NameOrRunePT(NameP(_, StrI("Marine")))),None))))) =>
    }
  }
*/
#[test]
fn destructure_with_runed_capture() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let parse_arena = Bump::new();
  let pattern = compile(&interner, &keywords, &parse_arena, "_ Muta[_ R]");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Muta");
  let destructure = pattern.destructure.as_ref().unwrap();
  let unnamed_pattern = expect_1(&destructure.patterns);
  let unnamed_destination = unnamed_pattern.destination.as_ref().unwrap();
  assert!(matches!(
    unnamed_destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(unnamed_destination.mutate.is_none());
  assert_templex_name(unnamed_pattern.templex.as_ref().unwrap(), "R");
  assert!(unnamed_pattern.destructure.is_none());
}
/*
  test("Destructure with runed capture") {
    compile("_ Muta[_ R]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
          Some(NameOrRunePT(NameP(_, StrI("Muta")))),
          Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),Some(NameOrRunePT(NameP(_, StrI("R")))),None))))) =>
        }
  }
}
*/
