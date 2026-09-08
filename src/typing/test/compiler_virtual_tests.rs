use crate::builtins::builtins::{builtin_source_for_arrays, builtin_source_for_as, empty_v_builtins_stub};
use crate::code_source::{CodeSource, Source};
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::load_expected;
use crate::tests::tests::new_test_code_map;
use crate::tests::tests::new_test_package_source;
use crate::typing::names::names::{
  FunctionNameT, FunctionTemplateNameT, INameT, IdT, InterfaceNameT, InterfaceTemplateNameT,
};
use crate::collect_only_tnode;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::test::traverse::NodeRefT;
use crate::typing::types::types::{BorrowRefT, DynInterfaceTT, InterfaceTT, KindT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::fx::HashMap;
use bumpalo::Bump;

#[test]
fn regular_interface_and_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
sealed interface Opt { }

struct Some { x int; }
impl Opt for Some;
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let drop_func_names: Vec<_> = coutputs
    .functions
    .iter()
    .map(|f| f.header.id)
    .filter_map(|f| match f {
      id @ IdT {
        local_name:
          INameT::Function(FunctionNameT {
            template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
            ..
          }),
        ..
      } => Some(id),
      _ => None,
    })
    .collect();
  assert_eq!(drop_func_names.len(), 2);

  let interface = coutputs.lookup_interface_by_human_name("Opt");
  let _ = interface.internal_methods;
}

/// An `impl` that supplies no override for an interface's method must be reported as a compile
/// error. `Impl` implements `Handler` but provides no `handle`, so the program must fail to compile.
#[test]
fn missing_interface_override_reports_a_compile_error() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
sealed interface Handler { func handle(virtual self &Handler) int; }
struct Impl { }
impl Handler for Impl;
exported func main() int { return 0; }
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  assert!(compile.get_compiler_outputs().is_err());
}

#[test]
fn regular_open_interface_and_struct_no_anonymous_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveAnonymousSubstruct
interface Opt { }

struct Some { x int; }
impl Opt for Some;
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let drop_func_names: Vec<_> = coutputs
    .functions
    .iter()
    .map(|f| f.header.id)
    .filter_map(|f| match f {
      id @ IdT {
        local_name:
          INameT::Function(FunctionNameT {
            template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
            ..
          }),
        ..
      } => Some(id),
      _ => None,
    })
    .collect();
  assert_eq!(drop_func_names.len(), 2);
}

#[test]
fn implementing_two_interfaces_causes_no_vdrop_conflict() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
struct MyStruct {}

interface IA {}
impl IA for MyStruct;

interface IB {}
impl IB for MyStruct;

func bork(a IA) {}
func zork(b IB) {}
exported func main() {
  bork(MyStruct());
  zork(MyStruct());
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn upcast() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface IShip {}
struct Raza { fuel int; }
impl IShip for Raza;

exported func main() {
  ship IShip = Raza(42);
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn virtual_with_body() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface IBork { }
struct Bork { }
impl IBork for Bork;

func rebork(virtual result *IBork) bool { true }
exported func main() {
  rebork(&Bork());
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let _compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
}

#[test]
fn templated_interface_and_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
sealed interface Opt<T>
where func drop(T)void
{ }

struct Some<T>
where func drop(T)void
{ x T; }

impl<T> Opt<T> for Some<T>
where func drop(T)void;
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();
  let drop_func_names: Vec<_> = coutputs
    .functions
    .iter()
    .map(|f| f.header.id)
    .filter_map(|f| match f {
      id @ IdT {
        local_name:
          INameT::Function(FunctionNameT {
            template: FunctionTemplateNameT { human_name: StrI("drop"), .. },
            ..
          }),
        ..
      } => Some(id),
      _ => None,
    })
    .collect();
  assert_eq!(drop_func_names.len(), 2);
}

#[test]
fn custom_drop_with_concept_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveInterfaceDrop
sealed interface Opt<T> { }

abstract func drop<T>(virtual opt Opt<T>)
where func drop(T)void;

#!DeriveStructDrop
struct Some<T> { x T; }
impl<T> Opt<T> for Some<T>;

func drop<T>(opt Some<T>)
where func drop(T)void
{
  [x] = ^opt;
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

#[test]
fn test_complex_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/genericvirtuals/templatedinterface.vale");
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "print"),
    Source::builtin_module(&parse_arena, &parser_keywords, "str"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
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
  compile.expect_compiler_outputs();
}

#[test]
fn test_specializing_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = load_expected("programs/genericvirtuals/specializeinterface.vale");
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "arith"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
    Source::builtin_module(&parse_arena, &parser_keywords, "print"),
    Source::builtin_module(&parse_arena, &parser_keywords, "str"),
    new_test_code_map(&parse_arena, code),
    new_test_package_source(&parse_arena, "printutils"),
    new_test_package_source(&parse_arena, "castutils"),
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
  compile.expect_compiler_outputs();
}

// VCOORD: enable this
#[test]
fn use_bound_from_struct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveStructDrop
struct BorkForwarder<Lam>
where func __call(&Lam)int // 3
{
  lam Lam;
}


