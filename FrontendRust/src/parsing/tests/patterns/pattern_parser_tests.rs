/*
package dev.vale.parsing.patterns

import dev.vale.{Collector, Err, StrI, vimpl}
import dev.vale.parsing.{PatternParser, TestParseUtils}
import dev.vale.parsing.ast.{AbstractP, DestructureP, IgnoredLocalNameDeclarationP, LocalNameDeclarationP, NameOrRunePT, NameP, PatternPP, Patterns, TuplePT}
import dev.vale.parsing.ast.Patterns._
import dev.vale.parsing._
import dev.vale.parsing.ast._
import org.scalatest._

class PatternParserTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): PatternPP = {
    compilePattern(code)
//    compile(new PatternParser().parsePattern(_), code)
  }

  private def checkFail[T](code: String) = {
    vimpl()
//    compileForError(new PatternParser().parsePattern(_), code)
  }

  private def checkRest[T](code: String, expectedRest: String) = {
    vimpl()
//    compileForRest(new PatternParser().parsePattern(_), code, expectedRest)
  }
*/
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::{INameDeclarationP, ITemplexPT, PatternPP};
use crate::parsing::tests::utils::{
  assert_destination_local_name, assert_templex_name, compile_pattern_expect, expect_1, expect_2,
};

fn compile<'a, 'i, 'k>(
  interner: &'i Interner<'a>,
  keywords: &'k Keywords<'a>,
  code: &str,
) -> PatternPP<'a>
where
  'a: 'i,
  'a: 'k,
{
  compile_pattern_expect(interner, keywords, code)
}
#[test]
fn simple_int() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "_ int");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}
/*
  test("Simple Int") {
    // Make sure every pattern on the way down to kind can match Int
//    compile(Parser.parseTypeName(_),"int") shouldHave { case "int" => }
//    compile(runeOrKindPattern,"int") shouldHave { case NameOrRunePT(NameP(_, StrI("int"))) => }
//    compile(patternType,"int") shouldHave { case PatternTypePPI(None, NameOrRunePT(NameP(_, StrI("int")))) => }
    compile("_ int") shouldHave { case Patterns.fromEnv("int") => }
  }
*/
#[test]
fn name_only_capture() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a");
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(destination.mutate.is_none());
  assert!(pattern.templex.is_none());
  assert!(pattern.destructure.is_none());
}
/*
  test("Name-only Capture") {
    compile("a") match {
      case PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)), None, None) =>
    }
  }
*/
#[test]
fn empty_pattern() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "_");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert!(pattern.templex.is_none());
  assert!(pattern.destructure.is_none());
}
/*
  test("Empty pattern") {
    compile("_") match { case PatternPP(_, Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),None,None) => }
  }
*/
#[test]
fn capture_with_type_with_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a Moo[a, b]");
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Moo");
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
  test("Capture with type with destructure") {
    compile("a Moo[a, b]") shouldHave {
      case PatternPP(
          _,
          Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),
          Some(NameOrRunePT(NameP(_, StrI("Moo")))),
          Some(DestructureP(_,Vector(capture("a"),capture("b"))))) =>
    }
  }
*/
#[test]
fn cstodts() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "moo T[a int]");
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "moo");
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "T");
  let destructure = pattern.destructure.as_ref().unwrap();
  let a_pattern = expect_1(&destructure.patterns);
  let a_destination = a_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(a_destination, "a");
  assert!(a_destination.mutate.is_none());
  assert_templex_name(a_pattern.templex.as_ref().unwrap(), "int");
  assert!(a_pattern.destructure.is_none());
}
/*

  test("CSTODTS") {
    // This tests us handling an ambiguity properly, see CSTODTS in docs.
    compile("moo T[a int]") shouldHave {
      case PatternPP(
          _,
          Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("moo"))), None)),
          Some(NameOrRunePT(NameP(_, StrI("T")))),
          Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),Some(NameOrRunePT(NameP(_, StrI("int")))),None))))) =>
    }
  }
*/
#[test]
fn capture_with_destructure_with_type_outside() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a (int, bool)[a, b]");
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(destination.mutate.is_none());
  let tuple = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Tuple);
  let (int_t, bool_t) = expect_2(&tuple.elements);
  assert_templex_name(int_t, "int");
  assert_templex_name(bool_t, "bool");
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
  test("Capture with destructure with type outside") {
    compile("a (int, bool)[a, b]") shouldHave {
      case PatternPP(
          _,
          Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),
          Some(
            TuplePT(_,
                  Vector(
                    NameOrRunePT(NameP(_, StrI("int"))),
                    NameOrRunePT(NameP(_, StrI("bool")))))),
          Some(DestructureP(_,Vector(capture("a"), capture("b"))))) =>
    }
  }

}
*/
