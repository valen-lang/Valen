/*
package dev.vale.parsing

import dev.vale.lexing.ImportL
import dev.vale.options.GlobalOptions
import dev.vale.{Collector, FileCoordinate, FileCoordinateMap, IPackageResolver, Interner, PackageCoordinate, StrI, vassert, vassertOne, vassertSome}
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
use crate::interner::StrI;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn simple_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "struct Moo { }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [],
      mutability: None,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP { contents: [], .. },
      ..
    }) => {}
    _ => panic!("expected simple struct Moo shape"),
  }
}
/*
  test("Simple struct") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
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
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(
    &parse_arena,
    &keywords,
    "
      struct Mork {
        a @ListNode<T>;
      }
    ",
  );
  match expect_1(&struct_.members.contents) {
    IStructContent::NormalStructMember(NormalStructMemberP {
      name: NameP(_, StrI("a")),
      variability: VariabilityP::Final,
      tyype: ITemplexPT::Interpreted(InterpretedPT {
        maybe_ownership: Some(OwnershipPT(_, OwnershipP::Share)),
        maybe_region: None,
        inner: ITemplexPT::Call(CallPT {
          template: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("ListNode")))),
          args: [ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("T"))))],
          ..
        }),
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected struct Mork {{ a @ListNode<T>; }} member structure"),
  }
}
/*
  test("17a") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
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
    }
  }
*/
#[test]
fn imm_generic_param() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct MyImmContainer<T Ref imm> imm { value T; }
    ",
  );
  let struct_ = cast!(&denizen, IDenizenP::TopLevelStruct);
  match expect_1(&struct_.identifying_runes.as_ref().unwrap().params) {
    GenericParameterP {
      attributes: [IRuneAttributeP::ImmutableRuneAttribute(_)],
      ..
    } => {}
    _ => panic!("expected exactly one imm generic param attribute"),
  }
  match expect_1(&struct_.members.contents) {
    IStructContent::NormalStructMember(NormalStructMemberP {
      name: NameP(_, StrI("value")),
      variability: VariabilityP::Final,
      tyype: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("T")))),
      ..
    }) => {}
    _ => panic!("expected struct MyImmContainer member structure"),
  }
}
/*
  test("Imm generic param") {
    val denizen =
      compileDenizenExpect(
          """
            |struct MyImmContainer<T Ref imm> imm { value T; }
            |""".stripMargin)

    // MIGALLOW: can use cast! here
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
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(
    &parse_arena,
    &keywords,
    "
      struct Mork {
        a []<imm>T;
      }
    ",
  );
  match expect_1(&struct_.members.contents) {
    IStructContent::NormalStructMember(NormalStructMemberP {
      name: NameP(_, StrI("a")),
      variability: VariabilityP::Final,
      tyype: ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
        mutability: ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Immutable)),
        element: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("T")))),
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected struct Mork {{ a []<imm>T; }} member structure"),
  }
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
      // MIGALLOW: UCMTRS, Rust can be stricter here
      case NormalStructMemberP(_,NameP(_, StrI("a")),FinalP,RuntimeSizedArrayPT(_,MutabilityPT(_,ImmutableP),NameOrRunePT(NameP(_, StrI("T"))))) =>
    }
  }
*/
#[test]
fn variadic_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(&parse_arena, &keywords, "struct Moo<T> { _ ..T; }");
  match expect_1(&struct_.members.contents) {
    IStructContent::VariadicStructMember(VariadicStructMemberP {
      variability: VariabilityP::Final,
      tyype: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("T")))),
      ..
    }) => {}
    _ => panic!("expected variadic struct Moo<T> {{ _ ..T; }} structure"),
  }
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
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let struct_ = compile_struct_expect(&parse_arena, &keywords, "struct Moo<T> { _! ..T; }");
  match expect_1(&struct_.members.contents) {
    IStructContent::VariadicStructMember(VariadicStructMemberP {
      variability: VariabilityP::Varying,
      tyype: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("T")))),
      ..
    }) => {}
    _ => panic!("expected variadic struct Moo<T> {{ _! ..T; }} structure"),
  }
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
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "struct Moo { x &&int; }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [],
      mutability: None,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("x")),
          variability: VariabilityP::Final,
          tyype: ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(OwnershipPT(_, OwnershipP::Weak)),
            maybe_region: None,
            inner: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("int")))),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Moo {{ x &&int; }} full structure"),
  }
}
/*
  test("Struct with weak") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
    vassertOne(compileFile("struct Moo { x &&int; }").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_, NameP(_, StrI("Moo")), Vector(), None, None, None, _, _, StructMembersP(_, Vector(NormalStructMemberP(_, NameP(_, StrI("x")), FinalP, InterpretedPT(_,Some(OwnershipPT(_, WeakP)),None,NameOrRunePT(NameP(_, StrI("int"))))))))) =>
    }
  }
*/
#[test]
fn struct_with_heap() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "struct Moo { x ^Marine; }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [],
      mutability: None,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("x")),
          variability: VariabilityP::Final,
          tyype: ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(OwnershipPT(_, OwnershipP::Own)),
            maybe_region: None,
            inner: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("Marine")))),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Moo {{ x ^Marine; }} full structure"),
  }
}
/*
  test("Struct with heap") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
    vassertOne(compileFile("struct Moo { x ^Marine; }").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_,NameP(_, StrI("Moo")),Vector(), None,None,None,_, _, StructMembersP(_,Vector(NormalStructMemberP(_,NameP(_, StrI("x")),FinalP,InterpretedPT(_,Some(OwnershipPT(_, OwnP)),None,NameOrRunePT(NameP(_, StrI("Marine"))))))))) =>
    }
  }
*/
#[test]
fn export_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "exported struct Moo { x &int; }");
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Moo")),
      attributes: [IAttributeP::ExportAttribute(_)],
      mutability: None,
      identifying_runes: None,
      template_rules: None,
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("x")),
          variability: VariabilityP::Final,
          tyype: ITemplexPT::Interpreted(InterpretedPT {
            maybe_ownership: Some(OwnershipPT(_, OwnershipP::Borrow)),
            maybe_region: None,
            inner: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("int")))),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected exported struct Moo {{ x &int; }} full structure"),
  }
}
/*
  test("Export struct") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
    vassertOne(compileFile("exported struct Moo { x &int; }").getOrDie().denizens) shouldHave {
      case TopLevelStructP(StructP(_, NameP(_, StrI("Moo")), Vector(ExportAttributeP(_)), None, None, None, _, _, StructMembersP(_, Vector(NormalStructMemberP(_, NameP(_, StrI("x")), FinalP, InterpretedPT(_,Some(OwnershipPT(_, BorrowP)),None,NameOrRunePT(NameP(_, StrI("int"))))))))) =>
    }
  }
*/
#[test]
fn struct_with_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct ListNode<E> {
        value E;
        next ListNode<E>;
      }
    ",
  );
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("ListNode")),
      attributes: [],
      mutability: None,
      identifying_runes: Some(GenericParametersP {
        params: [GenericParameterP {
          name: NameP(_, StrI("E")),
          maybe_type: None,
          coord_region: None,
          attributes: [],
          maybe_default: None,
          ..
        }],
        ..
      }),
      template_rules: None,
      members: StructMembersP {
        contents: [
          IStructContent::NormalStructMember(NormalStructMemberP {
            name: NameP(_, StrI("value")),
            variability: VariabilityP::Final,
            tyype: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("E")))),
            ..
          }),
          IStructContent::NormalStructMember(NormalStructMemberP {
            name: NameP(_, StrI("next")),
            variability: VariabilityP::Final,
            tyype: ITemplexPT::Call(CallPT {
              template: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("ListNode")))),
              args: [ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("E"))))],
              ..
            }),
            ..
          }),
        ],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct ListNode<E> full structure"),
  }
}
/*
  test("Struct with rune") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
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
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct Vecf<N> where N Int
      {
        values [#N]float;
      }
    ",
  );
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Vecf")),
      attributes: [],
      mutability: None,
      identifying_runes: Some(GenericParametersP {
        params: [GenericParameterP {
          name: NameP(_, StrI("N")),
          maybe_type: None,
          coord_region: None,
          attributes: [],
          maybe_default: None,
          ..
        }],
        ..
      }),
      template_rules: Some(TemplateRulesP {
        rules: [IRulexPR::Typed(TypedPR {
          rune: Some(NameP(_, StrI("N"))),
          tyype: ITypePR::IntType,
          ..
        })],
        ..
      }),
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("values")),
          variability: VariabilityP::Final,
          tyype: ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            mutability: ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable)),
            variability: ITemplexPT::Variability(VariabilityPT(_, VariabilityP::Final)),
            size: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("N")))),
            element: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("float")))),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Vecf<N> full structure"),
  }
}
/*
  test("Struct with int rune") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
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
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "
      struct Vecf<N> where N Int
      {
        values [#N]float;
      }
    ",
  );
  match denizen {
    IDenizenP::TopLevelStruct(StructP {
      name: NameP(_, StrI("Vecf")),
      attributes: [],
      mutability: None,
      identifying_runes: Some(GenericParametersP {
        params: [GenericParameterP {
          name: NameP(_, StrI("N")),
          maybe_type: None,
          coord_region: None,
          attributes: [],
          maybe_default: None,
          ..
        }],
        ..
      }),
      template_rules: Some(TemplateRulesP {
        rules: [IRulexPR::Typed(TypedPR {
          rune: Some(NameP(_, StrI("N"))),
          tyype: ITypePR::IntType,
          ..
        })],
        ..
      }),
      members: StructMembersP {
        contents: [IStructContent::NormalStructMember(NormalStructMemberP {
          name: NameP(_, StrI("values")),
          variability: VariabilityP::Final,
          tyype: ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
            mutability: ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable)),
            variability: ITemplexPT::Variability(VariabilityPT(_, VariabilityP::Final)),
            size: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("N")))),
            element: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("float")))),
            ..
          }),
          ..
        })],
        ..
      },
      ..
    }) => {}
    _ => panic!("expected struct Vecf<N> full structure"),
  }
}
/*
  test("Struct with int rune, array sequence specifies mutability") {
    // MIGALLOW: UCMTRS, Rust can be stricter here
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
  test("Struct with internal extern methods") {
    val denizen =
      vassertOne(
        compileFile(
          """
            |extern struct Vec<T> imm {
            |  extern func with_capacity(c i64) Vec<T>;
            |  extern func capacity(self &Vec<T>) i64;
            |}
            |""".stripMargin).getOrDie().denizens)
    val struct =
      denizen match {
        case TopLevelStructP(s) => s
      }
    val methods =
      struct.members.contents.collect { case StructMethodP(f) => f }
    vassert(methods.length == 2)
    methods(0).header.name match { case Some(NameP(_, StrI("with_capacity"))) => }
    vassertOne(methods(0).header.attributes.collect({ case ExternAttributeP(_) => }))
    methods(1).header.name match { case Some(NameP(_, StrI("capacity"))) => }
    vassertOne(methods(1).header.attributes.collect({ case ExternAttributeP(_) => }))
  }
}
*/