func bork<Lam>( // 1
  self &BorkForwarder<Lam> // 2
) int {
  return (&self.lam)();
}

exported func main() {
  b = BorkForwarder({ 7 });
  (&b).bork();
  [_] = ^b;
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: enable this
#[test]
fn basic_interface_forwarder() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveInterfaceDrop
sealed interface Bork {
  func bork(virtual self &Bork) int;
}

#!DeriveStructDrop
struct BorkForwarder<Lam>
where func drop(Lam)void, func __call(&Lam)int {
  lam Lam;
}

impl<Lam> Bork for BorkForwarder<Lam>;

func bork<Lam>(self &BorkForwarder<Lam>) int {
  return (&self.lam)();
}

exported func main() int {
  f = BorkForwarder({ 7 });
  z = (&f).bork();
  [_] = ^f;
  return ^z;
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: enable this
#[test]
fn generic_interface_forwarder() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveInterfaceDrop
sealed interface Bork<T> {
  func bork(virtual self &Bork<T>) int;
}

#!DeriveStructDrop
struct BorkForwarder<T, Lam>
where func drop(Lam)void, func __call(&Lam)T {
  lam Lam;
}

impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;

func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
  return (&self.lam)();
}

exported func main() int {
  f = BorkForwarder<int>({ 7 });
  z = (&f).bork();
  [_] = ^f;
  return ^z;
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: enable this
#[test]
#[ignore]
fn generic_interface_forwarder_with_bound() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveInterfaceDrop
sealed interface Bork<T>
where func threeify(T)T {
  func bork(virtual self &Bork<T>) int;
}

#!DeriveStructDrop
struct BorkForwarder<T, Lam>
where func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {
  lam Lam;
}

impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;

func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
  return threeify((&self.lam)());
}

func threeify(x int) int { 3 }

exported func main() int {
  f = BorkForwarder<int>({ 7 });
  z = (&f).bork();
  [_] = ^f;
  return ^z;
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: re-enable anonymous interface macro after we do the ITypeST migration
#[test]
#[ignore]
fn basic_interface_anonymous_subclass() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface Bork {
  func bork(virtual self &Bork) int;
}

exported func main() int {
  f = Bork({ 7 });
  return f.bork();
}
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: re-enable anonymous interface macro after we do the ITypeST migration
#[test]
#[ignore]
fn integer_is_compatible_with_interface_anonymous_substruct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: x6 int → x6 &int — anonymous-interface-macro forwarder accesses captured `6` as a borrowed field
  let code = r#"
import v.builtins.drop.*;
interface AFunction2<R, P1> {
  func doCall(virtual this &AFunction2<R, P1>, a P1) R;
}
func __call(x6 &int, x42 int)str { "hi" }
exported func main() str {
  func = AFunction2<str, int>(6);
  return func.doCall(42);
}
"#;
  let code_source = CodeSource::new(vec![
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
  compile.expect_compiler_outputs();
}

// VCOORD: re-enable anonymous interface macro after we do the ITypeST migration
#[test]
#[ignore]
fn lambda_is_compatible_with_interface_anonymous_substruct() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.str.*;

interface AFunction2<R, P1> {
  func __call(virtual this &AFunction2<R, P1>, a P1) R;
}
exported func main() str {
  func = AFunction2<str, int>((i) => { str(i) });
  return func(42);
}
";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "str"),
    Source::builtin_module(&parse_arena, &parser_keywords, "drop"),
    Source::builtin_module(&parse_arena, &parser_keywords, "implicit_clone"),
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
  compile.expect_compiler_outputs();
}

#[test]
fn implementing_a_non_generic_interface_call() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
#!DeriveInterfaceDrop
interface IObserver<T> { }

#!DeriveStructDrop
struct MyThing { }

impl<T> IObserver<T> for MyThing;

";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

// VCOORD: re-enable anonymous interface macro after we do the ITypeST migration
#[test]
#[ignore]
fn anonymous_substruct_8() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  // TSUGAR: a.3 is &int
  let code = r"
import v.builtins.arrays.*;
//import array.make.*;

interface IThing {
  func __call(virtual self &IThing, i int) int;
}

struct MyThing { }
func __call(self &MyThing, i int) int { i }

impl IThing for MyThing;

exported func main() int {
  i IThing = MyThing();
  a = Array<int>(10, &i);
  return __copy_prim(&a.3);
}
";
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
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
  compile.expect_compiler_outputs();
}

#[test]
fn dyn_interface_borrow_kind() {
  // `&dyn IShip` must type to `BorrowRef(DynInterface(IShip))`.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface IShip {}
func foo(ship &dyn IShip) { }
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  // `foo`'s param types to `&dyn IShip` = BorrowRef(DynInterface(IShip)).
  let foo = coutputs.lookup_function_by_str("foo");
  assert!(
    matches!(
      foo.header.id.local_name,
      INameT::Function(FunctionNameT {
        parameters:
          [KindT::BorrowRef(BorrowRefT {
            inner:
              KindT::DynInterface(DynInterfaceTT {
                inner:
                  InterfaceTT {
                    id:
                      IdT {
                        local_name:
                          INameT::Interface(InterfaceNameT {
                            template: InterfaceTemplateNameT { human_namee: StrI("IShip") },
                            ..
                          }),
                        ..
                      },
                    ..
                  },
                ..
              }),
          })],
        ..
      })
    ),
    "expected foo with param &dyn IShip = BorrowRef(DynInterface(IShip)), got {:?}",
    foo.header.id.local_name
  );
}

/// A plain `interface` is sealed by default, so an abstract method declared in the same crate
/// OUTSIDE the interface body is allowed.
#[test]
fn sealed_default_allows_abstract_method_outside_interface() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface IShip {}
struct Raza {}
impl IShip for Raza;
abstract func fuel(virtual s &IShip) int;
func fuel(s &Raza) int { return 42; }
";
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let coutputs = compile.expect_compiler_outputs();

  let iship = coutputs.lookup_interface_by_human_name("IShip");
  let raza = coutputs.lookup_struct_by_str("Raza");
  let edge = coutputs.lookup_impl(*raza.instantiated_citizen.id, *iship.instantiated_interface.id);
  assert!(
    !edge.abstract_func_to_override_func.is_empty(),
    "expected the external abstract fuel to be overridden on the Raza impl edge"
  );
}

