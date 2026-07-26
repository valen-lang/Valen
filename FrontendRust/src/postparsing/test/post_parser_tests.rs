// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib postparsing::test::post_parser_tests

use bumpalo::Bump;
use crate::cast;
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::parsing::ast::{IMacroInclusionP, LoadAsP};
use crate::postparsing::ast::{IFunctionAttributeS, IStructMemberS, ProgramS};
use crate::postparsing::expressions::{
  ConstantIntSE, DotSE, FunctionCallSE, IExpressionSE, IVariableUseCertainty, LetSE, LoadPartSE, LocalLoadSE,
  LocalS, OutsideLoadSE, OverloadSetSE, OwnershippedSE, ReturnSE,
};
use crate::postparsing::patterns::patterns::{AtomSP, CaptureS};
use crate::postparsing::names::{CodeNameS, CodeRuneS, IFunctionDeclarationNameS, IImpreciseNameS, IRuneS, IRuneValS, IVarNameS};
use crate::postparsing::post_parser::{ICompileErrorS, PostParser};
use crate::postparsing::rules::rules::{CallSR, ILiteralSL, ImplBoundS, LiteralSR, LookupSR};
use crate::postparsing::test::traverse::NodeRefS;
use crate::parsing::tests::utils::compile_file;
use crate::parsing::tests::utils::{expect_1, expect_2, expect_3};
use crate::postparsing::ast::IBodyS;
use crate::parsing::ast::SharednessP;
use crate::postparsing::ast::IGenericParameterTypeS;
use crate::postparsing::expressions::ConstantBoolSE;
use crate::postparsing::ast::ParameterS;
use crate::postparsing::rules::RuneUsage;
use crate::postparsing::expressions::ConsecutorSE;
use crate::postparsing::post_parser::VariableNameAlreadyExists;
use crate::collect_only_snode;
use crate::collect_only_snodes;
use crate::collect_where_snode;
use crate::collect_where_snodes;
use crate::postparsing::test::utils::{assert_rune_absent_from_rules, assert_rune_resolves_to, expect_code_body_expr};


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

#[test]
fn test_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(&scout_arena, &keywords, &parse_arena, "struct Moo { x int; }");
  let imoo = program.lookup_struct("Moo");

  assert_eq!(imoo.sharedness, SharednessP::Single);

  let only_member = expect_1(&imoo.members);
  // `int` (a bare type-name) lowers to Lookup(int) + Call([]); the member's type is the
  // Call's result rune, not the raw Lookup rune.
  collect_only_snode!(
    NodeRefS::Struct(imoo),
    NodeRefS::LookupRule(
      LookupSR {
        name: IImpreciseNameS::CodeName(code_name),
        ..
      }
    ) if code_name.name.as_str() == "int" => Some(())
  );
  collect_only_snode!(
    NodeRefS::Struct(imoo),
    NodeRefS::CallRule(CallSR { result_rune, args, .. })
      if result_rune.rune == only_member.type_rune().rune && args.is_empty() => Some(())
  );

  let normal_member = cast!(only_member, IStructMemberS::NormalStructMember);
  assert_eq!(normal_member.name.as_str(), "x");
}

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
  assert!(matches!(&first_generic_param.tyype, IGenericParameterTypeS::KindGenericParameterType(_)));
  assert!(matches!(&second_generic_param.tyype, IGenericParameterTypeS::KindGenericParameterType(_)));
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

#[test]
fn impl_() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(&scout_arena, &keywords, &parse_arena, "impl IMoo for Moo;");
  let impl_ = expect_1(program.impls);

  // Each of `Moo` / `IMoo` (bare type-names) lowers to Lookup(name) + Call([]); the kind runes
  // are the Call results, not the raw Lookup runes.
  collect_only_snode!(
    NodeRefS::Impl(impl_),
    NodeRefS::LookupRule(LookupSR {
      name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("Moo"), .. }),
      ..
    }) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Impl(impl_),
    NodeRefS::LookupRule(LookupSR {
      name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("IMoo"), .. }),
      ..
    }) => Some(())
  );
  collect_only_snode!(
    NodeRefS::Impl(impl_),
    NodeRefS::CallRule(CallSR { result_rune, args, .. })
      if result_rune.rune == impl_.struct_kind_rune.rune && args.is_empty() => Some(())
  );
  collect_only_snode!(
    NodeRefS::Impl(impl_),
    NodeRefS::CallRule(CallSR { result_rune, args, .. })
      if result_rune.rune == impl_.interface_kind_rune.rune && args.is_empty() => Some(())
  );
}

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
              name: IVarNameS::CodeVarName(StrI("x")),              ..
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

