# Regression fixtures preserved from the retired `higher_typing` pass

The `higher_typing/` pass was retired during the onion typing arc. Its
`tests/` directory held 14 positive-behavior tests and 3 error-humanization
tests. A per-test coverage-gap scan against `typing/test/` found that **9 of
the 17 tests are already covered** at typing; **3 test axes are genuine gaps**
worth re-authoring; **3 more are partial gaps** (behavior compiles but the
rune-type map assertion is absent); and **2 error-humanizer tests are
un-portable** (they pin exact humanized strings with higher_typing-specific
internal rune numbers).

This doc preserves the exact Rust source of the 6 gap tests so they can be
transcribed as literally as possible into `typing/test/` when the rune-type
solver comes back at typing (via `typing/rune_typing/` or similar). Under
onion, the assertions need light rewiring:

- `CoordTemplataType` → `KindTemplataType`
- `PackTemplataType<CoordTemplataType>` → `PackTemplataType<KindTemplataType>` (and the SR side becomes `KindListSR`)
- `HigherTypingCompilation` → whatever the typing entry helper ends up called
- `program.lookup_function_by_str` / `lookup_struct_by_str` / `.rune_to_type` / `.header_rune_to_type` → moves onto the new `coutputs.function_name_to_rune_types` / `coutputs.type_name_to_rune_types` maps described in `vcoord-handoff.md`

The surface (Vale source strings) is what to preserve exactly. The type-side
assertions are the shape to reproduce, not the byte-identical code.

---

## True gaps (highest priority — no adjacent coverage at all)

### `test_evaluate_pack` — `Refs(int, bool)` explicit RefList literal

Exercises the pack-templata literal path in the rune-type solver. Under
onion, becomes `KindListSR` inference from an explicit pack literal. Nothing
in `typing/test/` exercises pack literal inference at all — confirmed via
grep-for-`RefList` and grep-for-`Refs(` returning zero hits.

```rust
#[test]
fn test_evaluate_pack() {
    let scout_bump = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let scout_keywords = Keywords::new_for_scout(&scout_arena);
    let parse_arena = ParseArena::new(&parser_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, r"
func moo<T RefList>()
where T = Refs(int, bool)
{
}
"),
    ]);
    let mut compilation = setup_test(&scout_arena, &scout_keywords, &parser_keywords, &parse_arena, &code_source);
    let astrouts = compilation.expect_astrouts();
    let test_module_s = scout_arena.intern_str("test");
    let test_tld = *scout_arena.intern_package_coordinate(test_module_s, &[]);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&scout_arena.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: scout_keywords.t })
        )).unwrap(),
        ITemplataType::PackTemplataType(PackTemplataType {
            element_type: &*scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {}))
        })
    );
}
```

### `test_infer_pack_from_empty_result` — `Refs()` empty pack + `Prot[P, str]` composition

Empty-pack + Prot-rule back-solve. Two independent axes both absent from
`typing/test/`.

```rust
#[test]
fn test_infer_pack_from_empty_result() {
    let scout_bump = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let scout_keywords = Keywords::new_for_scout(&scout_arena);
    let parse_arena = ParseArena::new(&parser_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, r"
func moo<P RefList>()
where P = Refs(), Prot[P, str]
{
}
"),
    ]);
    let mut compilation = setup_test(&scout_arena, &scout_keywords, &parser_keywords, &parse_arena, &code_source);
    let astrouts = compilation.expect_astrouts();
    let test_module_s = scout_arena.intern_str("test");
    let test_tld = *scout_arena.intern_package_coordinate(test_module_s, &[]);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&scout_arena.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str("P") })
        )).unwrap(),
        ITemplataType::PackTemplataType(PackTemplataType {
            element_type: &*scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {}))
        })
    );
}
```

### `report_type_not_found` — bare `Bork` at plain param position hits `CouldntFindType`

Typing's existing coverage of `CouldntFindType` is only through
array-callable-slot variants (`reports_when_ssa_from_callable_has_unknown_element_type`
and siblings). The plain-param path isn't covered. The `error_text.contains(...)`
assertion is robust — doesn't depend on internal rune numbers — so it can
transcribe cleanly.

