use super::compiler_test_compilation::compiler_test_compilation;
use crate::builtins::builtins::empty_v_builtins_stub;
use crate::code_source::{CodeSource, Source};
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::new_test_code_map;
use crate::typing::typing_interner::TypingInterner;
use bumpalo::Bump;

pub struct CompilerGenericsTests;
impl CompilerGenericsTests {}

fn read_code_from_resource(resource_filename: &str) -> String {
  panic!("Unimplemented: read_code_from_resource");
}

// VINTERFACE: parked while interfaces migrate to the enum representation; re-enable after the enum work lands.
#[ignore = "VINTERFACE: re-enable after the enum work"]
#[test]
fn upcasting_with_generic_bounds() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: the `return (^m).harvest();` line below was `  return (m).harvest();` pre-sugar.
  let code = r#"
import v.builtins.box.*;
import v.builtins.panic.*;
import v.builtins.drop.*;

#!DeriveInterfaceDrop
interface XOpt<T> where func drop(T)void {
  func harvest(virtual opt XOpt<T>) T;
}

#!DeriveStructDrop
struct XNone<T> where func drop(T)void  { }

impl<T> XOpt<T> for XNone<T>;

func harvest<T>(opt XNone<T>) T {
  __vbi_panic();
}

exported func main() int {
  m Box<dyn XOpt<int>> = Box<XNone<int>>(XNone<int>());
  return (^m).harvest();
}

"#;
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "box"),
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
  let _coutputs = compile.expect_compiler_outputs();
}
