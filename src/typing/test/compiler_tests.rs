use super::compiler_test_compilation::compiler_test_compilation;
use crate::builtins::builtins::{
  builtin_source_bundle, builtin_source_for_arith, builtin_source_for_arrays, builtin_source_for_as,
  builtin_source_for_opt, builtin_source_for_panicutils, builtin_source_for_weak,
  empty_v_builtins_stub, get_embedded_modulized_code_map,
};
use crate::code_source::{CodeSource, Source};
use crate::collect_only_tnode;
use crate::postparsing::ast::LocationInDenizen;
use crate::collect_where_tnode;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::parsing::tests::utils::expect_1;
use crate::postparsing::names::IRuneS;
use crate::postparsing::names::{
  CodeNameS, CodeNameValS, CodeRuneS, FunctionNameS, IFunctionDeclarationNameS, IImpreciseNameS,
  IImpreciseNameValS, INameS, IRuneValS, TopLevelStructDeclarationNameS,
};
use crate::scout_arena::ScoutArena;
use crate::solver::solver::{FailedSolve, ISolverError, RuleError, Step};
use crate::tests::tests::load_expected;
use crate::tests::tests::new_test_package_source;
use crate::tests::tests::{new_humanizer_test_code_map, new_test_code_map};
use crate::typing::ast::ast::FunctionHeaderT;
use crate::typing::ast::ast::LocT;
use crate::typing::ast::ast::ParameterT;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::ast::ast::{KindExportT, SignatureValT};
use crate::typing::ast::citizens::StructDefinitionT;
use crate::typing::ast::citizens::StructMemberT;
use crate::typing::ast::expressions::ConstantIntTE;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::ast::expressions::LetAndLendTE;
use crate::typing::ast::expressions::MemberLookupTE;
use crate::typing::ast::expressions::UpcastTE;
use crate::typing::ast::expressions::{LetNormalTE, LocalLookupTE};
use crate::typing::compiler_error_humanizer::humanize;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::infer_compiler::IResolvingError;
use crate::typing::names::names::FunctionNameT;
use crate::typing::names::names::InterfaceNameT;
use crate::typing::names::names::KindPlaceholderNameT;
use crate::typing::names::names::KindPlaceholderTemplateNameT;
use crate::typing::names::names::{
  MemberNameT, ExportNameT, ExportTemplateNameT, FunctionNameValT, FunctionTemplateNameT, LocalNameT,
  IStructTemplateNameT, IdT, IdValT, InterfaceNameValT, InterfaceTemplateNameT, StructNameT,
  StructNameValT, StructTemplateNameT,
};
use crate::typing::names::names::{INameT, IVarNameT};
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
use crate::typing::templata::templata_utils::unapply_simple_name;
use crate::typing::test::humanize_helper::{assert_humanized_eq, humanize_compile_error};
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::ISuperKindTT;
use crate::typing::types::types::InterfaceTT;
use crate::typing::types::types::KindPlaceholderT;
use crate::typing::types::types::{BoolT, InterfaceTTValT, StructTT, StructTTValT};
use crate::typing::types::types::{BorrowRefT, IntT, KindT, RegionT};
use crate::typing::types::types::{NeverT, SharednessT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::fx::HashMap;
use crate::utils::fx::HashSet;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::utils::source_code_utils::{
  humanize_pos_code_map, line_containing, line_range_containing, lines_between,
};
use bumpalo::Bump;
use std::iter::empty;
use std::marker::PhantomData;

pub struct CompilerTests {}
impl CompilerTests {}

fn read_code_from_resource(resource_filename: &str) -> String {
  panic!("Unimplemented: read_code_from_resource");
}

#[test]
fn simple_program_returning_an_int_explicit() {
  // We had a bug once looking up "int" in the environment, hence this test.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "func main() int { return 3; }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  assert_eq!(main.header.return_type, KindT::Int(IntT { bits: 32 }));
}

#[test]
fn hardcoding_negative_numbers() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() int { return -3; }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::ConstantInt(
          ConstantIntTE {
              value: ITemplataT::Integer(-3),
              ..
          }
      ) => Some(())
  );
}

#[test]
fn simple_local() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
exported func main() int {
  a = 42;
  return ^a;
}";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  assert!(main.header.return_type == KindT::Int(IntT { bits: 32 }));
}

// VCOORD: enable this
#[test]
fn tests_panic_return_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.panic.*;
exported func main() int {
  x = { __vbi_panic() }();
}";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "panic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::LetNormal(LetNormalTE {
          variable: LocalVariable {
              tyype: KindT::Never(NeverT { from_break: false }),
              ..
          },
          ..
      }) => Some(())
  );
}

#[test]
fn taking_an_argument_and_returning_it() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: let code = "func main(a int) int { return a; }";
  let code = "func main(a int) int { return __copy_prim(&a); }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let param: &ParameterT = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Parameter(p) => Some(p)
  );
  assert!(param.tyype == KindT::Int(IntT { bits: 32 }));

  let lookup: &LocalLookupTE = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::LocalLookup(l) => Some(l)
  );
  match lookup.local_variable.name {
    IVarNameT::Local(c) => assert!(c.imprecise_name.name.as_str() == "a"),
    _ => panic!("Expected LocalNameT"),
  }
  match lookup.local_variable.tyype {
    KindT::Int(IntT { bits: 32 }) => {}
    other => panic!("Expected CoordT(Own, _, Int(32)), got {:?}", other),
  }
}

#[test]
fn tests_adding_two_numbers() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: let code = "import v.builtins.arith.*;\nexported func main() int { return +(2, 3); }";
  let code = "import v.builtins.arith.*;\nexported func main() int { a = 2; b = 3; return +(&a, &b); }";
  let code_source = CodeSource::new(vec![
    builtin_source_for_arith(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::ConstantInt(
          ConstantIntTE {
              value: ITemplataT::Integer(2),
              ..
          }
      ) => Some(())
  );

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::ConstantInt(
          ConstantIntTE {
              value: ITemplataT::Integer(3),
              ..
          }
      ) => Some(())
  );

  let func_call: &FunctionCallTE = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(call) => Some(call)
  );

  match func_call.callable.id.local_name {
    INameT::Function(fname) => {
      assert!(fname.template.human_name.as_str() == "+");
    }
    _ => panic!("Expected function name for + operator"),
  }

  assert_eq!(func_call.args.len(), 2);
  // We wrote `a = 2; b = 3; +(&a, &b)` to match the `+(&int, &int)` signature in arith.vale.
  // Borrowing a local is a plain LocalLookup (a lookup is itself a borrow) — no temp, no Defer.
  match &func_call.args[0] {
    ExpressionTE::LocalLookup(LocalLookupTE {
      local_variable: LocalVariable {
        name: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("a"), .. }, .. }),
        ..
      },
      ..
    }) => {}
    other => panic!("Expected arg 0 shape LocalLookup(a), got {:?}", other),
  }
  match &func_call.args[1] {
    ExpressionTE::LocalLookup(LocalLookupTE {
      local_variable: LocalVariable {
        name: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("b"), .. }, .. }),
        ..
      },
      ..
    }) => {}
    other => panic!("Expected arg 1 shape LocalLookup(b), got {:?}", other),
  }
}

// Two sibling rvalue-borrow temporaries (`&Foo()` and `&Bar()` as two args of one call) must have
// their drops spliced in last-created-first (LIFO): the `&Bar()` temp drops before the `&Foo()` one.
// (The hammer dropped siblings in evaluation order (FIFO) — an artifact of its accumulator, pinned
// here to LIFO to match `drop_since` and the intended semantics.)
#[test]
fn sibling_borrow_temps_drop_lifo() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r#"
struct Foo {}
struct Bar {}
func destructor(m Foo) { Foo[ ] = ^m; }
func destructor(m Bar) { Bar[ ] = ^m; }
func bork(a &Foo, b &Bar) { }
exported func main() {
  bork(&Foo(), &Bar());
}
"#;
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let dropped_structs: Vec<&str> = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI(name), .. },
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(name)
  );
  assert_eq!(dropped_structs.len(), 2);
  assert_eq!(dropped_structs[0], "Bar");
  assert_eq!(dropped_structs[1], "Foo");
}

// A non-void consumer's result is preserved across the borrow-temp's drop: `frob(&Foo())` returns an
// int that `main` returns, and the `&Foo()` temp is still dropped (once) — no Defer node remains.
#[test]
fn borrow_temp_preserves_consumer_value() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r#"
struct Foo {}
func destructor(m Foo) { Foo[ ] = ^m; }
func frob(a &Foo) int { return 7; }
exported func main() int {
  return frob(&Foo());
}
"#;
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let foo_drops = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI("Foo"), .. },
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(())
  );
  assert_eq!(foo_drops.len(), 1);
}

// Site 5 (If) — a borrow-temp created in an `if` condition must be dropped right after the condition,
// before the (diverging) branch runs, so it is not orphaned on the `return` path. A correct drain
// discharges the condition's temp there; a buggy one that bubbles it to the statement boundary would
// leave the temp un-unstackified on the return path (a stackifier error, so the fixture would panic).
#[test]
fn if_condition_borrow_temp_drops_before_diverging_branch() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.panic.*;
struct Foo {}
func destructor(m Foo) { Foo[ ] = ^m; }
func check(a &Foo) bool { return true; }
exported func main() int {
  if (check(&Foo())) {
    return 3;
  } else {
    return 5;
  }
  __vbi_panic();
}
";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "panic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let foo_drops = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI("Foo"), .. },
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(())
  );
  assert_eq!(foo_drops.len(), 1);
}

// Site 5 (While) — a borrow-temp in a `while` condition must drop after the condition, not be orphaned
// when the loop `break`s out of the body.
#[test]
fn while_condition_borrow_temp_drops_with_break() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
struct Foo {}
func destructor(m Foo) { Foo[ ] = ^m; }
func check(a &Foo) bool { return true; }
exported func main() {
  while (check(&Foo())) {
    break;
  }
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let foo_drops = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI("Foo"), .. },
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(())
  );
  assert_eq!(foo_drops.len(), 1);
}

// Site 4 — per-statement drain: a borrow-temp created in statement 1 must drop at the end of that
// statement, before statement 2 (`marker()`) runs, rather than bubbling to block end.
#[test]
fn borrow_temp_drops_at_statement_end_before_next_statement() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
struct Foo {}
func destructor(m Foo) { Foo[ ] = ^m; }
func check(a &Foo) bool { return true; }
func marker() { }
exported func main() {
  check(&Foo());
  marker();
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let foo_drops = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI("Foo"), .. },
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(())
  );
  assert_eq!(foo_drops.len(), 1);
}

// Discard-past-Never — when the consuming call itself diverges (`check` returns `__Never`), the
// borrow-temp is unlet WITHOUT dropping: no destructor runs on a path that never returns.
#[test]
fn borrow_temp_discarded_when_consumer_diverges() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.panic.*;
struct Foo {}
func destructor(m Foo) { Foo[ ] = ^m; }
func check(a &Foo) __Never { __vbi_panic(); }
exported func main() {
  check(&Foo());
}
";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "panic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  let foo_drops = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI("Foo"), .. },
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(())
  );
  assert_eq!(foo_drops.len(), 0);
}

#[test]
fn simple_struct_read() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: moo.hp is &int
  let code = r"
exported struct Moo { hp int; }
exported func main(moo &Moo) int {
  return __copy_prim(&moo.hp);
}";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
}

#[test]
fn make_array_and_dot_it() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  x = arr.2;\n"
  let code = r#"
exported func main() int {
  arr = [#]int(6, 60, 103);
  x = __copy_prim(&arr.2);
  [_, _, _] = ^arr;
  return ^x;
}
"#;
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn simple_struct_instantiate() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r#"
exported struct Moo { hp int; }
exported func main() Moo {
  return Moo(42);
}
"#;
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let _main = coutputs.lookup_function_by_str("main");
}

#[test]
fn call_destructor() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  return Moo(42).hp;\n"
  let code = r#"
exported struct Moo { hp int; }
exported func main() int {
  return __copy_prim(&Moo(42).hp);
}
"#;
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  let _drop_call: &FunctionCallTE = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(call @ FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(call)
  );
}

#[test]
fn custom_destructor() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  return Moo(42).hp;\n"
  let code = concat!(
    "#!DeriveStructDrop\n",
    "exported struct Moo { hp int; }\n",
    "func drop(self Moo) {\n",
    "  [_] = ^self;\n",
    "}\n",
    "exported func main() int {\n",
    "  return __copy_prim(&Moo(42).hp);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(
          FunctionCallTE {
              callable: PrototypeT {
                  id: IdT {
                      local_name: INameT::Function(FunctionNameT {
                          template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                          ..
                      }),
                      ..
                  },
                  ..
              },
              ..
          }
      ) => Some(())
  );
}

#[test]
fn make_constraint_reference() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r#"
struct Moo {}
exported func main() void {
  m = Moo();
  b = &m;
}
"#;
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  let let_normal: &LetNormalTE = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::LetNormal(ln @ LetNormalTE {
          variable: LocalVariable {
              name: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("b"), .. }, .. }),
              ..
          },
          ..
      }) => Some(ln)
  );
  assert!(matches!(let_normal.variable.tyype, KindT::BorrowRef(_)));
}

#[test]
fn recursion() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() int { return main(); }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  // Make sure it inferred the param type and return type correctly
  assert!(
    coutputs.lookup_function_by_str("main").header.return_type == KindT::Int(IntT { bits: 32 })
  );
}

#[test]
fn test_overloads() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/functions/overloads.vale");
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "str"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  assert!(matches!(
    coutputs.lookup_function_by_str("main").header.return_type,
    KindT::Int(IntT { bits: 32 })
  ));
}