```rust
#[test]
fn report_type_not_found() {
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let compilation_bump = bumpalo::Bump::new();
    report_type_not_found_inner(&parse_bump, &scout_bump, &compilation_bump);
}

fn report_type_not_found_inner<'s>(
    parse_bump: &'s bumpalo::Bump,
    scout_bump: &'s bumpalo::Bump,
    compilation_bump: &'s bumpalo::Bump,
) {
    let parse_arena = ParseArena::new(parse_bump);
    let scout_arena = ScoutArena::new(scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main(a Bork) {\n}\n";
    let mut compilation = test(
        compilation_bump, &scout_arena, &keywords, &parser_keywords, &parse_arena, code);
    let err = compile_program_for_error(&mut compilation);
    match &err {
        ICompileErrorA::CouldntSolveRules(
            CouldntSolveRulesA {
                error: RuneTypeSolveError {
                    failed_solve: FailedSolve {
                        error: ISolverError::RuleError(RuleError {
                            err: IRuneTypeRuleError::CouldntFindType(
                                RuneTypingCouldntFindType {
                                    name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("Bork") }),
                                    ..
                                }),
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }
        ) => {
            let code_map = compilation.get_code_map().unwrap();
            let humanize_pos_fn = |x: CodeLocationS<'s>| humanize_pos_code_map(&code_map, &x);
            let lines_between_fn = |x: CodeLocationS<'s>, y: CodeLocationS<'s>| lines_between(&code_map, &x, &y);
            let line_range_containing_fn = |x: CodeLocationS<'s>| line_range_containing(&code_map, &x);
            let line_containing_fn = |x: CodeLocationS<'s>| line_containing(&code_map, &x);
            let error_text =
                humanize(
                    &humanize_pos_fn, &lines_between_fn, &line_range_containing_fn, &line_containing_fn, &err);
            assert!(error_text.contains("Couldn't find anything with the name 'Bork'"));
        }
        _ => panic!("expected CouldntSolveRules(...RuneTypingCouldntFindType(CodeNameS(\"Bork\")))"),
    }
}
```

**Onion-side error-variant path:** `ICompileErrorA::CouldntSolveRules` becomes
whatever the typing-side rune-solve error type is (currently
`ICompileErrorT::HigherTypingInferError` in gated code; may be renamed).
`IRuneTypeRuleError::CouldntFindType` and `RuneTypingCouldntFindType` survive
the move to `typing/rune_typing/`.

---

## Partial gaps (behavior exists, direct rune-type-map assertion absent)

`typing/test/` exercises each of these shapes through end-to-end compilation
success, but nothing asserts the `rune_to_type` / `header_rune_to_type` map
contents directly. When the rune-type solver moves into `typing/`, these are
the minimal set of direct-assertion regressions.

### `infer_coord_type_from_parameters` — infer `T`'s rune type from param position

`compiler_solver_tests.rs::test_having_drop_function_concept_function` (line
144) uses the same shape and successfully compiles, but no test asserts
`rune_to_type[T] == CoordTemplataType`.

```rust
#[test]
fn infer_coord_type_from_parameters() {
    let scout_bump = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let scout_keywords = Keywords::new_for_scout(&scout_arena);
    let parse_arena = ParseArena::new(&parser_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, "exported func moo<T>(x T) {\n}\n"),
    ]);
    let mut compilation = setup_test(&scout_arena, &scout_keywords, &parser_keywords, &parse_arena, &code_source);
    let astrouts = compilation.expect_astrouts();
    let test_module_s = scout_arena.intern_str("test");
    let test_tld = *scout_arena.intern_package_coordinate(test_module_s, &[]);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&scout_arena.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: scout_keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
}
```

### `infer_generic_type_through_param_type_template_call` — infer `T` through `List<T>` in param position

`compiler_solver_tests.rs::descendant_satisfying_call` (line 855) uses `func
moo<T>(a IShip<T>)` — same axis — but doesn't assert the rune-type table.

```rust
#[test]
fn infer_generic_type_through_param_type_template_call() {
    let scout_bump = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let scout_keywords = Keywords::new_for_scout(&scout_arena);
    let parse_arena = ParseArena::new(&parser_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, r"
struct List<T> {
  moo T;
}
exported func moo<T>(x List<T>) {
}
"),
    ]);
    let mut compilation = setup_test(&scout_arena, &scout_keywords, &parser_keywords, &parse_arena, &code_source);
    let astrouts = compilation.expect_astrouts();
    let test_module_s = scout_arena.intern_str("test");
    let test_tld = *scout_arena.intern_package_coordinate(test_module_s, &[]);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&scout_arena.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: scout_keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
}
```