#[test]
fn function_with_magic_lambda_and_regular_lambda() {
  // Lambda params get the right ParameterS: a magic-param lambda's 2nd param is a MagicParamName
  // with a MagicParamRune; a regular lambda's named 2nd param `a` is a CodeVarName with an implicit rune.
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

  match first_lambda.function.params {
    [_, ParameterS {
      pre_checked: false,
      name: IVarNameS::MagicParamName(_),
      full_type_rune: RuneUsage { rune: IRuneS::MagicParamRune(_), .. },
      ..
    }] => {}
    other => panic!("expected first lambda's 2nd param to be a magic param, got {:?}", other),
  }

  match second_lambda.function.params {
    [_, ParameterS {
      pre_checked: false,
      name: IVarNameS::CodeVarName(StrI("a")),
      full_type_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      ..
    }] => {}
    other => panic!("expected second lambda's 2nd param to be code var a with implicit rune, got {:?}", other),
  }
}

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
            name: IVarNameS::ConstructingMemberName(StrI("x")),            ..
          }),
          IExpressionSE::LocalLoad(LocalLoadSE {
            name: IVarNameS::ConstructingMemberName(StrI("y")),            ..
          }),
        ],
        ..
      })
    ) => Some(())
  );
}

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
                  name: IVarNameS::CodeVarName(StrI("moo")),                  ..
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
        name: IVarNameS::ConstructingMemberName(StrI("x")),        ..
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
          name: IVarNameS::ConstructingMemberName(StrI("x")),          ..
        }),
        IExpressionSE::LocalLoad(LocalLoadSE {
          name: IVarNameS::ConstructingMemberName(StrI("y")),          ..
        }),
      ],
      ..
    })) => Some(())
  );
}

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
        kind_rune: None,
        destructure: None,
        ..
      },
      expr:
        IExpressionSE::LocalLoad(LocalLoadSE {
          name: IVarNameS::CodeVarName(StrI("myList")),          ..
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
        kind_rune: None,
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
        kind_rune: None,
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
        kind_rune: None,
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
              name: IVarNameS::IterationOptionName(_),              ..
            })],
          ..
        }),
      ..
    })) => Some(())
  );
  let iteration_option_uses = collect_where_snode!(
    NodeRefS::Expression(root_expr),
    NodeRefS::Expression(IExpressionSE::LocalLoad(LocalLoadSE {
      name: IVarNameS::IterationOptionName(_),      ..
    })) => Some(())
  );
  assert!(!iteration_option_uses.is_empty());
}

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

  let function_calls = collect_where_snode!(
    NodeRefS::Program(&program),
    NodeRefS::Expression(IExpressionSE::FunctionCall(_)) => Some(())
  );
  assert_eq!(function_calls.len(), 1);

  let _ = code_body;
}

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
#[test]
fn test_named_param_keeps_its_name_at_postparse() {
  // A user-named param keeps its real name as `ParameterS.name` (no desugaring): `foo(x int)`
  // -> CodeVarName("x"). A synthetic DesugaredParamName is only minted for an anonymous or
  // ignored param, and no body-head LetSE is synthesized when there's no destructure.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(x int) int { return x; }",
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS { name: IVarNameS::CodeVarName(StrI("x")), .. }] => {}
    other => panic!("expected one param named x, got {:?}", other),
  }
}

use crate::postparsing::rules::rules::{IRulexSR, RegionSR};