#[test]
fn test_readonly_ufcs() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/ufcs.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn test_readwrite_ufcs() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/readwriteufcs.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn test_templates() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "func bork<T>(a T) T { return ^a; }\n",
    "exported func main() int { bork(true); bork(2); bork(3) }\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  // Tests that there's only two functions, because we have generics not templates
  assert!(coutputs.get_all_user_functions().len() == 2);
}

// VCOORD: enable this
#[test]
fn test_taking_a_callable_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "func do<F>(callable F) int\n",
    "where func(&F)int, func drop(F)void\n",
    "{\n",
    "  return callable();\n",
    "}\n",
    "exported func main() int { return do({ return 3; }); }\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let do_fn = coutputs.lookup_function_by_str("do");
  assert!(matches!(do_fn.header.return_type, KindT::Int(IntT { bits: 32 })));
}

#[test]
fn simple_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "#!DeriveStructDrop\n",
    "struct MyStruct { a int; }\n",
    "exported func main() {\n",
    "  ms = MyStruct(7);\n",
    "  [_] = ^ms;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  // Check the struct was made
  coutputs
    .structs
    .iter()
    .find(|def| {
      matches!(
        def,
        StructDefinitionT {
          template_name: IdT {
            local_name: INameT::StructTemplate(StructTemplateNameT {
              human_name: StrI("MyStruct"),
              ..
            }),
            ..
          },
          instantiated_citizen: StructTT {
            id: IdT {
              local_name: INameT::Struct(StructNameT {
                template: IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("MyStruct"),
                  ..
                }),
                ..
              }),
              ..
            },
            ..
          },
          weakable: false,
          sharedness: SharednessT::Single,
          members: [StructMemberT {
            name: MemberNameT { imprecise_name: CodeNameS { name: StrI("a"), .. }, .. },
            tyype: KindT::Int(IntT { bits: 32 }),
          }],
          ..
        }
      )
    })
    .unwrap();
  // Check there's a constructor
  let _ = collect_where_tnode!(
      NodeRefT::FunctionDefinition(coutputs.lookup_function_by_str("MyStruct")),
      NodeRefT::FunctionHeader(h @ FunctionHeaderT {
          id: IdT {
              local_name: INameT::Function(FunctionNameT {
                  template: FunctionTemplateNameT { human_name: StrI("MyStruct"), .. },
                  ..
              }),
              ..
          },
          params: [ParameterT {
              name: IVarNameT::Member(MemberNameT { imprecise_name: CodeNameS { name: StrI("a"), .. }, .. }),
              virtuality: None,
              tyype: KindT::Int(IntT { bits: 32 }),
              ..
          }],
          return_type: KindT::Struct(StructTT {
              id: IdT {
                  local_name: INameT::Struct(StructNameT {
                      template: IStructTemplateNameT::StructTemplate(
                          StructTemplateNameT { human_name: StrI("MyStruct"), .. }
                      ),
                      ..
                  }),
                  ..
              },
              ..
          }),
          ..
      }) => Some(h)
  );
  let main = coutputs.lookup_function_by_str("main");
  // Check that we call the constructor
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(
          FunctionCallTE {
              callable: PrototypeT {
                  id: IdT {
                      local_name: INameT::Function(FunctionNameT {
                          template: FunctionTemplateNameT { human_name: StrI("MyStruct"), .. },
                          ..
                      }),
                      ..
                  },
                  ..
              },
              args: [ExpressionTE::ConstantInt(
                  ConstantIntTE {
                      value: ITemplataT::Integer(7),
                      ..
                  }
              )],
              ..
          }
      ) => Some(())
  );
}

#[test]
fn calls_destructor_on_local_var() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Muta { }\n",
    "func destructor(m Muta) {\n",
    "  Muta[ ] = ^m;\n",
    "}\n",
    "exported func main() {\n",
    "  a = Muta();\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(
          FunctionCallTE {
              callable: PrototypeT {
                  id: IdT {
                      local_name: INameT::Function(FunctionNameT {
                          template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                          ..
                      }),
                      ..
                  },
                  ..
              },
              ..
          }
      ) => Some(())
  );
  let all_calls = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(_fpc) => Some(())
  );
  assert_eq!(all_calls.len(), 2);
}

#[test]
fn tests_defining_an_empty_interface_and_an_implementing_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "sealed interface MyInterface { }\n",
    "struct MyStruct { }\n",
    "impl MyInterface for MyStruct;\n",
    "func main(a MyStruct) {}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let interfaces_matching: Vec<_> = coutputs
    .interfaces
    .iter()
    .filter(|d| {
      unapply_simple_name(&d.template_name).as_deref() == Some("MyInterface")
        && !d.weakable
        && matches!(d.sharedness, SharednessT::Single)
        && d.internal_methods.is_empty()
    })
    .collect();
  let interface_def = expect_1(&interfaces_matching);

  let structs_matching: Vec<_> = coutputs
    .structs
    .iter()
    .filter(|d| {
      unapply_simple_name(&d.template_name).as_deref() == Some("MyStruct")
        && !d.weakable
        && matches!(d.sharedness, SharednessT::Single)
    })
    .collect();
  let struct_def = expect_1(&structs_matching);

  assert!(coutputs
    .interface_to_sub_citizen_to_edge
    .iter()
    .flat_map(|(_, sub_map)| sub_map.values())
    .any(|edge| {
      edge.sub_citizen.id() == struct_def.instantiated_citizen.id
        && edge.super_interface == interface_def.instantiated_interface.id
    }));
}

#[test]
fn tests_defining_a_non_empty_interface_and_an_implementing_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "exported sealed interface MyInterface {\n",
    "  func bork(virtual self &MyInterface);\n",
    "}\n",
    "exported struct MyStruct { }\n",
    "impl MyInterface for MyStruct;\n",
    "func bork(self &MyStruct) {}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let interfaces_matching: Vec<_> = coutputs
    .interfaces
    .iter()
    .filter(|d| {
      unapply_simple_name(&d.template_name).as_deref() == Some("MyInterface")
        && !d.weakable
        && matches!(d.sharedness, SharednessT::Single)
    })
    .collect();
  let interface_def = expect_1(&interfaces_matching);

  let bork_method = interface_def
    .internal_methods
    .iter()
    .find(|(proto, _)| unapply_simple_name(&proto.id).as_deref() == Some("bork"))
    .unwrap();
  let _ = bork_method;

  let structs_matching: Vec<_> = coutputs
    .structs
    .iter()
    .filter(|d| {
      unapply_simple_name(&d.template_name).as_deref() == Some("MyStruct")
        && !d.weakable
        && matches!(d.sharedness, SharednessT::Single)
    })
    .collect();
  let struct_def = expect_1(&structs_matching);

  assert!(coutputs
    .interface_to_sub_citizen_to_edge
    .iter()
    .flat_map(|(_, sub_map)| sub_map.values())
    .any(|edge| {
      edge.sub_citizen.id() == struct_def.instantiated_citizen.id
        && edge.super_interface == interface_def.instantiated_interface.id
    }));
}

#[test]
fn stamps_an_interface_template_via_a_function_return() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "\n",
    "sealed interface MyInterface<X> where func drop(X)void { }\n",
    "\n",
    "struct SomeStruct<X> where func drop(X)void { x X; }\n",
    "impl<X> MyInterface<X> for SomeStruct<X>;\n",
    "\n",
    "func doAThing<T>(t T) SomeStruct<T>\n",
    "where func drop(T)void {\n",
    "  return SomeStruct<T>(^t);\n",
    "}\n",
    "\n",
    "exported func main() {\n",
    "  doAThing(4);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn reads_a_struct_member() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  x = ms.a;\n",
  let code = concat!(
    "#!DeriveStructDrop\n",
    "struct MyStruct { a int; }\n",
    "exported func main() int {\n",
    "  ms = MyStruct(7);\n",
    "  x = __copy_prim(&ms.a);\n",
    "  [_] = ^ms;\n",
    "  return ^x;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::MemberLookup(MemberLookupTE {
          member_name: IVarNameT::Member(MemberNameT { imprecise_name: CodeNameS { name: StrI("a"), .. }, .. }),
          result: BorrowRefT { inner: KindT::Int(IntT { bits: 32 }), .. },
          ..
      }) => Some(())
  );
}

#[test]
fn automatically_drops_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  return ms.a;\n"
  let code = concat!(
    "struct MyStruct { a int; }\n",
    "exported func main() int {\n",
    "  ms = MyStruct(7);\n",
    "  return __copy_prim(&ms.a);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");
  // check for the call to drop
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(
          FunctionCallTE {
              callable: PrototypeT {
                  id: IdT {
                      init_steps: [INameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyStruct"), .. })],
                      local_name: INameT::Function(FunctionNameT {
                          template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                          template_args: &[],
                          parameters: [KindT::Struct(StructTT {
                              id: IdT {
                                  local_name: INameT::Struct(StructNameT {
                                      template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("MyStruct"), .. }),
                                      template_args: &[],
                                      ..
                                  }),
                                  ..
                              },
                              ..
                          })],
                          ..
                      }),
                      ..
                  },
                  return_type: KindT::Void(_),
              },
              ..
          }
      ) => Some(())
  );
}

#[test]
fn tests_stamping_an_interface_template_from_a_function_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!("interface MyOption<T> { }\n", "func main(a &MyOption<int>) { }\n",);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let interface_template_name =
    compile.typing_interner.intern_interface_template_name(InterfaceTemplateNameT {
      human_namee: scout_arena.intern_str("MyOption"),
    });
  let template_args_vec = vec![ITemplataT::Kind(
    compile.typing_interner.alloc(KindTemplataT { kind: KindT::Int(IntT { bits: 32 }) }),
  )];
  let interface_name = compile.typing_interner.intern_interface_name(InterfaceNameValT {
    template: interface_template_name,
    template_args: &template_args_vec,
  });
  let test_tld = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);
  let interface_id = compile.typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Interface(interface_name),
  });
  let interface_tt =
    compile.typing_interner.intern_interface_tt(InterfaceTTValT { id: *interface_id });
  let expected_coord = KindT::BorrowRef(
    compile
      .typing_interner
      .alloc(BorrowRefT { inner: KindT::Interface(interface_tt)}),
  );

  let coutputs = compile.expect_compiler_outputs();
  coutputs.lookup_interface_by_template_name(interface_template_name);
  let main = coutputs.lookup_function_by_str("main");
  assert_eq!(main.header.params[0].tyype, expected_coord);
}

#[test]
fn reports_mismatched_return_type_when_expecting_void() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() { 73 }\n";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile
    .get_compiler_outputs()
    .err()
    .unwrap_or_else(|| panic!("expected Err(BodyResultDoesntMatch), got Ok"));
  match &err {
    ICompileErrorT::BodyResultDoesntMatch {
      function_name,
      expected_return_type,
      result_type,
      ..
    } => {
      match function_name {
        IFunctionDeclarationNameS::FunctionName(fn_name) => {
          assert_eq!(fn_name.imprecise_name.name.as_str(), "main")
        }
        other => panic!("expected FunctionName: {:?}", other),
      }
      match expected_return_type {
        KindT::Void(_) => {}
        other => panic!("expected VoidT: {:?}", other),
      }
      match result_type {
        KindT::Int(_) => {}
        other => panic!("expected IntT: {:?}", other),
      }
    }
    _other => panic!("expected BodyResultDoesntMatch"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() { 73 }
At test:0.vale:1:1:
exported func main() { 73 }
Function test:0.vale:1:1: main return type void doesn't match body's result: i32
"#,
  );
}

#[test]
fn tests_exporting_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func moo() { }\n";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let moo = coutputs.lookup_function_by_str("moo");
  let export = expect_1(&coutputs.function_exports);
  assert_eq!(export.prototype, moo.header.to_prototype());
}

#[test]
fn tests_exporting_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported struct Moo { a int; }\n";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let moo = coutputs.lookup_struct_by_str("Moo");
  let export = expect_1(&coutputs.kind_exports);
  assert_eq!(export.tyype, KindT::from(&moo.instantiated_citizen));
}

// VCOORD: enable this after the export/extern gate rework. (Currently also blocked upstream at the
// interface vtable edge, edge_compiler.rs:163, not the export gate.)
#[test]
fn tests_exporting_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported sealed interface IMoo { func hi(virtual this &IMoo) void; }\n";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let moo = coutputs.lookup_interface_by_human_name("IMoo");
  let export = expect_1(&coutputs.kind_exports);
  assert_eq!(export.tyype, KindT::from(&moo.instantiated_interface));
}

#[test]
fn tests_single_expression_and_single_statement_functions_returns() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct MyThing { value int; }\n",
    "func moo() MyThing { return MyThing(4); }\n",
    "exported func main() { moo(); }\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let moo = coutputs.lookup_function_by_str("moo");
  match moo.header.return_type {
    KindT::Struct(StructTT {
      id:
        IdT {
          init_steps: &[],
          local_name:
            INameT::Struct(StructNameT {
              template:
                IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("MyThing"),
                  ..
                }),
              ..
            }),
          ..
        },
      ..
    }) => {}
    other => panic!("moo.header.returnType: {:?}", other),
  }
  let main = coutputs.lookup_function_by_str("main");
  match main.header.return_type {
    KindT::Void(_) => {}
    other => panic!("main.header.returnType: {:?}", other),
  }
}

