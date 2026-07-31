<!-- session: 7667e11b-10e8-4f9c-91a1-b3bf39d8f94a; exported-bytes: 91117; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale2/7667e11b-10e8-4f9c-91a1-b3bf39d8f94a.jsonl -->
# Claude Conversation Log

Session ID: 7667e11b-10e8-4f9c-91a1-b3bf39d8f94a
Date: 2026-07-02 00:09:13

---

## 👤 User

## Your Task

You are a code review checker for a specific rule, described in the "Rule" section. Review the code change in the "Code Change" section and then flag any of the violations it describes.

## Rule


# Immediate Interning Discipline (IIDX)

This codebase uses arena-based interning. Interned handles (`StrI<'a>`, `IRuneS<'a>`, `INameS<'a>`, `IVarNameS<'a>`, `IImpreciseNameS<'a>`, `&'a PackageCoordinate<'a>`, `&'a FileCoordinate<'a>`) are lightweight canonical references — storing, returning, and passing them is correct and expected.

Transient Val types (`IRuneValS`, `INameValS`, `IFunctionDeclarationNameValS`, `IVarNameValS`, `IImpreciseNameValS`) are lookup keys that exist only to be passed to `intern_*()` methods. They must not be stored in struct fields or returned from functions.

## Examples

**DENY** — storing a Val type:
```rust
pub struct BadStruct<'a> {
    pub rune_key: IRuneValS<'a>,  // Val types must not be stored
}
```

**DENY** — manual StrI construction outside Interner:
```rust
fn make_name(s: &'a str) -> StrI<'a> {
    StrI(s)  // must use interner.intern(s)
}
```

**ALLOW** — storing interned handles:
```rust
pub struct MyStruct<'a> {
    pub name: StrI<'a>,
    pub rune: IRuneS<'a>,
    pub coord: &'a FileCoordinate<'a>,
}
```

**ALLOW** — returning interned handles:
```rust
fn get_name(&self) -> INameS<'s> { self.name }
```

**ALLOW** — collections of interned handles:
```rust
let runes: Vec<IRuneS<'a>> = vec![rune_a, rune_b];
```

**ALLOW** — pattern destructuring (not construction):
```rust
match name {
    StrI("MyHashSet") => { /* ... */ }   // pattern, inspects existing value
    _ => panic!(),
}
```

## Clarifications

* `IRuneS<'a>` is NOT `IRuneValS<'a>`. Only flag types whose name literally contains `Val` before the `S` suffix.
* Converting between error enum variants that contain interned handles is not a violation — the handles are just being moved.
* `Vec<IRuneS<'a>>` or `&[IRuneS<'a>]` is a collection of interned handles, not Val types.
* `StrI("foo")` / `IRuneS::Variant(...)` in **pattern position** (inside `match`, `if let`, `while let`, or function-parameter destructuring) is destructuring an already-interned value, not constructing one. Only flag `StrI(...)` etc. in **expression position** (right-hand side of `=`, function arguments, return values).

## Important: How to Read the Diff

Below is the code around a change to a single definition (function, struct, impl block, etc.), not the entire file. It uses these prefixes:
- Lines starting with `+` are NEWLY ADDED code.
- Lines starting with `-` are REMOVED code — you do not need to evaluate removed lines.
- Lines with NO prefix are UNCHANGED existing code that is part of this definition.

Identify violations in both the added (`+`) lines AND the unchanged (no prefix) lines in this definition. Ignore violations in removed (`-`) lines. Do not evaluate violations anywhere else.

Only flag violations of the specific rule described above. Do not flag violations unrelated to this rule or violations of different rules, even if you notice other issues in the code.

## Code Change

FILE: /Volumes/V/Vale2/FrontendRust/src/typing/test/compiler_tests.rs