#[test]
fn test_param_no_outer_wrap_routing() {
  // A param routes its rules to its own slices, not the shared FunctionS.rules. For `x int`
  // (no wraps): type_outer_ref_rules is empty, value_type_rules is [Lookup(int)], and the int
  // Lookup didn't leak into FunctionS.rules (just [Lookup(void)] for the void return).
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(x int) void { }",
  );
  let foo = program.lookup_function("foo");
  // A bare type-name is a zero-arg call: `int` lowers to Lookup(int) + Call([]) (and the
  // explicit `void` return likewise). value_type_rune is the Call's result rune.
  match (foo.params, foo.rules) {
    ([ParameterS {
        type_outer_ref_rules: [],
        value_type_rules: [
          IRulexSR::Lookup(LookupSR { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. }), .. }),
          IRulexSR::Call(CallSR { result_rune: value_call_result, args: [], .. }),
        ],
        full_type_rune, value_type_rune, .. }],
     [IRulexSR::Lookup(LookupSR { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("void"), .. }), .. }),
      IRulexSR::Call(CallSR { args: [], .. })]) => {
      assert_eq!(full_type_rune.rune, value_type_rune.rune, "full == value when there's no outer wrap");
      assert_eq!(value_type_rune.rune, value_call_result.rune, "value type is the Call's result rune");
    }
    other => panic!("expected `x int` param (no outer wraps, value [Lookup(int), Call([])]) and fn rules [Lookup(void), Call([])]; got {:?}", other),
  }
}

#[test]
fn test_param_single_ref_wrap_routing() {
  // One param `x &int`: value_type_rules is [Lookup(int)] and type_outer_ref_rules is exactly one
  // BorrowRef whose result is full_type_rune and whose inner is value_type_rune (so full != value).
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(x &int) int { return 0; }",
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS {
        value_type_rules: [
          IRulexSR::Lookup(LookupSR { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. }), .. }),
          IRulexSR::Call(CallSR { args: [], .. }),
        ],
        type_outer_ref_rules: [IRulexSR::BorrowRef(br)],
        full_type_rune, value_type_rune, .. }] => {
      assert_ne!(full_type_rune.rune, value_type_rune.rune, "full != value when there IS an outer wrap");
      assert_eq!(br.result_rune.rune, full_type_rune.rune);
      assert_eq!(br.inner_rune.rune, value_type_rune.rune);
    }
    other => panic!("expected `x &int`: one BorrowRef wrapping [Lookup(int)]; got {:?}", other),
  }
}

#[test]
fn test_param_held_ref_wrap_routing() {
  // One param `x held int`: value_type_rules is [Lookup(int)] and type_outer_ref_rules is exactly
  // one BorrowRef with region Held, whose result is full_type_rune and inner is value_type_rune.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    r#"
exported func foo(x held int) int { return 0; }
"#,
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS {
        value_type_rules: [
          IRulexSR::Lookup(LookupSR { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. }), .. }),
          IRulexSR::Call(CallSR { args: [], .. }),
        ],
        type_outer_ref_rules: [IRulexSR::BorrowRef(br)],
        full_type_rune, value_type_rune, .. }] => {
      assert_eq!(br.region, RegionSR::Held);
      assert_ne!(full_type_rune.rune, value_type_rune.rune, "full != value when there IS an outer wrap");
      assert_eq!(br.result_rune.rune, full_type_rune.rune);
      assert_eq!(br.inner_rune.rune, value_type_rune.rune);
    }
    other => panic!("expected `x held int`: one BorrowRef(Held) wrapping [Lookup(int)]; got {:?}", other),
  }
}

#[test]
fn test_param_own_ref_wrap_routing() {
  // One param `x own int`: value_type_rules is [Lookup(int)] and type_outer_ref_rules is exactly
  // one OwnRef whose result is full_type_rune and inner is value_type_rune.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    r#"
exported func foo(x own int) int { return 0; }
"#,
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS {
        value_type_rules: [
          IRulexSR::Lookup(LookupSR { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. }), .. }),
          IRulexSR::Call(CallSR { args: [], .. }),
        ],
        type_outer_ref_rules: [IRulexSR::OwnRef(or)],
        full_type_rune, value_type_rune, .. }] => {
      assert_ne!(full_type_rune.rune, value_type_rune.rune, "full != value when there IS an outer wrap");
      assert_eq!(or.result_rune.rune, full_type_rune.rune);
      assert_eq!(or.inner_rune.rune, value_type_rune.rune);
    }
    other => panic!("expected `x own int`: one OwnRef wrapping [Lookup(int)]; got {:?}", other),
  }
}