#[test]
fn tests_calling_a_templated_struct_s_constructor() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  return MySome<int>(4).value;\n"
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "struct MySome<T> where func drop(T)void { value T; }\n",
    "exported func main() int {\n",
    "  return __copy_prim(&MySome<int>(4).value);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  coutputs.lookup_struct_by_template_name(StructTemplateNameT {
    human_name: scout_arena.intern_str("MySome"),
  });

  let constructor = coutputs.lookup_function_by_str("MySome");
  match constructor.header {
    FunctionHeaderT {
      id:
        IdT {
          local_name:
            INameT::Function(FunctionNameT {
              template: FunctionTemplateNameT { human_name: StrI("MySome"), .. },
              template_args:
                [ITemplataT::Kind(KindTemplataT {
                  kind:
                    KindT::KindPlaceholder(KindPlaceholderT {
                      id:
                        IdT {
                          local_name:
                            INameT::KindPlaceholder(KindPlaceholderNameT {
                              template:
                                KindPlaceholderTemplateNameT {
                                  index: 0,
                                  rune: IRuneS::CodeRune(CodeRuneS { name: StrI("T") }),
                                  ..
                                },
                            }),
                          ..
                        },
                      ..
                    }),
                })],
              parameters:
                [KindT::KindPlaceholder(KindPlaceholderT {
                  id:
                    IdT {
                      local_name:
                        INameT::KindPlaceholder(KindPlaceholderNameT {
                          template: KindPlaceholderTemplateNameT { index: 0, .. },
                        }),
                      ..
                    },
                  ..
                })],
              ..
            }),
          ..
        },
      attributes: &[],
      params:
        [ParameterT {
          name: IVarNameT::Member(MemberNameT { imprecise_name: CodeNameS { name: StrI("value"), .. }, .. }),
          virtuality: None,
          tyype:
            KindT::KindPlaceholder(KindPlaceholderT {
              id:
                IdT {
                  local_name:
                    INameT::KindPlaceholder(KindPlaceholderNameT {
                      template: KindPlaceholderTemplateNameT { index: 0, .. },
                    }),
                  ..
                },
              ..
            }),
          ..
        }],
      return_type:
        KindT::Struct(StructTT {
          id:
            IdT {
              local_name:
                INameT::Struct(StructNameT {
                  template:
                    IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                      human_name: StrI("MySome"),
                      ..
                    }),
                  template_args:
                    [ITemplataT::Kind(KindTemplataT {
                      kind:
                        KindT::KindPlaceholder(KindPlaceholderT {
                          id:
                            IdT {
                              local_name:
                                INameT::KindPlaceholder(KindPlaceholderNameT {
                                  template: KindPlaceholderTemplateNameT { index: 0, .. },
                                }),
                              ..
                            },
                          ..
                        }),
                    })],
                  ..
                }),
              ..
            },
          ..
        }),
      maybe_origin_function_templata: Some(_),
      ..
    } => {}
    other => panic!("constructor.header: {:?}", other),
  }

  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(
          FunctionCallTE {
              callable: PrototypeT {
                  id: IdT {
                      local_name: INameT::Function(FunctionNameT {
                          template: FunctionTemplateNameT { human_name: StrI("MySome"), .. },
                          ..
                      }),
                      ..
                  },
                  ..
              },
              ..
          }
      ) => Some(())
  );
}

#[test]
fn tests_upcasting_from_a_struct_to_an_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = include_str!("../../tests/programs/virtuals/upcasting.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::LetNormal(LetNormalTE {
          variable: LocalVariable {
              name: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("x"), .. }, .. }),
              tyype: KindT::Interface(InterfaceTT {
                  id: IdT {
                      local_name: INameT::Interface(InterfaceNameT {
                          template: InterfaceTemplateNameT { human_namee: StrI("MyInterface"), .. },
                          ..
                      }),
                      ..
                  },
                  ..
              }),
          },
          ..
      }) => Some(())
  );

  let upcast: &UpcastTE = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Upcast(u) => Some(u)
  );

  match upcast.result {
    KindT::Interface(InterfaceTT {
      id:
        IdT {
          package_coord: x,
          init_steps: &[],
          local_name:
            INameT::Interface(InterfaceNameT {
              template: InterfaceTemplateNameT { human_namee: StrI("MyInterface"), .. },
              template_args: &[],
              ..
            }),
          ..
        },
      ..
    }) => assert!(x.is_test()),
    other => panic!("upcast result kind: {:?}", other),
  }
  match upcast.inner_expr.result() {
    KindT::Struct(StructTT {
      id:
        IdT {
          package_coord: x,
          init_steps: &[],
          local_name:
            INameT::Struct(StructNameT {
              template:
                IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("MyStruct"),
                  ..
                }),
              template_args: &[],
              ..
            }),
          ..
        },
      ..
    }) => assert!(x.is_test()),
    other => panic!("inner expr kind: {:?}", other),
  }
}

#[test]
fn tests_calling_a_virtual_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = include_str!("../../tests/programs/virtuals/calling.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Upcast(u @ UpcastTE {
          target_super_kind: ISuperKindTT::Interface(InterfaceTT {
              id: IdT {
                  local_name: INameT::Interface(InterfaceNameT {
                      template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                      ..
                  }),
                  ..
              },
              ..
          }),
          ..
      }) => {
          match u.inner_expr.result() {
              KindT::Struct(StructTT {
                  id: IdT {
                      local_name: INameT::Struct(StructNameT {
                          template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Toyota"), .. }),
                          ..
                      }),
                      ..
                  },
                  ..
              }) => {}
              other => panic!("inner expr kind: {:?}", other),
          }
          match u.result {
              KindT::Interface(InterfaceTT {
                  id: IdT {
                      package_coord: pc,
                      init_steps: &[],
                      local_name: INameT::Interface(InterfaceNameT {
                          template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                          template_args: &[],
                          ..
                      }),
                      ..
                  },
                  ..
              }) => {
                  assert!(pc.is_test());
              }
              other => panic!("upcast result kind: {:?}", other),
          }
          Some(())
      }
  );
}

#[test]
fn tests_upcasting_has_the_right_stuff() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = include_str!("../../tests/programs/virtuals/calling.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  let upcast: &UpcastTE = collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Upcast(u @ UpcastTE {
          target_super_kind: ISuperKindTT::Interface(InterfaceTT {
              id: IdT {
                  local_name: INameT::Interface(InterfaceNameT {
                      template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
                      ..
                  }),
                  ..
              },
              ..
          }),
          ..
      }) => Some(u)
  );

  match upcast.inner_expr.result() {
    KindT::Struct(StructTT {
      id:
        IdT {
          local_name:
            INameT::Struct(StructNameT {
              template:
                IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("Toyota"),
                  ..
                }),
              ..
            }),
          ..
        },
      ..
    }) => {}
    other => panic!("inner expr kind: {:?}", other),
  }
  match upcast.result {
    KindT::Interface(InterfaceTT {
      id:
        IdT {
          package_coord: x,
          init_steps: &[],
          local_name:
            INameT::Interface(InterfaceNameT {
              template: InterfaceTemplateNameT { human_namee: StrI("Car"), .. },
              template_args: &[],
              ..
            }),
          ..
        },
      ..
    }) => assert!(x.is_test()),
    other => panic!("upcast result kind: {:?}", other),
  }

  let impl_edge = coutputs.lookup_edge(upcast.impl_name);
  assert!(impl_edge.sub_citizen.id() == upcast.inner_expr.result().expect_citizen().id());
  assert!(impl_edge.super_interface == upcast.result.expect_citizen().id());

  //    freePrototype.fullName.last.parameters.head shouldEqual up.result.reference
}

#[test]
fn tests_calling_a_virtual_function_through_a_borrow_ref() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = include_str!("../../tests/programs/virtuals/callingThroughBorrow.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
          NodeRefT::FunctionDefinition(main),
          NodeRefT::FunctionCall(FunctionCallTE {
              callable: PrototypeT {
                  id: IdT {
                      local_name: INameT::Function(
                          FunctionNameT {
                              template: FunctionTemplateNameT { human_name: StrI("doCivicDance"), .. },
                              ..
                          }
                      ),
                      ..
                  },
                  return_type: KindT::Int(IntT::I32),
                  ..
              },
              ..
          }) => {
  //        vassert(f.callable.paramTypes == Vector(Coord(Borrow,InterfaceRef2(simpleName("Car")))))
              Some(())
          }
      );
}

#[test]
fn tests_calling_a_templated_function_with_explicit_template_args() {
  // Tests putting MyOption<int> as the type of x.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!("func moo<T> () { }\n", "exported func main() {\n", "  moo<int>();\n", "}\n",);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn tests_destructuring_borrow_doesnt_compile_to_destroy() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  return y;\n"
  let code = concat!(
    "\n",
    "struct Vec3i {\n",
    "  x int;\n",
    "  y int;\n",
    "  z int;\n",
    "}\n",
    "\n",
    "exported func main() int {\n",
    "  v = Vec3i(3, 4, 5);\n",
    "\t [x, y, z] = &v;\n",
    "  return __copy_prim(&y);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  let destroys = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Destroy(_) => Some(())
  );
  assert_eq!(destroys.len(), 0);
  // Destructuring `&v` reads each field through the borrow, so member `x` comes out as a
  // borrow of an int (`&int`) rather than a destroy of `v`.
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::MemberLookup(MemberLookupTE {
          member_name: IVarNameT::Member(MemberNameT { imprecise_name: CodeNameS { name: StrI("x"), .. }, .. }),
          result: BorrowRefT { inner: KindT::Int(IntT { bits: 32 }), .. },
          ..
      }) => Some(())
  );
}

#[test]
fn tests_making_a_variable_with_a_pattern() {
  // Tests putting MyOption<int> as the type of x.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "\n",
    "sealed interface MyOption<T> { }\n",
    "\n",
    "struct MySome<T> {}\n",
    "impl<T> MyOption<T> for MySome<T>;\n",
    "\n",
    "func doSomething(opt MyOption<int>) int {\n",
    "  return 9;\n",
    "}\n",
    "\n",
    "exported func main() int {\n",
    "\tx MyOption<int> = MySome<int>();\n",
    "\treturn doSomething(^x);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn tests_a_linked_list() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/virtuals/ordinarylinkedlist.vale");
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "print"),
    Source::builtin_module(&parse_arena, &parser_keywords, "str"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn tup0_returned_and_assigned() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.tup0.*;
func make_tup0() () { return (); }
func main() () {
  x = make_tup0();
  return ();
}";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "tup0"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let make_tup0 = coutputs.lookup_function_by_str("make_tup0");
  match make_tup0.header.return_type {
    KindT::Struct(StructTT {
      id:
        IdT {
          local_name:
            INameT::Struct(StructNameT {
              template:
                IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("Tup0"),
                  ..
                }),
              ..
            }),
          ..
        },
      ..
    }) => {}
    other => panic!("Expected make_tup0's return to be Tup0, got {:?}", other),
  }

  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::LetNormal(LetNormalTE {
          variable: LocalVariable {
              name: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("x"), .. }, .. }),
              tyype: KindT::Struct(StructTT {
                  id: IdT {
                      local_name: INameT::Struct(StructNameT {
                          template: IStructTemplateNameT::StructTemplate(
                              StructTemplateNameT { human_name: StrI("Tup0"), .. }
                          ),
                          ..
                      }),
                      ..
                  },
                  ..
              }),
          },
          ..
      }) => Some(())
  );
}

#[test]
fn passing_bare_local_to_borrow_param_does_not_need_ampersand() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
struct SomeStruct { i int; }
func bork(x &SomeStruct) int { return 7; }
exported func main() int {
  x = SomeStruct(3);
  return bork(x);
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT { local_name: INameT::Function(FunctionNameT {
                  template: FunctionTemplateNameT { human_name: StrI("bork"), .. }, .. }), .. },
              ..
          },
          args: [ExpressionTE::LocalLookup(LocalLookupTE {
              result: BorrowRefT { inner: KindT::Struct(_), .. },
              ..
          })],
          ..
      }) => Some(())
  );
}

