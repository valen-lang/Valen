// cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::top_level_tests

#![allow(nonstandard_style)]

use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ParseError;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

/*
package dev.vale.parsing

import dev.vale.{Collector, Interner, StrI, vassertOne, vassertSome}
import dev.vale.parsing.ast.{BlockPE, CallPT, ExportAsP, FileP, FunctionP, ImportP, NameOrRunePT, NameP, RegionRunePT, TopLevelExportAsP, TopLevelFunctionP, TopLevelImportP, TopLevelStructP, VoidPE}
import dev.vale.lexing.{BadStartOfStatementError, IParseError, Lexer, UnrecognizedDenizenError}
import dev.vale.options.GlobalOptions
import org.scalatest._



class TopLevelTests extends FunSuite with Matchers with Collector with TestParseUtils {
  def compile(code: String): FileP = {
    compileFile(code).getOrDie()
  }

  def compileForError(code: String): IParseError = {
    compileFile(code).expectErr().error
  }
*/
#[test]
fn function_then_struct() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "exported func main() int {} struct mork { }");
  assert!(matches!(
    program.denizens[0],
    IDenizenP::TopLevelFunction(_)
  ));
  assert!(matches!(program.denizens[1], IDenizenP::TopLevelStruct(_)));
}
/*
  test("Function then struct") {
    val program = compile(
      """
        |exported func main() int {}
        |
        |struct mork { }
        |""".stripMargin)
    program.denizens(0) match { case TopLevelFunctionP(_) => }
    program.denizens(1) match { case TopLevelStructP(_) => }
  }
*/
#[test]
fn ellipses_ignored() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  // Unicode … symbol is treated as an expression by the parser
  compile(&interner, &keywords, "exported func main() int {x = …;}");
  compile(&interner, &keywords, "exported func main() int {set x = …;}");
  // Three dots is treated as a comment
  compile(&interner, &keywords, "exported func main(...) int {}");
  compile(&interner, &keywords, "exported func main() ... {}");
  compile(&interner, &keywords, "exported func main() int {} ... ");
  compile(&interner, &keywords, "exported func main() int {...}");
  compile(&interner, &keywords, "exported func main() int {moo(...)}");
  compile(&interner, &keywords, "struct Moo {} ... ");
  compile(&interner, &keywords, "struct Moo {...}");
}
/*
  test("Ellipses ignored") {
    // Unicode … symbol is treated as an expression by the parser
    compile("""exported func main() int {x = …;}""".stripMargin)
    compile("""exported func main() int {set x = …;}""".stripMargin)

    // Three dots is treated as a comment
    compile("""exported func main(...) int {}""".stripMargin)
    compile("""exported func main() ... {}""".stripMargin)
    compile("""exported func main() int {} ... """.stripMargin)
    compile("""exported func main() int {...}""".stripMargin)
    compile("""exported func main() int {moo(...)}""".stripMargin)
    compile("""struct Moo {} ... """.stripMargin)
    compile("""struct Moo {...}""".stripMargin)
  }
*/
#[test]
fn comments_ignored() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  compile(
    &interner,
    &keywords,
    r#"
        exported func main(
                // moo
        ) int {}
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        exported func main()
                // moo
        {}
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        exported func main() int {}
                // moo
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        exported func main() int {
                // moo
        }
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        exported func main() int {
          moo(
                // moo
          )
        }
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        struct Moo {}
                // moo
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        struct Moo {
                // moo
        }
        "#,
  );
  compile(
    &interner,
    &keywords,
    r#"
        struct Moo {
        }
        // moo
        "#,
  );
}
/*
  test("Comments ignored") {
    compile(
      """
        |exported func main(
        |        // moo
        |) int {}
        |""".stripMargin)
    compile(
      """
        |exported func main()
        |        // moo
        |{}
        |""".stripMargin)
    compile(
      """
        |exported func main() int {}
        |        // moo
        |""".stripMargin)
    compile(
      """
        |exported func main() int {
        |        // moo
        |}
        |""".stripMargin)
    compile(
      """
        |exported func main() int {
        |  moo(
        |        // moo
        |  )
        |}
        |""".stripMargin)
    compile(
      """
        |struct Moo {}
        |        // moo
        |""".stripMargin)
    compile(
      """
        |struct Moo {
        |        // moo
        |}
        |""".stripMargin)
    compile(
      """
        |struct Moo {
        |}
        |// moo""".stripMargin)
  }
*/
#[test]
fn function_containing_if() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    r#"
        func main() int {
          if true { 3 } else { 4 }
        }
        "#,
  );
  let main = find_func_named(&program, "main");
  assert!(main.body.is_some());
}

