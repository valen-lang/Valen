// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::patterns::destructure_parser_tests

/*
package dev.vale.parsing.patterns

import dev.vale.{Collector, StrI}
import dev.vale.parsing.ast.{DestinationLocalP, DestructureP, IgnoredLocalNameDeclarationP, LocalNameDeclarationP, NameOrRunePT, NameP, PatternPP}
import dev.vale.parsing.ast.Patterns._
import dev.vale.parsing._
import dev.vale.Collector
import org.scalatest._

class DestructureParserTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): PatternPP = {
    compilePattern(code)
//    compile(new PatternParser().parsePattern(_), code)
  }
*/
use crate::parsing::ast::{INameDeclarationP, PatternPP};
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::tests::utils::{
  assert_destination_local_name, assert_templex_name, compile_pattern_expect, expect_1, expect_2,
};

fn compile<'a, 'i, 'k>(
  interner: &'i Interner<'a>,
  keywords: &'k Keywords<'a>,
  code: &str,
) -> PatternPP<'a>
where
  'i: 'a,
  'k: 'a,
{
  compile_pattern_expect(interner, keywords, code)
}
/*
  test("Only empty destructure") {
    compile("[]") shouldHave {
      case withDestructure(Vector()) =>
    }
  }
*/
#[test]
fn only_empty_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  assert!(destructure.patterns.is_empty());
}

#[test]
fn one_element_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[a]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let a_pattern = expect_1(&destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
}
/*
  test("One element destructure") {
    compile("[a]") shouldHave {
      case withDestructure(Vector(capture("a"))) =>
    }
  }
*/
#[test]
fn one_typed_element_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[ _ A ]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let inner_pattern = expect_1(&destructure.patterns);
  assert!(matches!(
    inner_pattern
      .destination
      .as_ref()
      .map(|destination| &destination.decl),
    None | Some(INameDeclarationP::IgnoredLocalNameDeclaration(_))
  ));
  assert_templex_name(inner_pattern.templex.as_ref().unwrap(), "A");
  assert!(inner_pattern.destructure.is_none());
}
/*
  test("One typed element destructure") {
    compile("[ _ A ]") shouldHave {
      case withDestructure(Vector(withType(NameOrRunePT(NameP(_, StrI("A")))))) =>
    }
  }
*/
#[test]
fn only_two_element_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[a, b]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (a_pattern, b_pattern) = expect_2(&destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  assert_destination_local_name(b_pattern.destination.as_ref().unwrap(), "b");
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Only two-element destructure") {
    compile("[a, b]") shouldHave {
      case withDestructure(Vector(capture("a"), capture("b"))) =>
    }
  }
*/
#[test]
fn two_element_destructure_with_ignore() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[_, b]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (ignored_pattern, b_pattern) = expect_2(&destructure.patterns);
  let ignored_destination = ignored_pattern.destination.as_ref().unwrap();
  assert!(matches!(
    ignored_destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(ignored_destination.mutate.is_none());
  assert!(ignored_pattern.templex.is_none());
  assert!(ignored_pattern.destructure.is_none());
  assert_destination_local_name(b_pattern.destination.as_ref().unwrap(), "b");
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Two-element destructure with ignore") {
    compile("[_, b]") shouldHave {
      case PatternPP(_,
          None,None,
          Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)), None, None), capture("b"))))) =>
    }
  }
*/
#[test]
fn capture_with_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a [x, y]");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (x_pattern, y_pattern) = expect_2(&destructure.patterns);
  assert_destination_local_name(x_pattern.destination.as_ref().unwrap(), "x");
  assert!(x_pattern.templex.is_none());
  assert!(x_pattern.destructure.is_none());
  assert_destination_local_name(y_pattern.destination.as_ref().unwrap(), "y");
  assert!(y_pattern.templex.is_none());
  assert!(y_pattern.destructure.is_none());
}
/*
  test("Capture with destructure") {
    compile("a [x, y]") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),
        None,
        Some(DestructureP(_,Vector(capture("x"), capture("y"))))) =>
    }
  }
*/
#[test]
fn type_with_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "A[a, b]");
  assert!(pattern.destination.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "A");
  let destructure = pattern.destructure.as_ref().unwrap();
  let (a_pattern, b_pattern) = expect_2(&destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  assert_destination_local_name(b_pattern.destination.as_ref().unwrap(), "b");
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Type with destructure") {
    compile("A[a, b]") shouldHave {
      case PatternPP(_,
        None,
        Some(NameOrRunePT(NameP(_, StrI("A")))),
        Some(DestructureP(_,Vector(capture("a"), capture("b"))))) =>
    }
  }