CONTEXTIFIED DIFF (shows enclosing functions/structs around each change):
```
// Lines 1939-1989 of modified file:
@@ -0,0 +1,51 @@
+// Phase 2 slice 1 tracer: bare-use of an Own struct local at a Borrow target
+// resolves without an explicit `&`. Today (pre-Phase-2), the Own arm of
+// coerce_to_reference_expression forces wrap_in_implicit_clone; after Phase 2,
+// bare-use produces a Borrow-flavored coord and the call resolves against
+// `func bork(&Struct)` directly. Verifies bork's param is CoordT { Borrow, Struct SomeStruct }.
+#[test]
+fn bare_own_struct_local_borrows_at_borrow_target() {
+    let parse_bump = Bump::new();
+    let scout_bump = Bump::new();
+    let typing_bump = Bump::new();
+    let parse_arena = ParseArena::new(&parse_bump);
+    let scout_arena = ScoutArena::new(&scout_bump);
+    let keywords = Keywords::new_for_scout(&scout_arena);
+    let parser_keywords = Keywords::new_for_parse(&parse_arena);
+    let code = r"
+struct SomeStruct { i int; }
+func bork(x &SomeStruct) int { return 7; }
+exported func main() int {
+  x = SomeStruct(3);
+  return bork(x);
+}
+";
+    let resolver = code_hierarchy::test_from_vec(&parse_arena, vec![code.to_string()])
+        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
+    let typing_interner = TypingInterner::new(&typing_bump);
+    let mut compile = compiler_test_compilation(
+        &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &resolver,
+    );
+    let coutputs = compile.expect_compiler_outputs();
+
+    let bork = coutputs.lookup_function_by_str("bork");
+    match bork.header.params[0].tyype {
+        CoordT {
+            ownership: OwnershipT::Borrow,
+            kind: KindT::Struct(StructTT {
+                id: IdT {
+                    local_name: INameT::Struct(StructNameT {
+                        template: IStructTemplateNameT::StructTemplate(
+                            StructTemplateNameT { human_name: StrI("SomeStruct"), .. }
+                        ),
+                        ..
+                    }),
+                    ..
+                },
+                ..
+            }),
+            ..
+        } => {}
+        other => panic!("Expected bork's first param to be Borrow SomeStruct, got {:?}", other),
+    }
+}

```

## Referenced Definitions (context only — do NOT evaluate these for violations)

The following definitions are referenced by the changed code. They are provided
for context only so you can understand the types, signatures, and contracts
involved. Do NOT flag violations in these definitions.

// /Volumes/V/Vale2/FrontendRust/src/parse_arena.rs:38
/// Arena + interning maps for the parsing pass.
/// Holds the `'p` Bump arena and deduplication maps for strings,
/// package coordinates, and file coordinates.
pub struct ParseArena<'p> {
  bump: &'p Bump,
  inner: RefCell<ParseArenaInner<'p>>,
}

// /Volumes/V/Vale2/FrontendRust/src/parse_arena.rs:50
  pub fn new(bump: &'p Bump) -> Self {

// /Volumes/V/Vale2/FrontendRust/src/scout_arena.rs:60
pub struct ScoutArena<'s> {
  bump: &'s Bump,
  inner: RefCell<ScoutArenaInner<'s>>,
}