/*
//  test("Function containing if") {
//    val program = compile(
//      """
//        |func main() int {
//        |  if true { 3 } else { 4 }
//        |}
//        |""".stripMargin)
//    val main = program.lookupFunction("main")
//    main.body.get
//  }
*/
#[test]
fn reports_unrecognized_at_top_level() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let err = compile_for_error(
    &interner,
    &keywords,
    r#"
      func main(){}
      blort
      "#,
  );
  assert!(matches!(err, ParseError::UnrecognizedDenizenError(_)));
}
/*
  test("Reports unrecognized at top level") {
    val code =
      """func main(){}
        |blort
        |""".stripMargin
    val err = compileForError(code)
    err match {
      case UnrecognizedDenizenError(_) =>
    }
  }
*/
// lol
#[test]
fn funky_function() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    r#"
      funky main() { }
      "#,
  );
  find_func_named(&program, "main");
}
/*
  // lol
  test("Funky function") {
    compile("funky main() { }")
    // MIGALLOW: Rust is looking for the main func
  }
*/
#[test]
fn empty() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    r#"
      func foo() { ... }
      "#,
  );
  let main = &program.denizens[0];
  assert!(matches!(
    main,
    IDenizenP::TopLevelFunction(FunctionP {
      body:
        Some(box BlockPE {
          inner: box IExpressionPE::Void(VoidPE { .. }),
          ..
        }),
      ..
    })
  ));
}
/*
  // To support the examples on the site for the syntax highlighter
  test("empty") {
    val program = compile("func foo() { ... }")
    program.denizens(0) match {
      case TopLevelFunctionP(
      FunctionP(_,
      _,
      Some(BlockPE(_,None,None,VoidPE(_))))) =>
    }
    // MIGALLOW: It's okay that Rust isnt checking for None and None in BlockPE
  }
*/
#[test]
fn exporting_int() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "export int as NumberThing;");
  assert!(
    matches!(program.denizens[0], IDenizenP::TopLevelExportAs(ExportAsP {
    struct_: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }),
    exported_name: NameP { str: ref e, .. },
    ..
  }) if s.str == "int" && e.str == "NumberThing")
  );
}
/*
  test("exporting int") {
    val program = compile("export int as NumberThing;")
    program.denizens(0) match {
      case TopLevelExportAsP(ExportAsP(_,NameOrRunePT(NameP(_, StrI("int"))),NameP(_, StrI("NumberThing")))) =>
    }
  }
*/
#[test]
fn exporting_imm_array_1() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "export []<mut>int as IntArray;");
  assert!(
    matches!(program.denizens[0], IDenizenP::TopLevelExportAs(ExportAsP {
    exported_name: NameP { str: ref IntArray_, .. },
    ..
  }) if IntArray_.str == "IntArray")
  );
}

/*
  test("exporting imm array 1") {
    val program = compile("export []<mut>int as IntArray;")
    program.denizens(0) match {
      case TopLevelExportAsP(ExportAsP(_,_,NameP(_, StrI("IntArray")))) =>
    }
  }
*/
#[test]
fn exporting_imm_array_2() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "export #[]int as IntArray;");
  assert!(
    matches!(program.denizens[0], IDenizenP::TopLevelExportAs(ExportAsP {
    exported_name: NameP { str: ref IntArray_, .. },
    ..
  }) if IntArray_.str == "IntArray")
  );
}

/*
  test("exporting imm array 2") {
    val program = compile("export #[]int as IntArray;")
    program.denizens(0) match {
      case TopLevelExportAsP(ExportAsP(_,_,NameP(_, StrI("IntArray")))) =>
    }
  }
*/
#[test]
fn import_wildcard() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "import somemodule.*;");
  assert!(
    matches!(program.denizens[0], IDenizenP::TopLevelImport(ImportP {
    module_name: NameP { str: ref somemodule_, .. },
    package_steps: ref p,
    importee_name: NameP { str: ref star_, .. },
    ..
  }) if somemodule_.str == "somemodule" && star_.str == "*")
  );
}

