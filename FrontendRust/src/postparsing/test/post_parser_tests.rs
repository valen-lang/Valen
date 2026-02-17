// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib postparsing::test::post_parser_tests

use bumpalo::Bump;
use crate::cast;
use crate::compile_options::GlobalOptions;
use crate::{Interner, Keywords};
use crate::parsing::ast::{IMacroInclusionP, LoadAsP, VariabilityP};
use crate::postparsing::ast::{IStructMemberS, ProgramS};
use crate::postparsing::expressions::{
  DotSE, FunctionCallSE, IExpressionSE, IVariableUseCertainty, LocalLoadSE, OutsideLoadSE,
  OwnershippedSE, ReturnSE,
};
use crate::postparsing::names::{IFunctionDeclarationNameS, IImpreciseNameS, IVarNameS};
use crate::postparsing::post_parser::{ICompileErrorS, PostParser};
use crate::postparsing::rules::rules::{ILiteralSL, LiteralSR, MaybeCoercingLookupSR};
use crate::postparsing::test::traverse::NodeRefS;
use crate::parsing::tests::utils::compile_file;
use crate::parsing::tests::utils::{expect_1, expect_2, expect_3};

/*
package dev.vale.postparsing

import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{FinalP, LoadAsBorrowP, MutableP, UseP}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules.{LiteralSR, MaybeCoercingLookupSR, MutabilityLiteralSL, RuneUsage}
import dev.vale.solver.IncompleteSolve
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._
import dev.vale.solver.IncompleteSolve
import org.scalatest._

class PostParserTests extends FunSuite with Matchers with Collector {
*/
fn compile<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p Bump,
  code: &str,
) -> ProgramS<'a, 'p>
where
  'a: 'ctx,
  'a: 'p,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let only_file = compile_file(interner, keywords, arena, code).unwrap();
  let post_parser = PostParser::new(options, interner, keywords, arena);
  post_parser
    .scout_program(only_file.file_coord, &only_file)
    .unwrap()
}