// /Volumes/V/Vale2/FrontendRust/src/scout_arena.rs:76
  pub fn new(bump: &'s Bump) -> Self {

// /Volumes/V/Vale2/FrontendRust/src/keywords.rs:7
/// All Vale keywords and commonly used identifiers
// TODO: let's bake this into the ParseArena, ScoutArena, etc. and only include ones that are
// actually frequently used by that pass.
pub struct Keywords<'a> {
  pub func: StrI<'a>,
  pub impoort: StrI<'a>,
  pub export: StrI<'a>,
  pub truue: StrI<'a>,
  pub faalse: StrI<'a>,
  pub own: StrI<'a>,
  pub borrow: StrI<'a>,
  pub weak: StrI<'a>,
  pub share: StrI<'a>,
  pub r#where: StrI<'a>,
  pub additive: StrI<'a>,
  pub inl: StrI<'a>,
  pub heap: StrI<'a>,
  pub imm: StrI<'a>,
  pub r#mut: StrI<'a>,
  pub vary: StrI<'a>,
  pub fiinal: StrI<'a>,
  pub exists: StrI<'a>,
  pub resolve: StrI<'a>,
  pub self_: StrI<'a>,
  pub iff: StrI<'a>,
  pub elsse: StrI<'a>,
  pub foreeach: StrI<'a>,
  pub r#in: StrI<'a>,
  pub parallel: StrI<'a>,
  pub r#break: StrI<'a>,
  pub retuurn: StrI<'a>,
  pub whiile: StrI<'a>,
  pub destruct: StrI<'a>,
  pub set: StrI<'a>,
  pub unlet: StrI<'a>,
  pub block: StrI<'a>,
  pub pure: StrI<'a>,
  pub pre: StrI<'a>,
  pub r#unsafe: StrI<'a>,
  pub and: StrI<'a>,
  pub or: StrI<'a>,
  pub r#as: StrI<'a>,
  pub ro: StrI<'a>,
  pub rw: StrI<'a>,
  pub r#virtual: StrI<'a>,
  pub r#impl: StrI<'a>,
  pub int_capitalized: StrI<'a>,
  pub ref_: StrI<'a>,
  pub kind: StrI<'a>,
  pub region: StrI<'a>,
  pub prot: StrI<'a>,
  pub ref_list: StrI<'a>,
  pub ownership: StrI<'a>,
  pub location: StrI<'a>,
  pub refs: StrI<'a>,
  pub underscore: StrI<'a>,
  pub dot_dot: StrI<'a>,
  pub int: StrI<'a>,
  pub bool: StrI<'a>,
  pub float: StrI<'a>,
  pub __never: StrI<'a>,
  pub str: StrI<'a>,
  pub void: StrI<'a>,
  pub i64: StrI<'a>,
  pub i32: StrI<'a>,
  pub i16: StrI<'a>,
  pub i8: StrI<'a>,
  pub u64: StrI<'a>,
  pub u32: StrI<'a>,
  pub u16: StrI<'a>,
  pub u8: StrI<'a>,
  pub plus: StrI<'a>,
  pub asterisk: StrI<'a>,
  pub slash: StrI<'a>,
  pub minus: StrI<'a>,
  pub spaceship: StrI<'a>,
  pub less_equals: StrI<'a>,
  pub less: StrI<'a>,
  pub greater_equals: StrI<'a>,
  pub greater: StrI<'a>,
  pub triple_equals: StrI<'a>,
  pub double_equals: StrI<'a>,
  pub not_equals: StrI<'a>,
  pub drop: StrI<'a>,
  pub free: StrI<'a>,
  pub linear: StrI<'a>,
  pub not: StrI<'a>,
  pub range: StrI<'a>,
  pub begin: StrI<'a>,
  pub next: StrI<'a>,
  pub is_empty: StrI<'a>,
  pub get: StrI<'a>,
  pub underscores_call: StrI<'a>,
  pub tuple_human_name: Vec<StrI<'a>>,
  pub derive_struct_drop: StrI<'a>,
  pub derive_anonymous_substruct: StrI<'a>,
  pub derive_interface_drop: StrI<'a>,
  pub free_generator: StrI<'a>,
  pub drop_generator: StrI<'a>,
  pub interface_free_generator: StrI<'a>,
  pub vale_static_sized_array_drop_into: StrI<'a>,
  pub vale_runtime_sized_array_push: StrI<'a>,
  pub vale_runtime_sized_array_pop: StrI<'a>,
  pub vale_runtime_sized_array_new: StrI<'a>,
  pub vale_runtime_sized_array_capacity: StrI<'a>,
  pub vale_runtime_sized_array_len: StrI<'a>,
  pub vale_runtime_sized_array_free: StrI<'a>,
  pub vale_runtime_sized_array_drop_into: StrI<'a>,
  pub abstract_body: StrI<'a>,
  pub vale_as_subtype: StrI<'a>,
  pub vale_lock_weak: StrI<'a>,
  pub vale_same_instance: StrI<'a>,
  pub struct_constructor_generator: StrI<'a>,
  pub derive_struct_constructor: StrI<'a>,
  pub vale_static_sized_array_free: StrI<'a>,
  pub vale_static_sized_array_len: StrI<'a>,
  pub empty_string: StrI<'a>,
  pub thiss: StrI<'a>,
  pub box_human_name: StrI<'a>,
  pub box_member_name: StrI<'a>,
  pub t: StrI<'a>,
  pub v: StrI<'a>,
  pub drop_p1k: StrI<'a>,
  pub drop_p1: StrI<'a>,
  pub drop_r: StrI<'a>,
  pub drop_struct: StrI<'a>,
  pub drop_struct_template: StrI<'a>,
  pub drop_v: StrI<'a>,
  pub drop_vk: StrI<'a>,
  pub free_p1: StrI<'a>,
  pub free_struct_template: StrI<'a>,
  pub free_struct: StrI<'a>,
  pub free_v: StrI<'a>,
  pub x: StrI<'a>,
  pub d: StrI<'a>,
  pub builtins: StrI<'a>,
  pub arrays: StrI<'a>,
  pub is_interface: StrI<'a>,
  pub implements: StrI<'a>,
  pub is_callable: StrI<'a>,
  pub ref_list_compound_mutability: StrI<'a>,
  pub any: StrI<'a>,
  pub ifunction: StrI<'a>,
  pub m: StrI<'a>,
  pub e: StrI<'a>,
  pub f: StrI<'a>,
  pub array: StrI<'a>,
  pub static_array: StrI<'a>,
  pub list: StrI<'a>,
  pub add: StrI<'a>,
  pub opt: StrI<'a>,
  pub some: StrI<'a>,
  pub none: StrI<'a>,
  pub result: StrI<'a>,
  pub ok: StrI<'a>,
  pub err: StrI<'a>,
  pub functor1: StrI<'a>,
  pub my_module: StrI<'a>,
  pub rust: StrI<'a>,
  pub implicit_clone: StrI<'a>,
}

