use super::compiler_test_compilation::compiler_test_compilation;
use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
/*
package dev.vale.typing

import dev.vale.postparsing._
import dev.vale.{CodeLocationS, FileCoordinate, PackageCoordinate, RangeS, StrI, Tests, vassert, vassertSome, vimpl}
import dev.vale.typing.ast.SignatureT
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.types._
import org.scalatest._

import scala.collection.immutable.List

class CompilerProjectTests extends FunSuite with Matchers {
*/
// mig: fn function_has_correct_name
#[test]
fn function_has_correct_name() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() { }";
    let resolver = code_hierarchy::test_from_map(
            &parse_arena,
            HashMap::from([("test.vale".to_string(), code.to_string())]),
        )
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let id = {
        let typing_interner = &compile.typing_interner;
        let package_coord = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);
        let main_loc = crate::utils::range::CodeLocationS {
            file: scout_arena.intern_file_coordinate(package_coord, "test.vale"),
            offset: 0,
        };
        let main_template_name = typing_interner.intern_function_template_name(
            crate::typing::names::names::FunctionTemplateNameT {
                human_name: scout_arena.intern_str("main"),
                code_location: main_loc,
                _phantom: std::marker::PhantomData,
            });
        let main_name = typing_interner.intern_function_name(
            crate::typing::names::names::FunctionNameValT {
                template: main_template_name,
                template_args: &[],
                parameters: &[],
            });
        *typing_interner.intern_id(crate::typing::names::names::IdValT {
            package_coord,
            init_steps: &[],
            local_name: crate::typing::names::names::INameT::Function(main_name),
        })
    };
    let coutputs = compile.expect_compiler_outputs();
    assert_eq!(coutputs.functions.first().unwrap().header.id, id);
}
/*
  test("Function has correct name") {
    val compile =
      CompilerTestCompilation.test(
        """exported func main() { }""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner

    val packageCoord = interner.intern(PackageCoordinate(interner.intern(StrI("test")),Vector()))
    val mainLoc = CodeLocationS(interner.intern(FileCoordinate(packageCoord, "test.vale")), 0)
    val mainTemplateName = interner.intern(FunctionTemplateNameT(interner.intern(StrI("main")), mainLoc))
    val mainName = interner.intern(FunctionNameT(mainTemplateName, Vector(), Vector()))
    val id = IdT(packageCoord, Vector(), mainName)
    vassertSome(coutputs.functions.headOption).header.id shouldEqual id
  }
*/
// mig: fn lambda_has_correct_name
#[test]
fn lambda_has_correct_name() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() { {}() }";
    let resolver = code_hierarchy::test_from_map(
            &parse_arena,
            HashMap::from([("test.vale".to_string(), code.to_string())]),
        )
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let lambda_func_id = {
        let typing_interner = &compile.typing_interner;
        let package_coord = scout_arena.intern_package_coordinate(scout_arena.intern_str("test"), &[]);
        let file_coord = scout_arena.intern_file_coordinate(package_coord, "test.vale");
        let main_loc = crate::utils::range::CodeLocationS { file: file_coord, offset: 0 };
        let main_template_name = typing_interner.intern_function_template_name(
            crate::typing::names::names::FunctionTemplateNameT {
                human_name: scout_arena.intern_str("main"),
                code_location: main_loc,
                _phantom: std::marker::PhantomData,
            });
        let main_name = typing_interner.intern_function_name(
            crate::typing::names::names::FunctionNameValT {
                template: main_template_name,
                template_args: &[],
                parameters: &[],
            });
        let lambda_loc = crate::utils::range::CodeLocationS { file: file_coord, offset: 23 };
        let lambda_citizen_template_name = typing_interner.intern_lambda_citizen_template_name(
            crate::typing::names::names::LambdaCitizenTemplateNameT {
                code_location: lambda_loc,
                _phantom: std::marker::PhantomData,
            });
        let lambda_citizen_name = typing_interner.intern_lambda_citizen_name(
            crate::typing::names::names::LambdaCitizenNameT { template: lambda_citizen_template_name });
        let lambda_citizen_id = typing_interner.intern_id(crate::typing::names::names::IdValT {
            package_coord,
            init_steps: &[crate::typing::names::names::INameT::Function(main_name)],
            local_name: crate::typing::names::names::INameT::LambdaCitizen(lambda_citizen_name),
        });
        let lambda_struct = typing_interner.intern_struct_tt(
            crate::typing::types::types::StructTTValT { id: *lambda_citizen_id });
        let lambda_share_coord = crate::typing::types::types::CoordT {
            ownership: crate::typing::types::types::OwnershipT::Share,
            region: crate::typing::types::types::RegionT,
            kind: crate::typing::types::types::KindT::Struct(lambda_struct),
        };
        let lambda_func_template_name = typing_interner.intern_lambda_call_function_template_name(
            crate::typing::names::names::LambdaCallFunctionTemplateNameValT {
                code_location: lambda_loc,
                param_types: &[lambda_share_coord],
            });
        let lambda_func_name = typing_interner.intern_lambda_call_function_name(
            crate::typing::names::names::LambdaCallFunctionNameValT {
                template: lambda_func_template_name,
                template_args: &[],
                parameters: &[lambda_share_coord],
            });
        *typing_interner.intern_id(crate::typing::names::names::IdValT {
            package_coord,
            init_steps: &[
                crate::typing::names::names::INameT::Function(main_name),
                crate::typing::names::names::INameT::LambdaCitizenTemplate(lambda_citizen_template_name),
            ],
            local_name: crate::typing::names::names::INameT::LambdaCallFunction(lambda_func_name),
        })
    };
    let coutputs = compile.expect_compiler_outputs();
    let lam_func = coutputs.lookup_lambda_in("main");
    assert_eq!(lam_func.header.id, lambda_func_id);
}
/*
  test("Lambda has correct name") {
    val compile =
      CompilerTestCompilation.test(
        """exported func main() { {}() }""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val interner = compile.interner

    val packageCoord = interner.intern(PackageCoordinate(interner.intern(StrI("test")),Vector()))
    val mainLoc = CodeLocationS(interner.intern(FileCoordinate(packageCoord, "test.vale")), 0)
    val mainTemplateName = interner.intern(FunctionTemplateNameT(interner.intern(StrI("main")), mainLoc))
    val mainName = interner.intern(FunctionNameT(mainTemplateName, Vector(), Vector()))

    val lambdaLoc = CodeLocationS(interner.intern(FileCoordinate(packageCoord, "test.vale")), 23)
    val lambdaCitizenTemplateName = interner.intern(LambdaCitizenTemplateNameT(lambdaLoc))
    val lambdaCitizenName = interner.intern(LambdaCitizenNameT(lambdaCitizenTemplateName))
    val lambdaFuncTemplateName = interner.intern(LambdaCallFunctionTemplateNameT(lambdaLoc, Vector(CoordT(ShareT,RegionT(), interner.intern(StructTT(IdT(packageCoord, Vector(mainName), lambdaCitizenName)))))))
    val lambdaCitizenId = IdT(packageCoord, Vector(mainName), lambdaCitizenName)
    val lambdaStruct = interner.intern(StructTT(lambdaCitizenId))
    val lambdaShareCoord = CoordT(ShareT, RegionT(), lambdaStruct)
    val lambdaFuncName = interner.intern(LambdaCallFunctionNameT(lambdaFuncTemplateName, Vector(), Vector(lambdaShareCoord)))
    val lambdaFuncId =
      IdT(packageCoord, Vector(mainName, lambdaCitizenTemplateName), lambdaFuncName)

    val lamFunc = coutputs.lookupLambdaIn("main")
    lamFunc.header.id shouldEqual lambdaFuncId
  }
*/
// mig: fn struct_has_correct_name
#[test]
fn struct_has_correct_name() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "\n\nexported struct MyStruct { a int; }\n";
    let resolver = code_hierarchy::test_from_map(
            &parse_arena,
            HashMap::from([("test.vale".to_string(), code.to_string())]),
        )
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let coutputs = compile.expect_compiler_outputs();
    let struct_ = coutputs.lookup_struct_by_str("MyStruct");
    match (struct_.template_name.init_steps, struct_.template_name.local_name) {
        (
            [],
            crate::typing::names::names::INameT::StructTemplate(t),
        ) if t.human_name.0 == "MyStruct" => {
            assert!(struct_.template_name.package_coord.is_test());
        }
        _ => panic!("struct.templateName didn't match expected pattern"),
    }
}
/*
  test("Struct has correct name") {
    val compile =
      CompilerTestCompilation.test(
        """
          |
          |exported struct MyStruct { a int; }
          |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    val struct = coutputs.lookupStruct("MyStruct")
    struct.templateName match {
      case IdT(x,Vector(),StructTemplateNameT(StrI("MyStruct"))) => {
        vassert(x.isTest)
      }
    }
  }
}
*/