*/
#[test]
fn capture_and_type_with_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a A[x, y]");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "A");
  let destructure = pattern.destructure.as_ref().unwrap();
  let (x_pattern, y_pattern) = expect_2(&destructure.patterns);
  assert_destination_local_name(x_pattern.destination.as_ref().unwrap(), "x");
  assert!(x_pattern.templex.is_none());
  assert!(x_pattern.destructure.is_none());
  assert_destination_local_name(y_pattern.destination.as_ref().unwrap(), "y");
  assert!(y_pattern.templex.is_none());
  assert!(y_pattern.destructure.is_none());
}
/*
  test("Capture and type with destructure") {
    compile("a A[x, y]") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),
        Some(NameOrRunePT(NameP(_, StrI("A")))),
        Some(DestructureP(_,Vector(capture("x"), capture("y"))))) =>
    }
  }
*/
#[test]
fn capture_with_types_inside() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a [_ int, _ bool]");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (int_pattern, bool_pattern) = expect_2(&destructure.patterns);
  assert!(matches!(
    int_pattern
      .destination
      .as_ref()
      .map(|destination| &destination.decl),
    None | Some(INameDeclarationP::IgnoredLocalNameDeclaration(_))
  ));
  assert_templex_name(int_pattern.templex.as_ref().unwrap(), "int");
  assert!(int_pattern.destructure.is_none());
  assert!(matches!(
    bool_pattern
      .destination
      .as_ref()
      .map(|destination| &destination.decl),
    None | Some(INameDeclarationP::IgnoredLocalNameDeclaration(_))
  ));
  assert_templex_name(bool_pattern.templex.as_ref().unwrap(), "bool");
  assert!(bool_pattern.destructure.is_none());
}
/*
  test("Capture with types inside") {
    compile("a [_ int, _ bool]") shouldHave {
      case PatternPP(_,
          Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),
          None,
          Some(DestructureP(_,Vector(fromEnv("int"), fromEnv("bool"))))) =>
    }
  }
*/
#[test]
fn destructure_with_type_inside() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[a int, b bool]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (a_pattern, b_pattern) = expect_2(&destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(a_pattern.templex.as_ref().unwrap(), "int");
  assert!(a_pattern.destructure.is_none());
  assert_destination_local_name(b_pattern.destination.as_ref().unwrap(), "b");
  assert_templex_name(b_pattern.templex.as_ref().unwrap(), "bool");
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Destructure with type inside") {
    compile("[a int, b bool]") shouldHave {
      case withDestructure(
      Vector(
          capturedWithType("a", NameOrRunePT(NameP(_, StrI("int")))),
          capturedWithType("b", NameOrRunePT(NameP(_, StrI("bool")))))) =>
    }
  }
*/
#[test]
fn nested_destructures_a() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[a, [b, c]]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (a_pattern, nested_pattern) = expect_2(&destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  assert!(nested_pattern.destination.is_none());
  assert!(nested_pattern.templex.is_none());
  let nested_destructure = nested_pattern.destructure.as_ref().unwrap();
  let (b_pattern, c_pattern) = expect_2(&nested_destructure.patterns);
  assert_destination_local_name(b_pattern.destination.as_ref().unwrap(), "b");
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
  assert_destination_local_name(c_pattern.destination.as_ref().unwrap(), "c");
  assert!(c_pattern.templex.is_none());
  assert!(c_pattern.destructure.is_none());
}
/*
  test("Nested destructures A") {
    compile("[a, [b, c]]") shouldHave {
      case withDestructure(
        Vector(
          capture("a"),
          withDestructure(
          Vector(
            capture("b"),
            capture("c"))))) =>
    }
  }
*/
#[test]
fn nested_destructures_b() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[[a], b]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (nested_pattern, b_pattern) = expect_2(&destructure.patterns);
  assert!(nested_pattern.destination.is_none());
  assert!(nested_pattern.templex.is_none());
  let nested_destructure = nested_pattern.destructure.as_ref().unwrap();
  let a_pattern = expect_1(&nested_destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  assert_destination_local_name(b_pattern.destination.as_ref().unwrap(), "b");
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
}
/*
  test("Nested destructures B") {
    compile("[[a], b]") shouldHave {
      case withDestructure(
      Vector(
          withDestructure(
          Vector(
            capture("a"))),
          capture("b"))) =>
    }
  }
*/
#[test]
fn nested_destructures_c() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "[[[a]]]");
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let outer_destructure = pattern.destructure.as_ref().unwrap();
  let middle_pattern = expect_1(&outer_destructure.patterns);
  assert!(middle_pattern.destination.is_none());
  assert!(middle_pattern.templex.is_none());
  let middle_destructure = middle_pattern.destructure.as_ref().unwrap();
  let inner_pattern = expect_1(&middle_destructure.patterns);
  assert!(inner_pattern.destination.is_none());
  assert!(inner_pattern.templex.is_none());
  let inner_destructure = inner_pattern.destructure.as_ref().unwrap();
  let a_pattern = expect_1(&inner_destructure.patterns);
  assert_destination_local_name(a_pattern.destination.as_ref().unwrap(), "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
}
/*
  test("Nested destructures C") {
    compile("[[[a]]]") shouldHave {
      case withDestructure(
      Vector(
          withDestructure(
          Vector(
            withDestructure(
            Vector(
              capture("a"))))))) =>
    }
  }
}
*/
