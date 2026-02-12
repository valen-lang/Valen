/*
package dev.vale.parsing.patterns

import dev.vale.{Collector, StrI, parsing}
import dev.vale.parsing.{PatternParser, TestParseUtils}
import dev.vale.parsing.ast.{AnonymousRunePT, BorrowP, CallPT, FinalP, IgnoredLocalNameDeclarationP, ImmutableP, IntPT, InterpretedPT, MutabilityPT, MutableP, NameOrRunePT, NameP, PatternPP, StaticSizedArrayPT, TuplePT, VariabilityPT, VaryingP, WeakP}
import dev.vale.parsing.ast.Patterns.{fromEnv, withType}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import org.scalatest._

class TypeTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): PatternPP = {
    compilePattern(code)
//    compile(new PatternParser().parsePattern(_), code)
  }
*/
use crate::cast;
use crate::parsing::ast::{
  INameDeclarationP, ITemplexPT, MutabilityP, OwnershipP, PatternPP, VariabilityP,
};
use crate::parsing::tests::utils::{
  assert_templex_name, compile_pattern_expect, expect_1, expect_2,
};

fn compile(code: &str) -> PatternPP {
  compile_pattern_expect(code)
}
#[test]
fn ignoring_name() {
  let pattern = compile("_ int");
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
  test("Ignoring name") {
    compile("_ int") shouldHave { case fromEnv("int") => }
  }

*/
#[test]
fn static_sized_array() {
  let pattern = compile("_ [#3]MutableStruct");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let ssa = cast!(
    pattern.templex.as_ref().unwrap(),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(ssa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(ssa.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_eq!(cast!(ssa.size.as_ref(), ITemplexPT::Int).value, 3);
  assert_templex_name(ssa.element.as_ref(), "MutableStruct");
  assert!(pattern.destructure.is_none());
}
/*
  test("15a") {
    compile("_ [#3]MutableStruct") shouldHave {
      case withType(
          StaticSizedArrayPT(_,
              MutabilityPT(_,MutableP),
              VariabilityPT(_,FinalP),
              IntPT(_,3),
              NameOrRunePT(NameP(_, StrI("MutableStruct"))))) =>
    }
  }

*/
#[test]
fn static_sized_array_with_imm() {
  let pattern = compile("_ [#3]<imm>MutableStruct");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let ssa = cast!(
    pattern.templex.as_ref().unwrap(),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(ssa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert_eq!(
    cast!(ssa.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_eq!(cast!(ssa.size.as_ref(), ITemplexPT::Int).value, 3);
  assert_templex_name(ssa.element.as_ref(), "MutableStruct");
  assert!(pattern.destructure.is_none());
}
/*
  test("15b") {
    compile("_ [#3]<imm>MutableStruct") shouldHave {
      case withType(
        StaticSizedArrayPT(_,
          MutabilityPT(_,ImmutableP),
          VariabilityPT(_,FinalP),
          IntPT(_,3),
          NameOrRunePT(NameP(_, StrI("MutableStruct"))))) =>
    }
  }

*/
#[test]
fn static_sized_array_with_imm_and_vary() {
  let pattern = compile("_ [#3]<imm, vary>MutableStruct");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let ssa = cast!(
    pattern.templex.as_ref().unwrap(),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(ssa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert_eq!(
    cast!(ssa.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Varying
  );
  assert_eq!(cast!(ssa.size.as_ref(), ITemplexPT::Int).value, 3);
  assert_templex_name(ssa.element.as_ref(), "MutableStruct");
  assert!(pattern.destructure.is_none());
}
/*
  test("15c") {
    compile("_ [#3]<imm, vary>MutableStruct") shouldHave {
      case withType(
      StaticSizedArrayPT(_,
      MutabilityPT(_,ImmutableP),
      VariabilityPT(_,VaryingP),
      IntPT(_,3),
      NameOrRunePT(NameP(_, StrI("MutableStruct"))))) =>
    }
  }

*/
#[test]
fn runtime_sized_array() {
  let pattern = compile("_ #[]int");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let rsa = cast!(
    pattern.templex.as_ref().unwrap(),
    ITemplexPT::RuntimeSizedArray
  );
  assert_eq!(
    cast!(rsa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert_templex_name(rsa.element.as_ref(), "int");
  assert!(pattern.destructure.is_none());
}
/*
  test("15d") {
    compile("_ #[]int") shouldHave {
      case withType(
        RuntimeSizedArrayPT(_,
          MutabilityPT(_,ImmutableP),
          NameOrRunePT(NameP(_, StrI("int"))))) =>
    }
  }



*/
#[test]
fn sequence_type() {
  let pattern = compile("_ (int, bool)");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let tuple = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Tuple);
  let (int_t, bool_t) = expect_2(&tuple.elements);
  assert_templex_name(int_t, "int");
  assert_templex_name(bool_t, "bool");
  assert!(pattern.destructure.is_none());
}
/*
  test("Sequence type") {
    compile("_ (int, bool)") shouldHave {
      case withType(
          TuplePT(_,
            Vector(
              NameOrRunePT(NameP(_, StrI("int"))),
              NameOrRunePT(NameP(_, StrI("bool")))))) =>
    }
  }
*/
#[test]
fn static_sized_array_with_borrow() {
  let pattern = compile("_ &[#3]MutableStruct");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let interpreted = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Borrow
  );
  assert!(interpreted.maybe_region.is_none());
  let ssa = cast!(interpreted.inner.as_ref(), ITemplexPT::StaticSizedArray);
  assert_eq!(
    cast!(ssa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(ssa.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_eq!(cast!(ssa.size.as_ref(), ITemplexPT::Int).value, 3);
  assert_templex_name(ssa.element.as_ref(), "MutableStruct");
  assert!(pattern.destructure.is_none());
}
/*
  test("15") {
    compile("_ &[#3]MutableStruct") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
        Some(
          InterpretedPT(_,
            Some(OwnershipPT(_, BorrowP)),
            None,
            StaticSizedArrayPT(_,
              MutabilityPT(_,MutableP),
              VariabilityPT(_,FinalP),
              IntPT(_,3),
              NameOrRunePT(NameP(_, StrI("MutableStruct")))))),
        None) =>
    }
  }
*/
#[test]
fn static_sized_array_with_weak() {
  let pattern = compile("_ &&[#3]<_, _>MutableStruct");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let interpreted = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Weak
  );
  assert!(interpreted.maybe_region.is_none());
  let ssa = cast!(interpreted.inner.as_ref(), ITemplexPT::StaticSizedArray);
  cast!(ssa.mutability.as_ref(), ITemplexPT::AnonymousRune);
  cast!(ssa.variability.as_ref(), ITemplexPT::AnonymousRune);
  assert_eq!(cast!(ssa.size.as_ref(), ITemplexPT::Int).value, 3);
  assert_templex_name(ssa.element.as_ref(), "MutableStruct");
  assert!(pattern.destructure.is_none());
}
/*
  test("15m") {
    compile("_ &&[#3]<_, _>MutableStruct") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
        Some(
          InterpretedPT(_,
            Some(OwnershipPT(_, WeakP)),
            None,
            StaticSizedArrayPT(_,
              AnonymousRunePT(_),
              AnonymousRunePT(_),
              IntPT(_,3),
              NameOrRunePT(NameP(_, StrI("MutableStruct")))))),
        None) =>
    }
  }
*/
#[test]
fn call_type() {
  let pattern = compile("_ MyOption<MyList<int>>");
  let destination = pattern.destination.as_ref().unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let myoption_call = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Call);
  assert_templex_name(myoption_call.template.as_ref(), "MyOption");
  let mylist_type = expect_1(&myoption_call.args);
  let mylist_call = cast!(mylist_type, ITemplexPT::Call);
  assert_templex_name(mylist_call.template.as_ref(), "MyList");
  let int_type = expect_1(&mylist_call.args);
  assert_templex_name(int_type, "int");
  assert!(pattern.destructure.is_none());
}
/*
  test("15z") {
    compile("_ MyOption<MyList<int>>") shouldHave {
      case PatternPP(_,
        Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)),
        Some(
          CallPT(
            _,
            NameOrRunePT(NameP(_, StrI("MyOption"))),
            Vector(
              CallPT(_,
                NameOrRunePT(NameP(_, StrI("MyList"))),
                Vector(
                  NameOrRunePT(NameP(_, StrI("int")))))))),
        None) =>
    }
  }
}
*/