// Ensures that when a callsite passes a bare Own local to a parameter that expects
// Own, but the compiler has no `implicit_clone(&T) T` available to auto-copy the
// borrow that bare-use produces, the resolver reports NoImplicitCloneDefinedT and
// the humanized message lists all three options (consume with `^`, explicit
// `clone(&x)`, or define an `implicit_clone(&T) T`), e.g. `consume(s)` with an Own
// Ship local when nothing named `implicit_clone` matches `&Ship`.
// VCOORD: enable this. Error-message check for the retired implicit_clone probe; panics at
// is_type_convertible's bare-to-borrow hole. Re-enable or delete when implicit_clone is removed.
#[test]
#[ignore]
fn error_when_no_implicit_clone_for_borrow_to_own_conversion() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.implicit_clone.*;
struct Ship { hp int; }
func consume(s Ship) int { [hp] = ^s; return hp; }
exported func main() int {
  s = Ship(7);
  return consume(s);
}
";
  let code_source = CodeSource::new(vec![
    Source::from_code_map(&get_embedded_modulized_code_map(&parse_arena, &parser_keywords)),
    new_test_code_map(&parse_arena, code),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile
    .get_compiler_outputs()
    .err()
    .unwrap_or_else(|| panic!("expected Err(NoImplicitCloneDefinedT), got Ok"));
  match &err {
    ICompileErrorT::NoImplicitCloneDefinedT { source_type, target_type, .. } => {
      assert!(matches!(source_type, KindT::BorrowRef(_)));
      assert!(!matches!(target_type, KindT::BorrowRef(_)));
    }
    other => panic!("expected NoImplicitCloneDefinedT, got {:?}", other),
  }
  let humanized = humanize_compile_error(&mut compile, err);
  assert!(
    humanized.contains("^"),
    "message should suggest `^local` to consume; got: {}",
    humanized
  );
  assert!(
    humanized.contains("clone("),
    "message should suggest `clone(&local)`; got: {}",
    humanized
  );
  assert!(
    humanized.contains("implicit_clone"),
    "message should suggest defining `implicit_clone`; got: {}",
    humanized
  );
}

// Ensures that when the user has defined `implicit_clone` for their type but the
// resolver rejected every candidate (e.g. the signature takes Own instead of
// Borrow), the compiler reports ImplicitCloneRejectedT with the FindFunctionFailure
// preserved, so the humanized message can surface the rejection detail (which
// candidate was tried and why) alongside the fallback options.
// VCOORD: enable this. Error-message check for the retired implicit_clone probe; panics at
// is_type_convertible's bare-to-borrow hole. Re-enable or delete when implicit_clone is removed.
#[test]
#[ignore]
fn error_when_implicit_clone_is_defined_but_rejected() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.implicit_clone.*;
struct Ship { hp int; }
func implicit_clone(s Ship) Ship { return Ship(__copy_prim(&s.hp)); }
func consume(s Ship) int { [hp] = ^s; return hp; }
exported func main() int {
  s = Ship(7);
  return consume(s);
}
";
  let code_source = CodeSource::new(vec![
    Source::from_code_map(&get_embedded_modulized_code_map(&parse_arena, &parser_keywords)),
    new_test_code_map(&parse_arena, code),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile
    .get_compiler_outputs()
    .err()
    .unwrap_or_else(|| panic!("expected Err(ImplicitCloneRejectedT), got Ok"));
  match &err {
    ICompileErrorT::ImplicitCloneRejectedT { source_type, target_type, fff, .. } => {
      assert!(matches!(source_type, KindT::BorrowRef(_)));
      assert!(!matches!(target_type, KindT::BorrowRef(_)));
      assert!(
        !fff.rejected_callee_to_reason.is_empty(),
        "expected at least one rejected candidate, got empty"
      );
    }
    other => panic!("expected ImplicitCloneRejectedT, got {:?}", other),
  }
  let humanized = humanize_compile_error(&mut compile, err);
  assert!(
    humanized.contains("implicit_clone"),
    "message should mention implicit_clone; got: {}",
    humanized
  );
  assert!(
    humanized.contains("Rejected") || humanized.contains("rejected"),
    "message should surface the rejection detail from fff; got: {}",
    humanized
  );
}

#[test]
fn test_borrow_ref() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/borrowRef.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn tests_calling_a_function_with_an_upcast() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "interface ISpaceship {}\n",
    "struct Firefly {}\n",
    "impl ISpaceship for Firefly;\n",
    "func launch(ship &ISpaceship) { }\n",
    "func main() {\n",
    "  launch(&Firefly());\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Upcast(UpcastTE {
          target_super_kind: ISuperKindTT::Interface(InterfaceTT {
              id: IdT {
                  local_name: INameT::Interface(InterfaceNameT {
                      template: InterfaceTemplateNameT { human_namee: StrI("ISpaceship"), .. },
                      ..
                  }),
                  ..
              },
              ..
          }),
          ..
      }) => Some(())
  );
}

#[test]
fn tests_calling_a_templated_function_with_an_upcast() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "interface ISpaceship<T> {}\n",
    "struct Firefly<T> {}\n",
    "impl<T> ISpaceship<T> for Firefly<T>;\n",
    "func launch<T>(ship &ISpaceship<T>) { }\n",
    "func main() {\n",
    "  launch(&Firefly<int>());\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Upcast(UpcastTE {
          target_super_kind: ISuperKindTT::Interface(InterfaceTT {
              id: IdT {
                  local_name: INameT::Interface(InterfaceNameT {
                      template: InterfaceTemplateNameT { human_namee: StrI("ISpaceship"), .. },
                      ..
                  }),
                  ..
              },
              ..
          }),
          ..
      }) => Some(())
  );
}

#[test]
fn tests_upcast_with_generics_has_the_right_stuff() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "interface ISpaceship<T> {}\n",
    "struct Firefly<T> {}\n",
    "impl<T> ISpaceship<T> for Firefly<T>;\n",
    "func launch<T>(ship &ISpaceship<T>) { }\n",
    "func main() {\n",
    "  launch(&Firefly<int>());\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let main = coutputs.lookup_function_by_str("main");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Upcast(UpcastTE {
          target_super_kind: ISuperKindTT::Interface(InterfaceTT {
              id: IdT {
                  local_name: INameT::Interface(InterfaceNameT {
                      template: InterfaceTemplateNameT { human_namee: StrI("ISpaceship"), .. },
                      ..
                  }),
                  ..
              },
              ..
          }),
          ..
      }) => Some(())
  );
}

#[test]
fn tests_a_templated_linked_list() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/genericvirtuals/templatedlinkedlist.vale");
  let code_source = CodeSource::new(vec![
    builtin_source_for_opt(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "logic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
// VCOORD: enable this. Blocked on closures: forEach(&list, { print(__copy_prim(_)); }) passes a
// closure whose templated-light-banner resolution hits the @PFVSZ per-param-fold stub
// (function_compiler_solving_layer.rs:230). Re-enable when the lambda/closure cluster lands.
fn tests_a_foreach_for_a_linked_list() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/genericvirtuals/foreachlinkedlist.vale");
  let code_source = CodeSource::new(vec![
    builtin_source_for_opt(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "logic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn test_return_from_inside_if_destroys_locals() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "      m.hp\n"
  let code = concat!(
    "struct Marine { hp int; }\n",
    "exported func main() int {\n",
    "  m = Marine(5);\n",
    "  x =\n",
    "    if (true) {\n",
    "      return 7;\n",
    "    } else {\n",
    "      __copy_prim(&m.hp)\n",
    "    };\n",
    "  return ^x;\n",
    "}",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  let destructor_calls = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(fpc @ FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
                      parameters: [KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name: StrI("Marine"), .. }),
                                  ..
                              }),
                              ..
                          },
                          ..
                      })],
                      ..
                  }),
                  init_steps: [INameT::StructTemplate(StructTemplateNameT { human_name: StrI("Marine"), .. })],
                  ..
              },
              ..
          },
          ..
      }) => Some(fpc)
  );
  assert_eq!(destructor_calls.len(), 2);
}

// VCOORD: re-enable share things after onion
#[test]
fn recursive_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct ListNode share {\n",
    "  tail ListNode;\n",
    "}\n",
    "func main(a ListNode) {}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn recursive_struct_with_opt() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.opt.*;\n",
    "struct ListNode {\n",
    "  tail Opt<ListNode>;\n",
    "}\n",
    "func main(a ListNode) {}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_opt(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

// VCOORD: re-enable share things after onion
#[test]
fn templated_imm_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "struct ListNode<T> share {\n",
    "  tail ListNode<T>;\n",
    "}\n",
    "func main(a ListNode<int>) {}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_bundle(&parse_arena, &parser_keywords, &["drop", "implicit_clone"]),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn borrow_load_member() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "func getX(bork &Bork) int { return bork.x; }\n",
  let code = concat!(
    "struct Bork {\n",
    "  x int;\n",
    "}\n",
    "func getX(bork &Bork) int { return __copy_prim(&bork.x); }\n",
    "struct List {\n",
    "  array Bork;\n",
    "}\n",
    "exported func main() int {\n",
    "  l = List(Bork(0));\n",
    "  return getX(&l.array);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: re-enable share things after onion
#[test]
fn test_vector_of_struct_templata() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "\n",
    "struct Vec2 share {\n",
    "  x float;\n",
    "  y float;\n",
    "}\n",
    "struct Pattern share {\n",
    "  patternTiles []Vec2;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn if_branches_returns_never_and_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.panicutils.*;\n",
    "exported struct Moo {}\n",
    "exported func main() Moo {\n",
    "  if true {\n",
    "    Moo()\n",
    "  } else {\n",
    "    panic(\"Error in CreateDir\");\n",
    "  }\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_panicutils(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn test_return() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() int {\n  return 7;\n}";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Return(_) => Some(())
  );
}

#[test]
fn test_return_from_inside_if() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.panic.*;
exported func main() int {
  if (true) {
    return 7;
  } else {
    return 9;
  }
  __vbi_panic();
}";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "panic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  let returns = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Return(_) => Some(())
  );
  assert_eq!(returns.len(), 2);
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::ConstantInt(
          ConstantIntTE {
              value: ITemplataT::Integer(7),
              ..
          }
      ) => Some(())
  );
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::ConstantInt(
          ConstantIntTE {
              value: ITemplataT::Integer(9),
              ..
          }
      ) => Some(())
  );
}

// VCOORD: re-enable anonymous interface macro after we do the ITypeST migration
#[test]
#[ignore]
fn zero_method_anonymous_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "interface MyInterface {}\n",
    "exported func main() {\n",
    "  x = MyInterface();\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: enable this after the export/extern gate rework (is_primitive split + peel).
#[test]
#[ignore]
fn reports_when_exported_function_depends_on_non_exported_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "struct Firefly { }\nexported func moo(firefly &Firefly) { }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExportedFunctionDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
exported func moo(firefly &Firefly) { }
Exported function:
moo(&Firefly)
depends on kind:
Firefly
that wasn't exported from package test
"#,
  );
}

// VCOORD: enable this after the export/extern gate rework (is_primitive split + peel).
#[test]
#[ignore]
fn reports_when_exported_function_depends_on_non_exported_return() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "import panicutils.*;\nstruct Firefly { }\nexported func moo() &Firefly { __pretend<&Firefly>() }";
  let code_source = CodeSource::new(vec![
    builtin_source_for_panicutils(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "logic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "panicutils"),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExportedFunctionDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:3:1:
exported func moo() &Firefly { __pretend<&Firefly>() }
Exported function:
moo
depends on kind:
Firefly
that wasn't exported from package test
"#,
  );
}

// VCOORD: enable this after the export/extern gate rework (is_primitive split + peel).
#[test]
#[ignore]
fn reports_when_extern_function_depends_on_non_exported_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "struct Firefly { }\nextern func moo(firefly &Firefly);";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExternFunctionDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExternFunctionDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
extern func moo(firefly &Firefly);
Extern function moo depends on kind Firefly that wasn't exported from package test
"#,
  );
}

// VCOORD: re-enable share things after onion
#[test]
#[ignore]
fn reports_when_extern_function_depends_on_non_exported_return() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "struct Firefly share { }\nextern func moo() &Firefly;";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExternFunctionDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExternFunctionDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
extern func moo() &Firefly;
Extern function moo depends on kind Firefly that wasn't exported from package test
"#,
  );
}

// VCOORD: re-enable share things after onion
#[test]
fn reports_when_exported_struct_depends_on_non_exported_member() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: imm → share
  let code = r"
exported struct Firefly share {
  raza Raza;
}
struct Raza share { }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExportedKindDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExportedKindDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
exported struct Firefly share {
Exported kind Firefly depends on kind Raza that wasn't exported from package test
"#,
  );
}

// The transitive-export check at `ensure_deep_exports` guards its member walk on
// `sharedness == Shared`, so a non-shared exported struct's members go unchecked. This
// pins whether that silence is real, and which way it should be resolved: either a
// non-shared struct crosses as an opaque handle and needs no member export (in which
// case the guard is right and wants a comment saying so), or the guard is an
// under-approximation that passes a program it should reject.
#[test]
fn reports_when_exported_nonshared_struct_depends_on_non_exported_member() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
exported struct Firefly {
  raza Raza;
}
struct Raza { }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExportedKindDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExportedKindDependedOnNonExportedKind"),
  }
}

#[test]
fn checks_that_we_stored_a_borrowed_temporary_in_a_local() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Muta { }\n",
    "func doSomething(m &Muta, i int) {}\n",
    "exported func main() {\n",
    "  doSomething(&Muta(), 1)\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::LetAndLend(LetAndLendTE {
          result: BorrowRefT { inner: KindT::Struct(_), .. },
          ..
      }) => Some(())
  );
}

#[test]
fn reports_when_ssa_from_callable_has_unknown_element_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "exported func main() int {\n",
    "  a = [#5]NoSuchType(&{_ * 42});\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::HigherTypingInferError { .. } => {}
    other => panic!("expected HigherTypingInferError, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int {
At test:0.vale:2:7:
  a = [#5]NoSuchType(&{_ * 42});
: Couldn't solve generics types:
Couldn't find anything with the name 'NoSuchType'
"#,
  );
}