/*
  test("import wildcard") {
    val program = compile("import somemodule.*;")
    program.denizens(0) match {
      case TopLevelImportP(ImportP(_, NameP(_, StrI("somemodule")), Vector(), NameP(_, StrI("*")))) =>
    }
  }
*/
#[test]
fn import_just_module_and_thing() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "import somemodule.List;");
  assert!(
    matches!(program.denizens[0], IDenizenP::TopLevelImport(ImportP {
    module_name: NameP { str: ref somemodule_, .. },
    package_steps: ref p,
    importee_name: NameP { str: ref List_, .. },
    ..
  }) if somemodule_.str == "somemodule" && List_.str == "List" && p.is_empty())
  );
}

/*
  test("import just module and thing") {
    val program = compile("import somemodule.List;")
    program.denizens(0) match {
      case TopLevelImportP(ImportP(_, NameP(_, StrI("somemodule")), Vector(), NameP(_, StrI("List")))) =>
    }
  }
*/
#[test]
fn full_import() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "import somemodule.subpackage.List;");
  assert!(
    matches!(program.denizens[0], IDenizenP::TopLevelImport(ImportP {
    module_name: NameP { str: ref somemodule_, .. },
    package_steps: ref p,
    importee_name: NameP { str: ref List_, .. },
    ..
  }) if somemodule_.str == "somemodule" && List_.str == "List" && p.len() == 1 && p[0].str.str == "subpackage")
  );
}
/*
  test("full import") {
    val program = compile("import somemodule.subpackage.List;")
    program.denizens(0) match {
      case TopLevelImportP(ImportP(_, NameP(_, StrI("somemodule")), Vector(NameP(_, StrI("subpackage"))), NameP(_, StrI("List")))) =>
    }
  }
*/

#[test]
fn return_with_region_generics() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let program = compile(&interner, &keywords, "func strongestDesire() IDesire<r', i'> { }");
  let func = find_func_named(&program, "strongestDesire");
  let ret_type = func
    .header
    .ret
    .ret_type
    .as_ref()
    .expect("Expected return type");
  let ret_call = cast!(ret_type, ITemplexPT::Call);
  let ret_name = &cast!(ret_call.template.as_ref(), ITemplexPT::NameOrRune);
  assert!(ret_name.name.str.str == "IDesire");
  assert!(ret_call.args.len() == 2);
  assert_eq!(
    cast!(&ret_call.args[0], ITemplexPT::RegionRune)
      .name
      .as_ref()
      .unwrap()
      .str
      .str,
    "r"
  );
  assert_eq!(
    cast!(&ret_call.args[1], ITemplexPT::RegionRune)
      .name
      .as_ref()
      .unwrap()
      .str
      .str,
    "i"
  );
}
/*
  test("Return with region generics") {
    val program = compile(
      """
        |func strongestDesire() IDesire<r', i'> { }
        |""".stripMargin)
    val func = program.lookupFunction("strongestDesire")
    vassertSome(func.header.ret.retType) match {
      case CallPT(_,
        NameOrRunePT(NameP(_,StrI("IDesire"))),
        Vector(RegionRunePT(_,Some(NameP(_,StrI("r")))), RegionRunePT(_,Some(NameP(_,StrI("i")))))) =>
    }
  }
*/

#[test]
fn bad_start_of_statement() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let err = compile_for_error(
    &interner,
    &keywords,
    r#"
    func doCivicDance(virtual this Car) {
      )
    }
    "#,
  );
  assert!(matches!(err, ParseError::BadStartOfStatementError(_)));
  let err = compile_for_error(
    &interner,
    &keywords,
    r#"
    func doCivicDance(virtual this Car) {
      ]
    }
    "#,
  );
  assert!(matches!(err, ParseError::BadStartOfStatementError(_)));
}
/*
  test("Bad start of statement") {
    compileForError(
      """
        |func doCivicDance(virtual this Car) {
        |  )
        |}
        """.stripMargin) match {
      case BadStartOfStatementError(_) =>
    }
    compileForError(
      """
        |func doCivicDance(virtual this Car) {
        |  ]
        |}
        """.stripMargin) match {
      case BadStartOfStatementError(_) =>
    }
  }
}
*/
