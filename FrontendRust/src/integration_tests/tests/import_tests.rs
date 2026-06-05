/*
package dev.vale

import dev.vale.passmanager.FullCompilationOptions
import dev.vale.finalast._
import dev.vale.von.VonInt
import dev.vale.{finalast => m}
import org.scalatest._

import scala.collection.immutable.List
*/
// mig: struct ImportTests
pub struct ImportTests;
/*
class ImportTests extends FunSuite with Matchers {
*/
// mig: fn tests_import
#[test]
fn tests_import() {
    use crate::utils::code_hierarchy::IPackageResolver;
    let module_a_code =
        "\nimport moduleB.moo;\n\nexported func main() int {\n  a = moo();\n  return a;\n}\n".to_string();

    let module_b_code =
        "\nfunc moo() int { return 42; }\n".to_string();

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
    let mut map: crate::utils::code_hierarchy::FileCoordinateMap<String> = crate::utils::code_hierarchy::FileCoordinateMap::new();
    map.put(
        parse_arena.intern_file_coordinate(
            parse_arena.intern_package_coordinate(parse_arena.intern_str("moduleA"), &[]),
            "moduleA.vale"),
        module_a_code);
    map.put(
        parse_arena.intern_file_coordinate(
            parse_arena.intern_package_coordinate(parse_arena.intern_str("moduleB"), &[]),
            "moduleB.vale"),
        module_b_code);

    let packages_to_build: Vec<&crate::utils::code_hierarchy::PackageCoordinate> = vec![
        crate::utils::code_hierarchy::PackageCoordinate::builtin(&parse_arena, &parser_keywords),
        parse_arena.intern_package_coordinate(parse_arena.intern_str("moduleA"), &[]),
    ];
    let base_code_map =
        crate::builtins::builtins::get_code_map(&parse_arena, &parser_keywords)
            .expect("Builtins code map failed to load");
    let resolver_concrete = base_code_map
        .or(map)
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let resolver: &dyn crate::utils::code_hierarchy::IPackageResolver<std::collections::HashMap<String, String>> =
        compilation_bump.alloc(resolver_concrete);
    let options = crate::simplifying::hammer_compilation::HammerCompilationOptions {
        global_options: crate::compile_options::GlobalOptions {
            sanity_check: true,
            use_overload_index: true,
            use_optimized_solver: true,
            verbose_errors: true,
            debug_output: true,
        },
        ..crate::simplifying::hammer_compilation::HammerCompilationOptions::new()
    };
    let mut compile = crate::integration_tests::tests::run_compilation::RunCompilation {
        interner: &typing_interner,
        hammer_compilation: crate::simplifying::hammer_compilation::HammerCompilation::new(
            &scout_arena, &hammer_interner, &typing_interner, &keywords, &parser_keywords, &parse_arena,
            packages_to_build, resolver, options, &instantiating_bump,
        ),
    };

    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Tests import") {
    val moduleACode =
      """
        |import moduleB.moo;
        |
        |exported func main() int {
        |  a = moo();
        |  return a;
        |}
      """.stripMargin

    val moduleBCode =
      """
        |func moo() int { return 42; }
      """.stripMargin

    val interner = new Interner()
    val keywords = new Keywords(interner)
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(
          interner.intern(StrI("moduleA")),
          Vector.empty)),
        "moduleA.vale")),
      moduleACode)
    map.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector.empty)),
        "moduleB.vale")),
      moduleBCode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_non_imported_module_isnt_brought_in