// VCOORD: enable this
#[test]
fn reports_when_ssa_callable_returns_wrong_element_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arith.*;\n",
    "exported func main() int {\n",
    "  a = [#5]int(&{ _ == 0 });\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arith(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::UnexpectedArrayElementType { .. } => {}
    other => panic!("expected UnexpectedArrayElementType, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
exported func main() int {
At test:0.vale:3:7:
  a = [#5]int(&{ _ == 0 });
Unexpected type for array element, tried to put a bool into an array of i32
"#,
  );
}

#[test]
fn reports_when_rsa_from_callable_has_unknown_element_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "exported func main() int {\n",
    "  a = []NoSuchType(3, &(i int) => { i });\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::HigherTypingInferError { .. } => {}
    other => panic!("expected HigherTypingInferError, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:3:1:
exported func main() int {
At test:0.vale:4:7:
  a = []NoSuchType(3, &(i int) => { i });
: Couldn't solve generics types:
Couldn't find anything with the name 'NoSuchType'
"#,
  );
}

// VCOORD: enable this
#[test]
fn array_map_with_single_lambda_types_cleanly() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // Same Vale fixture as integration_tests::tests::array_tests::array_map_with_single_lambda,
  // but typing-pass only — verifies the code_source matches user's __call(&Lam, ...) against
  // Array's func(&G, int)E bound when Lam is Single, without running the full pipeline.
  let code = r"
import v.builtins.arrays.*;
import v.builtins.arith.*;
import v.builtins.drop.*;

struct Lam {}
func __call(lam &Lam, i int) int { return __copy_prim(&i); }

func main() int {
  a = []int(10, Lam());
  return __copy_prim(&a.3);
}";
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  // Spirit: the closure's `__call` must resolve with a Borrow first param (not
  // Own), which is what lets it satisfy Array's `func(&G, int)E` bound when
  // G = Lam. Match that exact shape: a `__call` whose first param is `&Lam`.
  collect_only_tnode!(
      NodeRefT::Hinputs(coutputs),
      NodeRefT::FunctionHeader(FunctionHeaderT {
          id: IdT {
              local_name: INameT::Function(FunctionNameT {
                  template: FunctionTemplateNameT { human_name: StrI("__call"), .. },
                  ..
              }),
              ..
          },
          params: [
              ParameterT {
                  tyype: KindT::BorrowRef(BorrowRefT {
                      inner: KindT::Struct(StructTT {
                          id: IdT {
                              local_name: INameT::Struct(StructNameT {
                                  template: IStructTemplateNameT::StructTemplate(
                                      StructTemplateNameT { human_name: StrI("Lam"), .. }
                                  ),
                                  ..
                              }),
                              ..
                          },
                          ..
                      }),
                  }),
                  ..
              },
              ..
          ],
          ..
      }) => Some(())
  );
}

#[test]
#[ignore = "runtime-array-from-callable delegates to find_function on the Array library function (array_compiler.rs:299), so a wrong-element generator fails as generic CouldntFindFunctionToCallT (:322) before the dedicated element-type check (:337, itself a bare panic). Un-ignore once the runtime path compares the generator's return type to the element type directly, like the static path (:150)."]
fn reports_when_rsa_callable_returns_wrong_element_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.arith.*;\n",
    "import v.builtins.drop.*;\n",
    "exported func main() int {\n",
    "  a = []int(5, &{ _ == 0 });\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::UnexpectedArrayElementType { .. } => {}
    other => panic!("expected UnexpectedArrayElementType, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:4:1:
exported func main() int {
At test:0.vale:5:7:
  a = #[]int(5, &{ _ == 0 });
Unexpected type for array element, tried to put a bool into an array of i32
"#,
  );
}

#[test]
fn reports_when_ssa_from_values_has_unknown_element_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "exported func main() int {\n",
    "  a = [#]NoSuchType(1, 2, 3);\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::HigherTypingInferError { .. } => {}
    other => panic!("expected HigherTypingInferError, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int {
At test:0.vale:2:7:
  a = [#]NoSuchType(1, 2, 3);
: Couldn't solve generics types:
Couldn't find anything with the name 'NoSuchType'
"#,
  );
}

#[test]
fn reports_when_ssa_values_have_wrong_element_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "exported func main() int {\n",
    "  a = [#]int(true, false, true);\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::UnexpectedArrayElementType { .. } => {}
    other => panic!("expected UnexpectedArrayElementType, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int {
At test:0.vale:2:7:
  a = [#]int(true, false, true);
Unexpected type for array element, tried to put a bool into an array of i32
"#,
  );
}

#[test]
fn reports_when_rsa_indexed_with_non_integer() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "exported func main() int {\n",
    "  a = Array<int>(3);\n",
    "  return a[true];\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::IndexedArrayWithNonInteger { .. } => {}
    other => panic!("expected IndexedArrayWithNonInteger, got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:3:1:
exported func main() int {
At test:0.vale:5:10:
  return a[true];
At test:0.vale:5:10:
  return a[true];
Indexed array with non-integer: bool
"#,
  );
}

// VCOORD: check if this is redundant
#[test]
fn runtime_sized_array_local_drops_cleanly() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "exported func main() int {\n",
    "  arr = Array<int>(3);\n",
    "  return 0;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn reports_when_dot_applied_to_non_container() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!("exported func main() int {\n", "  x = 5;\n", "  return x.foo;\n", "}\n",);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::RangedInternalErrorT { message, .. } if message.contains("Can't apply") => {}
    other => panic!("expected RangedInternalErrorT 'Can't apply', got {:?}", other),
  }
  // TODO: the RangedInternalErrorT message itself includes a Debug-format of the kind; replace at the error-construction site with a humanize_kind call and re-capture.
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int {
At test:0.vale:3:10:
  return x.foo;
Internal error: Can't apply .foo to Int(IntT { bits: 32 })
"#,
  );
}

#[test]
fn reports_when_rsa_dot_member_is_not_digit() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "exported func main() int {\n",
    "  a = Array<int>(3);\n",
    "  return a.foo;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::RangedInternalErrorT { message, .. }
      if message.contains("Array has no member") => {}
    other => panic!("expected RangedInternalErrorT 'Array has no member', got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:3:1:
exported func main() int {
At test:0.vale:5:10:
  return a.foo;
Internal error: Array has no member named foo
"#,
  );
}

#[test]
fn reports_when_ssa_dot_member_is_not_digit() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code =
    concat!("exported func main() int {\n", "  a = [#](1, 2, 3);\n", "  return a.foo;\n", "}\n",);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::RangedInternalErrorT { message, .. }
      if message.contains("Sequence has no member") => {}
    other => panic!("expected RangedInternalErrorT 'Sequence has no member', got {:?}", other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int {
At test:0.vale:3:10:
  return a.foo;
Internal error: Sequence has no member named foo
"#,
  );
}

#[test]
fn reports_when_if_branches_have_different_kinds() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "exported func main() int {\n",
    "  x = if true { 5 } else { 6.0 };\n",
    "  return 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::CantReconcileBranchesResults { .. } => {}
    _other => panic!("expected CantReconcileBranchesResults"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int {
At test:0.vale:2:7:
  x = if true { 5 } else { 6.0 };
If branches return different types: i32 and float
"#,
  );
}

#[test]
fn reports_when_if_condition_isnt_boolean() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() int { if 3 { return 5; } else { return 7; } }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ConditionIsntBoolean { .. } => {}
    _other => panic!("expected ConditionIsntBoolean"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported func main() int { if 3 { return 5; } else { return 7; } }
At test:0.vale:1:31:
exported func main() int { if 3 { return 5; } else { return 7; } }
Condition should be a bool, but was: i32
"#,
  );
}

#[test]
fn reports_when_while_condition_isnt_boolean() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() int { while (3) { } return 7; }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ConditionIsntBoolean { .. } => {}
    _other => panic!("expected ConditionIsntBoolean"),
  }
  let humanized = humanize_compile_error(&mut compile, err);
  assert!(
    humanized.contains("Condition should be a bool, but was: i32"),
    "expected the shared condition error, got:\n{}",
    humanized
  );
}

#[test]
fn reports_when_mutating_after_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Weapon { ammo int; }\n",
    "struct Marine { weapon Weapon; }\n",
    "exported func main() int {\n",
    "  m = Marine(Weapon(7));\n",
    "  newWeapon = Weapon(10);\n",
    "  set m.weapon = ^newWeapon;\n",
    "  set newWeapon.ammo = 11;\n",
    "  return 42;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::CantUseUnstackifiedLocal {
      local_id: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("newWeapon"), .. }, .. }),
      ..
    } => {}
    _other => panic!("expected CantUseUnstackifiedLocal"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:7:7:
  set newWeapon.ammo = 11;
Can't use local that was already moved: newWeapon
"#,
  );
}

#[test]
fn tests_export_struct_twice() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!("exported struct Moo { }\n", "export Moo as Bork;\n",);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::TypeExportedMultipleTimes { exports, .. } => {
      assert_eq!(exports.len(), 2);
    }
    _ => panic!("Expected TypeExportedMultipleTimes"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
exported struct Moo { }
Type exported multiple times:
  test:0.vale:1:1: exported struct Moo { }
  test:0.vale:2:1: export Moo as Bork;
"#,
  );
}

#[test]
fn reports_when_reading_after_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Weapon { ammo int; }\n",
    "struct Marine { weapon Weapon; }\n",
    "exported func main() int {\n",
    "  m = Marine(Weapon(7));\n",
    "  newWeapon = Weapon(10);\n",
    "  set m.weapon = ^newWeapon;\n",
    "  println(newWeapon.ammo);\n",
    "  return 42;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::CantUseUnstackifiedLocal {
      local_id: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("newWeapon"), .. }, .. }),
      ..
    } => {}
    _other => panic!("expected CantUseUnstackifiedLocal"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:7:11:
  println(newWeapon.ammo);
Can't use local that was already moved: newWeapon
"#,
  );
}

#[test]
fn reports_when_moving_from_inside_a_while() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Marine { ammo int; }\n",
    "exported func main() int {\n",
    "  m = Marine(7);\n",
    "  while (false) {\n",
    "    drop(^m);\n",
    "  }\n",
    "  return 42;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile {
      local_id: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("m"), .. }, .. }),
      ..
    } => {}
    _other => panic!("expected CantUnstackifyOutsideLocalFromInsideWhile"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r##"At test:0.vale:2:1:
exported func main() int {
At test:0.vale:4:3:
  while (false) {
Can't move a local (m) from inside a while loop.
"##,
  );
}

// Ignored because it fails today. The move-tracker skips its entire outer-local-move
// check when the while body never falls through. The `IExpressionSE::While` arm of
// `evaluate_expression` (typing/expression/expression_compiler.rs) gates that check on
// `match uncoerced_body_block_2.result { KindT::Never(_) => {} .. }`, so a body ending in
// break/return makes an illegal move of an outer local go unreported. The sibling
// `reports_when_moving_from_inside_a_while` (no break) still errors. The fix drops that
// Never guard so the check runs regardless of the body's result.
#[test]
#[ignore]
fn reports_when_moving_from_inside_a_while_that_never_falls_through() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Marine { ammo int; }\n",
    "exported func main() int {\n",
    "  m = Marine(7);\n",
    "  while (false) {\n",
    "    drop(^m);\n",
    "    break;\n",
    "  }\n",
    "  return 42;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile {
      local_id: IVarNameT::Local(LocalNameT { imprecise_name: CodeNameS { name: StrI("m"), .. }, .. }),
      ..
    } => {}
    _other => panic!("expected CantUnstackifyOutsideLocalFromInsideWhile"),
  }
}

#[test]
fn cant_subscript_non_subscriptable_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Weapon { ammo int; }\n",
    "exported func main() int {\n",
    "  weapon = Weapon(10);\n",
    "  return weapon[42];\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::CannotSubscriptT {
      tyype:
        KindT::BorrowRef(BorrowRefT {
          inner:
            KindT::Struct(StructTT {
              id:
                IdT {
                  local_name:
                    INameT::Struct(StructNameT {
                      template:
                        IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                          human_name: StrI("Weapon"),
                          ..
                        }),
                      template_args: &[],
                      ..
                    }),
                  ..
                },
              ..
            }),
          ..
        }),
      ..
    } => {}
    _other => panic!("expected CannotSubscriptT for Weapon struct: {:?}", _other),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
exported func main() int {
At test:0.vale:4:10:
  return weapon[42];
Cannot subscript type: &Weapon
"#,
  );
}

