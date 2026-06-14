// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib postparsing::test::post_parser_tests

use bumpalo::Bump;
use crate::cast;
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::parsing::ast::{IMacroInclusionP, LoadAsP, VariabilityP};
use crate::postparsing::ast::{IStructMemberS, ProgramS};
use crate::postparsing::expressions::{
  ConstantIntSE, DotSE, FunctionCallSE, IExpressionSE, IVariableUseCertainty, LetSE, LoadPartSE, LocalLoadSE,
  LocalS, OutsideLoadSE, OverloadSetSE, OwnershippedSE, ReturnSE,
};
use crate::postparsing::patterns::patterns::{AtomSP, CaptureS};
use crate::postparsing::names::{CodeNameS, CodeRuneS, IFunctionDeclarationNameS, IImpreciseNameS, IRuneS, IRuneValS, IVarNameS};
use crate::postparsing::post_parser::{ICompileErrorS, PostParser};
use crate::postparsing::rules::rules::{ILiteralSL, LiteralSR, MaybeCoercingLookupSR};
use crate::postparsing::test::traverse::NodeRefS;
use crate::parsing::tests::utils::compile_file;
use crate::parsing::tests::utils::{expect_1, expect_2, expect_3};
use crate::postparsing::ast::IBodyS;
use crate::parsing::ast::MutabilityP;
use crate::postparsing::ast::IGenericParameterTypeS;
use crate::postparsing::expressions::ConstantBoolSE;
use crate::postparsing::ast::ParameterS;
use crate::postparsing::rules::RuneUsage;
use crate::postparsing::expressions::ConsecutorSE;
use crate::postparsing::post_parser::VariableNameAlreadyExists;
use crate::postparsing::post_parser::RuneExplicitTypeConflictS;
use crate::collect_only_snode;
use crate::collect_only_snodes;
use crate::collect_where_snode;
use crate::collect_where_snodes;

/*
package dev.vale.postparsing

import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{FinalP, LoadAsBorrowP, MutableP, UseP}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules.{LiteralSR, MaybeCoercingLookupSR, MutabilityLiteralSL, RuneUsage}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, SolveIncomplete}
import org.scalatest._

class PostParserTests extends FunSuite with Matchers with Collector {
*/
fn compile<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ProgramS<'s>
where 'p: 's,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let keywords_p = Keywords::new_for_parse(parse_arena);
  let only_file = compile_file(parse_arena, &keywords_p, code).unwrap();
  // Re-intern FileCoordinate from 'p into 's
  let file_coord_s = scout_arena.intern_file_coordinate(
    scout_arena.intern_package_coordinate(
      scout_arena.intern_str(only_file.file_coord.package_coord.module.as_str()),
      &only_file.file_coord.package_coord.packages.iter().map(|s| scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
    ),
    only_file.file_coord.filepath.as_str(),
  );
  let post_parser = PostParser::new(options, scout_arena, keywords, &keywords_p, parse_arena);
  post_parser
    .scout_program(file_coord_s, &only_file)
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
fn compile_for_error<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ICompileErrorS<'s>
where 'p: 's,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let keywords_p = Keywords::new_for_parse(parse_arena);
  let only_file = compile_file(parse_arena, &keywords_p, code).unwrap();
  // Re-intern FileCoordinate from 'p into 's
  let file_coord_s = scout_arena.intern_file_coordinate(
    scout_arena.intern_package_coordinate(
      scout_arena.intern_str(only_file.file_coord.package_coord.module.as_str()),
      &only_file.file_coord.package_coord.packages.iter().map(|s| scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
    ),
    only_file.file_coord.filepath.as_str(),
  );
  let post_parser = PostParser::new(options, scout_arena, keywords, &keywords_p, parse_arena);
  match post_parser.scout_program(file_coord_s, &only_file) {
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
      case IdentifyingRunesIncompleteS(_, IdentifiabilitySolveError(_, FailedSolve(_, _, _, unsolvedRunes, SolveIncomplete()))) => {
        // The param rune, and the _ rune are both unknown
        vassert(unsolvedRunes.size == 2)
      }
    }
  }
*/
#[test]
fn lookup_plus() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { return +(3, 4); }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  match code_body.body.block.expr {
    IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OverloadSet(OverloadSetSE {
              lookup: OutsideLoadSE { parts, .. },
            }),
          ..
        }),
      ..
    }) => match &parts.first().expect("non-empty parts").name {
      IImpreciseNameS::CodeName(code_name) => assert_eq!(code_name.name.as_str(), "+"),
      _ => panic!("expected CodeName in OverloadSet first part"),
    },
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
    Collector.only(call.callableExpr, { case x @ OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("+")), _)))) => x })
  }
