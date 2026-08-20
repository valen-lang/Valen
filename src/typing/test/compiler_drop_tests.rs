//! Auto-generated struct drop for generic structs.

use crate::builtins::builtins::{builtin_source_bundle, empty_v_builtins_stub};
use crate::code_source::CodeSource;
use crate::code_source::Source;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::new_test_code_map;
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::typing_interner::TypingInterner;
use bumpalo::Bump;

// A generic struct's auto-generated drop must be able to drop a bare rune member (`val T`).
#[test]
fn generic_struct_type_param_member_auto_drops() {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "struct Box<T> { val T; }\n",
    "exported func main() { b = Box<int>(5); }\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_bundle(&parse_arena, &parser_keywords, &["drop", "implicit_clone"]),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
  );
  compile.expect_compiler_outputs();
}

// A generic struct (`Holder`) must be able to drop a member that's also a generic struct (`Box`).
// The Holder's auto-generated drop function must declare a bound for itself, and use it to call
// Box's drop.
#[test]
fn generic_struct_nested_generic_member_auto_drops() {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "import v.builtins.drop.*;\n",
    "struct Box<T> { val T; }\n",
    "struct Holder<T> { held Box<T>; }\n",
    "exported func main() { h = Holder<int>(Box<int>(5)); }\n",
  );
  let code_source = CodeSource::new(vec![
    builtin_source_bundle(&parse_arena, &parser_keywords, &["drop", "implicit_clone"]),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
  );
  compile.expect_compiler_outputs();
}

// An empty generic struct (like None<T>) should generate a drop function that *doesn't* have a
// bound. After all, we don't need to know T is droppable to drop a None<T>.
// (We could change this at some point, to make the struct drop generator simpler...)
#[test]
fn empty_generic_struct_stays_droppable() {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = concat!(
    "struct None<T> { }\n",
    "exported func main() { n = None<int>(); }\n",
  );
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &code_source,
  );
  compile.expect_compiler_outputs();
}