/*
  private def compile(code: String, interner: Interner = new Interner()): ProgramS = {
    val compile = PostParserTestCompilation.test(code, interner)
    compile.getScoutput() match {
      case Err(e) => {
        val codeMap = compile.getCodeMap().getOrDie()
        vfail(
          PostParserErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e))
      }
      case Ok(t) => t.expectOne()
    }
  }
*/
fn compile_for_error<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p Bump,
  code: &str,
) -> ICompileErrorS<'a>
where
  'a: 'ctx,
  'a: 'p,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let only_file = compile_file(interner, keywords, arena, code).unwrap();
  let post_parser = PostParser::new(options, interner, keywords, arena);
  match post_parser.scout_program(only_file.file_coord, &only_file) {
    Ok(_) => panic!("Accidentally compiled!"),
    Err(e) => e,
  }
}
/*
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Accidentally compiled!\n")
    }
  }
*/
/*
  vregionmut() // Put this back in with regions
  // test("Every function gets region generic param") {
  //   val program1 = compile("func moo() int { 3 }")
  //
  //   val moo = program1.lookupFunction("moo")
  //   moo.genericParams match {
  //     case Vector(
  //       GenericParameterS(_,
  //         RuneUsage(_,DenizenDefaultRegionRuneS(_)),
  //         RegionGenericParameterTypeS(ReadWriteRegionS),
  //         None)) =>
  //   }
  // }
*/
/*
  // See: User Must Specify Enough Identifying Runes (UMSEIR)
  test("Test UMSEIR") {
    // This should work, its fine that the _ is there because we can always figure out what
    // that rune is, from the identifying runes.
    val main =
    compile(
      """
        |func moo<T>(a T)
        |where K Ref, T = Map<K, _> { ... }
        |""".stripMargin).lookupFunction("moo")

    // This should fail, because we can't figure out what it is, given the identifying runes.
    val error = compileForError(
      """
        |func moo<K, V>(a Map<K, V, _>) { ... }
        |""".stripMargin)
    error match {
      case IdentifyingRunesIncompleteS(_, IdentifiabilitySolveError(_, IncompleteSolve(_, _,runes, _))) => {
        // The param rune, and the _ rune are both unknown
        vassert(runes.size == 2)
      }
    }
  }
*/
#[test]
fn lookup_plus() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "exported func main() int { return +(3, 4); }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, crate::postparsing::ast::IBodyS::CodeBody);
  match code_body.body.block.expr {
    IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OutsideLoad(OutsideLoadSE {
              name: IImpreciseNameS::CodeName(code_name),
              ..
            }),
          ..
        }),
      ..
    }) => assert_eq!(code_name.name.as_str(), "+"),
    _ => panic!("expected return +(3, 4) structure"),
  }
}
/*
  test("Lookup +") {
    val program1 = compile("exported func main() int { return +(3, 4); }")
    val main = program1.lookupFunction("main")

    // MIGALLOW: Rust can just do one big match on code_body.body.block.expr
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block.expr, { case x @ ReturnSE(_, _) => x })
    val call = Collector.only(ret.inner, { case x @ FunctionCallSE(_, _, _, _) => x })
    Collector.only(call.callableExpr, { case x @ OutsideLoadSE(_, _, CodeNameS(StrI("+")), _, _) => x })
  }
*/
#[test]
fn test_struct() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, &parse_arena, "struct Moo { x int; }");
  let imoo = program.lookup_struct("Moo");

  crate::collect_only_snode!(
    NodeRefS::Struct(imoo),
    NodeRefS::LiteralRule(
      literal_rule @ LiteralSR {
        literal: ILiteralSL::MutabilityLiteral(mutability_literal),
        ..
      }
    ) if mutability_literal.mutability == crate::parsing::ast::MutabilityP::Mutable
      && literal_rule.rune == imoo.mutability_rune => Some(())
  );
  
  let only_member = expect_1(&imoo.members);
  crate::collect_only_snode!(
    NodeRefS::Struct(imoo),
    NodeRefS::MaybeCoercingLookupRule(
      MaybeCoercingLookupSR {
        name: IImpreciseNameS::CodeName(code_name),
        rune,
        ..
      }
    ) if code_name.name.as_str() == "int" && *rune == *only_member.type_rune() => Some(())
  );

  let normal_member = cast!(only_member, IStructMemberS::NormalStructMember);
  assert_eq!(normal_member.name.as_str(), "x");
  assert_eq!(normal_member.variability, VariabilityP::Final);
}
/*
  test("Struct") {
    val program1 = compile("struct Moo { x int; }")
    val imoo = program1.lookupStruct("Moo")

    imoo.headerRules shouldHave {
      case LiteralSR(_, r, MutabilityLiteralSL(MutableP)) => vassert(r == imoo.mutabilityRune)
    }
    imoo.memberRules shouldHave {
      case MaybeCoercingLookupSR(_, m, CodeNameS(StrI("int"))) => vassert(m == imoo.members(0).typeRune)
    }
    imoo.members match {
      case Vector(NormalStructMemberS(_, StrI("x"), FinalP, _)) =>
    }
  }
*/
#[test]
fn linear_struct() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, &parse_arena, "linear struct Moo { x int; }");
  let moo_struct = program.lookup_struct("Moo");
  crate::collect_only_snode!(
    NodeRefS::Struct(moo_struct),
    NodeRefS::MacroCallAttribute(macro_call)
      if macro_call.include == IMacroInclusionP::DontCallMacro
        && macro_call.macro_name.as_str() == "DeriveStructDrop" => Some(())
  );
}
/*
  test("Linear struct") {
    val program1 = compile("linear struct Moo { x int; }")

    val myStruct = program1.lookupStruct("Moo")
    Collector.only(myStruct.attributes, {
      case MacroCallS(_, DontCallMacroP, StrI("DeriveStructDrop")) =>
    })
  }
*/
/*
  test("Lambda") {
    val program1 = compile("exported func main() int { return {_ + _}(4, 6); }")

    val CodeBodyS(BodySE(_, _, BlockSE(_, _, expr))) = program1.lookupFunction("main").body
    val lambda =
      Collector.only(expr, {
        case ReturnSE(_, FunctionCallSE(_, _, OwnershippedSE(_, FunctionSE(lambda@FunctionS(_, _, _, _, _, _, _, _, _, _)), LoadAsBorrowP), _)) => lambda
      })
    // See: Lambdas Dont Need Explicit Identifying Runes (LDNEIR)
    lambda.genericParams match {
      case Vector(
        GenericParameterS(_,RuneUsage(_,mp1b @ MagicParamRuneS(_)),CoordGenericParameterTypeS(None,_,false),None),
        GenericParameterS(_,RuneUsage(_,mp2b @ MagicParamRuneS(_)),CoordGenericParameterTypeS(None,_,false),None)
        // Put this back in when we have regions
        // , _
        ) => {
        vassert(mp1b != mp2b) // Two different runes
      }
    }

    vregionmut() // see above
  }
*/
#[test]
fn interface() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, &parse_arena, "interface IMoo { func blork(virtual this &IMoo, a bool)void; }");
  let imoo = program.lookup_interface("IMoo");
  let blork = expect_1(&imoo.internal_methods);
  let function_name = cast!(&blork.name, IFunctionDeclarationNameS::FunctionName);
  assert_eq!(function_name.name.as_str(), "blork");
}
/*
  test("Interface") {
    val program1 = compile("interface IMoo { func blork(virtual this &IMoo, a bool)void; }")
    val imoo = program1.lookupInterface("IMoo")

    val blork = imoo.internalMethods.head
    blork.name match {
      case FunctionNameS(StrI("blork"), _) =>
    }
  }
*/
/*
  test("Generic interface") {
    val interner = new Interner()
    val program1 = compile("interface IMoo<T> { func blork(virtual this &IMoo, a T)void; }", interner)
    val imoo = program1.lookupInterface("IMoo")

    val blork = imoo.internalMethods.head
    blork.name match {
      case FunctionNameS(StrI("blork"), _) =>
    }

    val imooRunes = imoo.genericParams.map(_.rune)
    val t = CodeRuneS(interner.intern(StrI("T")))
    vassert(imooRunes(0).rune == t)
    vassert(imooRunes.exists(_.rune == t))
    // Interface methods of generic interfaces will have the same identifying runes of their
    // generic interfaces, see IMCBT.
    vassert(blork.genericParams.map(_.rune.rune).contains(CodeRuneS(interner.intern(StrI("T")))))
  }
*/
/*
  test("Impl") {
    val program1 = compile("impl IMoo for Moo;")
    val impl = program1.impls.head
    impl.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("Moo"))) => vassert(r == impl.structKindRune)
    }
    impl.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("IMoo"))) => vassert(r == impl.interfaceKindRune)
    }
  }
*/
/*
  test("Method call") {
    val program1 = compile("exported func main() int { return true.shout(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block, { case r @ ReturnSE(_, _) => r })
    Collector.only(ret, { case FunctionCallSE(_, _, OutsideLoadSE(_, _, CodeNameS(StrI("shout")), _, _), Vector(ConstantBoolSE(_,true))) => })
//    { case ReturnSE(_,FunctionCallSE(_,_,Vector()) => }
  }
*/
/*
  test("Moving method call") {
    val program1 = compile("exported func main() int { x = 4; return (x).shout(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block, { case r @ ReturnSE(_, _) => r })
    Collector.only(ret, { case FunctionCallSE(_, _, OutsideLoadSE(_, _, CodeNameS(StrI("shout")), _, _), Vector(LocalLoadSE(_,CodeVarNameS(StrI("x")), UseP))) => })
  }

  vregionmut() // Put this back in with regions
  // test("Pure regioned function") {
  //   val program1 = compile("pure func moo<r'>(ship &r'Spaceship) { }")
  //   val moo = program1.lookupFunction("moo")
  //
  //   moo.genericParams match {
  //     case Vector(
  //       GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI(r))),RegionGenericParameterTypeS(ReadOnlyRegionS),None)
  //       // Put this back in when we have regions
  //       // ,GenericParameterS(_,RuneUsage(_,DenizenDefaultRegionRuneS(FunctionNameS(StrI("moo"),_))),RegionGenericParameterTypeS(ReadWriteRegionS),None)
  //     ) =>
  //   }
  // }

  vregionmut() // Put this back in with regions
//   test("Pure regioned function with explicit self region") {
//     val program1 = compile("pure func moo<r', t' rw>(ship &r'Spaceship) t'{ }")
//     val moo = program1.lookupFunction("moo")
//
//     moo.genericParams match {
// //      case Vector(
// //        GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("r"))),Vector(),None),
// //        GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("t"))),Vector(),None),
// //        GenericParameterS(_,RuneUsage(_,DefaultRegionRuneS()),Vector(ReadWriteRuneAttributeS(_)),None)) (of class scala.collection.immutable.Vector)
//       case Vector(
//         GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("r"))), RegionGenericParameterTypeS(ReadOnlyRegionS), None),
//         GenericParameterS(_, RuneUsage(_,CodeRuneS(StrI("t"))), RegionGenericParameterTypeS(ReadWriteRegionS), None)) =>
//     }
//   }
*/
/*
  test("Function with magic lambda and regular lambda") {
    // There was a bug that confused the two, and an underscore would add a magic param to every lambda after it

    val program1 =
      compile(
        """exported func main() int {
          |  {_};
          |  (a) => {a};
          |}
        """.stripMargin)
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val BlockSE(_, _, ConsecutorSE(things)) = block
    val lambdas = Collector.all(things, { case f @ FunctionSE(_) => f }).toList
    lambdas.head.function.params match {
      case Vector(_, ParameterS(_, _, false, AtomSP(_, Some(CaptureS(MagicParamNameS(_), false)), Some(RuneUsage(_, MagicParamRuneS(_))), None))) =>
    }
    lambdas.last.function.params match {
      case Vector(_, ParameterS(_, _, false, AtomSP(_, Some(CaptureS(CodeVarNameS(StrI("a")), false)), Some(RuneUsage(_, ImplicitRuneS(_))), None))) =>
    }
  }
*/
#[test]
fn constructing_members() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func MyStruct() {
      self.x = 4;
      self.y = true;
    }",
  );
  let mystruct = program.lookup_function("MyStruct");
  let code_body = cast!(&mystruct.body, crate::postparsing::ast::IBodyS::CodeBody);
  let block = &code_body.body.block;

  match &block.locals[..] {
    [
      crate::postparsing::expressions::LocalS {
        var_name: IVarNameS::ConstructingMemberName(crate::interner::StrI("x")),
        self_borrowed: IVariableUseCertainty::NotUsed,
        self_moved: IVariableUseCertainty::Used,
        self_mutated: IVariableUseCertainty::NotUsed,
        child_borrowed: IVariableUseCertainty::NotUsed,
        child_moved: IVariableUseCertainty::NotUsed,
        child_mutated: IVariableUseCertainty::NotUsed,
      },
      crate::postparsing::expressions::LocalS {
        var_name: IVarNameS::ConstructingMemberName(crate::interner::StrI("y")),
        self_borrowed: IVariableUseCertainty::NotUsed,
        self_moved: IVariableUseCertainty::Used,
        self_mutated: IVariableUseCertainty::NotUsed,
        child_borrowed: IVariableUseCertainty::NotUsed,
        child_moved: IVariableUseCertainty::NotUsed,
        child_mutated: IVariableUseCertainty::NotUsed,
      },
    ] => {}
    other => panic!("unexpected constructing_members locals: {:?}", other),
  }

  let exprs = match block.expr {
    IExpressionSE::Consecutor(crate::postparsing::expressions::ConsecutorSE { exprs }) => exprs,
    _ => panic!("expected consecutor in constructing_members"),
  };
  let expr_nodes = exprs
    .iter()
    .map(|expr| NodeRefS::Expression(*expr))
    .collect::<Vec<_>>();

  let _ = crate::collect_only_snodes!(
    &expr_nodes,
    NodeRefS::Expression(
      IExpressionSE::Let(crate::postparsing::expressions::LetSE {
        pattern:
          crate::postparsing::patterns::AtomSP {
            name:
              Some(crate::postparsing::patterns::CaptureS {
                name: IVarNameS::ConstructingMemberName(crate::interner::StrI("x")),
                mutate: false,
              }),
            destructure: None,
            ..
          },
        expr: IExpressionSE::ConstantInt(crate::postparsing::expressions::ConstantIntSE { value: 4, .. }),
        ..
      })
    ) => Some(())
  );

  let _ = crate::collect_only_snodes!(
    &expr_nodes,
    NodeRefS::Expression(
      IExpressionSE::Let(crate::postparsing::expressions::LetSE {
        pattern:
          crate::postparsing::patterns::AtomSP {
            name:
              Some(crate::postparsing::patterns::CaptureS {
                name: IVarNameS::ConstructingMemberName(crate::interner::StrI("y")),
                mutate: false,
              }),
            destructure: None,
            ..
          },
        expr: IExpressionSE::ConstantBool(crate::postparsing::expressions::ConstantBoolSE { value: true, .. }),
        ..
      })
    ) => Some(())
  );

  let _ = crate::collect_only_snodes!(
    &expr_nodes,
    NodeRefS::Expression(
      IExpressionSE::FunctionCall(FunctionCallSE {
        callable_expr:
          IExpressionSE::OutsideLoad(OutsideLoadSE {
            name: IImpreciseNameS::CodeName(crate::postparsing::names::CodeNameS {
              name: crate::interner::StrI("MyStruct"),
            }),
            ..
          }),
        arg_exprs: [
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(crate::interner::StrI("x")),
            target_ownership: LoadAsP::Use,
            ..
          }),
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(crate::interner::StrI("y")),
            target_ownership: LoadAsP::Use,
            ..
          }),
        ],
        ..
      })
    ) => Some(())
  );
}
/*
  test("Constructing members") {
    val program1 = compile(
      """func MyStruct() {
        |  self.x = 4;
        |  self.y = true;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("MyStruct")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    block.locals match {
      case Vector(
        LocalS(ConstructingMemberNameS(StrI("x")), NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed),
        LocalS(ConstructingMemberNameS(StrI("y")), NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed)) =>
    }
    val exprs = block.expr match { case ConsecutorSE(exprs) => exprs }
    Collector.only(exprs, {
      case LetSE(_,
      _,
      AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("x")), false)), _, None),
      ConstantIntSE(_, 4, _)) =>
    })
    Collector.only(exprs, {
      case LetSE(_,
        _,
        AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("y")), false)), _, None),
        ConstantBoolSE(_, true)) =>
    })
    Collector.only(exprs, {
      case FunctionCallSE(_, _,
        OutsideLoadSE(_, _, CodeNameS(StrI("MyStruct")), _, _),
        Vector(
          LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), UseP),
          LocalLoadSE(_, ConstructingMemberNameS(StrI("y")), UseP))) =>
    })
  }
*/
/*
  test("InitializingRuntimeSizedArrayRequiresSizeAndCallable too few") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = []();
        |}
        |""".stripMargin)
    error match {
      case InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) =>
    }
  }
*/
/*
  test("InitializingRuntimeSizedArrayRequiresSizeAndCallable too many") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = [](4, {_}, 10);
        |}
        |""".stripMargin)
    error match {
      case InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) =>
    }
  }
*/
/*
  test("InitializingStaticSizedArrayRequiresSizeAndCallable too few") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = [#5]();
        |}
        |""".stripMargin)
    error match {
      case InitializingStaticSizedArrayRequiresSizeAndCallable(_) =>
    }
  }
*/
/*
  test("InitializingStaticSizedArrayRequiresSizeAndCallable too many") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = [#5](4, {_});
        |}
        |""".stripMargin)
    error match {
      case InitializingStaticSizedArrayRequiresSizeAndCallable(_) =>
    }
  }
*/
#[test]
fn test_loading_from_member() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func MyStruct() {
      return moo.x;
    }",
  );
  let mystruct = program.lookup_function("MyStruct");
  let code_body = cast!(&mystruct.body, crate::postparsing::ast::IBodyS::CodeBody);
  match code_body.body.block.expr {
    IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::Dot(DotSE {
          left:
            IExpressionSE::OutsideLoad(OutsideLoadSE {
              name: IImpreciseNameS::CodeName(outside_name),
              maybe_template_args: None,
              target_ownership: LoadAsP::LoadAsBorrow,
              ..
            }),
          member,
          borrow_container: true,
          ..
        }),
      ..
    }) => {
      assert_eq!(outside_name.name.as_str(), "moo");
      assert_eq!(member.as_str(), "x");
    }
    other => panic!("unexpected shape: {:?}", other),
  }
}
/*
  test("Test loading from member") {
    val program1 = compile(
      """func MyStruct() {
        |  return moo.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("MyStruct")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    Collector.only(block,
      { case ReturnSE(_, DotSE(_,OutsideLoadSE(_,_,CodeNameS(StrI("moo")),None,LoadAsBorrowP),StrI("x"),true)) => })

  }
*/
#[test]
fn test_loading_from_member_2() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func MyStruct() {
      return &moo.x;
    }",
  );
  let mystruct = program.lookup_function("MyStruct");
  let code_body = cast!(&mystruct.body, crate::postparsing::ast::IBodyS::CodeBody);
  match code_body.body.block.expr {
    IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::Ownershipped(OwnershippedSE {
          target_ownership: LoadAsP::LoadAsBorrow,
          inner_expr:
            IExpressionSE::Dot(DotSE {
              left:
                IExpressionSE::OutsideLoad(OutsideLoadSE {
                  name: IImpreciseNameS::CodeName(outside_name),
                  maybe_template_args: None,
                  target_ownership: LoadAsP::LoadAsBorrow,
                  ..
                }),
              member,
              borrow_container: true,
              ..
            }),
          ..
        }),
      ..
    }) => {
      assert_eq!(outside_name.name.as_str(), "moo");
      assert_eq!(member.as_str(), "x");
    }
    other => panic!("unexpected shape: {:?}", other),
  }
}
/*
  test("Test loading from member 2") {
    val program1 = compile(
      """func MyStruct() {
        |  return &moo.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("MyStruct")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    Collector.only(block, {
      case ReturnSE(_, OwnershippedSE(_, DotSE(_,OutsideLoadSE(_,_,CodeNameS(StrI("moo")),None,LoadAsBorrowP),x,true),LoadAsBorrowP)) =>
    })
  }
*/
#[test]
fn constructing_members_borrowing_another_member() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func MyStruct() {
      self.x = 4;
      self.y = &self.x;
    }",
  );
  let mystruct = program.lookup_function("MyStruct");
  let code_body = cast!(&mystruct.body, crate::postparsing::ast::IBodyS::CodeBody);
  let block = &code_body.body.block;

  let (first_local, second_local) = expect_2(&block.locals);
  assert!(matches!(
    first_local.var_name,
    IVarNameS::ConstructingMemberName(ref member_name) if member_name.as_str() == "x"
  ));
  assert_eq!(first_local.self_borrowed, IVariableUseCertainty::Used);
  assert_eq!(first_local.self_moved, IVariableUseCertainty::Used);
  assert_eq!(first_local.self_mutated, IVariableUseCertainty::NotUsed);
  assert_eq!(first_local.child_borrowed, IVariableUseCertainty::NotUsed);
  assert_eq!(first_local.child_moved, IVariableUseCertainty::NotUsed);
  assert_eq!(first_local.child_mutated, IVariableUseCertainty::NotUsed);

  assert!(matches!(
    second_local.var_name,
    IVarNameS::ConstructingMemberName(ref member_name) if member_name.as_str() == "y"
  ));
  assert_eq!(second_local.self_borrowed, IVariableUseCertainty::NotUsed);
  assert_eq!(second_local.self_moved, IVariableUseCertainty::Used);
  assert_eq!(second_local.self_mutated, IVariableUseCertainty::NotUsed);
  assert_eq!(second_local.child_borrowed, IVariableUseCertainty::NotUsed);
  assert_eq!(second_local.child_moved, IVariableUseCertainty::NotUsed);
  assert_eq!(second_local.child_mutated, IVariableUseCertainty::NotUsed);

  let consecutor = cast!(block.expr, IExpressionSE::Consecutor);
  let (first_expr, second_expr, third_expr) = expect_3(&consecutor.exprs);

  let let_x = cast!(first_expr, IExpressionSE::Let);
  let let_x_capture = let_x.pattern.name.as_ref().unwrap();
  assert!(matches!(
    let_x_capture.name,
    IVarNameS::ConstructingMemberName(ref member_name) if member_name.as_str() == "x"
  ));
  assert_eq!(let_x_capture.mutate, false);
  assert_eq!(cast!(let_x.expr, IExpressionSE::ConstantInt).value, 4);

  let let_y = cast!(second_expr, IExpressionSE::Let);
  let let_y_capture = let_y.pattern.name.as_ref().unwrap();
  assert!(matches!(
    let_y_capture.name,
    IVarNameS::ConstructingMemberName(ref member_name) if member_name.as_str() == "y"
  ));
  assert_eq!(let_y_capture.mutate, false);
  let local_load_x_borrow = cast!(let_y.expr, IExpressionSE::LocalLoad);
  assert!(matches!(
    local_load_x_borrow.name,
    IVarNameS::ConstructingMemberName(ref member_name) if member_name.as_str() == "x"
  ));
  assert_eq!(local_load_x_borrow.target_ownership, LoadAsP::LoadAsBorrow);

  match third_expr {
    IExpressionSE::FunctionCall(FunctionCallSE {
      callable_expr:
        IExpressionSE::OutsideLoad(OutsideLoadSE {
          name: IImpreciseNameS::CodeName(callable_name),
          ..
        }),
      arg_exprs,
      ..
    }) => {
      assert_eq!(callable_name.name.as_str(), "MyStruct");
      match arg_exprs {
        [
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(x_name),
            target_ownership: LoadAsP::Use,
            ..
          }),
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(y_name),
            target_ownership: LoadAsP::Use,
            ..
          }),
        ] => {
          assert_eq!(x_name.as_str(), "x");
          assert_eq!(y_name.as_str(), "y");
        }
        other => panic!("unexpected constructor args: {:?}", other),
      }
    }
    other => panic!("unexpected constructor call shape: {:?}", other),
  }
}
/*
  test("Constructing members, borrowing another member") {
    val program1 = compile(
      """func MyStruct() {
        |  self.x = 4;
        |  self.y = &self.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("MyStruct")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    block.locals match {
      case Vector(
        LocalS(ConstructingMemberNameS(StrI("x")), Used, Used, NotUsed, NotUsed, NotUsed, NotUsed),
        LocalS(ConstructingMemberNameS(StrI("y")), NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed)) =>
    }
    Collector.only(block, {
      case LetSE(_, _,
        AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("x")), false)), _, None),
        ConstantIntSE(_, 4, _)) =>
    })
    Collector.only(block, {
      case LetSE(_, _,
        AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("y")), false)), _, None),
        LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), LoadAsBorrowP)) =>
    })
    Collector.only(block, {
      case FunctionCallSE(_, _,
        OutsideLoadSE(_, _, CodeNameS(StrI("MyStruct")), _, _),
        Vector(
        LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), UseP),
        LocalLoadSE(_, ConstructingMemberNameS(StrI("y")), UseP))) =>
    })
  }
*/
/*
  test("foreach") {
    val program1 = compile(
      """func main() {
        |  foreach i in myList { }
        |}
        |""".stripMargin)

    val function = program1.lookupFunction("main")
    val CodeBodyS(body) = function.body
    body.block shouldHave {
      case LocalS(IterableNameS(_),Used,NotUsed,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LocalS(IteratorNameS(_),Used,NotUsed,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LocalS(IterationOptionNameS(_),Used,Used,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LocalS(CodeVarNameS(StrI("i")),NotUsed,NotUsed,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IterableNameS(_), false)),None,None),
        OutsideLoadSE(_,_,CodeNameS(StrI("myList")),None,UseP)) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IteratorNameS(_), false)),None,None),
        FunctionCallSE(_,_,
          OutsideLoadSE(_,_,CodeNameS(StrI("begin")),None,LoadAsBorrowP),
          Vector(LocalLoadSE(_,IterableNameS(_),LoadAsBorrowP)))) =>
    }
    body.block shouldHave {
      case WhileSE(_, _) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IterationOptionNameS(_), false)),None,None),
        FunctionCallSE(_,_,
          OutsideLoadSE(_,_,CodeNameS(StrI("next")),None,LoadAsBorrowP),
          Vector(
            LocalLoadSE(_,IteratorNameS(_),LoadAsBorrowP)))) =>
    }
    body.block shouldHave {
      case FunctionCallSE(_,_,
        OutsideLoadSE(_,_,CodeNameS(StrI("isEmpty")),_,_),
        Vector(
          LocalLoadSE(_,IterationOptionNameS(_),LoadAsBorrowP))) =>
    }
    body.block shouldHave {
      case BreakSE(_) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(CodeVarNameS(StrI("i")), false)),None,None),
        FunctionCallSE(_,_,
          OutsideLoadSE(_,_,CodeNameS(StrI("get")),None,LoadAsBorrowP),
          Vector(LocalLoadSE(_,IterationOptionNameS(_),UseP)))) =>
    }
    body.block shouldHave {
      case LocalLoadSE(_,IterationOptionNameS(_),UseP) =>
    }
  }
*/
#[test]
fn this_isnt_special_if_was_explicit_param() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func moo(self &MyStruct) {
      println(self.x);
    }",
  );
  let moo = program.lookup_function("moo");
  let code_body = cast!(&moo.body, crate::postparsing::ast::IBodyS::CodeBody);
  let function_call = crate::collect_only_snode!(
    NodeRefS::Program(&program),
    NodeRefS::Expression(IExpressionSE::FunctionCall(function_call)) => Some(function_call)
  );
  let outside_load = cast!(function_call.callable_expr, IExpressionSE::OutsideLoad);
  let code_name = cast!(&outside_load.name, IImpreciseNameS::CodeName);
  assert_eq!(code_name.name.as_str(), "println");
  let dot = cast!(expect_1(&function_call.arg_exprs), IExpressionSE::Dot);
  assert_eq!(dot.member.as_str(), "x");
  assert!(dot.borrow_container);
  let local_load = cast!(dot.left, IExpressionSE::LocalLoad);
  let code_var_name = cast!(&local_load.name, IVarNameS::CodeVarName);
  assert_eq!(code_var_name.as_str(), "self");
  assert_eq!(local_load.target_ownership, LoadAsP::LoadAsBorrow);

  let function_calls = crate::collect_where_snode!(
    NodeRefS::Program(&program),
    NodeRefS::Expression(IExpressionSE::FunctionCall(_)) => Some(())
  );
  assert_eq!(function_calls.len(), 1);

  let _ = code_body;
}
/*
  test("this isnt special if was explicit param") {
    val program1 = compile(
      """func moo(self &MyStruct) {
        |  println(self.x);
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("moo")
    Collector.only(main.body, {
      case FunctionCallSE(_,_,
        OutsideLoadSE(_, _, CodeNameS(StrI("println")), _, _),
        Vector(DotSE(_, LocalLoadSE(_, CodeVarNameS(StrI("self")), LoadAsBorrowP), StrI("x"), true))) =>
    })
    Collector.all(main.body, { case FunctionCallSE(_, _, _, _) => }).size shouldEqual 1
  }
*/
#[test]
fn reports_when_mutating_nonexistant_local() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let err = compile_for_error(
    &interner,
    &keywords,
    &parse_arena,
    "exported func main() int {\n  set a = a + 1;\n}",
  );
  match &err {
    ICompileErrorS::CouldntFindVarToMutateS(c) => assert_eq!(c.name, "a"),
    _ => panic!("expected CouldntFindVarToMutateS(_, \"a\"), got {:?}", err),
  }
}
/*
  test("Reports when mutating nonexistant local") {
    val err = compileForError(
      """exported func main() int {
        |  set a = a + 1;
        |}
        |""".stripMargin)
    err match {
      case CouldntFindVarToMutateS(_, "a") =>
    }
  }
*/
/*
  test("Reports when extern function has body") {
    val err = compileForError(
      """
        |extern func bork() int {
        |  3
        |}
        |""".stripMargin)
    err match {
      case ExternHasBody(_) =>
    }
  }
*/
/*
  test("Reports when we forget set") {
    val err = compileForError(
      """
        |exported func main() {
        |  x = "world!";
        |  x = "changed";
        |}
        |""".stripMargin)
    err match {
      case VariableNameAlreadyExists(_, CodeVarNameS(StrI("x"))) =>
      case _ => vfail()
    }
  }
*/
/*
  test("Reports when interface method doesnt have self") {
    val err = compileForError("interface IMoo { func blork(a bool)void; }")
    err match {
      case InterfaceMethodNeedsSelf(_) =>
      case _ => vfail()
    }
  }
*/
/*
  test("Statement after result or return") {
    compileForError(
      """
        |func doCivicDance(virtual this Car) {
        |  return 4;
        |  7
        |}
        """.stripMargin) match {
      case StatementAfterReturnS(_) =>
    }
  }
*/
/*
  test("Report type mismatch") {
    compileForError(
      """
        |struct Vec<N, T> where N Int
        |{
        |  values [#N]<imm>T;
        |}
        |
      """.stripMargin) match {
      case RuneExplicitTypeConflictS(_, CodeRuneS(StrI("N")), _) =>
    }
  }
*/
/*
}
*/