// /Volumes/V/Vale2/FrontendRust/src/keywords.rs:339
  pub fn new_for_scout(scout_arena: &ScoutArena<'a>) -> Self {

// /Volumes/V/Vale2/FrontendRust/src/keywords.rs:167
  pub fn new_for_parse(parse_arena: &ParseArena<'a>) -> Self {

// /Volumes/V/Vale2/FrontendRust/src/builtins/builtins.rs:70
pub fn get_embedded_modulized_code_map<'a>(
    parse_arena: &ParseArena<'a>,
    keywords: &Keywords<'a>,
) -> FileCoordinateMap<'a, String> {

// /Volumes/V/Vale2/FrontendRust/src/utils/code_hierarchy.rs:352
  fn or<F>(self, fallback: F) -> OrResolver<Self, F>
  where
    Self: Sized,
    F: IPackageResolver<'a, T>,
  {

// /Volumes/V/Vale2/FrontendRust/src/utils/code_hierarchy.rs:164
pub fn test_from_vec<'a, T: Clone>(
    parse_arena: &ParseArena<'a>,
    contents: Vec<T>,
  ) -> FileCoordinateMap<'a, T> {

// /Volumes/V/Vale2/FrontendRust/src/utils/code_hierarchy.rs:73
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate<'a> {
  pub module: StrI<'a>,
  pub packages: InternedSlice<'a, StrI<'a>>,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/typing_interner.rs:29
/// Temporary state (see @TFITCX)
pub struct TypingInterner<'s, 't>
where 's: 't,
{
    bump: &'t Bump,
    inner: RefCell<Inner<'s, 't>>,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/typing_interner.rs:123
    pub fn new(bump: &'t Bump) -> Self {

// /Volumes/V/Vale2/FrontendRust/src/typing/test/compiler_test_compilation.rs:14
pub fn compiler_test_compilation<'s, 'ctx, 't, 'p>(
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
) -> TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{

// /Volumes/V/Vale2/FrontendRust/src/typing/compilation.rs:131
pub fn expect_compiler_outputs(&mut self) -> &HinputsT<'s, 't> {

// /Volumes/V/Vale2/FrontendRust/src/typing/hinputs_t.rs:193
    pub fn lookup_function_by_str(&self, human_name: &str) -> &'t FunctionDefinitionT<'s, 't> {

// /Volumes/V/Vale2/FrontendRust/src/typing/ast/ast.rs:121
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/typing/ast/ast.rs:323
    pub maybe_origin_function_templata: Option<FunctionTemplataT<'s, 't>>,

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:62
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordT<'s, 't> {
  pub ownership: OwnershipT,
  pub region: RegionT,
  pub kind: KindT<'s, 't>,
  _sealed: (), // Force construction via `CoordT::new(...)`
}

impl<'s, 't> CoordT<'s, 't> {

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:63
  pub ownership: OwnershipT,

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:9
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum OwnershipT {
    Share,
    Own,
    Borrow,
    Weak,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:11
    Own,

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:65
  pub kind: KindT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:103
  Int(IntT),
  Bool(BoolT),
  Str(StrT),
  Float(FloatT),
  Struct(&'t StructTT<'s, 't>),
  Interface(&'t InterfaceTT<'s, 't>),
  StaticSizedArray(&'t StaticSizedArrayTT<'s, 't>),
  RuntimeSizedArray(&'t RuntimeSizedArrayTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
  OverloadSet(&'t OverloadSetT<'s, 't>),
}

impl<'s, 't> KindT<'s, 't> {
  pub fn expect_citizen(&self) -> ICitizenTT<'s, 't> {
    match self {
      KindT::Struct(c) => ICitizenTT::Struct(c),
      KindT::Interface(c) => ICitizenTT::Interface(c),
      _ => panic!("vfail"),
    }

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:110
impl<'s, 't> KindT<'s, 't> {

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:380
pub struct StructTTValT<'s, 't> {
  pub id: IdT<'s, 't>,
}

/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:381


// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:20
/// Interned (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub struct IdT<'s, 't>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'t [INameT<'s, 't>],
    pub local_name: INameT<'s, 't>,
    pub _must_intern: MustIntern,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:25
    pub local_name: INameT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:146
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameT<'s, 't> {
    ExportTemplate(&'t ExportTemplateNameT<'s>),
    Export(&'t ExportNameT<'s, 't>),
    ImplTemplate(&'t ImplTemplateNameT<'s>),
    Impl(&'t ImplNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    Let(&'t LetNameT<'s>),
    ExportAs(&'t ExportAsNameT<'s>),
    RawArray(&'t RawArrayNameT<'s, 't>),
    ReachablePrototype(&'t ReachablePrototypeNameT),
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT),
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    NonKindNonRegionPlaceholder(&'t NonKindNonRegionPlaceholderNameT<'s>),
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    TypingPassBlockResultVar(&'t TypingPassBlockResultVarNameT<'t>),
    TypingPassFunctionResultVar(&'t TypingPassFunctionResultVarNameT),
    TypingPassTemporaryVar(&'t TypingPassTemporaryVarNameT<'t>),
    TypingPassPatternMember(&'t TypingPassPatternMemberNameT<'t>),
    TypingIgnoredParam(&'t TypingIgnoredParamNameT),
    TypingPassPatternDestructuree(&'t TypingPassPatternDestructureeNameT<'t>),
    UnnamedLocal(&'t UnnamedLocalNameT<'s>),
    ClosureParam(&'t ClosureParamNameT<'s>),
    ConstructingMember(&'t ConstructingMemberNameT<'s>),
    WhileCondResult(&'t WhileCondResultNameT<'s>),
    Iterable(&'t IterableNameT<'s>),
    Iterator(&'t IteratorNameT<'s>),
    IterationOption(&'t IterationOptionNameT<'s>),
    MagicParam(&'t MagicParamNameT<'s>),
    CodeVar(&'t CodeVarNameT<'s>),
    AnonymousSubstructMember(&'t AnonymousSubstructMemberNameT),
    Primitive(&'t PrimitiveNameT<'s>),
    PackageTopLevel(&'t PackageTopLevelNameT),
    Project(&'t ProjectNameT<'s>),
    Package(&'t PackageNameT<'s>),
    Rune(&'t RuneNameT<'s>),
    BuildingFunctionNameWithClosureds(&'t BuildingFunctionNameWithClosuredsT<'s, 't>),
    ExternTemplate(&'t ExternTemplateNameT<'s>),
    Extern(&'t ExternNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s>),
    Self_(&'t SelfNameT),
    Arbitrary(&'t ArbitraryNameT),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
    ResolvingEnv(&'t ResolvingEnvNameT),
    CallEnv(&'t CallEnvNameT),
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:205
    Struct(&'t StructNameT<'s, 't>),

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1369
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructNameT<'s, 't> {
    pub template: IStructTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub _must_intern: MustIntern,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1370
    pub template: IStructTemplateNameT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:587
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IStructTemplateNameT<'s, 't> {
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s>),
    StructTemplate(&'t StructTemplateNameT<'s>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:589
    StructTemplate(&'t StructTemplateNameT<'s>),

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1415
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructTemplateNameT<'s> {
    pub human_name: StrI<'s>,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1416
    pub human_name: StrI<'s>,

// /Volumes/V/Vale2/FrontendRust/src/interner.rs:13
/// Interned string: a by-value wrapper around arena-backed `&'a str`.
/// Never arena-allocated; just holds a reference to canonical storage.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct StrI<'a>(pub &'a str);

// /Volumes/V/Vale2/FrontendRust/src/typing/test/traverse.rs:1696
macro_rules! collect_only_tnode {

// /Volumes/V/Vale2/FrontendRust/src/typing/test/traverse.rs:41
pub enum NodeRefT<'s, 't> {
    // ---- Top-level ----
    Hinputs(&'t HinputsT<'s, 't>),
    FunctionDefinition(&'t FunctionDefinitionT<'s, 't>),
    FunctionHeader(&'t FunctionHeaderT<'s, 't>),
    StructDefinition(&'t StructDefinitionT<'s, 't>),
    InterfaceDefinition(&'t InterfaceDefinitionT<'s, 't>),
    Edge(&'t EdgeT<'s, 't>),
    InterfaceEdgeBlueprint(&'t InterfaceEdgeBlueprintT<'s, 't>),
    Parameter(&'t ParameterT<'s, 't>),
    InstantiationBoundArguments(&'t InstantiationBoundArgumentsT<'s, 't>),

    // ---- Expression hierarchy ----
    Expression(ExpressionTE<'s, 't>),
    ReferenceExpression(ReferenceExpressionTE<'s, 't>),
    AddressExpression(AddressExpressionTE<'s, 't>),

    // 48 reference expression variants
    LetAndLend(&'t LetAndLendTE<'s, 't>),
    LockWeak(&'t LockWeakTE<'s, 't>),
    BorrowToWeak(&'t BorrowToWeakTE<'s, 't>),
    LetNormal(&'t LetNormalTE<'s, 't>),
    Unlet(&'t UnletTE<'s, 't>),
    Discard(&'t DiscardTE<'s, 't>),
    Defer(&'t DeferTE<'s, 't>),
    If(&'t IfTE<'s, 't>),
    While(&'t WhileTE<'s, 't>),
    Mutate(&'t MutateTE<'s, 't>),
    Restackify(&'t RestackifyTE<'s, 't>),
    Return(&'t ReturnTE<'s, 't>),
    Break(&'t BreakTE),
    Block(&'t BlockTE<'s, 't>),
    Consecutor(&'t ConsecutorTE<'s, 't>),
    Tuple(&'t TupleTE<'s, 't>),
    StaticArrayFromValues(&'t StaticArrayFromValuesTE<'s, 't>),
    ArraySize(&'t ArraySizeTE<'s, 't>),
    IsSameInstance(&'t IsSameInstanceTE<'s, 't>),
    AsSubtype(&'t AsSubtypeTE<'s, 't>),
    VoidLiteral(&'t VoidLiteralTE),
    ConstantInt(&'t ConstantIntTE<'s, 't>),
    ConstantBool(&'t ConstantBoolTE),
    ConstantStr(&'t ConstantStrTE<'s>),
    ConstantFloat(&'t ConstantFloatTE),
    ArgLookup(&'t ArgLookupTE<'s, 't>),
    ArrayLength(&'t ArrayLengthTE<'s, 't>),
    InterfaceFunctionCall(&'t InterfaceFunctionCallTE<'s, 't>),
    ExternFunctionCall(&'t ExternFunctionCallTE<'s, 't>),
    FunctionCall(&'t FunctionCallTE<'s, 't>),
    Reinterpret(&'t ReinterpretTE<'s, 't>),
    Construct(&'t ConstructTE<'s, 't>),
    NewRuntimeSizedArray(&'t NewRuntimeSizedArrayTE<'s, 't>),
    StaticArrayFromCallable(&'t StaticArrayFromCallableTE<'s, 't>),
    DestroyStaticSizedArrayIntoFunction(&'t DestroyStaticSizedArrayIntoFunctionTE<'s, 't>),
    DestroyStaticSizedArrayIntoLocals(&'t DestroyStaticSizedArrayIntoLocalsTE<'s, 't>),
    DestroyRuntimeSizedArray(&'t DestroyRuntimeSizedArrayTE<'s, 't>),
    RuntimeSizedArrayCapacity(&'t RuntimeSizedArrayCapacityTE<'s, 't>),
    PushRuntimeSizedArray(&'t PushRuntimeSizedArrayTE<'s, 't>),
    PopRuntimeSizedArray(&'t PopRuntimeSizedArrayTE<'s, 't>),
    InterfaceToInterfaceUpcast(&'t InterfaceToInterfaceUpcastTE<'s, 't>),
    Upcast(&'t UpcastTE<'s, 't>),
    SoftLoad(&'t SoftLoadTE<'s, 't>),
    Destroy(&'t DestroyTE<'s, 't>),

    // 5 address expression variants
    LocalLookup(&'t LocalLookupTE<'s, 't>),
    StaticSizedArrayLookup(&'t StaticSizedArrayLookupTE<'s, 't>),
    RuntimeSizedArrayLookup(&'t RuntimeSizedArrayLookupTE<'s, 't>),
    ReferenceMemberLookup(&'t ReferenceMemberLookupTE<'s, 't>),
    AddressMemberLookup(&'t AddressMemberLookupTE<'s, 't>),

    // ---- Templata hierarchy ----
    Templata(&'t ITemplataT<'s, 't>),
    CoordTemplata(&'t CoordTemplataT<'s, 't>),
    KindTemplata(&'t KindTemplataT<'s, 't>),
    PlaceholderTemplata(&'t PlaceholderTemplataT<'s, 't>),
    PrototypeTemplata(&'t PrototypeTemplataT<'s, 't>),
    IsaTemplata(&'t IsaTemplataT<'s, 't>),
    CoordListTemplata(&'t CoordListTemplataT<'s, 't>),
    FunctionTemplata(&'t FunctionTemplataT<'s, 't>),
    StructDefinitionTemplata(&'t StructDefinitionTemplataT<'s, 't>),
    InterfaceDefinitionTemplata(&'t InterfaceDefinitionTemplataT<'s, 't>),
    ImplDefinitionTemplata(&'t ImplDefinitionTemplataT<'s, 't>),
    ExternFunctionTemplata(&'t ExternFunctionTemplataT<'s, 't>),

    // ---- Kinds + types ----
    Kind(&'t KindT<'s, 't>),
    StructTT(&'t StructTT<'s, 't>),
    InterfaceTT(&'t InterfaceTT<'s, 't>),
    StaticSizedArrayTT(&'t StaticSizedArrayTT<'s, 't>),
    RuntimeSizedArrayTT(&'t RuntimeSizedArrayTT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
    OverloadSet(&'t OverloadSetT<'s, 't>),
    Coord(&'t CoordT<'s, 't>),
    Id(&'t IdT<'s, 't>),
    Signature(&'t SignatureT<'s, 't>),
    Prototype(&'t PrototypeT<'s, 't>),

    // ---- Names + envs (trait-level only; we do not enumerate sub-variants) ----
    Name(&'t INameT<'s, 't>),
    VarName(&'t IVarNameT<'s, 't>),
    Environment(IEnvironmentT<'s, 't>),

    // ---- Auxiliaries (trait-level only) ----
    FunctionAttribute(&'t IFunctionAttributeT<'s>),
    CitizenAttribute(&'t ICitizenAttributeT<'s>),
    StructMember(&'t IStructMemberT<'s, 't>),
    ReferenceMemberType(&'t ReferenceMemberTypeT<'s, 't>),
    AddressMemberType(&'t AddressMemberTypeT<'s, 't>),
    LocalVariable(&'t ILocalVariableT<'s, 't>),

    // ---- Override / Edge children ----
    Override(&'t OverrideT<'s, 't>),

    // ---- Exports / externs ----
    KindExport(&'t KindExportT<'s, 't>),
    FunctionExport(&'t FunctionExportT<'s, 't>),
    KindExtern(&'t KindExternT<'s, 't>),
    FunctionExtern(&'t FunctionExternT<'s, 't>),
}

// /Volumes/V/Vale2/FrontendRust/src/typing/test/traverse.rs:44
    FunctionDefinition(&'t FunctionDefinitionT<'s, 't>),

// /Volumes/V/Vale2/FrontendRust/src/typing/test/traverse.rs:62
    LetNormal(&'t LetNormalTE<'s, 't>),

// /Volumes/V/Vale2/FrontendRust/src/typing/ast/expressions.rs:351
pub struct LetNormalTE<'s, 't>
where 's: 't,
{
    pub variable: ILocalVariableT<'s, 't>,
    pub expr: ReferenceExpressionTE<'s, 't>,
}

impl<'s, 't> LetNormalTE<'s, 't> {

// /Volumes/V/Vale2/FrontendRust/src/typing/ast/expressions.rs:354
}

// /Volumes/V/Vale2/FrontendRust/src/typing/env/function_environment_t.rs:970
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ILocalVariableT<'s, 't>
where 's: 't,
{
  Addressible(AddressibleLocalVariableT<'s, 't>),
  Reference(ReferenceLocalVariableT<'s, 't>),
}

// /Volumes/V/Vale2/FrontendRust/src/typing/env/function_environment_t.rs:974
  Reference(ReferenceLocalVariableT<'s, 't>),

// /Volumes/V/Vale2/FrontendRust/src/typing/env/function_environment_t.rs:1011
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceLocalVariableT<'s, 't>
where 's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub coord: CoordT<'s, 't>,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/env/function_environment_t.rs:1014
  pub name: IVarNameT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1091
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVarNameT<'s, 't> {
    TypingPassBlockResultVar(&'t TypingPassBlockResultVarNameT<'t>),
    TypingPassFunctionResultVar(&'t TypingPassFunctionResultVarNameT),
    TypingPassTemporaryVar(&'t TypingPassTemporaryVarNameT<'t>),
    TypingPassPatternMember(&'t TypingPassPatternMemberNameT<'t>),
    TypingIgnoredParam(&'t TypingIgnoredParamNameT),
    TypingPassPatternDestructuree(&'t TypingPassPatternDestructureeNameT<'t>),
    UnnamedLocal(&'t UnnamedLocalNameT<'s>),
    ClosureParam(&'t ClosureParamNameT<'s>),
    ConstructingMember(&'t ConstructingMemberNameT<'s>),
    WhileCondResult(&'t WhileCondResultNameT<'s>),
    Iterable(&'t IterableNameT<'s>),
    Iterator(&'t IteratorNameT<'s>),
    IterationOption(&'t IterationOptionNameT<'s>),
    MagicParam(&'t MagicParamNameT<'s>),
    CodeVar(&'t CodeVarNameT<'s>),
    AnonymousSubstructMember(&'t AnonymousSubstructMemberNameT),
    Self_(&'t SelfNameT),
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1106
    CodeVar(&'t CodeVarNameT<'s>),

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1196
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CodeVarNameT<'s> {
    pub name: StrI<'s>,
}

// /Volumes/V/Vale2/FrontendRust/src/typing/names/names.rs:1197
    pub name: StrI<'s>,

// /Volumes/V/Vale2/FrontendRust/src/typing/env/function_environment_t.rs:1015
  pub coord: CoordT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/tests/tests.rs:20
pub fn load_expected(resource_filename: &str) -> String {

// /Volumes/V/Vale2/FrontendRust/src/typing/ast/ast.rs:322
    pub return_type: CoordT<'s, 't>,

// /Volumes/V/Vale2/FrontendRust/src/typing/ast/ast.rs:180
}

// /Volumes/V/Vale2/FrontendRust/src/typing/types/types.rs:12
    Borrow,

## Your Response Format

You must respond with ONLY valid JSON (no markdown fences) matching this schema:

{"observations": [{"reason": "explanation", "violation": true}]}

Each observation is something you noticed about the code change. Set "violation" to true only if it is a genuine violation of the rule after considering all exceptions. If an exception applies, set "violation" to false.

If you have no observations at all, respond with:
{"observations": []}


---

## 🤖 Claude

```json
{"observations": []}
```

The code change is a new test function that uses pattern matching to verify compiler output. The key line of interest is:

```rust
StructTemplateNameT { human_name: StrI("SomeStruct"), .. }
```

This appears within a `match` expression (pattern position), where `StrI("SomeStruct")` destructures an existing `StrI<'s>` value to verify it matches "SomeStruct". According to the rule's clarifications, pattern destructuring in `match` positions is explicitly ALLOWED — it inspects an already-interned value rather than constructing a new one.

No Val types (`IRuneValS`, `INameValS`, etc.) are stored, returned, or constructed. No manual `StrI` construction occurs in expression position. The code complies with the Immediate Interning Discipline rule.

---