*/
#[test]
fn test_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(&scout_arena, &keywords, &parse_arena, "struct Moo { x int; }");
  let imoo = program.lookup_struct("Moo");

  collect_only_snode!(
    NodeRefS::Struct(imoo),
    NodeRefS::LiteralRule(
      literal_rule @ LiteralSR {
        literal: ILiteralSL::MutabilityLiteral(mutability_literal),
        ..
      }
    ) if mutability_literal.mutability == MutabilityP::Mutable
      && literal_rule.rune == imoo.mutability_rune => Some(())
  );
  
  let only_member = expect_1(&imoo.members);
  collect_only_snode!(
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
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(&scout_arena, &keywords, &parse_arena, "linear struct Moo { x int; }");
  let moo_struct = program.lookup_struct("Moo");
  collect_only_snode!(
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
#[test]
fn lambda() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { return {_ + _}(4, 6); }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let lambda = match code_body.body.block.expr {
    IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::Ownershipped(OwnershippedSE {
              inner_expr: IExpressionSE::Function(lambda_function),
              target_ownership: LoadAsP::LoadAsBorrow,
              ..
            }),
          arg_exprs:
            [
              IExpressionSE::ConstantInt(ConstantIntSE {
                value: 4,
                ..
              }),
              IExpressionSE::ConstantInt(ConstantIntSE {
                value: 6,
                ..
              }),
            ],
          ..
        }),
      ..
    }) => &lambda_function.function,
    _ => panic!("expected return {{_ + _}}(4, 6) structure"),
  };

  let (first_generic_param, second_generic_param) = expect_2(lambda.generic_params);
  match &first_generic_param.tyype {
    IGenericParameterTypeS::CoordGenericParameterType(coord_type) => {
      assert_eq!(coord_type.coord_region, None);
      assert!(!coord_type.region_mutable);
    }
    _ => panic!("expected first lambda generic param to be a CoordGenericParameterType"),
  }
  match &second_generic_param.tyype {
    IGenericParameterTypeS::CoordGenericParameterType(coord_type) => {
      assert_eq!(coord_type.coord_region, None);
      assert!(!coord_type.region_mutable);
    }
    _ => panic!("expected second lambda generic param to be a CoordGenericParameterType"),
  }
  let first_magic_param_rune = match first_generic_param.rune.rune {
    IRuneS::MagicParamRune(magic_param_rune) => magic_param_rune,
    _ => panic!("expected first lambda generic param to be a magic param rune"),
  };
  let second_magic_param_rune = match second_generic_param.rune.rune {
    IRuneS::MagicParamRune(magic_param_rune) => magic_param_rune,
    _ => panic!("expected second lambda generic param to be a magic param rune"),
  };
  assert_ne!(
    first_magic_param_rune, second_magic_param_rune,
    "expected two different magic param runes"
  );
}
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
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(&scout_arena, &keywords, &parse_arena, "interface IMoo { func blork(virtual this &IMoo, a bool)void; }");
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
#[test]
fn generic_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "interface IMoo<T> { func blork(virtual this &IMoo, a T)void; }",
  );
  let imoo = program.lookup_interface("IMoo");
  let blork = expect_1(imoo.internal_methods);
  let blork_name = cast!(&blork.name, IFunctionDeclarationNameS::FunctionName);
  assert_eq!(blork_name.name.as_str(), "blork");

  let t_ = scout_arena.intern_str("T");
  let t_rune = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: t_ }));
  let imoo_first_rune = &expect_1(imoo.generic_params).rune.rune;
  assert_eq!(*imoo_first_rune, t_rune);
  assert!(imoo.generic_params.iter().any(|generic_param| generic_param.rune.rune == t_rune));
  assert!(blork.generic_params.iter().any(|generic_param| generic_param.rune.rune == t_rune));
}
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
#[test]
fn impl_() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(&scout_arena, &keywords, &parse_arena, "impl IMoo for Moo;");
  let impl_ = expect_1(program.impls);

  collect_only_snode!(
    NodeRefS::Impl(impl_),
    NodeRefS::MaybeCoercingLookupRule(MaybeCoercingLookupSR {
      name: IImpreciseNameS::CodeName(CodeNameS {
        name: StrI("Moo"),
        ..
      }),
      rune,
      ..
    }) if *rune == impl_.struct_kind_rune => Some(())
  );
  collect_only_snode!(
    NodeRefS::Impl(impl_),
    NodeRefS::MaybeCoercingLookupRule(MaybeCoercingLookupSR {
      name: IImpreciseNameS::CodeName(CodeNameS {
        name: StrI("IMoo"),
        ..
      }),
      rune,
      ..
    }) if *rune == impl_.interface_kind_rune => Some(())
  );
}
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
#[test]
fn method_call() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { return true.shout(); }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  collect_only_snode!(
    NodeRefS::Expression(code_body.body.block.expr),
    NodeRefS::Expression(IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OverloadSet(OverloadSetSE {
              lookup: OutsideLoadSE { parts, .. },
            }),
          arg_exprs:
            [IExpressionSE::ConstantBool(ConstantBoolSE {
              value: true,
              ..
            })],
          ..
        }),
      ..
    })) if matches!(
      parts.first().map(|p| &p.name),
      Some(IImpreciseNameS::CodeName(CodeNameS { name, .. })) if name.as_str() == "shout"
    ) => Some(())
  );
}
/*
  test("Method call") {
    val program1 = compile("exported func main() int { return true.shout(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block, { case r @ ReturnSE(_, _) => r })
    Collector.only(ret, { case FunctionCallSE(_, _, OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("shout")), _)))), Vector(ConstantBoolSE(_,true))) => })
//    { case ReturnSE(_,FunctionCallSE(_,_,Vector()) => }
  }
*/
#[test]
fn moving_method_call() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; return (x).shout(); }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  collect_only_snode!(
    NodeRefS::Expression(code_body.body.block.expr),
    NodeRefS::Expression(IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OverloadSet(OverloadSetSE {
              lookup: OutsideLoadSE { parts, .. },
            }),
          arg_exprs:
            [IExpressionSE::LocalLoad(LocalLoadSE {
              name: IVarNameS::CodeVarName(StrI("x")),
              target_ownership: LoadAsP::Use,
              ..
            })],
          ..
        }),
      ..
    })) if matches!(
      parts.first().map(|p| &p.name),
      Some(IImpreciseNameS::CodeName(CodeNameS { name, .. })) if name.as_str() == "shout"
    ) => Some(())
  );
}
/*
  test("Moving method call") {
    val program1 = compile("exported func main() int { x = 4; return (x).shout(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block, { case r @ ReturnSE(_, _) => r })
    Collector.only(ret, { case FunctionCallSE(_, _, OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("shout")), _)))), Vector(LocalLoadSE(_,CodeVarNameS(StrI("x")), UseP))) => })
  }
  test("Method call with explicit template arg preserves the <T>") {
    val program1 = compile("exported func main() { x = 4; x.foo<int>(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val outside =
      Collector.only(block, {
        case OverloadSetSE(o @ OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("foo")), _)))) => o
      })
    val Vector(LoadPartSE(_, fooExplicitArgs)) = outside.parts
    // Expected: <int> appears as one explicit arg rune, with a corresponding lookup rule.
    // Actual: fooExplicitArgs is empty and rules is empty — `<int>` was dropped.
    vassert(fooExplicitArgs.size == 1)
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == fooExplicitArgs.head)
    }
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
#[test]
fn function_with_magic_lambda_and_regular_lambda() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
      {_};
      (a) => {a};
    }",
  );
  let main = program.lookup_function("main");

  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let block = &code_body.body.block;
  let things = cast!(&block.expr, IExpressionSE::Consecutor).exprs;
  let thing_nodes = things
    .iter()
    .map(|thing| NodeRefS::Expression(*thing))
    .collect::<Vec<_>>();
  let lambdas = collect_where_snodes!(
    &thing_nodes,
    NodeRefS::Expression(IExpressionSE::Function(function)) => Some(function)
  );
  let (first_lambda, second_lambda) = expect_2(&lambdas);

  let (_, first_lambda_second_param) = expect_2(first_lambda.function.params);
  match first_lambda_second_param {
    ParameterS {
      pre_checked: false,
      pattern:
        AtomSP {
          name:
            Some(CaptureS {
              name: IVarNameS::MagicParamName(_),
              mutate: false,
            }),
          coord_rune:
            Some(RuneUsage {
              rune: IRuneS::MagicParamRune(_),
              ..
            }),
          destructure: None,
          ..
        },
      ..
    } => {}
    _ => panic!("expected second param on first lambda to be a magic param"),
  }

  let (_, second_lambda_second_param) = expect_2(second_lambda.function.params);
  match second_lambda_second_param {
    ParameterS {
      pre_checked: false,
      pattern:
        AtomSP {
          name:
            Some(CaptureS {
              name: IVarNameS::CodeVarName(StrI("a")),
              mutate: false,
            }),
          coord_rune:
            Some(RuneUsage {
              rune: IRuneS::ImplicitRune(_),
              ..
            }),
          destructure: None,
          ..
        },
      ..
    } => {}
    _ => panic!("expected second param on second lambda to be code var a with implicit rune"),
  }
}
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
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func MyStruct() {
      self.x = 4;
      self.y = true;
    }",
  );
  let mystruct = program.lookup_function("MyStruct");
  let code_body = cast!(&mystruct.body, IBodyS::CodeBody);
  let block = &code_body.body.block;

  match &block.locals[..] {
    [
      LocalS {
        var_name: IVarNameS::ConstructingMemberName(StrI("x")),
        self_borrowed: IVariableUseCertainty::NotUsed,
        self_moved: IVariableUseCertainty::Used,
        self_mutated: IVariableUseCertainty::NotUsed,
        child_borrowed: IVariableUseCertainty::NotUsed,
        child_moved: IVariableUseCertainty::NotUsed,
        child_mutated: IVariableUseCertainty::NotUsed,
      },
      LocalS {
        var_name: IVarNameS::ConstructingMemberName(StrI("y")),
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
    IExpressionSE::Consecutor(ConsecutorSE { exprs }) => exprs,
    _ => panic!("expected consecutor in constructing_members"),
  };
  let expr_nodes = exprs
    .iter()
    .map(|expr| NodeRefS::Expression(*expr))
    .collect::<Vec<_>>();

  let _ = collect_only_snodes!(
    &expr_nodes,
    NodeRefS::Expression(
      IExpressionSE::Let(LetSE {
        pattern:
          AtomSP {
            name:
              Some(CaptureS {
                name: IVarNameS::ConstructingMemberName(StrI("x")),
                mutate: false,
              }),
            destructure: None,
            ..
          },
        expr: IExpressionSE::ConstantInt(ConstantIntSE { value: 4, .. }),
        ..
      })
    ) => Some(())
  );

  let _ = collect_only_snodes!(
    &expr_nodes,
    NodeRefS::Expression(
      IExpressionSE::Let(LetSE {
        pattern:
          AtomSP {
            name:
              Some(CaptureS {
                name: IVarNameS::ConstructingMemberName(StrI("y")),
                mutate: false,
              }),
            destructure: None,
            ..
          },
        expr: IExpressionSE::ConstantBool(ConstantBoolSE { value: true, .. }),
        ..
      })
    ) => Some(())
  );

  let _ = collect_only_snodes!(
    &expr_nodes,
    NodeRefS::Expression(
      IExpressionSE::FunctionCall(FunctionCallSE {
        callable_expr:
          IExpressionSE::OverloadSet(OverloadSetSE {
            lookup: OutsideLoadSE {
              parts: [LoadPartSE {
                name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("MyStruct") }),
                ..
              }],
              ..
            },
          }),
        arg_exprs: [
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(StrI("x")),
            target_ownership: LoadAsP::Use,
            ..
          }),
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(StrI("y")),
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
        OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("MyStruct")), _)))),
        Vector(
          LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), UseP),
          LocalLoadSE(_, ConstructingMemberNameS(StrI("y")), UseP))) =>
    })
  }
*/
#[test]
fn initializing_runtime_sized_array_requires_size_and_callable_too_few() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func MyStruct() {\n  ship = []();\n}",
  );
  match &err {
    ICompileErrorS::InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) => {}
    _ => panic!(
      "expected InitializingRuntimeSizedArrayRequiresSizeAndCallable(_), got {:?}",
      err
    ),
  }
}
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
#[test]
fn initializing_runtime_sized_array_requires_size_and_callable_too_many() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func MyStruct() {\n  ship = [](4, {_}, 10);\n}",
  );
  match &err {
    ICompileErrorS::InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) => {}
    _ => panic!(
      "expected InitializingRuntimeSizedArrayRequiresSizeAndCallable(_), got {:?}",
      err
    ),
  }
}
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
#[test]
fn initializing_static_sized_array_requires_size_and_callable_too_few() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func MyStruct() {\n  ship = [#5]();\n}",
  );
  match &err {
    ICompileErrorS::InitializingStaticSizedArrayRequiresSizeAndCallable(_) => {}
    _ => panic!(
      "expected InitializingStaticSizedArrayRequiresSizeAndCallable(_), got {:?}",
      err
    ),
  }
}
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
#[test]
fn initializing_static_sized_array_requires_size_and_callable_too_many() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func MyStruct() {\n  ship = [#5](4, {_});\n}",
  );
  match &err {
    ICompileErrorS::InitializingStaticSizedArrayRequiresSizeAndCallable(_) => {}
    _ => panic!(
      "expected InitializingStaticSizedArrayRequiresSizeAndCallable(_), got {:?}",
      err
    ),
  }
}
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
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func main() {
      moo = MyStruct();
      return moo.x;
    }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  collect_only_snode!(
    NodeRefS::Expression(code_body.body.block.expr),
    NodeRefS::Expression(IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::Dot(DotSE {
          left:
            IExpressionSE::LocalLoad(LocalLoadSE {
              name: IVarNameS::CodeVarName(StrI("moo")),
              target_ownership: LoadAsP::LoadAsBorrow,
              ..
            }),
          member: StrI("x"),
          borrow_container: true,
          ..
        }),
      ..
    })) => Some(())
  );
}
/*
  test("Test loading from member") {
    val program1 = compile(
      """func main() {
        |  moo = MyStruct();
        |  return moo.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    Collector.only(block,
      { case ReturnSE(_, DotSE(_,LocalLoadSE(_,CodeVarNameS(StrI("moo")),LoadAsBorrowP),StrI("x"),true)) => })

  }
*/
#[test]
fn test_loading_from_member_2() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func main() {
      moo = MyStruct();
      return &moo.x;
    }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  collect_only_snode!(
    NodeRefS::Expression(code_body.body.block.expr),
    NodeRefS::Expression(IExpressionSE::Return(ReturnSE {
      inner:
        IExpressionSE::Ownershipped(OwnershippedSE {
          target_ownership: LoadAsP::LoadAsBorrow,
          inner_expr:
            IExpressionSE::Dot(DotSE {
              left:
                IExpressionSE::LocalLoad(LocalLoadSE {
                  name: IVarNameS::CodeVarName(StrI("moo")),
                  target_ownership: LoadAsP::LoadAsBorrow,
                  ..
                }),
              borrow_container: true,
              ..
            }),
          ..
        }),
      ..
    })) => Some(())
  );
}
/*
  test("Test loading from member 2") {
    val program1 = compile(
      """func main() {
        |  moo = MyStruct();
        |  return &moo.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    Collector.only(block, {
      case ReturnSE(_, OwnershippedSE(_, DotSE(_,LocalLoadSE(_,CodeVarNameS(StrI("moo")),LoadAsBorrowP),x,true),LoadAsBorrowP)) =>
    })
  }
*/
#[test]
fn constructing_members_borrowing_another_member() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func MyStruct() {
      self.x = 4;
      self.y = &self.x;
    }",
  );
  let main = program.lookup_function("MyStruct");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let block = &code_body.body.block;

  match &*block.locals {
    [
      LocalS {
        var_name: IVarNameS::ConstructingMemberName(StrI("x")),
        self_borrowed: IVariableUseCertainty::Used,
        self_moved: IVariableUseCertainty::Used,
        self_mutated: IVariableUseCertainty::NotUsed,
        child_borrowed: IVariableUseCertainty::NotUsed,
        child_moved: IVariableUseCertainty::NotUsed,
        child_mutated: IVariableUseCertainty::NotUsed,
      },
      LocalS {
        var_name: IVarNameS::ConstructingMemberName(StrI("y")),
        self_borrowed: IVariableUseCertainty::NotUsed,
        self_moved: IVariableUseCertainty::Used,
        self_mutated: IVariableUseCertainty::NotUsed,
        child_borrowed: IVariableUseCertainty::NotUsed,
        child_moved: IVariableUseCertainty::NotUsed,
        child_mutated: IVariableUseCertainty::NotUsed,
      },
    ] => {}
    other => panic!("unexpected locals: {:?}", other),
  }

  collect_only_snode!(
    NodeRefS::Expression(block.expr),
    NodeRefS::Expression(IExpressionSE::Let(LetSE {
      pattern: AtomSP {
        name: Some(CaptureS {
          name: IVarNameS::ConstructingMemberName(StrI("x")),
          mutate: false,
        }),
        destructure: None,
        ..
      },
      expr: IExpressionSE::ConstantInt(ConstantIntSE { value: 4, .. }),
      ..
    })) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(block.expr),
    NodeRefS::Expression(IExpressionSE::Let(LetSE {
      pattern: AtomSP {
        name: Some(CaptureS {
          name: IVarNameS::ConstructingMemberName(StrI("y")),
          mutate: false,
        }),
        destructure: None,
        ..
      },
      expr: IExpressionSE::LocalLoad(LocalLoadSE {
        name: IVarNameS::ConstructingMemberName(StrI("x")),
        target_ownership: LoadAsP::LoadAsBorrow,
        ..
      }),
      ..
    })) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(block.expr),
    NodeRefS::Expression(IExpressionSE::FunctionCall(FunctionCallSE {
      callable_expr: IExpressionSE::OverloadSet(OverloadSetSE {
        lookup: OutsideLoadSE {
          parts: [LoadPartSE {
            name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("MyStruct") }),
            ..
          }],
          ..
        },
      }),
      arg_exprs: [
        IExpressionSE::LocalLoad(LocalLoadSE {
          name: IVarNameS::ConstructingMemberName(StrI("x")),
          target_ownership: LoadAsP::Use,
          ..
        }),
        IExpressionSE::LocalLoad(LocalLoadSE {
          name: IVarNameS::ConstructingMemberName(StrI("y")),
          target_ownership: LoadAsP::Use,
          ..
        }),
      ],
      ..
    })) => Some(())
  );
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
        OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("MyStruct")), _)))),
        Vector(
        LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), UseP),
        LocalLoadSE(_, ConstructingMemberNameS(StrI("y")), UseP))) =>
    })
  }