#[test]
fn tests_non_imported_module_isnt_brought_in() {
    use crate::utils::code_hierarchy::IPackageResolver;
    let module_a_code =
        "\nexported func main() int {\n  a = 42;\n  return a;\n}\n".to_string();

    let module_b_code =
        "\nfunc moo() int { return 73; }\n".to_string();

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
    let module_a_coord = parse_arena.intern_package_coordinate(parse_arena.intern_str("moduleA"), &[]);
    let module_b_coord = parse_arena.intern_package_coordinate(parse_arena.intern_str("moduleB"), &[]);
    let mut map: crate::utils::code_hierarchy::FileCoordinateMap<String> = crate::utils::code_hierarchy::FileCoordinateMap::new();
    map.put(
        parse_arena.intern_file_coordinate(module_a_coord, "moduleA.vale"),
        module_a_code);
    map.put(
        parse_arena.intern_file_coordinate(module_b_coord, "moduleB.vale"),
        module_b_code);

    let packages_to_build: Vec<&crate::utils::code_hierarchy::PackageCoordinate> = vec![
        crate::utils::code_hierarchy::PackageCoordinate::builtin(&parse_arena, &parser_keywords),
        module_a_coord,
    ];
    let base_code_map =
        crate::builtins::builtins::get_code_map(&parse_arena, &parser_keywords)
            .expect("Builtins code map failed to load");
    let resolver_concrete = base_code_map
        .or(map)
        .or(crate::tests::tests::get_package_to_resource_resolver());
    let resolver: &dyn crate::utils::code_hierarchy::IPackageResolver<std::collections::HashMap<String, String>> =
        compilation_bump.alloc(resolver_concrete);
    let options = crate::simplifying::hammer_compilation::HammerCompilationOptions {
        global_options: crate::compile_options::GlobalOptions {
            sanity_check: true,
            use_overload_index: true,
            use_optimized_solver: true,
            verbose_errors: true,
            debug_output: true,
        },
        ..crate::simplifying::hammer_compilation::HammerCompilationOptions::new()
    };
    let mut compile = crate::integration_tests::tests::run_compilation::RunCompilation {
        interner: &typing_interner,
        hammer_compilation: crate::simplifying::hammer_compilation::HammerCompilation::new(
            &scout_arena, &hammer_interner, &typing_interner, &keywords, &parser_keywords, &parse_arena,
            packages_to_build, resolver, options, &instantiating_bump,
        ),
    };

    assert!(!compile.get_parseds().unwrap().package_coord_to_file_coords.contains_key(&module_b_coord));

    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Tests non-imported module isn't brought in") {
    val moduleACode =
      """
        |exported func main() int {
        |  a = 42;
        |  return a;
        |}
      """.stripMargin

    val moduleBCode =
      """
        |func moo() int { return 73; }
      """.stripMargin


    val interner = new Interner()
    val keywords = new Keywords(interner)
    val moduleACoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))
    val moduleBCoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector.empty))
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(moduleACoord, "moduleA.vale")),
      moduleACode)
    map.put(
      interner.intern(FileCoordinate(moduleBCoord, "moduleB.vale")),
      moduleBCode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), moduleACoord),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())

    vassert(!compile.getParseds().getOrDie().packageCoordToFileCoords.contains(moduleBCoord))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_import_with_paackage
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_import_with_paackage() {
    panic!("Unmigrated test: tests_import_with_paackage");
}
/*
  test("Tests import with paackage") {
    val moduleACode =
      """
        |import moduleB.bork.*;
        |
        |exported func main() int {
        |  a = moo();
        |  return a;
        |}
      """.stripMargin

    val moduleBCode =
      """
        |func moo() int { return 42; }
      """.stripMargin

    val interner = new Interner()
    val keywords = new Keywords(interner)
    val moduleACoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))
    val moduleBCoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector(interner.intern(StrI("bork")))))
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(moduleACoord, "moduleA.vale")),
      moduleACode)
    map.put(
      interner.intern(FileCoordinate(moduleBCoord, "moduleB.vale")),
      moduleBCode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), moduleACoord),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_import_of_directory_with_no_vale_files
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_import_of_directory_with_no_vale_files() {
    panic!("Unmigrated test: tests_import_of_directory_with_no_vale_files");
}
/*
  test("Tests import of directory with no vale files") {
    val moduleACode =
      """
        |import moduleB.bork.*;
        |
        |exported func main() int {
        |  a = 42;
        |  return a;
        |}
      """.stripMargin

    val interner = new Interner()
    val keywords = new Keywords(interner)
    val moduleACoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))
    val moduleBCoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector(interner.intern(StrI("bork")))))
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(moduleACoord, "moduleA.vale")),
      moduleACode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))),
        Builtins.getCodeMap(interner, keywords)
          .or(Tests.getPackageToResourceResolver)
          .or(map)
          .or({ case PackageCoordinate(StrI("moduleB"), Vector(StrI("bork"))) => Some(Map()) }),
    FullCompilationOptions())

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/

/*
}

*/
