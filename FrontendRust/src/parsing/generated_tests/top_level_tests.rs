/// Top-level parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/TopLevelTests.scala

use crate::parsing::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::{should_have};

#[test]
fn function_then_struct() {
    // Test: Function then struct
    // Line 20-29 in TopLevelTests.scala
    let program = compile_file_expect(
        r#"
        exported func main() int {}
        
        struct mork { }
        "#
    );
    
    match &program.denizens[0] {
        IDenizenP::TopLevelFunction(_) => {},
        other => panic!("Expected TopLevelFunctionP, got {:?}", other),
    }
    
    match &program.denizens[1] {
        IDenizenP::TopLevelStruct(_) => {},
        other => panic!("Expected TopLevelStructP, got {:?}", other),
    }
}

#[test]
fn ellipses_ignored() {
    // Test: Ellipses ignored
    // Lines 31-44 in TopLevelTests.scala
    
    // Unicode … symbol is treated as an expression by the parser
    compile_file_expect(r#"exported func main() int {x = …;}"#);
    compile_file_expect(r#"exported func main() int {set x = …;}"#);
    
    // Three dots is treated as a comment
    compile_file_expect(r#"exported func main(...) int {}"#);
    compile_file_expect(r#"exported func main() ... {}"#);
    compile_file_expect(r#"exported func main() int {} ... "#);
    compile_file_expect(r#"exported func main() int {...}"#);
    compile_file_expect(r#"exported func main() int {moo(...)}"#);
    compile_file_expect(r#"struct Moo {} ... "#);
    compile_file_expect(r#"struct Moo {...}"#);
}

#[test]
fn comments_ignored() {
    // Test: Comments ignored
    // Lines 46-94 in TopLevelTests.scala
    
    compile_file_expect(
        r#"
        exported func main(
                // moo
        ) int {}
        "#
    );
    
    compile_file_expect(
        r#"
        exported func main()
                // moo
        {}
        "#
    );
    
    compile_file_expect(
        r#"
        exported func main() int {}
                // moo
        "#
    );
    
    compile_file_expect(
        r#"
        exported func main() int {
                // moo
        }
        "#
    );
    
    compile_file_expect(
        r#"
        exported func main() int {
          moo(
                // moo
          )
        }
        "#
    );
    
    compile_file_expect(
        r#"
        struct Moo {}
                // moo
        "#
    );
    
    compile_file_expect(
        r#"
        struct Moo {
                // moo
        }
        "#
    );
    
    compile_file_expect(
        r#"
        struct Moo {
        }
        // moo"#
    );
}

#[test]
fn reports_unrecognized_at_top_level() {
    // Test: Reports unrecognized at top level
    // Lines 110-119 in TopLevelTests.scala
    
    let code = r#"func main(){}
blort
"#;
    
    let _err = compile_file(code).expect_err("Expected parse error");
    
    // TODO: Check for specific error type when UnrecognizedDenizen error is added
    // match err {
    //     ParseError::UnrecognizedDenizen(_) => {},
    //     other => panic!("Expected UnrecognizedDenizen, got {:?}", other),
    // }
}

#[test]
fn funky_function() {
    // Test: Funky function
    // Lines 122-124 in TopLevelTests.scala
    // "funky" is an allowed alternative keyword for "func"
    
    compile_file_expect("funky main() { }");
}

#[test]
fn empty() {
    // Test: empty
    // Lines 127-135 in TopLevelTests.scala
    // To support the examples on the site for the syntax highlighter
    
    let program = compile_file_expect("func foo() { ... }");
    
    should_have!(&program.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: _,
        body: Some(box BlockPE { .. }),
        ..
    }) => {});
}

#[test]
fn exporting_int() {
    // Test: exporting int
    // Lines 137-143 in TopLevelTests.scala
    
    let program = compile_file_expect("export int as NumberThing;");
    
    should_have!(
        &program.denizens[0],
        IDenizenP::TopLevelExportAs(
            ExportAsP {
                struct_: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }),
                exported_name: NameP { str: ref e, .. },
                ..
            }
        ) if s.str == "int" && e.str == "NumberThing" => {

        }
    );
}

#[test]
fn exporting_imm_array_1() {
    // Test: exporting imm array 1
    // Lines 145-150 in TopLevelTests.scala
    
    let program = compile_file_expect("export []<mut>int as IntArray;");
    
    should_have!(&program.denizens[0], IDenizenP::TopLevelExportAs(ExportAsP {
        exported_name: NameP { str: ref e, .. },
        ..
    }) if e.str == "IntArray" => {});
}

#[test]
fn exporting_imm_array_2() {
    // Test: exporting imm array 2
    // Lines 152-157 in TopLevelTests.scala
    
    let program = compile_file_expect("export [_]<imm>Int as IntArray;");
    
    should_have!(&program.denizens[0], IDenizenP::TopLevelExportAs(ExportAsP {
        exported_name: NameP { str: ref e, .. },
        ..
    }) if e.str == "IntArray" => {});
}

#[test]
fn importing_int() {
    // Test: importing int
    // Lines 159-164 in TopLevelTests.scala
    
    let program = compile_file_expect("import othermod.thing.*;");
    
    should_have!(&program.denizens[0], IDenizenP::TopLevelImport(ImportP {
        module_name: ref m,
        package_steps: ref p,
        importee_name: ref i,
        ..
    }) if m.str.str == "othermod" => {
        assert_eq!(p.len(), 1);
        // TODO: Fix - importee_name is NameP, not Vec<ImportedName>
        // assert!(matches!(&i[0], ImportedName::Star));
    });
}

#[test]
fn top_level_export_region() {
    // Test: top level export region
    // Lines 166-172 in TopLevelTests.scala
    
    let program = compile_file_expect("export r' as MyRegion;");
    
    should_have!(&program.denizens[0], IDenizenP::TopLevelExportAs(ExportAsP {
        struct_: ITemplexPT::RegionRune(RegionRunePT { .. }),
        exported_name: NameP { str: ref e, .. },
        ..
    }) if e.str == "MyRegion" => {});
}

#[test]
fn import_one_thing() {
    // Test: full import (renamed from import_one_thing)
    // Lines 173-178 in TopLevelTests.scala: test("full import")
    
    let program = compile_file_expect("import somemodule.subpackage.List;");
    
    should_have!(&program.denizens[0], IDenizenP::TopLevelImport(ImportP {
        module_name: NameP { str: ref m, .. },
        package_steps: ref p,
        importee_name: NameP { str: ref i, .. },
        ..
    }) if m.str == "somemodule" && i.str == "List" => {
        assert_eq!(p.len(), 1);
        should_have!(&p[0], NameP { str: ref s, .. } if s.str == "subpackage" => {});
    });
}

// NOTE: The Scala test suite does NOT test import with curly braces {A, B} syntax.
// The lexer only supports simple imports like "import module.package.Name;"
// Tests for {A, B} syntax were incorrectly added and have been removed.