*/
#[test]
fn foreach() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func main() {
      myList = 0;
      foreach i in myList { }
    }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let root_expr = code_body.body.block.expr;

  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Local(LocalS {
      var_name: IVarNameS::IterableName(_),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    }) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Local(LocalS {
      var_name: IVarNameS::IteratorName(_),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    }) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Local(LocalS {
      var_name: IVarNameS::IterationOptionName(_),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    }) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Local(LocalS {
      var_name: IVarNameS::CodeVarName(StrI("i")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    }) => Some(())
  );

  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::Let(LetSE {
      pattern: AtomSP {
        name:
          Some(CaptureS {
            name: IVarNameS::IterableName(_),
            mutate: false,
          }),
        coord_rune: None,
        destructure: None,
        ..
      },
      expr:
        IExpressionSE::LocalLoad(LocalLoadSE {
          name: IVarNameS::CodeVarName(StrI("myList")),
          target_ownership: LoadAsP::Use,
          ..
        }),
      ..
    })) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::Let(LetSE {
      pattern: AtomSP {
        name:
          Some(CaptureS {
            name: IVarNameS::IteratorName(_),
            mutate: false,
          }),
        coord_rune: None,
        destructure: None,
        ..
      },
      expr:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OverloadSet(OverloadSetSE {
              lookup: OutsideLoadSE {
                parts: [LoadPartSE {
                  name: IImpreciseNameS::CodeName(CodeNameS {
                    name: StrI("begin"),
                  }),
                  ..
                }],
                ..
              },
            }),
          arg_exprs:
            [IExpressionSE::LocalLoad(LocalLoadSE {
              name: IVarNameS::IterableName(_),
              target_ownership: LoadAsP::LoadAsBorrow,
              ..
            })],
          ..
        }),
      ..
    })) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::While(_)) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::Let(LetSE {
      pattern: AtomSP {
        name:
          Some(CaptureS {
            name: IVarNameS::IterationOptionName(_),
            mutate: false,
          }),
        coord_rune: None,
        destructure: None,
        ..
      },
      expr:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OverloadSet(OverloadSetSE {
              lookup: OutsideLoadSE {
                parts: [LoadPartSE {
                  name: IImpreciseNameS::CodeName(CodeNameS {
                    name: StrI("next"),
                  }),
                  ..
                }],
                ..
              },
            }),
          arg_exprs:
            [IExpressionSE::LocalLoad(LocalLoadSE {
              name: IVarNameS::IteratorName(_),
              target_ownership: LoadAsP::LoadAsBorrow,
              ..
            })],
          ..
        }),
      ..
    })) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::FunctionCall(FunctionCallSE {
      callable_expr:
        IExpressionSE::OverloadSet(OverloadSetSE {
          lookup: OutsideLoadSE {
            parts: [LoadPartSE {
              name: IImpreciseNameS::CodeName(CodeNameS {
                name: StrI("isEmpty"),
              }),
              ..
            }],
            ..
          },
        }),
      arg_exprs:
        [IExpressionSE::LocalLoad(LocalLoadSE {
          name: IVarNameS::IterationOptionName(_),
          target_ownership: LoadAsP::LoadAsBorrow,
          ..
        })],
      ..
    })) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::Break(_)) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::Let(LetSE {
      pattern: AtomSP {
        name:
          Some(CaptureS {
            name: IVarNameS::CodeVarName(StrI("i")),
            mutate: false,
          }),
        coord_rune: None,
        destructure: None,
        ..
      },
      expr:
        IExpressionSE::FunctionCall(FunctionCallSE {
          callable_expr:
            IExpressionSE::OverloadSet(OverloadSetSE {
              lookup: OutsideLoadSE {
                parts: [LoadPartSE {
                  name: IImpreciseNameS::CodeName(CodeNameS {
                    name: StrI("get"),
                  }),
                  ..
                }],
                ..
              },
            }),
          arg_exprs:
            [IExpressionSE::LocalLoad(LocalLoadSE {
              name: IVarNameS::IterationOptionName(_),
              target_ownership: LoadAsP::Use,
              ..
            })],
          ..
        }),
      ..
    })) => Some(())
  );
  let iteration_option_uses = collect_where_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::LocalLoad(LocalLoadSE {
      name: IVarNameS::IterationOptionName(_),
      target_ownership: LoadAsP::Use,
      ..
    })) => Some(())
  );
  assert!(!iteration_option_uses.is_empty());
}
/*
  test("foreach") {
    val program1 = compile(
      """func main() {
        |  myList = 0;
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
        LocalLoadSE(_,CodeVarNameS(StrI("myList")),UseP)) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IteratorNameS(_), false)),None,None),
        FunctionCallSE(_,_,
            OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("begin")),_)))),
          Vector(LocalLoadSE(_,IterableNameS(_),LoadAsBorrowP)))) =>
    }
    body.block shouldHave {
      case WhileSE(_, _) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IterationOptionNameS(_), false)),None,None),
        FunctionCallSE(_,_,
            OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("next")),_)))),
          Vector(
            LocalLoadSE(_,IteratorNameS(_),LoadAsBorrowP)))) =>
    }
    body.block shouldHave {
      case FunctionCallSE(_,_,
          OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("isEmpty")),_)))),
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
            OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("get")),_)))),
          Vector(LocalLoadSE(_,IterationOptionNameS(_),UseP)))) =>
    }
    body.block shouldHave {
      case LocalLoadSE(_,IterationOptionNameS(_),UseP) =>
    }
  }
*/
#[test]
fn this_isnt_special_if_was_explicit_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func moo(self &MyStruct) {
      println(self.x);
    }",
  );
  let moo = program.lookup_function("moo");
  let code_body = cast!(&moo.body, IBodyS::CodeBody);
  let function_call = collect_only_snode!(
    NodeRefS::Program(&program),
    NodeRefS::Expression(IExpressionSE::FunctionCall(function_call)) => Some(function_call)
  );
  let overload_set = cast!(function_call.callable_expr, IExpressionSE::OverloadSet);
  let load_part = expect_1(overload_set.lookup.parts);
  let code_name = cast!(&load_part.name, IImpreciseNameS::CodeName);
  assert_eq!(code_name.name.as_str(), "println");
  let dot = cast!(expect_1(&function_call.arg_exprs), IExpressionSE::Dot);
  assert_eq!(dot.member.as_str(), "x");
  assert!(dot.borrow_container);
  let local_load = cast!(dot.left, IExpressionSE::LocalLoad);
  let code_var_name = cast!(&local_load.name, IVarNameS::CodeVarName);
  assert_eq!(code_var_name.as_str(), "self");
  assert_eq!(local_load.target_ownership, LoadAsP::LoadAsBorrow);

  let function_calls = collect_where_snode!(
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
        OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("println")), _)))),
        Vector(DotSE(_, LocalLoadSE(_, CodeVarNameS(StrI("self")), LoadAsBorrowP), StrI("x"), true))) =>
    })
    Collector.all(main.body, { case FunctionCallSE(_, _, _, _) => }).size shouldEqual 1
  }