#[test]
fn humanize_errors() {
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let typing_interner = TypingInterner::new(&typing_bump);

  let tz_code_loc = CodeLocationS::test_zero(&scout_arena);
  let tz = RangeS::test_zero(&scout_arena);
  let tz_slice: &[RangeS] = typing_bump.alloc_slice_copy(&[tz]);
  let test_tld = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);

  let filenames_and_sources =
    new_humanizer_test_code_map(&scout_arena, "blah blah blah\nblah blah blah");
  let humanize_pos = |x| humanize_pos_code_map(&filenames_and_sources, &x);
  let lines_between = |x, y| lines_between(&filenames_and_sources, &x, &y);
  let line_range_containing = |x| line_range_containing(&filenames_and_sources, &x);
  let line_containing = |x| line_containing(&filenames_and_sources, &x);

  let firefly_struct_template_name =
    typing_interner.intern_struct_template_name(StructTemplateNameT {
      human_name: scout_arena.intern_str("Firefly"),
    });
  let firefly_struct_name = typing_interner.intern_struct_name(StructNameValT {
    template: IStructTemplateNameT::StructTemplate(firefly_struct_template_name),
    template_args: &[],
  });
  let firefly_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Struct(firefly_struct_name),
  });
  let firefly_tt = typing_interner.intern_struct_tt(StructTTValT { id: *firefly_id });
  let firefly_kind = KindT::Struct(firefly_tt);
  let firefly_coord = firefly_kind;

  let serenity_struct_template_name =
    typing_interner.intern_struct_template_name(StructTemplateNameT {
      human_name: scout_arena.intern_str("Serenity"),
    });
  let serenity_struct_name = typing_interner.intern_struct_name(StructNameValT {
    template: IStructTemplateNameT::StructTemplate(serenity_struct_template_name),
    template_args: &[],
  });
  let serenity_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Struct(serenity_struct_name),
  });
  let serenity_tt = typing_interner.intern_struct_tt(StructTTValT { id: *serenity_id });
  let serenity_kind = KindT::Struct(serenity_tt);
  let serenity_coord = serenity_kind;

  let ispaceship_interface_template_name =
    typing_interner.intern_interface_template_name(InterfaceTemplateNameT {
      human_namee: scout_arena.intern_str("ISpaceship"),
    });
  let ispaceship_interface_name = typing_interner.intern_interface_name(InterfaceNameValT {
    template: ispaceship_interface_template_name,
    template_args: &[],
  });
  let ispaceship_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Interface(ispaceship_interface_name),
  });
  let ispaceship_tt = typing_interner.intern_interface_tt(InterfaceTTValT { id: *ispaceship_id });
  let ispaceship_kind = KindT::Interface(ispaceship_tt);

  let unrelated_struct_template_name =
    typing_interner.intern_struct_template_name(StructTemplateNameT {
      human_name: scout_arena.intern_str("Spoon"),
    });
  let unrelated_struct_name = typing_interner.intern_struct_name(StructNameValT {
    template: IStructTemplateNameT::StructTemplate(unrelated_struct_template_name),
    template_args: &[],
  });
  let unrelated_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Struct(unrelated_struct_name),
  });
  let unrelated_tt = typing_interner.intern_struct_tt(StructTTValT { id: *unrelated_id });
  let unrelated_kind = KindT::Struct(unrelated_tt);

  let myfunc_template_name = typing_interner.intern_function_template_name(FunctionTemplateNameT {
    human_name: scout_arena.intern_str("myFunc"),
    code_location: tz_code_loc,
  });
  let firefly_func_name = typing_interner.intern_function_name(FunctionNameValT {
    template: myfunc_template_name,
    template_args: &[],
    parameters: &[firefly_coord],
  });
  let firefly_signature_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Function(firefly_func_name),
  });
  let firefly_signature = typing_interner.intern_signature(SignatureValT {
    id: IdValT {
      package_coord: test_tld,
      init_steps: &[],
      local_name: INameT::Function(firefly_func_name),
    },
  });

  let export_template_name =
    typing_interner.intern_export_template_name(ExportTemplateNameT { code_loc: tz_code_loc });
  let export_name = typing_interner
    .intern_export_name(ExportNameT { template: export_template_name});
  let firefly_export_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Export(export_name),
  });
  let firefly_export = KindExportT {
    range: tz,
    tyype: firefly_kind,
    id: *firefly_export_id,
    exported_name: scout_arena.intern_str("Firefly"),
  };
  let serenity_export_id = typing_interner.intern_id(IdValT {
    package_coord: test_tld,
    init_steps: &[],
    local_name: INameT::Export(export_name),
  });
  let serenity_export = KindExportT {
    range: tz,
    tyype: firefly_kind,
    id: *serenity_export_id,
    exported_name: scout_arena.intern_str("Serenity"),
  };
  let exports_slice: &[KindExportT] =
    typing_bump.alloc_slice_fill_iter([firefly_export, serenity_export].into_iter());

  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntFindTypeT {
      range: tz_slice,
      name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS {
        name: scout_arena.intern_str("Spaceship")
      }))
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntFindFunctionToCallT {
      range: tz_slice,
      fff: FindFunctionFailure {
        name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS {
          name: scout_arena.intern_str("someFunc")
        })),
        args: &[],
        rejected_callee_to_reason: &[],
      }
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntFindFunctionToCallT {
      range: tz_slice,
      fff: FindFunctionFailure {
        name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS {
          name: scout_arena.intern_str("")
        })),
        args: &[],
        rejected_callee_to_reason: &[],
      }
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CannotSubscriptT { range: tz_slice, tyype: firefly_kind }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntFindIdentifierToLoadT {
      range: tz_slice,
      name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS {
        name: scout_arena.intern_str("spaceship")
      }))
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntFindMemberT { range: tz_slice, member_name: "hp" }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::BodyResultDoesntMatch {
      range: tz_slice,
      function_name: IFunctionDeclarationNameS::FunctionName(FunctionNameS {
        imprecise_name: scout_arena.intern_code_name(scout_arena.intern_str("myFunc")),
        code_location: tz_code_loc,
        lid: LocationInDenizen { path: &[] },
      }),
      expected_return_type: firefly_coord,
      result_type: serenity_coord,
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntConvertForReturnT {
      range: tz_slice,
      expected_type: firefly_coord,
      actual_type: serenity_coord
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntConvertForMutateT {
      range: tz_slice,
      expected_type: firefly_coord,
      actual_type: serenity_coord
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CouldntConvertForMutateT {
      range: tz_slice,
      expected_type: firefly_coord,
      actual_type: serenity_coord
    }
  )
  .is_empty());
  let hp_var_name: &MemberNameT =
    typing_bump.alloc(MemberNameT { imprecise_name: scout_arena.intern_code_name(scout_arena.intern_str("hp")), loct: LocT { path: &[] } });
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CantMoveOutOfMemberT { range: tz_slice, name: IVarNameT::Member(hp_var_name) }
  )
  .is_empty());
  let firefly_var_name: &MemberNameT =
    typing_bump.alloc(MemberNameT { imprecise_name: scout_arena.intern_code_name(scout_arena.intern_str("firefly")), loct: LocT { path: &[] } });
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CantUseUnstackifiedLocal {
      range: tz_slice,
      local_id: IVarNameT::Member(firefly_var_name)
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile {
      range: tz_slice,
      local_id: IVarNameT::Member(firefly_var_name)
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::FunctionAlreadyExists {
      old_function_range: tz,
      new_function_range: tz,
      signature: *firefly_signature_id
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::LambdaReturnDoesntMatchInterfaceConstructor { range: tz_slice }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::ConditionIsntBoolean { range: tz_slice, actual_type: firefly_coord }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CantImplNonInterface {
      range: tz_slice,
      templata: ITemplataT::Kind(typing_bump.alloc(KindTemplataT { kind: firefly_kind }))
    }
  )
  .is_empty());
  let spaceship_snapshot_name_s =
    scout_arena.intern_struct_declaration_name(TopLevelStructDeclarationNameS {
      name: scout_arena.intern_str("SpaceshipSnapshot"),
      range: tz,
    });
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::ImmStructCantHaveVaryingMember {
      range: tz_slice,
      struct_name: INameS::TopLevelStructDeclaration(spaceship_snapshot_name_s),
      member_name: "fuel"
    }
  )
  .is_empty());
  let candidates_slice: &[IResolvingError] = typing_bump.alloc_slice_fill_iter(empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CantDowncastUnrelatedTypes {
      range: tz_slice,
      source_kind: ispaceship_kind,
      target_kind: unrelated_kind,
      candidates: candidates_slice
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::CantDowncastToInterface { range: tz_slice, target_kind: *ispaceship_tt }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::ExportedFunctionDependedOnNonExportedKind {
      range: tz_slice,
      paackage: *test_tld,
      signature: firefly_signature,
      non_exported_kind: firefly_kind
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::ExportedKindDependedOnNonExportedKind {
      range: tz_slice,
      paackage: *test_tld,
      exported_kind: serenity_kind,
      non_exported_kind: firefly_kind
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::ExternFunctionDependedOnNonExportedKind {
      range: tz_slice,
      paackage: *test_tld,
      signature: firefly_signature,
      non_exported_kind: firefly_kind
    }
  )
  .is_empty());
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::TypeExportedMultipleTimes {
      range: tz_slice,
      paackage: *test_tld,
      exports: exports_slice
    }
  )
  .is_empty());
  let x_rune =
    scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("X") }));
  let mut step_conclusions = HashMap::default();
  step_conclusions
    .insert(x_rune, ITemplataT::Kind(typing_bump.alloc(KindTemplataT { kind: firefly_kind })));
  assert!(!humanize(
    &scout_arena,
    &typing_interner,
    false,
    &humanize_pos,
    &lines_between,
    &line_range_containing,
    &line_containing,
    ICompileErrorT::TypingPassSolverError {
      range: tz_slice,
      failed_solve: FailedSolve {
        steps: vec![Step {
          complex: false,
          solved_rules: vec![],
          added_rules: vec![],
          conclusions: step_conclusions
        }],
        conclusions: HashMap::default(),
        unsolved_rules: vec![],
        unsolved_runes: vec![],
        error: ISolverError::RuleError(RuleError {
          err: ITypingPassSolverError::KindIsNotConcrete { kind: ispaceship_kind },
          _phantom: PhantomData
        }),
      }
    }
  )
  .is_empty());
}

#[test]
fn report_when_multiple_types_in_array() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
exported func main() int {
  arr = [#](true, 42);
  return arr.1;
}";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ArrayElementsHaveDifferentTypes { types, .. } => {
      let types_set: HashSet<KindT> = types.iter().copied().collect();
      assert_eq!(types_set, HashSet::from_iter([KindT::Int(IntT::I32), KindT::Bool(BoolT),]));
    }
    _other => panic!("expected ArrayElementsHaveDifferentTypes"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:2:1:
exported func main() int {
At test:0.vale:3:9:
  arr = [#](true, 42);
Array's elements have different types: bool, i32
"#,
  );
}

#[test]
fn report_when_abstract_method_defined_outside_open_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.panic.*;
interface IBlah { }
abstract func bork(virtual moo &IBlah);
exported func main() {
  bork(__vbi_panic());
}";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "panic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::AbstractMethodOutsideOpenInterface { .. } => {}
    _other => panic!("expected AbstractMethodOutsideOpenInterface"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:4:1:
abstract func bork(virtual moo &IBlah);
At test:0.vale:4:20:
abstract func bork(virtual moo &IBlah);
Open (non-sealed) interfaces can't have abstract methods defined outside the interface.
"#,
  );
}

// Deleted `report_when_imm_struct_has_varying_member` and `report_imm_mut_mismatch_for_generic_type`
// — ImmStructCantHave*Member validators no longer exist, so the tests had no target error to assert.

#[test]
fn tests_stamping_a_struct_and_its_implemented_interface_from_a_function_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.panicutils.*;\n",
    "import v.builtins.drop.*;\n",
    "import panicutils.*;\n",
    "sealed interface MyOption<T> where func drop(T)void { }\n",
    "struct MySome<T> where func drop(T)void { value T; }\n",
    "impl<T> MyOption<T> for MySome<T> where func drop(T)void;\n",
    "func moo(a MySome<int>) { }\n",
    "exported func main() { moo(__pretend<MySome<int>>()); }\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_panicutils(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "logic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "panicutils"),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let interface_template_name =
    compile.typing_interner.intern_interface_template_name(InterfaceTemplateNameT {
      human_namee: scout_arena.intern_str("MyOption"),
    });
  let struct_template_name = StructTemplateNameT { human_name: scout_arena.intern_str("MySome") };

  let coutputs = compile.expect_compiler_outputs();

  let interface = coutputs.lookup_interface_by_template_name(interface_template_name);
  let my_struct = coutputs.lookup_struct_by_template_name(struct_template_name);

  coutputs.lookup_impl(my_struct.instantiated_citizen.id, interface.instantiated_interface.id);
}

// TSUGAR: deleted `report_when_imm_contains_varying_member` — ImmStructCantHaveVaryingMember validator was removed.

#[test]
fn tests_calling_an_abstract_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = include_str!("../../tests/programs/genericvirtuals/callingAbstract.vale");
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  coutputs
    .functions
    .iter()
    .find(|f| {
      matches!(f.header.id.local_name,
          INameT::Function(
              FunctionNameT {
                  template: FunctionTemplateNameT { human_name, .. },
                  ..
              }
          )
          if human_name == "doThing"
      ) && f.header.get_abstract_interface().is_some()
    })
    .unwrap();
}

#[test]
fn test_struct_default_generic_argument_in_type() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct MyHashSet<K, H Int = 5> { }\n",
    "struct MyStruct {\n",
    "  x MyHashSet<bool>();\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let moo = coutputs.lookup_struct_by_str("MyStruct");
  let tyype = collect_only_tnode!(
      NodeRefT::StructDefinition(moo),
      NodeRefT::StructMember(m) => Some(m.tyype)
  );
  match tyype {
    KindT::Struct(StructTT {
      id:
        IdT {
          local_name:
            INameT::Struct(StructNameT {
              template:
                IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("MyHashSet"),
                  ..
                }),
              template_args:
                [ITemplataT::Kind(KindTemplataT { kind: KindT::Bool(_) }), ITemplataT::Integer(5)],
              ..
            }),
          ..
        },
      ..
    }) => {}
    _ => panic!("unexpected tyype"),
  }
}

#[test]
#[ignore] // VCOORD: re enable weaks
fn lock_weak_member() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.opt.*;\n",
    "import v.builtins.weak.*;\n",
    "import v.builtins.logic.*;\n",
    "import v.builtins.drop.*;\n",
    "import panicutils.*;\n",
    "import printutils.*;\n",
    "\n",
    "struct Base {\n",
    "  name str;\n",
    "}\n",
    "struct Spaceship {\n",
    "  name str;\n",
    "  origin &&Base;\n",
    "}\n",
    "func printShipBase(ship &Spaceship) {\n",
    "  maybeOrigin = lock(ship.origin);\n",
    // TSUGAR: line below was: "  if (not maybeOrigin.isEmpty()) {\n",
    "  if (not &maybeOrigin.isEmpty()) {\n",
    "    o = maybeOrigin.get();\n",
    "    println(\"Ship base: \" + o.name);\n",
    "  } else {\n",
    "    println(\"Ship base unknown!\");\n",
    "  }\n",
    "}\n",
    "exported func main() {\n",
    "  base = Base(\"Zion\");\n",
    "  ship = Spaceship(\"Neb\", &&base);\n",
    "  printShipBase(&ship);\n",
    "  (^base).drop();\n",
    "  printShipBase(&ship);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_weak(&parse_arena, &parser_keywords),
    Source::builtin_module(&parse_arena, &parser_keywords, "logic"),
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "panicutils"),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

// VCOORD: re-enable share things after onion
#[test]
#[ignore]
fn tests_destructuring_shared_doesnt_compile_to_destroy() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  return y;\n"
  let code = concat!(
    "\n",
    "struct Vec3i share {\n",
    "  x int;\n",
    "  y int;\n",
    "  z int;\n",
    "}\n",
    "\n",
    "exported func main() int {\n",
    "\t Vec3i[x, y, z] = Vec3i(3, 4, 5);\n",
    "  return __copy_prim(&y);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  let destroys = collect_where_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::Destroy(_) => Some(())
  );
  assert_eq!(destroys.len(), 0);
}