// NOVEL CODE — exploratory: run the typing pass on roguelike.vale to gauge
// how close the migration is to handling a real-world program. Loads the
// roguelike.vale source from disk, rewrites `stdlib.*` imports to use the
// Rust-side on-disk file layout, and feeds the program through
// compiler_test_compilation. Currently #[ignore]'d so it doesn't gate CI.
#[test]
#[ignore]
fn typing_pass_on_roguelike() {
    use crate::builtins::builtins::get_embedded_modulized_code_map;
    use crate::tests::tests::get_package_to_resource_resolver;

    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);

    let source_raw =
        std::fs::read_to_string("src/tests/programs/roguelike.vale")
            .expect("could not read src/tests/programs/roguelike.vale");
    // Rewrite stdlib-style imports to the Rust on-disk test-package layout.
    let source = source_raw
        .replace("import stdlib.collections.hashmap.*;", "import hashmap.*;\nimport list.*;")
        .replace("import stdlib.stdin.*;", "import printutils.*;");

    let resolver = code_hierarchy::test_from_map(
            &parse_arena,
            HashMap::from([("0.vale".to_string(), source)]),
        )
        .or(get_embedded_modulized_code_map(&parse_arena, &parser_keywords))
        .or(get_package_to_resource_resolver());
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    let result = compile.get_compiler_outputs();
    match result {
        Ok(_) => println!("DIAG: roguelike typing pass succeeded"),
        Err(e) => panic!("DIAG: roguelike typing pass failed: {:#?}", e),
    }
}
