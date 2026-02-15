/*
package dev.vale.parsing

import dev.vale.lexing.ImportL
import dev.vale.options.GlobalOptions
import dev.vale.{Collector, FileCoordinate, FileCoordinateMap, IPackageResolver, Interner, PackageCoordinate, StrI, vassertOne, vassertSome}
import dev.vale.parsing.ast.{BorrowP, CallPT, ExportAttributeP, FinalP, GenericParameterP, GenericParametersP, ImmutableP, IntTypePR, InterpretedPT, MutabilityPT, MutableP, NameOrRunePT, NameP, NormalStructMemberP, OwnP, RuntimeSizedArrayPT, ShareP, StaticSizedArrayPT, StructMembersP, StructP, TemplateRulesP, TopLevelStructP, TypedPR, VariabilityPT, VariadicStructMemberP, VaryingP, WeakP}
import dev.vale.parsing.ast._
import org.scalatest._


class StructTests extends FunSuite with Collector with TestParseUtils {
//  private def compile[T](parser: CombinatorParsers.Parser[T], code: String): T = {
//    // The strip is in here because things inside the parser don't expect whitespace before and after
//    CombinatorParsers.parse(parser, code.strip().toCharArray()) match {
//      case CombinatorParsers.NoSuccess(msg, input) => {
//        fail("Couldn't parse!\n" + input.pos.longString);
//      }
//      case CombinatorParsers.Success(expr, rest) => {
//        vassert(rest.atEnd)
//        expr
//      }
//    }
//  }
*/
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn simple_struct() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(&interner, &keywords, "struct Moo { }");
  assert_eq!(struct_.name.str.str, "Moo");
  assert!(struct_.attributes.is_empty());
  assert!(struct_.mutability.is_none());
  assert!(struct_.identifying_runes.is_none());
  assert!(struct_.template_rules.is_none());
  assert!(struct_.maybe_default_region_rune.is_none());
  assert!(struct_.members.contents.is_empty());
}
/*
  test("Simple struct") {
    vassertOne(compileFile("""struct Moo { }""").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_,
      NameP(_, StrI("Moo")),
      Vector(),
      None,
      None,
      None,
      _,
      _,
      StructMembersP(_,
      Vector()))) =>
    }
  }
*/
#[test]
fn struct_with_list_node() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(
    &interner,
    &keywords,
    "
      struct Mork {
        a @ListNode<T>;
      }
    ",
  );
  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "a");
  assert_eq!(member.variability, VariabilityP::Final);
  let interpreted = cast!(&member.tyype, ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Share
  );
  assert!(interpreted.maybe_region.is_none());
  let listnode_call = cast!(interpreted.inner.as_ref(), ITemplexPT::Call);
  assert_templex_name(listnode_call.template.as_ref(), "ListNode");
  assert_templex_name(expect_1(&listnode_call.args), "T");
}
/*
  test("17a") {
    val denizen =
      vassertOne(
        compileFile(
          """
            |struct Mork {
            |  a @ListNode<T>;
            |}
            |""".stripMargin).getOrDie().denizens)
    denizen shouldHave {
      case NormalStructMemberP(_, NameP(_, StrI("a")), FinalP, InterpretedPT(_,Some(OwnershipPT(_, ShareP)),None,CallPT(_,NameOrRunePT(NameP(_, StrI("ListNode"))), Vector(NameOrRunePT(NameP(_, StrI("T"))))))) =>
//      case NormalStructMemberP(_,NameP(_,StrI(a)),final,InterpretedPT(_,share,CallPT(_,NameOrRunePT(NameP(_,StrI(ListNode))),Vector())))
    }
  }
*/
#[test]
fn imm_generic_param() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let denizen = compile_denizen_expect(
    &interner,
    &keywords,
    "
      struct MyImmContainer<T Ref imm> imm { value T; }
    ",
  );
  let struct_ = cast!(denizen, IDenizenP::TopLevelStruct);

  let generic_param = expect_1(&struct_.identifying_runes.as_ref().unwrap().params);
  let generic_param_attr = expect_1(&generic_param.attributes);
  cast!(generic_param_attr, IRuneAttributeP::ImmutableRuneAttribute);

  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "value");
  assert_eq!(member.variability, VariabilityP::Final);
  assert_templex_name(&member.tyype, "T");
}
/*
  test("Imm generic param") {
    val denizen =
      compileDenizenExpect(
          """
            |struct MyImmContainer<T Ref imm> imm { value T; }
            |""".stripMargin)

    val struct =
      denizen match {
        case TopLevelStructP(s) => s
      }

    vassertOne(vassertSome(struct.identifyingRunes).params).attributes match {
      case Vector(ImmutableRuneAttributeP(_)) =>
    }

    struct.members.contents match {
      case Vector(NormalStructMemberP(_,NameP(_,StrI("value")),FinalP,NameOrRunePT(NameP(_,StrI("T"))))) =>
    }
  }
*/
#[test]
fn struct_with_imm_generic_param() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(
    &interner,
    &keywords,
    "
      struct Mork {
        a []<imm>T;
      }
    ",
  );
  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "a");
  assert_eq!(member.variability, VariabilityP::Final);
  let rsa = cast!(&member.tyype, ITemplexPT::RuntimeSizedArray);
  assert_eq!(
    cast!(rsa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert_templex_name(rsa.element.as_ref(), "T");
}
/*
  test("18") {
    vassertOne(
      compileFile(
        """
          |struct Mork {
          |  a []<imm>T;
          |}
          |""".stripMargin).getOrDie().denizens) shouldHave {
      case NormalStructMemberP(_,NameP(_, StrI("a")),FinalP,RuntimeSizedArrayPT(_,MutabilityPT(_,ImmutableP),NameOrRunePT(NameP(_, StrI("T"))))) =>
    }
  }
*/
#[test]
fn variadic_struct() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(&interner, &keywords, "struct Moo<T> { _ ..T; }");
  let variadic =
    cast!(expect_1(&struct_.members.contents), IStructContent::VariadicStructMember);
  assert_eq!(variadic.variability, VariabilityP::Final);
  assert_templex_name(&variadic.tyype, "T");
}
/*
  test("Variadic struct") {
    Collector.only(
      vassertOne(compileFile("struct Moo<T> { _ ..T; }").getOrDie().denizens),
      {
        case StructMembersP(_, Vector(VariadicStructMemberP(_, FinalP, NameOrRunePT(NameP(_, StrI("T")))))) =>
      })
  }
*/
#[test]
fn variadic_struct_with_varying() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(&interner, &keywords, "struct Moo<T> { _! ..T; }");
  let variadic =
    cast!(expect_1(&struct_.members.contents), IStructContent::VariadicStructMember);
  assert_eq!(variadic.variability, VariabilityP::Varying);
  assert_templex_name(&variadic.tyype, "T");
}
/*
  test("Variadic struct with varying") {
    Collector.only(vassertOne(compileFile("struct Moo<T> { _! ..T; }").getOrDie().denizens), {
      case StructMembersP(_, Vector(VariadicStructMemberP(_, VaryingP, NameOrRunePT(NameP(_, StrI("T")))))) =>
    })
  }
*/
#[test]
fn struct_with_weak() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(&interner, &keywords, "struct Moo { x &&int; }");
  assert_eq!(struct_.name.str.str, "Moo");
  assert!(struct_.attributes.is_empty());
  assert!(struct_.mutability.is_none());
  assert!(struct_.identifying_runes.is_none());
  assert!(struct_.template_rules.is_none());
  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "x");
  assert_eq!(member.variability, VariabilityP::Final);
  let interpreted = cast!(&member.tyype, ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Weak
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner.as_ref(), "int");
}
/*
  test("Struct with weak") {
    vassertOne(compileFile("struct Moo { x &&int; }").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_, NameP(_, StrI("Moo")), Vector(), None, None, None, _, _, StructMembersP(_, Vector(NormalStructMemberP(_, NameP(_, StrI("x")), FinalP, InterpretedPT(_,Some(OwnershipPT(_, WeakP)),None,NameOrRunePT(NameP(_, StrI("int"))))))))) =>
    }
  }
*/
#[test]
fn struct_with_heap() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(&interner, &keywords, "struct Moo { x ^Marine; }");
  assert_eq!(struct_.name.str.str, "Moo");
  assert!(struct_.attributes.is_empty());
  assert!(struct_.mutability.is_none());
  assert!(struct_.identifying_runes.is_none());
  assert!(struct_.template_rules.is_none());
  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "x");
  assert_eq!(member.variability, VariabilityP::Final);
  let interpreted = cast!(&member.tyype, ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Own
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner.as_ref(), "Marine");
}
/*
  test("Struct with heap") {
    vassertOne(compileFile("struct Moo { x ^Marine; }").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_,NameP(_, StrI("Moo")),Vector(), None,None,None,_, _, StructMembersP(_,Vector(NormalStructMemberP(_,NameP(_, StrI("x")),FinalP,InterpretedPT(_,Some(OwnershipPT(_, OwnP)),None,NameOrRunePT(NameP(_, StrI("Marine"))))))))) =>
    }
  }
*/
#[test]
fn export_struct() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(&interner, &keywords, "exported struct Moo { x &int; }");
  assert_eq!(struct_.name.str.str, "Moo");
  let first_attr = expect_1(&struct_.attributes);
  cast!(first_attr, IAttributeP::ExportAttribute);
  assert!(struct_.mutability.is_none());
  assert!(struct_.identifying_runes.is_none());
  assert!(struct_.template_rules.is_none());
  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "x");
  assert_eq!(member.variability, VariabilityP::Final);
  let interpreted = cast!(&member.tyype, ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Borrow
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner.as_ref(), "int");
}
/*
  test("Export struct") {
    vassertOne(compileFile("exported struct Moo { x &int; }").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_, NameP(_, StrI("Moo")), Vector(ExportAttributeP(_)), None, None, None, _, _, StructMembersP(_, Vector(NormalStructMemberP(_, NameP(_, StrI("x")), FinalP, InterpretedPT(_,Some(OwnershipPT(_, BorrowP)),None,NameOrRunePT(NameP(_, StrI("int"))))))))) =>
    }
  }
*/
#[test]
fn struct_with_rune() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(
    &interner,
    &keywords,
    "
      struct ListNode<E> {
        value E;
        next ListNode<E>;
      }
    ",
  );
  assert_eq!(struct_.name.str.str, "ListNode");
  assert!(struct_.attributes.is_empty());
  assert!(struct_.mutability.is_none());
  let generic_param = expect_1(&struct_.identifying_runes.as_ref().unwrap().params);
  assert_eq!(generic_param.name.str.str, "E");
  assert!(generic_param.maybe_type.is_none());
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
  assert!(struct_.template_rules.is_none());

  let (value_member, next_member) = expect_2(&struct_.members.contents);
  let value_member = cast!(value_member, IStructContent::NormalStructMember);
  assert_eq!(value_member.name.str.str, "value");
  assert_eq!(value_member.variability, VariabilityP::Final);
  assert_templex_name(&value_member.tyype, "E");

  let next_member = cast!(next_member, IStructContent::NormalStructMember);
  assert_eq!(next_member.name.str.str, "next");
  assert_eq!(next_member.variability, VariabilityP::Final);
  let listnode_call = cast!(&next_member.tyype, ITemplexPT::Call);
  assert_templex_name(listnode_call.template.as_ref(), "ListNode");
  assert_templex_name(expect_1(&listnode_call.args), "E");
}
/*
  test("Struct with rune") {
    vassertOne(
      compileFile(
        """
          |struct ListNode<E> {
          |  value E;
          |  next ListNode<E>;
          |}
        """.stripMargin).getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(
        _,
        NameP(_, StrI("ListNode")),
        Vector(),
        None,
        Some(GenericParametersP(_, Vector(GenericParameterP(_, NameP(_, StrI("E")), _, _, Vector(), None)))),
        None,
        _,
        _,
        StructMembersP(_,
          Vector(
            NormalStructMemberP(_,NameP(_, StrI("value")),FinalP,NameOrRunePT(NameP(_, StrI("E")))),
            NormalStructMemberP(_,NameP(_, StrI("next")),FinalP,CallPT(_,NameOrRunePT(NameP(_, StrI("ListNode"))),Vector(NameOrRunePT(NameP(_, StrI("E")))))))))) =>
    }
  }
*/
#[test]
fn struct_with_int_rune() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(
    &interner,
    &keywords,
    "
      struct Vecf<N> where N Int
      {
        values [#N]float;
      }
    ",
  );
  assert_eq!(struct_.name.str.str, "Vecf");
  assert!(struct_.attributes.is_empty());
  assert!(struct_.mutability.is_none());
  let generic_param = expect_1(&struct_.identifying_runes.as_ref().unwrap().params);
  assert_eq!(generic_param.name.str.str, "N");
  assert!(generic_param.maybe_type.is_none());
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());

  let typed_rule = cast!(
    expect_1(&struct_.template_rules.as_ref().unwrap().rules),
    IRulexPR::Typed
  );
  assert_eq!(typed_rule.rune.as_ref().unwrap().str.str, "N");
  assert_eq!(typed_rule.tyype, ITypePR::IntType);

  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "values");
  assert_eq!(member.variability, VariabilityP::Final);
  let ssa = cast!(&member.tyype, ITemplexPT::StaticSizedArray);
  assert_eq!(
    cast!(ssa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(ssa.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_templex_name(ssa.size.as_ref(), "N");
  assert_templex_name(ssa.element.as_ref(), "float");
}
/*
  test("Struct with int rune") {
    vassertOne(
      compileFile(
        """
          |struct Vecf<N> where N Int
          |{
          |  values [#N]float;
          |}
          |
      """.stripMargin).getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(
        _,
        NameP(_, StrI("Vecf")),
        Vector(),
        None,
        Some(GenericParametersP(_, Vector(GenericParameterP(_, NameP(_, StrI("N")), _, _, Vector(), None)))),
        Some(TemplateRulesP(_, Vector(TypedPR(_,Some(NameP(_, StrI("N"))), IntTypePR)))),
        _,
        _,
        StructMembersP(_, Vector(NormalStructMemberP(_,NameP(_, StrI("values")), FinalP, StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_,FinalP), NameOrRunePT(NameP(_, StrI("N"))), NameOrRunePT(NameP(_, StrI("float"))))))))) =>
    }
  }
*/
#[test]
fn struct_with_int_rune_array_sequence_specifies_mutability() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let struct_ = compile_struct_expect(
    &interner,
    &keywords,
    "
      struct Vecf<N> where N Int
      {
        values [#N]float;
      }
    ",
  );
  assert_eq!(struct_.name.str.str, "Vecf");
  assert!(struct_.attributes.is_empty());
  assert!(struct_.mutability.is_none());
  let generic_param = expect_1(&struct_.identifying_runes.as_ref().unwrap().params);
  assert_eq!(generic_param.name.str.str, "N");
  assert!(generic_param.maybe_type.is_none());
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());

  let typed_rule = cast!(
    expect_1(&struct_.template_rules.as_ref().unwrap().rules),
    IRulexPR::Typed
  );
  assert_eq!(typed_rule.rune.as_ref().unwrap().str.str, "N");
  assert_eq!(typed_rule.tyype, ITypePR::IntType);

  let member = cast!(expect_1(&struct_.members.contents), IStructContent::NormalStructMember);
  assert_eq!(member.name.str.str, "values");
  assert_eq!(member.variability, VariabilityP::Final);
  let ssa = cast!(&member.tyype, ITemplexPT::StaticSizedArray);
  assert_eq!(
    cast!(ssa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(ssa.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_templex_name(ssa.size.as_ref(), "N");
  assert_templex_name(ssa.element.as_ref(), "float");
}
/*
  test("Struct with int rune, array sequence specifies mutability") {
    vassertOne(
      compileFile(
        """
          |struct Vecf<N> where N Int
          |{
          |  values [#N]float;
          |}
      """.stripMargin).getOrDie().denizens) shouldHave {
      case TopLevelStructP(
          StructP(
            _,
            NameP(_, StrI("Vecf")),
            Vector(),
            None,
            Some(GenericParametersP(_, Vector(GenericParameterP(_, NameP(_, StrI("N")), _, _, Vector(), None)))),
            Some(TemplateRulesP(_, Vector(TypedPR(_,Some(NameP(_, StrI("N"))),IntTypePR)))),
            _,
            _,
            StructMembersP(_, Vector(NormalStructMemberP(_,NameP(_, StrI("values")),FinalP,StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_, FinalP), NameOrRunePT(NameP(_, StrI("N"))), NameOrRunePT(NameP(_, StrI("float"))))))))) =>
//      case TopLevelStructP(
//        StructP(_,
//          NameP(_,StrI("Vecf")),
//          Vector(),
//          None,
//          Some(IdentifyingRunesP(_,Vector(IdentifyingRuneP(_,NameP(_,StrI("N")),Vector())))),
//          Some(TemplateRulesP(_,Vector(TypedPR(_,Some(NameP(_,StrI("N"))),IntTypePR)))),
//          StructMembersP(_,Vector(NormalStructMemberP(_,NameP(_,StrI("values")),FinalP,StaticSizedArrayPT(_,MutabilityPT(_,mut),VariabilityPT(_,final),NameOrRunePT(NameP(_,StrI(N))),NameOrRunePT(NameP(_,StrI(float)))))))))
    }
  }
}
*/