// VCOORD: re-enable share things after onion
#[test]
fn generates_free_function_for_imm_struct() {
  let code = r#"
        struct Vec3i share {
          x int;
          y int;
          z int;
        }
      "#;
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

// VCOORD: re-enable share things after onion
#[test]
fn reports_when_exported_ssa_depends_on_non_exported_element() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "export StaticArray<5, Raza> as RazaArray;\nstruct Raza share { }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExportedKindDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExportedKindDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
export StaticArray<5, Raza> as RazaArray;
Exported kind StaticArray<5, Raza> depends on kind Raza that wasn't exported from package test
"#,
  );
}

// VCOORD: re-enable share things after onion
#[test]
fn reports_when_exported_rsa_depends_on_non_exported_element() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "export []Raza as RazaArray;\nstruct Raza share { }";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected Err, got Ok");
  match &err {
    ICompileErrorT::ExportedKindDependedOnNonExportedKind { .. } => {}
    _other => panic!("expected ExportedKindDependedOnNonExportedKind"),
  }
  assert_humanized_eq(
    &humanize_compile_error(&mut compile, err),
    r#"At test:0.vale:1:1:
export []Raza as RazaArray;
Exported kind Array<Raza> depends on kind Raza that wasn't exported from package test
"#,
  );
}

// VCOORD: enable this
#[test]
fn test_make_array() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r#"
import v.builtins.arith.*;
import array.make.*;
import v.builtins.arrays.*;
import v.builtins.drop.*;

exported func main() int {
  a = MakeArray<int>(11, {__copy_prim(_)});
  return len(&a);
}
"#;
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "array.make"),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

// VCOORD: enable this
#[test]
fn test_array_push_pop_len_capacity_drop() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arrays.*;\n",
    "import v.builtins.drop.*;\n",
    "\n",
    "exported func main() void {\n",
    "  arr = Array<int>(9);\n",
    "  arr.push(420);\n",
    "  arr.push(421);\n",
    "  arr.push(422);\n",
    "  arr.len();\n",
    "  arr.capacity();\n",
    "  // implicit drop with pops\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

#[test]
fn upcast_generic() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "\n",
    "interface IShip {}\n",
    "\n",
    "struct Raza { fuel int; }\n",
    "impl IShip for Raza;\n",
    "\n",
    "func doUpcast<T>(x T) IShip\n",
    "where implements(T, IShip) {\n",
    "  i IShip = ^x;\n",
    "  return ^i;\n",
    "}\n",
    "\n",
    "exported func main() {\n",
    "  doUpcast(Raza(42));\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let do_upcast = coutputs.lookup_function_by_str("doUpcast");

  collect_only_tnode!(
      NodeRefT::FunctionDefinition(do_upcast),
      NodeRefT::Upcast(u) => {
          match u.inner_expr.result() {
              KindT::KindPlaceholder(_) => {}
              other => panic!("sourceExpr.result.coord.kind: {:?}", other),
          }
          match u.target_super_kind {
              ISuperKindTT::Interface(InterfaceTT {
                  id: IdT {
                      init_steps: &[],
                      local_name: INameT::Interface(InterfaceNameT {
                          template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                          template_args: &[],
                          ..
                      }),
                      ..
                  },
                  ..
              }) => {}
              other => panic!("targetSuperKind: {:?}", other),
          }
          Some(())
      }
  );
}

#[test]
fn downcast_function_rrbfs() {
  // Here we had something interesting happen: the complex solve had a race with the thing that
  // populates identifying runes.
  // Populating identifying runes only happens after the solver has done as much as it possibly
  // can... but the solver sometimes takes a leap (as part of CSALR, SMCMST) to figure out the best type
  // to meet some requirements.
  // The solution was to make it only do that leap when solving call sites.
  // See RRBFS.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "\n",
    "#!DeriveInterfaceDrop\n",
    "sealed interface Result<OkType, ErrType> { }\n",
    "\n",
    "#!DeriveStructDrop\n",
    "struct Ok<OkType, ErrType> { value OkType; }\n",
    "\n",
    "impl<OkType, ErrType> Result<OkType, ErrType> for Ok<OkType, ErrType>;\n",
    "\n",
    "#!DeriveStructDrop\n",
    "struct Err<OkType, ErrType> { value ErrType; }\n",
    "\n",
    "impl<OkType, ErrType> Result<OkType, ErrType> for Err<OkType, ErrType>;\n",
    "\n",
    "\n",
    "extern(\"vale_as_subtype\")\n",
    "func try_as<SubType, SuperType>(left &SuperType) Result<&SubType, &SuperType>\n",
    "where implements(SubType, SuperType);\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  {
    let as_funcs: Vec<_> = coutputs
      .functions
      .iter()
      .filter(|f| {
        matches!(
          f.header.id.local_name,
          INameT::Function(FunctionNameT {
            template: FunctionTemplateNameT { human_name: StrI("try_as"), .. },
            parameters: [KindT::BorrowRef(_)],
            ..
          })
        )
      })
      .copied()
      .collect();
    let as_func = expect_1(&as_funcs);
    let as_ = collect_only_tnode!(
        NodeRefT::FunctionDefinition(as_func),
        NodeRefT::AsSubtype(as_) => Some(as_)
    );
    let source_expr = as_.source_expr;
    let target_subtype = as_.target_type;
    let result_opt_type = as_.result;
    let ok_constructor = as_.ok_constructor;
    let err_constructor = as_.err_constructor;

    match source_expr.result() {
      KindT::BorrowRef(BorrowRefT {
        inner:
          KindT::KindPlaceholder(KindPlaceholderT {
            id:
              IdT {
                init_steps:
                  [INameT::FunctionTemplate(FunctionTemplateNameT {
                    human_name: StrI("try_as"), ..
                  })],
                local_name:
                  INameT::KindPlaceholder(KindPlaceholderNameT {
                    template: KindPlaceholderTemplateNameT { index: 1, .. },
                  }),
                ..
              },
            ..
          }),
        ..
      }) => {}
      other => panic!("sourceExpr.result: {:?}", other),
    }
    match target_subtype {
      KindT::BorrowRef(BorrowRefT {
        inner:
          KindT::KindPlaceholder(KindPlaceholderT {
            id:
              IdT {
                init_steps:
                  [INameT::FunctionTemplate(FunctionTemplateNameT {
                    human_name: StrI("try_as"), ..
                  })],
                local_name:
                  INameT::KindPlaceholder(KindPlaceholderNameT {
                    template: KindPlaceholderTemplateNameT { index: 0, .. },
                  }),
                ..
              },
            ..
          }),
        ..
      }) => {}
      KindT::Struct(StructTT {
        id:
          IdT {
            init_steps: &[],
            local_name:
              INameT::Struct(StructNameT {
                template:
                  IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                    human_name: StrI("Raza"),
                    ..
                  }),
                template_args: &[],
                ..
              }),
            ..
          },
        ..
      }) => {}
      other => panic!("targetSubtype.kind: {:?}", other),
    }
    let (first_generic_arg, second_generic_arg) = match result_opt_type {
      KindT::Interface(InterfaceTT {
        id:
          IdT {
            init_steps: &[],
            local_name:
              INameT::Interface(InterfaceNameT {
                template: InterfaceTemplateNameT { human_namee: StrI("Result"), .. },
                template_args: [first, second],
                ..
              }),
            ..
          },
        ..
      }) => (first, second),
      other => panic!("resultOptType: {:?}", other),
    };
    // They should both be pointers, since we dont really do borrows in structs yet
    match first_generic_arg {
      ITemplataT::Kind(KindTemplataT {
        kind:
          KindT::BorrowRef(BorrowRefT {
            inner:
              KindT::KindPlaceholder(KindPlaceholderT {
                id:
                  IdT {
                    init_steps:
                      [INameT::FunctionTemplate(FunctionTemplateNameT {
                        human_name: StrI("try_as"),
                        ..
                      })],
                    local_name:
                      INameT::KindPlaceholder(KindPlaceholderNameT {
                        template: KindPlaceholderTemplateNameT { index: 0, .. },
                      }),
                    ..
                  },
                ..
              }),
            ..
          }),
      }) => {}
      other => panic!("firstGenericArg: {:?}", other),
    }
    match second_generic_arg {
      ITemplataT::Kind(KindTemplataT {
        kind:
          KindT::BorrowRef(BorrowRefT {
            inner:
              KindT::KindPlaceholder(KindPlaceholderT {
                id:
                  IdT {
                    init_steps:
                      [INameT::FunctionTemplate(FunctionTemplateNameT {
                        human_name: StrI("try_as"),
                        ..
                      })],
                    local_name:
                      INameT::KindPlaceholder(KindPlaceholderNameT {
                        template: KindPlaceholderTemplateNameT { index: 1, .. },
                      }),
                    ..
                  },
                ..
              }),
            ..
          }),
      }) => {}
      other => panic!("secondGenericArg: {:?}", other),
    }
    assert_eq!(ok_constructor.id.local_name.parameters()[0], target_subtype);
    assert_eq!(err_constructor.id.local_name.parameters()[0], source_expr.result());
  }
}

// AFTERM: doublecheck this
#[test]
#[ignore] // VCOORD: re enable w borrowing
fn downcast_with_as() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.as.*;\n",
    "import v.builtins.logic.*;\n",
    "import v.builtins.drop.*;\n",
    "\n",
    "interface IShip {}\n",
    "\n",
    "struct Raza { fuel int; }\n",
    "impl IShip for Raza;\n",
    "\n",
    "exported func main() {\n",
    "  ship IShip = Raza(42);\n",
    "  ship.try_as<Raza>();\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_as(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  {
    let main_func = coutputs.lookup_function_by_str("main");
    let (as_prototype, as_arg) = collect_only_tnode!(
        NodeRefT::FunctionDefinition(main_func),
        NodeRefT::FunctionCall(c @ FunctionCallTE {
            callable: PrototypeT {
                id: IdT {
                    local_name: INameT::Function(FunctionNameT {
                        template: FunctionTemplateNameT { human_name: StrI("try_as"), .. },
                        ..
                    }),
                    init_steps: &[],
                    ..
                },
                ..
            },
            args: [_],
            ..
        }) => Some((c.callable, c.args[0]))
    );

    let (as_prototype_template_args, as_prototype_params, as_prototype_return) =
      match as_prototype.id.local_name {
        INameT::Function(fn_name) => {
          (fn_name.template_args, fn_name.parameters, as_prototype.return_type)
        }
        other => panic!("expected Function name: {:?}", other),
      };

    match as_prototype_template_args {
      [ITemplataT::Kind(KindTemplataT {
        kind:
          KindT::Struct(StructTT {
            id:
              IdT {
                init_steps: &[],
                local_name:
                  INameT::Struct(StructNameT {
                    template:
                      IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                        human_name: StrI("Raza"),
                        ..
                      }),
                    template_args: &[],
                    ..
                  }),
                ..
              },
            ..
          }),
      }), ITemplataT::Kind(KindTemplataT {
        kind:
          KindT::Interface(InterfaceTT {
            id:
              IdT {
                init_steps: &[],
                local_name:
                  INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                    template_args: &[],
                    ..
                  }),
                ..
              },
            ..
          }),
      })] => {}
      other => panic!("asPrototypeTemplateArgs: {:?}", other),
    }
    match as_prototype_params {
      [KindT::BorrowRef(BorrowRefT {
        inner:
          KindT::Interface(InterfaceTT {
            id:
              IdT {
                init_steps: &[],
                local_name:
                  INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                    template_args: &[],
                    ..
                  }),
                ..
              },
            ..
          }),
        ..
      })] => {}
      other => panic!("asPrototypeParams: {:?}", other),
    }
    match as_prototype_return {
      KindT::Interface(InterfaceTT {
        id:
          IdT {
            init_steps: &[],
            local_name:
              INameT::Interface(InterfaceNameT {
                template: InterfaceTemplateNameT { human_namee: StrI("Result"), .. },
                template_args:
                  [ITemplataT::Kind(KindTemplataT {
                    kind:
                      KindT::BorrowRef(BorrowRefT {
                        inner:
                          KindT::Struct(StructTT {
                            id:
                              IdT {
                                init_steps: &[],
                                local_name:
                                  INameT::Struct(StructNameT {
                                    template:
                                      IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                                        human_name: StrI("Raza"),
                                        ..
                                      }),
                                    template_args: &[],
                                    ..
                                  }),
                                ..
                              },
                            ..
                          }),
                        ..
                      }),
                  }), ITemplataT::Kind(KindTemplataT {
                    kind:
                      KindT::BorrowRef(BorrowRefT {
                        inner:
                          KindT::Interface(InterfaceTT {
                            id:
                              IdT {
                                init_steps: &[],
                                local_name:
                                  INameT::Interface(InterfaceNameT {
                                    template:
                                      InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                                    template_args: &[],
                                    ..
                                  }),
                                ..
                              },
                            ..
                          }),
                        ..
                      }),
                  })],
                ..
              }),
            ..
          },
        ..
      }) => {}
      other => panic!("asPrototypeReturn: {:?}", other),
    }
    match as_arg.result() {
      KindT::BorrowRef(BorrowRefT {
        inner:
          KindT::Interface(InterfaceTT {
            id:
              IdT {
                init_steps: &[],
                local_name:
                  INameT::Interface(InterfaceNameT {
                    template: InterfaceTemplateNameT { human_namee: StrI("IShip"), .. },
                    template_args: &[],
                    ..
                  }),
                ..
              },
            ..
          }),
        ..
      }) => {}
      other => panic!("asArg.result.coord: {:?}", other),
    }
  }

  {
    let as_funcs: Vec<_> = coutputs
      .functions
      .iter()
      .filter(|f| {
        matches!(
          f.header.id.local_name,
          INameT::Function(FunctionNameT {
            template: FunctionTemplateNameT { human_name: StrI("try_as"), .. },
            parameters: [KindT::BorrowRef(_)],
            ..
          })
        )
      })
      .copied()
      .collect();
    let as_func = expect_1(&as_funcs);
    let as_ = collect_only_tnode!(
        NodeRefT::FunctionDefinition(as_func),
        NodeRefT::AsSubtype(as_) => Some(as_)
    );
    let source_expr = as_.source_expr;
    let target_subtype = as_.target_type;
    let result_opt_type = as_.result;
    let ok_constructor = as_.ok_constructor;
    let err_constructor = as_.err_constructor;

    match source_expr.result() {
      KindT::BorrowRef(BorrowRefT {
        inner:
          KindT::KindPlaceholder(KindPlaceholderT {
            id:
              IdT {
                init_steps:
                  [INameT::FunctionTemplate(FunctionTemplateNameT {
                    human_name: StrI("try_as"), ..
                  })],
                local_name:
                  INameT::KindPlaceholder(KindPlaceholderNameT {
                    template: KindPlaceholderTemplateNameT { index: 1, .. },
                  }),
                ..
              },
            ..
          }),
        ..
      }) => {}
      other => panic!("sourceExpr.result: {:?}", other),
    }
    match target_subtype {
      KindT::BorrowRef(BorrowRefT {
        inner:
          KindT::KindPlaceholder(KindPlaceholderT {
            id:
              IdT {
                init_steps:
                  [INameT::FunctionTemplate(FunctionTemplateNameT {
                    human_name: StrI("try_as"), ..
                  })],
                local_name:
                  INameT::KindPlaceholder(KindPlaceholderNameT {
                    template: KindPlaceholderTemplateNameT { index: 0, .. },
                  }),
                ..
              },
            ..
          }),
        ..
      }) => {}
      KindT::Struct(StructTT {
        id:
          IdT {
            init_steps: &[],
            local_name:
              INameT::Struct(StructNameT {
                template:
                  IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                    human_name: StrI("Raza"),
                    ..
                  }),
                template_args: &[],
                ..
              }),
            ..
          },
        ..
      }) => {}
      other => panic!("targetSubtype.kind: {:?}", other),
    }
    match result_opt_type {
      KindT::Interface(InterfaceTT {
        id:
          IdT {
            init_steps: &[],
            local_name:
              INameT::Interface(InterfaceNameT {
                template: InterfaceTemplateNameT { human_namee: StrI("Result"), .. },
                template_args:
                  [ITemplataT::Kind(KindTemplataT {
                    kind:
                      KindT::BorrowRef(BorrowRefT {
                        inner:
                          KindT::KindPlaceholder(KindPlaceholderT {
                            id:
                              IdT {
                                local_name:
                                  INameT::KindPlaceholder(KindPlaceholderNameT {
                                    template: KindPlaceholderTemplateNameT { index: 0, .. },
                                  }),
                                ..
                              },
                            ..
                          }),
                        ..
                      }),
                  }), ITemplataT::Kind(KindTemplataT {
                    kind:
                      KindT::BorrowRef(BorrowRefT {
                        inner:
                          KindT::KindPlaceholder(KindPlaceholderT {
                            id:
                              IdT {
                                local_name:
                                  INameT::KindPlaceholder(KindPlaceholderNameT {
                                    template: KindPlaceholderTemplateNameT { index: 1, .. },
                                  }),
                                ..
                              },
                            ..
                          }),
                        ..
                      }),
                  })],
                ..
              }),
            ..
          },
        ..
      }) => {}
      other => panic!("resultOptType: {:?}", other),
    }
    assert_eq!(ok_constructor.id.local_name.parameters()[0], target_subtype);
    assert_eq!(err_constructor.id.local_name.parameters()[0], source_expr.result());
  }
}