#[test]
fn test_param_nested_ref_wrap_routing() {
  // One param `x &&int`: type_outer_ref_rules is two chained BorrowRefs. It's built during a
  // post-order recursion, so index 0 is the innermost wrap and index 1 the outer.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(x &&int) int { return 0; }",
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS {
        type_outer_ref_rules: [IRulexSR::BorrowRef(inner_br), IRulexSR::BorrowRef(outer_br)],
        full_type_rune, value_type_rune, .. }] => {
      assert_eq!(outer_br.result_rune.rune, full_type_rune.rune);
      assert_eq!(outer_br.inner_rune.rune, inner_br.result_rune.rune);
      assert_eq!(inner_br.inner_rune.rune, value_type_rune.rune);
    }
    other => panic!("expected `x &&int`: two chained BorrowRefs; got {:?}", other),
  }
}

#[test]
fn test_function_rules_no_longer_contains_param_rules() {
  // Param type rules live on their params, not on FunctionS.rules: for `foo(x int, y bool) void`,
  // FunctionS.rules is exactly the void return type's rules (Lookup(void) + Call([]) — the bare
  // type-name zero-arg application), with no int or bool rules leaking in.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(x int, y bool) void { }",
  );
  let foo = program.lookup_function("foo");
  match foo.rules {
    [IRulexSR::Lookup(LookupSR { name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("void"), .. }), .. }),
     IRulexSR::Call(CallSR { args: [], .. })] => {}
    other => panic!("FunctionS.rules should be exactly [Lookup(void), Call([])]; got {:?}", other),
  }
}

#[test]
fn test_function_where_implements_becomes_an_impl_bound() {
  // `where implements(T, IShip)` is a declared bound, not a rule: it never deduces anything, so it
  // is recorded on the denizen for the post-solve pass rather than pushed into FunctionS.rules.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "interface IShip { }\nfunc launch<T>(x T) void where implements(T, IShip) { }",
  );
  let launch = program.lookup_function("launch");
  let bound = expect_1(launch.impl_bounds);

  // The sub side is the declared generic param. The super side is a bare name, so it resolves to
  // a fresh rune that a Lookup rule constrains — the bound points at that rune, not at the name.
  match bound {
    ImplBoundS {
      sub_rune: RuneUsage { rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T"), .. }), .. },
      super_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      result_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      ..
    } => {}
    other => panic!("expected implements(T, <lookup>) with an implicit result rune; got {:?}", other),
  }

  assert_rune_resolves_to(launch.rules, bound.super_rune.rune, "IShip");
  assert_rune_absent_from_rules(launch.rules, bound.result_rune.rune);
}

#[test]
fn test_struct_where_implements_becomes_an_impl_bound() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "interface IShip { }\nstruct Fleet<T> where implements(T, IShip) { }",
  );
  let fleet = program.lookup_struct("Fleet");
  let bound = expect_1(fleet.impl_bounds);
  match bound {
    ImplBoundS {
      sub_rune: RuneUsage { rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T"), .. }), .. },
      super_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      result_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      ..
    } => {}
    other => panic!("expected implements(T, <lookup>) on the struct; got {:?}", other),
  }
  // A struct's where-clause rules land in header_rules, so that is where IShip's lookup goes.
  assert_rune_resolves_to(fleet.header_rules, bound.super_rune.rune, "IShip");
  assert_rune_absent_from_rules(fleet.header_rules, bound.result_rune.rune);
}

#[test]
fn test_interface_where_implements_becomes_an_impl_bound() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "interface IShip { }\ninterface IFleet<T> where implements(T, IShip) { }",
  );
  let fleet = program.lookup_interface("IFleet");
  let bound = expect_1(fleet.impl_bounds);
  match bound {
    ImplBoundS {
      sub_rune: RuneUsage { rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T"), .. }), .. },
      super_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      result_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      ..
    } => {}
    other => panic!("expected implements(T, <lookup>) on the interface; got {:?}", other),
  }
  assert_rune_resolves_to(fleet.rules, bound.super_rune.rune, "IShip");
  assert_rune_absent_from_rules(fleet.rules, bound.result_rune.rune);
}

