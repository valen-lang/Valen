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
#[ignore]
fn function_has_correct_name() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main() { }";
    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compile = compiler_test_compilation(
        &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver, &typing_bump,
    );
    compile.expect_compiler_outputs();
    // val packageCoord = interner.intern(PackageCoordinate(interner.intern(StrI("test")),Vector()))
    // val mainLoc = CodeLocationS(interner.intern(FileCoordinate(packageCoord, "test.vale")), 0)
    // val mainTemplateName = interner.intern(FunctionTemplateNameT(interner.intern(StrI("main")), mainLoc))
    // val mainName = interner.intern(FunctionNameT(mainTemplateName, Vector(), Vector()))
    // val id = IdT(packageCoord, Vector(), mainName)
    // vassertSome(coutputs.functions.headOption).header.id shouldEqual id
    panic!("Not yet implemented: function_has_correct_name assertions");
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
#[ignore]
fn lambda_has_correct_name() {
    panic!("Unmigrated test: lambda_has_correct_name");
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
#[ignore]
fn struct_has_correct_name() {
    panic!("Unmigrated test: struct_has_correct_name");
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