### `template_call_recursively_evaluate` — infer `Bork.T` through nested `Moo<T>` field

Structural recursion through a nested generic struct field.
`compiler_tests.rs::tests_a_linked_list` (line 1882) exercises the shape but
no `header_rune_to_type` assertion.

```rust
#[test]
fn template_call_recursively_evaluate() {
    let scout_bump = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let scout_keywords = Keywords::new_for_scout(&scout_arena);
    let parse_arena = ParseArena::new(&parser_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code_source = CodeSource::new(vec![
        new_test_code_map(&parse_arena, r"
struct Moo<T> {
  bork T;
}
struct Bork<T> {
  x Moo<T>;
}
"),
    ]);
    let mut compilation = setup_test(&scout_arena, &scout_keywords, &parser_keywords, &parse_arena, &code_source);
    let astrouts = compilation.expect_astrouts();
    let test_module_s = scout_arena.intern_str("test");
    let test_tld = *scout_arena.intern_package_coordinate(test_module_s, &[]);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_struct_by_str("Bork");
    assert_eq!(
        *main.header_rune_to_type.get(&scout_arena.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: scout_keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
}
```

---

## Shared helpers referenced above

Both `setup_test` (from `higher_typing_pass_tests.rs`) and the compilation
helper `test` (from `test_compilation.rs`) are higher_typing-specific and
won't survive. The typing-side equivalent will be whatever helper builds a
typing pass invocation from raw Vale source. Same-name conventions in
`typing/test/compiler_test_compilation.rs`. Below is the retired
`setup_test` for reference on the option flags that were being passed:

```rust
fn setup_test<'s, 'ctx, 'p>(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    code_source: &'ctx CodeSource<'p>,
) -> HigherTypingCompilation<'s, 'ctx, 'p> {
    let options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: false,
        debug_output: false,
    };
    let test_module = parse_arena.intern_str("test");
    let test_tld_ref = parse_arena.intern_package_coordinate(test_module, &[]);
    HigherTypingCompilation::new(
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        vec![test_tld_ref],
        code_source,
        options,
    )
}

fn compile_program_for_error<'s, 'ctx, 'p>(
    compilation: &mut HigherTypingCompilation<'s, 'ctx, 'p>,
) -> ICompileErrorA<'s>
{
    match compilation.get_astrouts() {
        Ok(result) => panic!("Expected error, but actually parsed invalid program:\n{:?}", result),
        Err(err) => err,
    }
}
```

Retired imports (for reference on what the fixtures needed):

```rust
use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::higher_typing::HigherTypingCompilation;
use crate::higher_typing::astronomer_error_reporter::ICompileErrorA;
use crate::higher_typing::astronomer_error_reporter::CouldntSolveRulesA;
use crate::higher_typing::higher_typing_error_humanizer::humanize;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::postparsing::itemplatatype::{CoordTemplataType, ITemplataType, PackTemplataType};
use crate::postparsing::names::{CodeRuneS, IRuneValS, CodeNameS, IImpreciseNameS};
use crate::postparsing::rune_type_solver::{IRuneTypeRuleError, RuneTypeSolveError, RuneTypingCouldntFindType};
use crate::pass_manager::CodeSource;
use crate::solver::solver::{FailedSolve, ISolverError, RuleError};
use crate::tests::tests::new_test_code_map;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::fx::HashMap;
use crate::interner::StrI;
use crate::utils::range::CodeLocationS;
use crate::utils::source_code_utils::{humanize_pos_code_map, line_containing, line_range_containing, lines_between};
```

---

## Not preserved

- **`report_type_not_found_with_literal_generic_arg`** and
  **`report_type_not_found_with_augment`** — both pinned exact humanized text
  containing higher_typing-specific internal rune numbers (`_211311`, `_2111`,
  `_211211`). The humanizer is retired; the strings won't survive re-authoring
  even in spirit. If the underlying "undefined name behind an augment" and
  "undefined name at generic-call site" cases matter, they'd need to be
  rewritten from scratch against typing's error humanizer, and there's no
  useful literal text to transcribe from the old assertions.

- **9 tests already covered at typing** — `type_simple_main_function`,
  `type_simple_generic_function`, `type_simple_struct`,
  `type_simple_generic_struct`, `type_simple_interface`,
  `type_simple_generic_interface`, `type_simple_generic_interface_method`,
  `type_simple_impl`, `test_infer_pack_from_result`. Coverage traced in the
  Explore-agent scan of this session (2026-07-05).