#[test]
fn test_impl_where_implements_becomes_an_impl_bound() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    concat!(
      "interface IShip { }\n",
      "interface IFleet { }\n",
      "struct Fleet<T> { }\n",
      "impl<T> IFleet for Fleet<T> where implements(T, IShip);",
    ),
  );
  let impl_ = expect_1(program.impls);
  let bound = expect_1(impl_.impl_bounds);
  match bound {
    ImplBoundS {
      sub_rune: RuneUsage { rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T"), .. }), .. },
      super_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      result_rune: RuneUsage { rune: IRuneS::ImplicitRune(_), .. },
      ..
    } => {}
    other => panic!("expected implements(T, <lookup>) on the impl; got {:?}", other),
  }
  assert_rune_resolves_to(impl_.rules, bound.super_rune.rune, "IShip");
  assert_rune_absent_from_rules(impl_.rules, bound.result_rune.rune);
}

#[test]
fn test_bare_param_keeps_name_and_gets_no_body_let() {
  // A body-head LetSE is synthesized only for a param that destructures. A bare param keeps its
  // real name and needs no let: the name IS the binding. `let [a, b] = <param>` is emitted for
  // `Pair[a, b]`, but nothing is emitted for `x int`.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(x int) void { }",
  );
  let foo = program.lookup_function("foo");
  // The bare param keeps its real name; no synthetic DesugaredParamName.
  match foo.params {
    [ParameterS { name: IVarNameS::CodeVarName(StrI("x")), .. }] => {}
    other => panic!("expected one param named x, got {:?}", other),
  }
  // A bare param produces no body-head let, so the empty body is left untouched: its head
  // is the plain Void of `{ }`, not a ConsecutorSE prepending a param LetSE.
  match expect_code_body_expr(&foo.body) {
    IExpressionSE::Void(_) => {}
    other => panic!("expected an untouched Void body head (no param let), got {:?}", other),
  }
}

#[test]
fn test_destructure_param_desugars_to_let_with_destructure() {
  // An anonymous destructuring param `Pair[a, b]` desugars to a body-head `let [a, b] =
  // load(<param>)`: the param's synthetic slot loaded into a nameless top pattern that
  // destructures into a and b.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(Pair[a, b]) void { }",
  );
  let foo = program.lookup_function("foo");
  match expect_code_body_expr(&foo.body) {
    IExpressionSE::Consecutor(ConsecutorSE { exprs: [
      IExpressionSE::Let(LetSE {
        pattern: AtomSP {
          name: None,
          destructure: Some([
            AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("a")), .. }), .. },
            AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("b")), .. }), .. },
          ]), .. },
        expr: IExpressionSE::LocalLoad(LocalLoadSE { name: IVarNameS::DesugaredParamName(_), .. }),
        .. }),
      ..
    ], .. }) => {}
    other => panic!("expected body head `let [a, b] = load(<param>)`, got {:?}", other),
  }
}

#[test]
fn test_named_destructure_param_keeps_name_and_gets_let() {
  // A named destructuring param `p Pair[a, b]` keeps its real name `p` on the ParameterS AND
  // gets a body-head `let [a, b] = load(p)`, so p, a, and b are all available. The let's top
  // pattern is nameless (the name is the param, not re-bound in the let).
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(p Pair[a, b]) void { }",
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS { name: IVarNameS::CodeVarName(StrI("p")), .. }] => {}
    other => panic!("expected one param named p, got {:?}", other),
  }
  match expect_code_body_expr(&foo.body) {
    IExpressionSE::Consecutor(ConsecutorSE { exprs: [
      IExpressionSE::Let(LetSE {
        pattern: AtomSP {
          name: None,
          destructure: Some([
            AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("a")), .. }), .. },
            AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("b")), .. }), .. },
          ]), .. },
        expr: IExpressionSE::LocalLoad(LocalLoadSE { name: IVarNameS::CodeVarName(StrI("p")), .. }),
        .. }),
      ..
    ], .. }) => {}
    other => panic!("expected body head `let [a, b] = load(p)`, got {:?}", other),
  }
}

// Ensures the synthesized LetSE preserves destructure edge cases: nesting, ignore, empty.