*/
#[test]
fn reports_when_mutating_nonexistant_local() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
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
#[test]
fn reports_when_extern_function_has_body() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "extern func bork() int {\n  3\n}",
  );
  match &err {
    ICompileErrorS::ExternHasBodyS(_) => {}
    _ => panic!("expected ExternHasBody(_), got {:?}", err),
  }
}
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
#[test]
fn reports_when_we_forget_set() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    r#"
exported func main() {
  x = "world!";
  x = "changed";
}"#,
  );
  match &err {
    ICompileErrorS::VariableNameAlreadyExists(
      VariableNameAlreadyExists {
        name: IVarNameS::CodeVarName(StrI("x")),
        ..
      },
    ) => {}
    _ => panic!("expected VariableNameAlreadyExists(_, CodeVarName(\"x\")), got {:?}", err),
  }
}
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
#[test]
fn reports_when_interface_method_doesnt_have_self() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "interface IMoo { func blork(a bool)void; }",
  );
  match &err {
    ICompileErrorS::InterfaceMethodNeedsSelf(_) => {}
    _ => panic!("expected InterfaceMethodNeedsSelf(_), got {:?}", err),
  }
}
/*
  test("Reports when interface method doesnt have self") {
    val err = compileForError("interface IMoo { func blork(a bool)void; }")
    err match {
      case InterfaceMethodNeedsSelf(_) =>
      case _ => vfail()
    }
  }
*/
#[test]
fn statement_after_result_or_return() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    r"
func doCivicDance(virtual this Car) {
  return 4;
  7
}",
  );
  match &err {
    ICompileErrorS::StatementAfterReturnS(_) => {}
    _ => panic!("expected StatementAfterReturnS(_), got {:?}", err),
  }
}
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
#[test]
fn report_type_mismatch() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    r"
struct Vec<N, T> where N Int
{
  values [#N]<imm>T;
}
",
  );
  match &err {
    ICompileErrorS::RuneExplicitTypeConflictS(
      RuneExplicitTypeConflictS {
        rune: IRuneS::CodeRune(CodeRuneS {
          name: StrI("N"),
        }),
        ..
      },
    ) => {}
    _ => panic!("expected RuneExplicitTypeConflictS(_, CodeRune(\"N\"), _), got {:?}", err),
  }
}
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
  test("Free function call with explicit template arg makes single-part OutsideLoadSE with one explicit arg rune") {
    val program1 = compile("exported func main() { f<int>(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val outside =
      Collector.only(block, {
        case OverloadSetSE(x @ OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("f")), Vector(_))))) => x
      })
    // The single explicit arg rune should be tied to a lookup of `int` in the rules.
    val Vector(LoadPartSE(_, Vector(argRune))) = outside.parts
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == argRune)
    }
  }
  test("Namespace method call makes two-part OutsideLoadSE in source order (container first, function last)") {
    val program1 = compile("exported func main() { Vec<int>.foo(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val (rules, intRune) =
      Collector.only(block, {
        case OverloadSetSE(OutsideLoadSE(_, rules,
          Vector(
            LoadPartSE(CodeNameS(StrI("Vec")), Vector(intRune)),
            LoadPartSE(CodeNameS(StrI("foo")), Vector())
          ))) => (rules, intRune)
      })
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == intRune)
    }
  }
  test("Namespace method call with multi-arg container makes part with multiple explicit arg runes") {
    val program1 = compile("exported func main() { Pair<int, bool>.make(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val (rules, intRune, boolRune) =
      Collector.only(block, {
        case OverloadSetSE(OutsideLoadSE(_, rules,
          Vector(
            LoadPartSE(CodeNameS(StrI("Pair")), Vector(intRune, boolRune)),
            LoadPartSE(CodeNameS(StrI("make")), Vector()))
        )) => (rules, intRune, boolRune)
      })
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == intRune)
    }
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("bool"))) => vassert(r == boolRune)
    }
  }
  // Ignored: postparser currently drops method-level template args on namespace method calls.
  // For `S<int>.foo<bool>()`, only `int` makes it into the rules/explicit-args; the `<bool>` on
  // `foo` is silently lost. Same blocker as `Namespace method call with both container and method
  // generic args` in CompilerTests. When that's lifted, this test should pass.
  ignore("Namespace method call with both container and method args makes both parts have explicit arg runes") {
    val program1 = compile("exported func main() { S<int>.foo<bool>(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val (rules, intRune, boolRune) =
      Collector.only(block, {
        case OverloadSetSE(OutsideLoadSE(_, rules,
          Vector(
            LoadPartSE(CodeNameS(StrI("S")), Vector(intRune)),
            LoadPartSE(CodeNameS(StrI("foo")), Vector(boolRune))),
        )) => (rules, intRune, boolRune)
      })
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == intRune)
    }
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("bool"))) => vassert(r == boolRune)
    }
  }
  // Ignored: pre-existing FunctionScout limitation rejects user-declared runes on
  // interface/struct internal methods (FunctionScout.scala:114 — `case ParentInterface(_, _, _, _)
  // => vassert(userDeclaredRunes.isEmpty)`). Same blocker as the "Namespace method call with both
  // container and method generic args" CompilerTests test. When that's lifted, this should pass
  // with parent-at-end ordering (Z before K before V).
  ignore("Interface internal method has identifying runes [own, parent...] (parent runes at end)") {
    val interner = new Interner()
    val program1 = compile(
      """interface IS<K, V> { func zork<Z>(virtual this &IS<K, V>, z Z)void; }""",
      interner)
    val is = program1.lookupInterface("IS")
    val zork = is.internalMethods.head
    val zorkRunes = zork.genericParams.map(_.rune.rune)
    val z = CodeRuneS(interner.intern(StrI("Z")))
    val k = CodeRuneS(interner.intern(StrI("K")))
    val v = CodeRuneS(interner.intern(StrI("V")))
    val zIdx = zorkRunes.indexOf(z)
    val kIdx = zorkRunes.indexOf(k)
    val vIdx = zorkRunes.indexOf(v)
    vassert(zIdx >= 0 && kIdx >= 0 && vIdx >= 0)
    vassert(zIdx < kIdx)
    vassert(kIdx < vIdx)
  }
  // Ignored: struct internal methods are silently dropped at the postparser today
  // (PostParser.scala:681 — `case StructMethodP(_) => Vector.empty`). PR 2.5 wires them
  // through the same way as interface internal methods. Once that lands, uncomment the
  // body below; it should pass with the same parent-at-end ordering as interfaces.
  ignore("Struct internal method has identifying runes [own, parent...] (parent runes at end)") {
//    val interner = new Interner()
//    val program1 = compile(
//      """struct S<K, V> { func zork<Z>(self &S<K, V>, z Z)void { } }""",
//      interner)
//    val s = program1.lookupStruct("S")
//
//    val zork = s.internalMethods.head
//    val zorkRunes = zork.genericParams.map(_.rune.rune)
//    val z = CodeRuneS(interner.intern(StrI("Z")))
//    val k = CodeRuneS(interner.intern(StrI("K")))
//    val v = CodeRuneS(interner.intern(StrI("V")))
//    // Own rune Z first; inherited K and V from S at the end (in their declared order).
//    val zIdx = zorkRunes.indexOf(z)
//    val kIdx = zorkRunes.indexOf(k)
//    val vIdx = zorkRunes.indexOf(v)
//    vassert(zIdx >= 0 && kIdx >= 0 && vIdx >= 0)
//    vassert(zIdx < kIdx)
//    vassert(kIdx < vIdx)
  }
  // Ignored: FunctionScout.scala:114 rejects user-declared runes on struct internal methods.
  // When that's lifted, zork's genericParams should be [N, K, V] per @PRIIROZ.
  ignore("Struct internal method zork<N> in S<K, V> has generic params [N, K, V] (own first, parent runes at end)") {
    val interner = new Interner()
    val program1 = compile(
      """struct S<K, V> { func zork<N>(self &S<K, V>, n N)void { } }""",
      interner)
    val s = program1.lookupStruct("S")
    val zork = s.internalMethods.head
    val zorkRunes = zork.genericParams.map(_.rune.rune)
    val n = CodeRuneS(interner.intern(StrI("N")))
    val k = CodeRuneS(interner.intern(StrI("K")))
    val v = CodeRuneS(interner.intern(StrI("V")))
    val nIdx = zorkRunes.indexOf(n)
    val kIdx = zorkRunes.indexOf(k)
    val vIdx = zorkRunes.indexOf(v)
    vassert(nIdx >= 0 && kIdx >= 0 && vIdx >= 0)
    vassert(nIdx < kIdx)
    vassert(kIdx < vIdx)
  }
  // Ignored: parser doesn't yet support nested container chains like S<X>.T<Y>.foo().
  // When it does, parts should come out in source order: [S(int,bool), T(float), zork(double)].
  // At the definition site, zork's genericParams would be [N, Q, K, V] per @PRIIROZ
  // (own first, then containers' innermost-first).
  ignore("Three-part chain S<K, V>.T<Q>.zork<N> makes three-part OutsideLoadSE in source order") {
    val program1 = compile("exported func main() { S<int, bool>.T<float>.zork<double>(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, block)) = main.body
    val outside =
      Collector.only(block, {
        case OverloadSetSE(x @ OutsideLoadSE(_, _,
          Vector(
            LoadPartSE(CodeNameS(StrI("S")), Vector(_, _)),
            LoadPartSE(CodeNameS(StrI("T")), Vector(_)),
            LoadPartSE(CodeNameS(StrI("zork")), Vector(_))),
        )) => x
      })
    val Vector(
      LoadPartSE(_, Vector(sArg0Rune, sArg1Rune)),
      LoadPartSE(_, Vector(tArgRune)),
      LoadPartSE(_, Vector(zorkArgRune))) = outside.parts
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == sArg0Rune)
    }
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("bool"))) => vassert(r == sArg1Rune)
    }
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("float"))) => vassert(r == tArgRune)
    }
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("double"))) => vassert(r == zorkArgRune)
    }
  }
