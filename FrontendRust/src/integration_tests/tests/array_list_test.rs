// mig: struct ArrayListTest
pub struct ArrayListTest;
/*
package dev.vale

import dev.vale.typing.ast.LetNormalTE
import dev.vale.typing.env.AddressibleLocalVariableT
import dev.vale.typing.names.{CodeVarNameT, IdT}
import dev.vale.typing.types.VaryingT
import dev.vale.typing.names.CodeVarNameT
import dev.vale.von.VonInt
import org.scalatest._

class ArrayListTest extends FunSuite with Matchers {
*/
// mig: fn simple_array_list_no_optionals
#[test]
fn simple_array_list_no_optionals() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nimport v.builtins.migrate.*;\n\n#!DeriveStructDrop\nstruct List<E Ref> {\n  array! []<mut>E;\n}\nfunc drop<E>(self List<E>)\nwhere func drop(E)void {\n  [array] = self;\n  drop(array);\n}\nfunc len<E>(list &List<E>) int { return len(&list.array); }\nfunc add<E>(list &List<E>, newElement E) {\n  oldArray = set list.array = Array<mut, E>(len(&list) + 1);\n  migrate(oldArray, list.array);\n  list.array.push(newElement);\n}\nfunc get<E>(list &List<E>, index int) &E {\n  a = list.array;\n  return a[index];\n}\nexported func main() int {\n  l = List<int>(Array<mut, int>(0));\n  add(&l, 5);\n  add(&l, 9);\n  add(&l, 7);\n  return l.get(1);\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Simple ArrayList, no optionals") {
    val compile = RunCompilation.test(
        """
          |import v.builtins.migrate.*;
          |
          |#!DeriveStructDrop
          |struct List<E Ref> {
          |  array! []<mut>E;
          |}
          |func drop<E>(self List<E>)
          |where func drop(E)void {
          |  [array] = self;
          |  drop(array);
          |}
          |func len<E>(list &List<E>) int { return len(&list.array); }
          |func add<E>(list &List<E>, newElement E) {
          |  oldArray = set list.array = Array<mut, E>(len(&list) + 1);
          |  migrate(oldArray, list.array);
          |  list.array.push(newElement);
          |}
          |func get<E>(list &List<E>, index int) &E {
          |  a = list.array;
          |  return a[index];
          |}
          |exported func main() int {
          |  l = List<int>(Array<mut, int>(0));
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  return l.get(1);
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn doubling_array_list
#[test]
fn doubling_array_list() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nimport list.*;\n\nexported func main() int {\n  l = List<int>(Array<mut, int>(0));\n  add(&l, 5);\n  add(&l, 9);\n  add(&l, 7);\n  return l.get(1);\n}\n\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Doubling ArrayList") {
    val compile = RunCompilation.test(
      """
        |import list.*;
        |
        |exported func main() int {
        |  l = List<int>(Array<mut, int>(0));
        |  add(&l, 5);
        |  add(&l, 9);
        |  add(&l, 7);
        |  return l.get(1);
        |}
        |
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn array_list_zero_constructor
#[test]
fn array_list_zero_constructor() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\n\nexported func main() int {\n  l = List<int>();\n  add(&l, 5);\n  add(&l, 9);\n  add(&l, 7);\n  return l.get(1);\n}\n\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Array list zero-constructor") {
    val compile = RunCompilation.test(
        """import list.*;
          |
          |exported func main() int {
          |  l = List<int>();
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  return l.get(1);
          |}
          |
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn array_list_len
#[test]
fn array_list_len() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\n\nexported func main() int {\n  l = List<int>();\n  add(&l, 5);\n  add(&l, 9);\n  add(&l, 7);\n  return l.len();\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Array list len") {
    val compile = RunCompilation.test(
        """import list.*;
          |
          |exported func main() int {
          |  l = List<int>();
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  return l.len();
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn array_list_set
#[test]
fn array_list_set() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\n\nexported func main() int {\n  l = List<int>();\n  add(&l, 5);\n  add(&l, 9);\n  add(&l, 7);\n  set(&l, 1, 11);\n  return l.get(1);\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 11 }) => {}
        other => panic!("expected VonInt(11), got {:?}", other),
    }
}
/*
  test("Array list set") {
    val compile = RunCompilation.test(
        """import list.*;
          |
          |exported func main() int {
          |  l = List<int>();
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  set(&l, 1, 11);
          |  return l.get(1);
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(11) => }
  }
*/
// mig: fn array_list_with_optionals_with_mutable_element
#[test]
fn array_list_with_optionals_with_mutable_element() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\nstruct Marine { hp int; }\n\nexported func main() int {\n  l =\n      List<Marine>(\n          Array<mut, Marine>(\n              0,\n              (index) => { Marine(index) }));\n  add(&l, Marine(5));\n  add(&l, Marine(9));\n  add(&l, Marine(7));\n  return l.get(1).hp;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Array list with optionals with mutable element") {
    val compile = RunCompilation.test(
        """import list.*;
          |struct Marine { hp int; }
          |
          |exported func main() int {
          |  l =
          |      List<Marine>(
          |          Array<mut, Marine>(
          |              0,
          |              (index) => { Marine(index) }));
          |  add(&l, Marine(5));
          |  add(&l, Marine(9));
          |  add(&l, Marine(7));
          |  return l.get(1).hp;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn mutate_mutable_from_in_lambda
#[test]
fn mutate_mutable_from_in_lambda() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\nstruct Marine { hp int; }\n\nexported func main() int {\n  m = Marine(6);\n  lam = {\n    set m = Marine(9);\n  };\n  lam();\n  lam();\n  return m.hp;\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::LetNormal(crate::typing::ast::expressions::LetNormalTE {
                variable: crate::typing::env::function_environment_t::ILocalVariableT::Addressible(crate::typing::env::function_environment_t::AddressibleLocalVariableT {
                    name: crate::typing::names::names::IVarNameT::CodeVar(crate::typing::names::names::CodeVarNameT { name: crate::interner::StrI("m"), .. }),
                    variability: crate::typing::types::types::VariabilityT::Varying,
                    ..
                }),
                ..
            }) => Some(())
        );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Mutate mutable from in lambda") {
    val compile = RunCompilation.test(
        """import list.*;
          |struct Marine { hp int; }
          |
          |exported func main() int {
          |  m = Marine(6);
          |  lam = {
          |    set m = Marine(9);
          |  };
          |  lam();
          |  lam();
          |  return m.hp;
          |}
        """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main");
    Collector.only(main, {
      case LetNormalTE(AddressibleLocalVariableT(CodeVarNameT(StrI("m")), VaryingT, _), _) => {
        vpass()
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn move_mutable_from_in_lambda
#[test]
fn move_mutable_from_in_lambda() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\nstruct Marine { hp int; }\n\nexported func main() int {\n  m Opt<Marine> = Some(Marine(6));\n  lam = {\n    m2 = (set m = None<Marine>()).get();\n    m2.hp\n  };\n  return lam();\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::LetNormal(crate::typing::ast::expressions::LetNormalTE {
                variable: crate::typing::env::function_environment_t::ILocalVariableT::Addressible(crate::typing::env::function_environment_t::AddressibleLocalVariableT {
                    name: crate::typing::names::names::IVarNameT::CodeVar(crate::typing::names::names::CodeVarNameT { name: crate::interner::StrI("m"), .. }),
                    variability: crate::typing::types::types::VariabilityT::Varying,
                    ..
                }),
                ..
            }) => Some(())
        );
    }

    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 6 }) => {}
        other => panic!("expected VonInt(6), got {:?}", other),
    }
}
/*
  test("Move mutable from in lambda") {
    val compile = RunCompilation.test(
      """import list.*;
        |struct Marine { hp int; }
        |
        |exported func main() int {
        |  m Opt<Marine> = Some(Marine(6));
        |  lam = {
        |    m2 = (set m = None<Marine>()).get();
        |    m2.hp
        |  };
        |  return lam();
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main");
    Collector.only(main, { case LetNormalTE(AddressibleLocalVariableT(CodeVarNameT(StrI("m")), VaryingT, _), _) => })

    compile.evalForKind(Vector()) match { case VonInt(6) => }
  }
*/
// mig: fn remove_from_middle
#[test]
fn remove_from_middle() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\nimport panicutils.*;\nstruct Marine { hp int; }\n\nexported func main() {\n  l = List<Marine>();\n  add(&l, Marine(5));\n  add(&l, Marine(7));\n  add(&l, Marine(9));\n  add(&l, Marine(11));\n  add(&l, Marine(13));\n  l.remove(2);\n  vassert(l.get(0).hp == 5);\n  vassert(l.get(1).hp == 7);\n  vassert(l.get(2).hp == 11);\n  vassert(l.get(3).hp == 13);\n}\n",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}
/*
  test("Remove from middle") {
    val compile = RunCompilation.test(
        """import list.*;
          |import panicutils.*;
          |struct Marine { hp int; }
          |
          |exported func main() {
          |  l = List<Marine>();
          |  add(&l, Marine(5));
          |  add(&l, Marine(7));
          |  add(&l, Marine(9));
          |  add(&l, Marine(11));
          |  add(&l, Marine(13));
          |  l.remove(2);
          |  vassert(l.get(0).hp == 5);
          |  vassert(l.get(1).hp == 7);
          |  vassert(l.get(2).hp == 11);
          |  vassert(l.get(3).hp == 13);
          |}
        """.stripMargin)

    compile.evalForKind(Vector())
  }
*/
// mig: fn remove_from_beginning
#[test]
fn remove_from_beginning() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import list.*;\nimport panicutils.*;\nstruct Marine { hp int; }\n\nexported func main() {\n  l = List<Marine>();\n  add(&l, Marine(5));\n  add(&l, Marine(7));\n  l.remove(0);\n  l.remove(0);\n  vassert(l.len() == 0);\n}\n",
    );
    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}
/*
  test("Remove from beginning") {
    val compile = RunCompilation.test(
        """import list.*;
          |import panicutils.*;
          |struct Marine { hp int; }
          |
          |exported func main() {
          |  l = List<Marine>();
          |  add(&l, Marine(5));
          |  add(&l, Marine(7));
          |  l.remove(0);
          |  l.remove(0);
          |  vassert(l.len() == 0);
          |}
        """.stripMargin)

    compile.evalForKind(Vector())
  }
}
*/
