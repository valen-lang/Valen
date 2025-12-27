/// Von (JSON serialization) tests
/// Mirrors Frontend/Von/test/dev/vale/von/VonTest.scala

use crate::von::ast::{IVonData, VonMember};
use crate::von::printer::{VonPrinter, VonSyntax, JsonSyntax};

#[test]
fn test_1() {
    // Test 1: Simple VonObject printing
    let data = IVonData::VonObject {
        name: "MyObj".to_string(),
        type_name: None,
        members: vec![VonMember {
            name: "mem".to_string(),
            value: Box::new(IVonData::VonInt(42)),
        }],
    };
    
    let printer = VonPrinter::new(VonSyntax);
    let result = printer.print(&data);
    
    assert_eq!(result, "MyObj(mem = 42)");
}

#[test]
fn json_doesnt_escape_apostrophe() {
    // We dont want it to escape apostrophes because Lift can't handle reading
    // an escaped one like "yes\'nt".
    let data = IVonData::VonStr("yes'nt".to_string());
    
    let printer = VonPrinter::new(JsonSyntax);
    let result = printer.print(&data);
    
    assert_eq!(result, "\"yes'nt\"");
}

#[test]
fn test_2() {
    // Test 2: Long object name wrapping
    let data = IVonData::VonObject {
        name: "MySuperSuperLongObject".to_string(),
        type_name: None,
        members: vec![VonMember {
            name: "member".to_string(),
            value: Box::new(IVonData::VonInt(42)),
        }],
    };
    
    let printer = VonPrinter::new(VonSyntax);
    let result = printer.print(&data);
    
    let expected = "MySuperSuperLongObject(\n  member = 42)";
    assert_eq!(result, expected);
}

#[test]
fn test_3() {
    // Test 3: Nested objects with different line widths
    let data = IVonData::VonObject {
        name: "MyObj".to_string(),
        type_name: None,
        members: vec![VonMember {
            name: "member".to_string(),
            value: Box::new(IVonData::VonObject {
                name: "MyObj".to_string(),
                type_name: None,
                members: vec![VonMember {
                    name: "member".to_string(),
                    value: Box::new(IVonData::VonObject {
                        name: "MyObj".to_string(),
                        type_name: None,
                        members: vec![VonMember {
                            name: "member".to_string(),
                            value: Box::new(IVonData::VonInt(42)),
                        }],
                    }),
                }],
            }),
        }],
    };
    
    let printer = VonPrinter::new(VonSyntax);
    let result = printer.print(&data);
    
    // With default line width (30)
    let expected = "MyObj(\n  member = MyObj(\n    member = MyObj(member = 42)))";
    assert_eq!(result, expected);
    
    // Note: The original Scala test also tested with line_width=25, but we removed
    // that field from VonPrinter. If line width control is needed, we can add it back.
}