// VCOORD: enable this
#[test]
#[ignore] // VCOORD: re enable w borrowing
fn closure_using_parent_function_s_bound() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: line below was: "  genFunc(7)\n"
  let code = concat!(
    "import v.builtins.arith.*;\n",
    "\n",
    "func genFunc<T>(a &T) T\n",
    "where func +(&T, &T)T {\n",
    "  { a + a }()\n",
    "}\n",
    "exported func main() int {\n",
    "  genFunc(&7)\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arith(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn test_struct_default_generic_argument_in_call() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct MyHashSet<K, H Int = 5> { }\n",
    "func moo() {\n",
    "  x = MyHashSet<bool>();\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let moo = coutputs.lookup_function_by_str("moo");
  let variable = collect_only_tnode!(
      NodeRefT::FunctionDefinition(moo),
      NodeRefT::LetNormal(let_normal) => Some(let_normal.variable)
  );
  match variable.tyype {
    KindT::Struct(StructTT {
      id:
        IdT {
          local_name:
            INameT::Struct(StructNameT {
              template:
                IStructTemplateNameT::StructTemplate(StructTemplateNameT {
                  human_name: StrI("MyHashSet"),
                  ..
                }),
              template_args:
                [ITemplataT::Kind(KindTemplataT { kind: KindT::Bool(_) }), ITemplataT::Integer(5)],
              ..
            }),
          ..
        },
      ..
    }) => {}
    _ => panic!("unexpected kind"),
  }
}

#[test]
fn structs_can_resolve_other_structs_instantiation_bound_arguments() {
  // The definition of Marine<T> was trying to resolve the existence of func drop(int)void.
  // Unfortunately, we don't have an overload index at the time of struct definitions yet, that comes later when
  // we define the functions.
  // Normally this wouldnt be a problem as we can usually use things before we compile them, we just use the templata
  // and solve the whole thing on our own, don't even need to know if it's been compiled yet.
  // However, now that we want to rely on the overload index, and the overload index doesn't exist until we compile
  // the functions, we rely on things being compiled before we use them, hence this problem.
  // The solution is to delay resolving function bounds until functions are compiled, see MCFBRBF.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "\n",
    "struct XNone<T> where func drop(T)void { }\n",
    "\n",
    "// This function will try to do a resolve for func drop(int)void.\n",
    "struct Marine { weapon XNone<int>; }\n",
    "\n",
    "exported func main() {\n",
    "  m = Marine(XNone<int>());\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

// VCOORD: revisit this, make sure its phrased well
// Compiling the struct discharges its own `where func drop(T)void` against the placeholder XNone$0,
// and drop.vale offers the borrow blanket `drop<T>(x &T)` as a candidate. Solving that candidate
// sends the placeholder at its `&T` parameter's full_type_rune, so the BorrowRef rule is asked to
// peel a kind that is not a borrow. A candidate whose shape the argument cannot satisfy must be
// rejected, never fault the solve. No instantiation is needed to reach this — the bound is
// discharged at the definition. Namespaces will eventually keep the blanket out of the candidate set
// entirely, since it belongs to &T's namespace, but nothing here depends on that.
#[test]
fn drop_bound_on_a_generic_struct_ignores_the_borrow_blanket() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "\n",
    "struct XNone<T> where func drop(T)void { }\n",
    "\n",
    "exported func main() { }\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}

// VCOORD: revisit to turn this into a real test
// arith probe — verifies source-level `__copy_prim(x)` flows correctly into
// binary operators. Rewrite to exercise auto-insertion when the syntax is retired.
#[test]
fn copy_prim_arith_probe() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.arith.*;\n",
    "exported func main() int {\n",
    "  x = 4;\n",
    "  return __copy_prim(&x) + 7;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_for_arith(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}
// VCOORD: revisit to turn this into a real test
// Bare-use of an Own local routes through `wrap_in_implicit_clone`. If no
// `implicit_clone(&T) T` is in scope for the local's type, the lookup fails with
// `CouldntFindFunctionToCallT` — confirming the error path of Step 1 auto-clone.
#[test]
fn bare_use_without_implicit_clone_errors() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // Deliberately no `import v.builtins.implicit_clone.*;`.
  let code = concat!(
    "struct MyStruct { }\n",
    "exported func main() {\n",
    "  x = MyStruct();\n",
    "  a MyStruct = x;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  match compile.get_compiler_outputs().err().unwrap() {
    ICompileErrorT::NoImplicitCloneDefinedT {
      source_type: KindT::BorrowRef(BorrowRefT { inner: KindT::Struct(_), .. }),
      ..
    } => {}
    other => panic!("expected NoImplicitCloneDefinedT for `implicit_clone`, got {:?}", other),
  }
}
// Ensures that when a user defines `implicit_clone(&T) T` for their struct kind, a bare
// assignment like `s2 = s;` silently fires that implicit_clone to give s2 a fresh Own T,
// e.g. `func implicit_clone(&Ship) Ship` + `s2 = s;` compiles and s2 ends up an owned Ship
// rather than a borrow. Verifies the FunctionCall to user's implicit_clone shows up in
// main's body.
#[test]
#[ignore = "silent auto-clone for Own struct locals via user-defined implicit_clone is not wired at let-binding sites; the RHS's Borrow flavor flows into s2, and downstream `^s2` hits vfail at soft_load BorrowT + MoveP. Un-ignore when let-binding routes through convert()'s (Borrow, Own) implicit_clone probe."]
fn user_defined_implicit_clone_allows_bare_use_of_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // VCOORD: doublecheck this logic, s2 = s; might be making a ref instead
  // when we might want it to copy? not sure yet.
  let code = concat!(
    "import v.builtins.implicit_clone.*;\n",
    "struct Ship { hp int; }\n",
    "func implicit_clone(s &Ship) Ship { return Ship(__copy_prim(&s.hp)); }\n",
    "func consume(s Ship) int { [hp] = ^s; return hp; }\n",
    "exported func main() int {\n",
    "  s = Ship(7);\n",
    "  s2 = s;\n",
    "  consume(^s);\n",
    "  return consume(^s2);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let main = coutputs.lookup_function_by_str("main");
  collect_only_tnode!(
      NodeRefT::FunctionDefinition(main),
      NodeRefT::FunctionCall(FunctionCallTE {
          callable: PrototypeT {
              id: IdT {
                  local_name: INameT::Function(FunctionNameT {
                      template: FunctionTemplateNameT { human_name: StrI("implicit_clone"), .. },
                      ..
                  }),
                  ..
              },
              ..
          },
          ..
      }) => Some(())
  );
}
// VCOORD: revisit to turn this into a real test
// `^x` (move) routes through the `Ownershipped` arm → `soft_load(LoadAsP::Move)`,
// bypassing `wrap_in_implicit_clone` entirely. No `implicit_clone` in scope —
// compiles fine.
#[test]
fn caret_bypasses_implicit_clone() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Ship {}\n",
    "func consume(s Ship) int { [] = ^s; return 7; }\n",
    "exported func main() int {\n",
    "  s = Ship();\n",
    "  return consume(^s);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}
// VCOORD: revisit to turn this into a real test
// `&x` (borrow) routes through the `Ownershipped` arm →
// `soft_load(LoadAsP::LoadAsBorrow)`, bypassing `wrap_in_implicit_clone`.
// No `implicit_clone` in scope — compiles fine.
#[test]
fn amp_bypasses_implicit_clone() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Ship {}\n",
    "func consume(s &Ship) int { return 7; }\n",
    "exported func main() int {\n",
    "  s = Ship();\n",
    "  a = consume(&s);\n",
    "  [] = ^s;\n",
    "  return ^a;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}
// VCOORD: revisit to turn this into a real test
// Bare member access through a borrow (`b.value` where `b: &MyBox`) hits the
// `coerce_to_reference_expression` auto-clone path (the other intervention site
// alongside `evaluate_lookup_for_load`). The Own+Int field is auto-cloned via
// the builtin `implicit_clone(&int)`.
#[test]
fn bare_member_access_auto_clones() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.implicit_clone.*;\n",
    "struct MyBox { value int; }\n",
    "func read(b &MyBox) int { return b.value; }\n",
    "exported func main() int {\n",
    "  b = MyBox(7);\n",
    "  a = read(&b);\n",
    "  [_] = ^b;\n",
    "  return a;\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}
// VCOORD: revisit to turn this into a real test
// probe for the source-level `__copy_prim(x)` syntax. The test compiles a
// tiny program that needs an Own+Int produced from a Borrow+Int field access (the
// natural Class A failure post-flip) and verifies wrapping with __copy_prim
// makes the call resolve. When auto-insertion replaces the syntax, this test
// should be rewritten to exercise the auto-insertion path (`&int → int` coerce)
// rather than the source-level syntax — the underlying invariant (CopyPrim
// resolves an Own+Int from a Borrow+Int field access) is still worth testing.
#[test]
fn copy_prim_probe() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.implicit_clone.*;\n",
    "struct MyBox { value int; }\n",
    "func consume(i int) int { return ^i; }\n",
    "exported func main() int {\n",
    "  b = MyBox(7);\n",
    "  return consume(__copy_prim(&b.value));\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}
// VCOORD: revisit to turn this into a real test
// VCOORD: re-enable share things after onion
#[test]
fn borrow_share_as_arg_to_generic_func_that_takes_borrowed_things() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct Ship share { }\n",
    "func drop<T>(x &T) {}\n",
    "exported func main() {\n",
    "  s = Ship();\n",
    "  drop(&s);\n",
    "}\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let _coutputs = compile.expect_compiler_outputs();
}