#[test]
fn test_nested_destructure_preserved() {
  // A nested destructure is preserved: `Pair[a, [b, c]]` desugars to `let [a, [b, c]] =
  // load(<param>)`, where the second slot is a nameless sub-destructure into b and c.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(Pair[a, [b, c]]) void { }",
  );
  let foo = program.lookup_function("foo");
  match expect_code_body_expr(&foo.body) {
    IExpressionSE::Consecutor(ConsecutorSE { exprs: [
      IExpressionSE::Let(LetSE {
        pattern: AtomSP { name: None, destructure: Some([
          AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("a")), .. }), .. },
          AtomSP { name: None, destructure: Some([
            AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("b")), .. }), .. },
            AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("c")), .. }), .. },
          ]), .. },
        ]), .. }, .. }),
      ..
    ], .. }) => {}
    other => panic!("expected body head `let [a, [b, c]] = load(<param>)`, got {:?}", other),
  }
}

#[test]
fn test_destructure_ignore() {
  // An ignore slot in a destructure gets no name capture: `Pair[_, b]` desugars to `let [_, b]`
  // where the `_` slot has name None (destructure translation drops IgnoredLocalNameDeclaration).
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(Pair[_, b]) void { }",
  );
  let foo = program.lookup_function("foo");
  match expect_code_body_expr(&foo.body) {
    IExpressionSE::Consecutor(ConsecutorSE { exprs: [
      IExpressionSE::Let(LetSE {
        pattern: AtomSP { name: None, destructure: Some([
          AtomSP { name: None, .. },
          AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("b")), .. }), .. },
        ]), .. }, .. }),
      ..
    ], .. }) => {}
    other => panic!("expected body head `let [_, b] = load(<param>)`, got {:?}", other),
  }
}

#[test]
fn test_empty_destructure() {
  // An empty destructure is preserved: `int[]` desugars to `let [] = load(<param>)`, a nameless
  // top pattern with an empty destructure.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func foo(int[]) void { }",
  );
  let foo = program.lookup_function("foo");
  match expect_code_body_expr(&foo.body) {
    IExpressionSE::Consecutor(ConsecutorSE { exprs: [
      IExpressionSE::Let(LetSE {
        pattern: AtomSP { name: None, destructure: Some([]), .. }, .. }),
      ..
    ], .. }) => {}
    other => panic!("expected body head `[] = load(<param>)`, got {:?}", other),
  }
}

#[test]
fn test_extern_param_destructure_rejected() {
  // An extern/abstract/generated body has no block to prepend a LetSE into, so a param destructure
  // is rejected at postparse with ParamDestructureRequiresBody.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "extern func foo(Pair[a, b]);",
  );
  match err {
    ICompileErrorS::ParamDestructureRequiresBody { .. } => {}
    other => panic!("expected ParamDestructureRequiresBody, got {:?}", other),
  }
}

#[test]
fn test_extern_bare_param_ok() {
  // An extern func with a bare param (no destructure) postparses fine: the param keeps its real
  // name and the body stays an ExternBody.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "extern func foo(x int);",
  );
  let foo = program.lookup_function("foo");
  match foo.params {
    [ParameterS { name: IVarNameS::CodeVarName(StrI("x")), .. }] => {}
    other => panic!("expected one param named x, got {:?}", other),
  }
  match &foo.body {
    IBodyS::ExternBody(_) => {}
    other => panic!("expected an extern body, got {:?}", other),
  }
}

#[test]
fn plain_function_is_marked_user_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func helper() int { return 3; }",
  );
  let helper = program.lookup_function("helper");
  assert!(
    helper.attributes.iter().any(|a| matches!(a, IFunctionAttributeS::UserFunction(_))),
    "expected a UserFunction attribute, got {:?}", helper.attributes);
}

#[test]
fn exported_function_keeps_export_and_is_user_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { return 3; }",
  );
  let main = program.lookup_function("main");
  // The source-written Export attribute survives, and UserFunction is stamped alongside it.
  assert!(
    main.attributes.iter().any(|a| matches!(a, IFunctionAttributeS::Export(_))),
    "expected the source-written Export attribute to survive, got {:?}", main.attributes);
  assert!(
    main.attributes.iter().any(|a| matches!(a, IFunctionAttributeS::UserFunction(_))),
    "expected a UserFunction attribute, got {:?}", main.attributes);
}
