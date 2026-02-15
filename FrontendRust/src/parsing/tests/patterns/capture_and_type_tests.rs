/*
package dev.vale.parsing.patterns

import dev.vale.{Collector, StrI}
import dev.vale.parsing.ast.{BorrowP, ConstructingMemberNameDeclarationP, InterpretedPT, LocalNameDeclarationP, NameOrRunePT, NameP, PatternPP, WeakP}
import dev.vale.parsing.ast.Patterns.{capturedWithType, capturedWithTypeRune}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.Collector
import org.scalatest._

class CaptureAndTypeTests extends FunSuite with Matchers with Collector with TestParseUtils {
//  private def compile[T](parser: CombinatorParsers.Parser[T], code: String): T = {
//    CombinatorParsers.parse(parser, code.toCharArray()) match {
//      case CombinatorParsers.NoSuccess(msg, input) => {
//        fail();
//      }
//      case CombinatorParsers.Success(expr, rest) => {
//        if (!rest.atEnd) {
//          vfail(rest.pos.longString)
//        }
//        expr
//      }
//    }
//  }
  private def compile[T](code: String): PatternPP = {
    compilePattern(code)
//    compile(new PatternParser().parsePattern(_), code)
  }
*/
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::{INameDeclarationP, ITemplexPT, OwnershipP, PatternPP};
use crate::parsing::tests::utils::{
  assert_destination_local_name, assert_templex_name, compile_pattern_expect,
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
#[test]
fn no_capture_with_type() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "_ int");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}
/*
  test("No capture, with type") {
    compile("_ int") shouldHave {
      case PatternPP(_, _, Some(NameOrRunePT(NameP(_, StrI("int")))), None) =>
    }
  }
*/
#[test]
fn capture_with_type() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a int");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}
/*
  test("Capture with type") {
    compile("a int") shouldHave {
      case capturedWithType("a", NameOrRunePT(NameP(_, StrI("int")))) =>
    }
  }
*/
#[test]
fn simple_capture_with_tame() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "a T");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "T");
  assert!(pattern.destructure.is_none());
}
/*
  test("Simple capture with tame") {
    compile("a T") shouldHave {
      case capturedWithTypeRune("a","T") =>
    }
  }
*/
#[test]
fn capture_with_borrow_tame() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "arr &R");
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "arr");
  let interpreted = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Borrow
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner.as_ref(), "R");
  assert!(pattern.destructure.is_none());
}
/*
  test("Capture with borrow tame") {
    compile("arr &R") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("arr"))), None)),
        Some(InterpretedPT(_,Some(OwnershipPT(_, BorrowP)), None, NameOrRunePT(NameP(_, StrI("R"))))),
        None) =>
    }
  }
*/
#[test]
fn capture_with_self_in_front() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let pattern = compile(&interner, &keywords, "self.arr &&R");
  let destination = pattern.destination.as_ref().unwrap();
  let member_name = cast!(
    &destination.decl,
    INameDeclarationP::ConstructingMemberNameDeclaration
  );
  assert_eq!(member_name.str.str, "arr");
  assert!(destination.mutate.is_none());
  let interpreted = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Weak
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner.as_ref(), "R");
  assert!(pattern.destructure.is_none());
}
/*
  test("Capture with self. in front") {
    compile("self.arr &&R") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(ConstructingMemberNameDeclarationP(NameP(_, StrI("arr"))), None)),
        Some(InterpretedPT(_,Some(OwnershipPT(_, WeakP)), None, NameOrRunePT(NameP(_, StrI("R"))))),
        None) =>
    }
  }
}
*/