/// Probe (dyn frontend, capability A, owned/construction): constructing a `Box<dyn IShip>` from a
/// concrete `Raza` — the owned-interface conversion the plan targets. NOTE: currently fails because
/// `Box<T>`'s constructor param is a bare generic `T` (explicitly bound to `dyn IShip`), and the
/// arg-upcast pass deliberately doesn't read explicit template args (a pre-existing limitation that
/// affects `Box<IShip>` too, not dyn-specific). Kept as the construction target under discussion.
#[test]
#[ignore = "deferred owned Box<dyn X> construction — see 4b open issue"]
fn dyn_construct_upcast() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.box.*;
interface IShip {}
struct Raza {}
impl IShip for Raza;
exported func main() {
  ship Box<dyn IShip> = Box<dyn IShip>(Raza());
}
";
  let code_source = CodeSource::new(vec![
    Source::builtin_module(&parse_arena, &parser_keywords, "box"),
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
  compile.expect_compiler_outputs();
}

#[test]
fn dyn_borrow_upcast() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface IShip {}
struct Raza { fuel int; }
impl IShip for Raza;
func take(ship &dyn IShip) { }
exported func main() {
  raza Raza = Raza(42);
  take(&raza);
}
";
  let code_source = CodeSource::new(vec![
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
  compile.expect_compiler_outputs();
}

#[test]
fn dyn_dispatch() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
interface Handler { func handle(virtual self &Handler) int; }
struct Impl {}
impl Handler for Impl;
func handle(self &Impl) int { 42 }
func run(h &dyn Handler) int { handle(h) }
";
  let code_source = CodeSource::new(vec![
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
  let coutputs = compile.expect_compiler_outputs();

  // The receiver keeps its dyn form: run's param is a borrow of a DynInterface, not a bare Interface.
  let run = coutputs.lookup_function_by_str("run");
  assert!(
    matches!(
      run.header.id.local_name,
      INameT::Function(FunctionNameT {
        parameters: [KindT::BorrowRef(BorrowRefT { inner: KindT::DynInterface(_) })],
        ..
      })
    ),
    "expected run(h &dyn Handler) with param BorrowRef(DynInterface(Handler)), got {:?}",
    run.header.id.local_name
  );
}

#[test]
fn dyn_downcast() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = r"
import v.builtins.as.*;
import v.builtins.logic.*;
import v.builtins.drop.*;

interface IShip {}
struct Raza { fuel int; }
impl IShip for Raza;

func moo(ship &dyn IShip) {
  ship.try_as<Raza>();
}
";
  let code_source = CodeSource::new(vec![
    builtin_source_for_as(&parse_arena, &parser_keywords),
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
  let coutputs = compile.expect_compiler_outputs();

  let moo = coutputs.lookup_function_by_str("moo");
  let try_as_prototype: PrototypeT<'_, '_> = collect_only_tnode!(
    NodeRefT::FunctionDefinition(moo),
    NodeRefT::FunctionCall(c @ FunctionCallTE {
      callable: PrototypeT {
        id: IdT {
          local_name: INameT::Function(FunctionNameT {
            template: FunctionTemplateNameT { human_name: StrI("try_as"), .. },
            ..
          }),
          ..
        },
        ..
      },
      ..
    }) => Some(c.callable)
  );
  let try_as_template_args = match try_as_prototype.id.local_name {
    INameT::Function(fn_name) => fn_name.template_args,
    other => panic!("expected try_as Function name, got {:?}", other),
  };
  assert!(
    matches!(
      try_as_template_args,
      [
        ITemplataT::Kind(KindTemplataT { kind: KindT::Struct(_) }),
        ITemplataT::Kind(KindTemplataT { kind: KindT::DynInterface(_) }),
      ]
    ),
    "expected try_as<Raza, dyn IShip>: SubType=Struct, SuperType=DynInterface; got {:?}",
    try_as_template_args
  );
}