*/

#[test]
fn foreach_expr() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func main() {
      c = 0;
      a = foreach i in c { i };
    }",
  );
  let main = program.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let root_expr = code_body.body.block.expr;

  let map_exprs = collect_where_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::Map(_)) => Some(())
  );
  assert_eq!(map_exprs.len(), 1);

  let while_exprs = collect_where_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::While(_)) => Some(())
  );
  assert_eq!(while_exprs.len(), 0);
}
/*
// MIGALLOW: new foreach_expr test is fine
*/
/*
}
*/

// NOVEL CODE — TDD reproducer for the `destruct` expression scout panic
// surfaced by typing_pass_on_roguelike. The Scala equivalent is `case
// DestructPE(range, innerPE) => ...` at ExpressionScout.scala:393.
#[test]
fn destruct_expression() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "struct MyStruct { a int; }\nexported func main() { m = MyStruct(7); destruct m; }",
  );
  let main = program.lookup_function("main");
  let _code_body = cast!(&main.body, IBodyS::CodeBody);
  // Just ensure scout completed without panicking.
}

// NOVEL CODE — TDD reproducer for the AndPE/OrPE expression scout panic
// surfaced by typing_pass_on_roguelike. Scala equivalent at
// ExpressionScout.scala:605/628 uses `newIf` to expand `&&` / `||` into
// short-circuiting conditionals.
#[test]
fn and_or_expression() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() bool { return true and false or true; }",
  );
  let main = program.lookup_function("main");
  let _code_body = cast!(&main.body, IBodyS::CodeBody);
}

// NOVEL CODE — TDD reproducer for the TuplePE expression scout panic
// surfaced by typing_pass_on_roguelike. The Scala equivalent is
// `case TuplePE(range, elementsPE) => ...` at ExpressionScout.scala:486.
#[test]
fn tuple_expression() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() { x = (3, 4); }",
  );
  let main = program.lookup_function("main");
  let _code_body = cast!(&main.body, IBodyS::CodeBody);
}

// NOVEL CODE — TDD reproducer for the StrInterpolatePE expression scout
// panic surfaced by typing_pass_on_roguelike. The Scala equivalent is
// `case StrInterpolatePE(range, partsPE) => ...` at ExpressionScout.scala:254.
#[test]
fn str_interpolate_expression() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() str { return \"\"; }",
  );
  let main = program.lookup_function("main");
  let _code_body = cast!(&main.body, IBodyS::CodeBody);
  // Just ensure scout completed without panicking.
}