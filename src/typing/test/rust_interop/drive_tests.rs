// Tests for the `valenc-rs` wrapper's engine (@DBAPIZ) and the `vale-stub-gen` seed
// (`generate_stub_source`). Unlike the corpus, these do not run a `Case` against a fixture stub on disk —
// they exercise a *generated* stub against a *caller-supplied* rlib, the shape cargo drives (handing the
// wrapper `--extern`/`-L dependency` per crate). Each test builds everything in its own `TempDir` (@TMBFIZ).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::typing::rust_interop::drive::{
  default_sysroot, run_wrapper, WrapperInputs, WrapperResult,
};
use crate::typing::rust_interop::stub_gen::generate_stub_source_from_vale;

use super::harness::build_dep_rlib;

// The one Vale program the tracer drives: import a Rust free function and return its value.
const TINY_VALE: &str = "import rust.tiny.seven; exported func main() int { return seven(); }";

// A Vale program that imports through a re-export: `greeter` only `pub use`s `helper::seven`.
const REEXPORT_VALE: &str =
  "import rust.greeter.seven; exported func main() int { return seven(); }";

// Build, with cargo on the fork toolchain, a `greeter` crate that re-exports `helper::seven` (helper's
// `seven` is the canonical item; greeter names it only via `pub use`). Returns the `deps` dir holding
// both hashed rlibs. `--target-dir` is explicit because this environment sets a shared cargo target dir.
// Hermetic: path dependency only, no network.
fn build_reexport_crate(root: &Path) -> PathBuf {
  let helper = root.join("helper");
  fs::create_dir_all(helper.join("src")).expect("mkdir helper/src");
  fs::write(
    helper.join("Cargo.toml"),
    "[package]\nname = \"helper\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"rlib\"]\n",
  )
  .expect("write helper Cargo.toml");
  fs::write(helper.join("src/lib.rs"), "pub fn seven() -> i32 { 7 }\n").expect("write helper lib.rs");

  let greeter = root.join("greeter");
  fs::create_dir_all(greeter.join("src")).expect("mkdir greeter/src");
  fs::write(
    greeter.join("Cargo.toml"),
    "[package]\nname = \"greeter\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"rlib\"]\n\n[dependencies]\nhelper = { path = \"../helper\" }\n",
  )
  .expect("write greeter Cargo.toml");
  fs::write(greeter.join("src/lib.rs"), "pub use helper::seven;\n").expect("write greeter lib.rs");
  fs::write(greeter.join("rust-toolchain.toml"), "[toolchain]\nchannel = \"rustc-fork\"\n")
    .expect("write rust-toolchain.toml");

  let target_dir = root.join("target");
  let sysroot = default_sysroot();
  let build_out = Command::new("cargo")
    .current_dir(&greeter)
    .arg("build")
    .arg("--offline")
    .arg("--target-dir")
    .arg(&target_dir)
    .env("RUSTUP_TOOLCHAIN", "rustc-fork")
    .env("DYLD_LIBRARY_PATH", format!("{sysroot}/lib"))
    .env_remove("RUSTC")
    .output()
    .expect("could not spawn cargo build");
  assert!(
    build_out.status.success(),
    "cargo build (fork) failed:\nstdout:\n{}\nstderr:\n{}",
    String::from_utf8_lossy(&build_out.stdout),
    String::from_utf8_lossy(&build_out.stderr),
  );
  target_dir.join("debug/deps")
}

// Drive a `.valen` binary crate through the wrapper the way cargo would — write it, build a
// `--crate-type=bin` arg set (plus `extra` flags like `--extern`/`-L` for imports), and hand back
// `run_wrapper`'s own result, so a test can observe a typing failure as the `Err` it is.
fn wrapper_drive(
  out_dir: &Path,
  vale_source: &str,
  extra: Vec<String>,
  borrow_check: bool,
) -> Result<WrapperResult, String> {
  let valen = out_dir.join("prog.valen");
  fs::write(&valen, vale_source).expect("could not write prog.valen");
  let mut rustc_args = vec![
    "valenc-rs".to_string(),
    valen.display().to_string(),
    "--crate-type=bin".to_string(),
    "--crate-name=prog".to_string(),
    "--edition=2021".to_string(),
    format!("--sysroot={}", default_sysroot()),
    format!("--out-dir={}", out_dir.display()),
  ];
  rustc_args.extend(extra);
  run_wrapper(&WrapperInputs { rustc_args, borrow_check })
}

// Run the `prog` binary a successful `wrapper_drive` left in `out_dir`, returning its exit code.
fn run_produced_binary(out_dir: &Path) -> i32 {
  let exe = out_dir.join("prog");
  let output = Command::new(&exe).output().expect("could not run the produced binary");
  output.status.code().unwrap_or(-1)
}

// `wrapper_drive` with the borrow checker on, asserting it drove Valen and rustc exited 0, then run
// the produced binary and return its exit code.
fn wrapper_run_binary(out_dir: &Path, vale_source: &str, extra: Vec<String>) -> i32 {
  let result = wrapper_drive(out_dir, vale_source, extra, /*borrow_check=*/ true)
    .expect("run_wrapper should succeed");
  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert!(result.drove_valen, "a .valen crate must drive the Valen engine");
  run_produced_binary(out_dir)
}

// Find cargo's content-hashed `lib<name>-<hash>.rlib` in a deps dir — the explicit path cargo hands the
// wrapper as `--extern <name>=<path>`. (The interim `drive` CLI resolved this itself; the wrapper does
// not, since real cargo always supplies the path.)
fn find_rlib(deps_dir: &Path, name: &str) -> PathBuf {
  let prefix = format!("lib{name}-");
  fs::read_dir(deps_dir)
    .expect("could not read the deps dir")
    .flatten()
    .map(|entry| entry.path())
    .find(|path| {
      path
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f.starts_with(&prefix) && f.ends_with(".rlib"))
    })
    .unwrap_or_else(|| panic!("no lib{name}-*.rlib in {}", deps_dir.display()))
}

// The generator emits the load-bearing stub shape from the parsed program: one `pub use` per import
// (@RTMEIZ), the marker, a `#[vale::emit_consumer_body]` root per exported func, and the bin shim.
#[test]
fn stub_gen_emits_pub_use_and_consumer_body_from_parsed_program() {
  let stub = generate_stub_source_from_vale(TINY_VALE).expect("stub generation should succeed");
  assert!(stub.contains("extern crate tiny;"), "stub:\n{stub}");
  assert!(stub.contains("pub use tiny::seven;"), "stub:\n{stub}");
  // The attr carries a digest of the `.valen` source (so rustc re-collects on a body-only edit —
  // the incremental fix, Part B), so match its prefix rather than the bare attr.
  assert!(stub.contains("#[vale::emit_consumer_body(digest = \""), "stub:\n{stub}");
  assert!(stub.contains("pub fn __vale_main() -> i32"), "stub:\n{stub}");
  assert!(stub.contains("__VALE_STUBS_MARKER"), "stub:\n{stub}");
  assert!(stub.contains("fn main()"), "stub:\n{stub}");
}

// A Vale struct that implements an imported Rust trait must be projected into the stub as a real Rust
// type + trait impl, so rustc can monomorphize the generic caller over it and reach the override body
// (which the Valen backend fills under the same mangled symbol). Without this projection the struct's
// type arg is unconvertible, the generic-method leaf never resolves, and the backend aborts on an
// undeclared extern (the NobiliaV `on_tick` crash). The override signature is rendered from the Vale
// one: a `self &Struct` receiver → `&self`, a `&ImportedType` param → `&ImportedType`, void return.
#[test]
fn stub_gen_projects_a_valen_struct_that_implements_a_rust_trait() {
  let vale = r#"
import rust.mycrate.Widget;
import rust.mycrate.Cb;
struct MyCb { }
impl Cb for MyCb;
func on_event(self &MyCb, w &Widget) {
  w.poke();
}
exported func main() int {
  return 7;
}
"#;
  let stub = generate_stub_source_from_vale(vale).expect("stub generation should succeed");
  // The non-generic struct is the degenerate case of the wrapper-as-field shape: a `__ValeOpaque<HASH>`
  // payload + an empty `PhantomData<()>` carrier, and a non-generic `impl Cb for MyCb`.
  assert!(stub.contains("pub struct MyCb(__ValeOpaque<"), "stub:\n{stub}");
  assert!(stub.contains(", ::std::marker::PhantomData<()>);"), "stub:\n{stub}");
  assert!(stub.contains("impl Cb for MyCb {"), "stub:\n{stub}");
  // The override is emitted inside the impl with a deferred body and the rendered Rust signature.
  assert!(stub.contains("fn on_event(&self, _w: &Widget) {"), "stub:\n{stub}");
  assert!(stub.contains("unreachable!()"), "stub:\n{stub}");
  // Only the trait impl is projected; a plain `pub use` of the imported trait is still emitted.
  assert!(stub.contains("pub use mycrate::Cb;"), "stub:\n{stub}");
}

// A Vale override that mirrors a Rust `&mut` trait method must project `&mut` into the stub, or rustc
// rejects the impl with E0053 ("types differ in mutability") — the NobiliaV `on_tick(&mut self, w: &mut
// NobiliaWindow, ...)` wall. Mutability lives in the override's `mut(g)` effect clause plus each borrow's
// `in g` region, never on the `&T` type, so the emitter reads the effect clause and each parameter's (and
// the receiver's) region: a borrow whose region is marked `mut` renders `&mut`. `input` has no `mut`
// region and is the negative control that stays a shared `&FrameInput`. (The non-mut `on_event` test
// above is the regression guard that a shared-only override still renders `&self`/`&Widget`.)
#[test]
fn stub_gen_projects_mut_borrows_as_mut_references() {
  let vale = r#"
import rust.mycrate.Widget;
import rust.mycrate.FrameInput;
import rust.mycrate.Cb;
struct MyCb { }
impl Cb for MyCb;
func on_tick<r', s'>(self &MyCb in s, w &Widget in r, input &FrameInput) mut(r) mut(s) {
  w.poke();
}
exported func main() int {
  return 7;
}
"#;
  let stub = generate_stub_source_from_vale(vale).expect("stub generation should succeed");
  // `&mut self` (region s is mut), `&mut Widget` (region r is mut), and a shared `&FrameInput` (no mut
  // region) — all three in one signature, so the mut-set read and the shared negative control are proven
  // together.
  assert!(
    stub.contains("fn on_tick(&mut self, _w: &mut Widget, _input: &FrameInput) {"),
    "stub:\n{stub}"
  );
}

// A generic, data-carrying forwarder (`MyCb<F>` holding a functor, implementing an imported trait) is the
// shape that lets a lambda be handed to a rust callback. It projects as the design's opaque wrapper-as-field
// shape (arch §10): `pub struct MyCb<F>(__ValeOpaque<HASH>, PhantomData<(F)>)` + `impl<F> Cb for MyCb<F>`,
// with the `__ValeOpaque<const T: u64>` wrapper predeclared once at the stub root. rustc keeps MyCb's own
// DefId (so the impl resolves) but never sees its fields; Vale owns the layout via a `layout_of` override
// (deferred). Mirrors Sky's `__ToylangOpaque` shape (toylangc/src/stub_gen.rs).
#[test]
fn stub_gen_projects_a_generic_forwarder_as_opaque_wrapper() {
  let vale = r#"
import rust.mycrate.Widget;
import rust.mycrate.Cb;
struct MyCb<F> where func drop(F)void, func __call(&F, &Widget)void { f F; }
impl<F> Cb for MyCb<F>;
func on_event<F>(self &MyCb<F>, w &Widget) {
  (&self.f)(w);
}
exported func main() int {
  return 7;
}
"#;
  let stub = generate_stub_source_from_vale(vale).expect("stub generation should succeed");
  // The universal opaque wrapper is predeclared once at the stub root, as the 3-marker-field struct:
  // a real `UnsafeCell<()>` for `!Freeze` (so rustc emits no `readonly` on a `&__ValeOpaque<..>` param;
  // a `PhantomData` of it would stay `Freeze`), `PhantomData<*mut ()>` for `!Send + !Sync`,
  // `PhantomPinned` for `!Unpin`; fields at all so rustc's debuginfo walker has something to visit.
  // Pinned as the exact text because each field is load-bearing and the composition is easy to
  // "simplify" back into a hole.
  assert!(
    stub.contains(
      "pub struct __ValeOpaque<const T: u64>(::core::cell::UnsafeCell<()>, \
       ::std::marker::PhantomData<*mut ()>, ::std::marker::PhantomPinned);"
    ),
    "stub:\n{stub}"
  );
  // The forwarder is the 2-field wrapper-as-field shape, generic over F, with a PhantomData carrier so the
  // declared generic is "used" (E0392). The typeid hash is elided (it is stability-fenced separately).
  assert!(stub.contains("pub struct MyCb<F>(__ValeOpaque<"), "stub:\n{stub}");
  assert!(stub.contains(">, ::std::marker::PhantomData<(F)>);"), "stub:\n{stub}");
  // The impl mirrors the struct's generics: `impl<F> Cb for MyCb<F>`.
  assert!(stub.contains("impl<F> Cb for MyCb<F> {"), "stub:\n{stub}");
  assert!(stub.contains("fn on_event(&self, _w: &Widget) {"), "stub:\n{stub}");
}

// The pure-Rust passthrough (@PRCCBIVRZ). A crate whose root is `.rs` is not a Valen crate, so
// `run_wrapper`'s extension dispatch takes the passthrough branch: it installs no query overrides and no
// fill_extra_modules hook (`drove_valen == false`, no firings), compiling exactly as vanilla rustc would.
// Here that produces `libplain.rlib` and exits 0.
#[test]
fn wrapper_passes_through_a_pure_rust_crate() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let rs = out_dir.join("plain.rs");
  fs::write(&rs, "pub fn f() -> i32 { 3 }\n").expect("write plain.rs");

  let result = run_wrapper(&WrapperInputs {
    rustc_args: vec![
      "valenc-rs".to_string(),
      rs.display().to_string(),
      "--crate-type=lib".to_string(),
      "--crate-name=plain".to_string(),
      "--edition=2021".to_string(),
      format!("--sysroot={}", default_sysroot()),
      format!("--out-dir={}", out_dir.display()),
    ],
    borrow_check: true,
  })
  .expect("run_wrapper should succeed");

  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert!(!result.drove_valen, "a .rs crate must take the passthrough branch");
  assert!(result.firings.is_empty(), "the passthrough installs no Valen machinery");
}

// The tracer: a `.valen` binary that imports a caller-supplied std-only rlib links and runs → 7. The
// wrapper generates the pass-1 stub, substitutes it for the `.valen`, drives rustc to a linked bin, and
// (via the helper) the bin is run to check the forwarded exit code.
#[test]
fn wrapper_drives_a_valen_binary_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  // The rlib the program imports (a bare std-only crate built the canonical way, as Pearl's cargo would).
  let tiny_rs = out_dir.join("tiny.rs");
  fs::write(&tiny_rs, "pub fn seven() -> i32 { 7 }\n").expect("could not write tiny.rs");
  build_dep_rlib("tiny", &tiny_rs, out_dir);
  let rlib = out_dir.join("libtiny.rlib");

  let exit = wrapper_run_binary(
    out_dir,
    TINY_VALE,
    vec![format!("--extern=tiny={}", rlib.display()), format!("-L{}", out_dir.display())],
  );
  assert_eq!(exit, 7);
}

// Valen operators (`+`, `==`, …) are library functions (src/builtins/resources/arith.vale), resolved only
// when the builtins package is compiled in. Driving `+` and `==` through an `if` — the shape NobiliaV's
// driver needs — proves the wrapper links the builtins and the `__vbi_*` intrinsics lower through the
// interop backend, with no rust import.
#[test]
fn wrapper_drives_int_operators_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let exit = wrapper_run_binary(
    out.path(),
    "exported func main() int { sum = 3 + 4; if sum == 7 { 7 } else { 0 } }",
    vec![],
  );
  assert_eq!(exit, 7);
}

// `!=` on ints — the one operator NobiliaV's driver needs that `==` didn't cover. `x != y` bare-uses the
// locals, hitting the generic `!=<T>`'s `&T` params. Guards that `!=<T>` is a builtin (it lived only in
// the stdlib before).
#[test]
fn wrapper_drives_int_not_equal_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let exit = wrapper_run_binary(
    out.path(),
    "exported func main() int { x = 3; y = 4; if x != y { 7 } else { 0 } }",
    vec![],
  );
  assert_eq!(exit, 7);
}

// Pearl's real scenario through the wrapper: a Valen program imports an item the named crate only
// *re-exports* (`greeter` does `pub use helper::seven`), the canonical crate (`helper`) being a separate
// dependency. Linked with the explicit `--extern greeter=<rlib>` cargo hands the wrapper + `-L
// dependency=<deps>`. Proves the generated `pub use` re-export and that the canonical (`helper`) symbol
// resolves from `-L dependency` alone — i.e. one `--extern`, not three.
#[test]
fn wrapper_links_a_cargo_crate_through_a_pub_use_re_export() {
  let build = TempDir::new().expect("could not create build dir");
  let deps_dir = build_reexport_crate(build.path());
  let greeter_rlib = find_rlib(&deps_dir, "greeter");

  let out = TempDir::new().expect("could not create scratch dir");
  let exit = wrapper_run_binary(
    out.path(),
    REEXPORT_VALE,
    vec![
      format!("--extern=greeter={}", greeter_rlib.display()),
      format!("-Ldependency={}", deps_dir.display()),
    ],
  );
  assert_eq!(exit, 7);
}

// The combined reverse + forward `&mut` shape, end to end through the wrapper — the exact two walls
// NobiliaV's driver hits the instant the reverse stub compiles, in one program:
//   - REVERSE: a Vale struct `impl`s a Rust trait whose method takes `&mut self` + a `&mut Window` (an
//     opaque imported struct) inbound; the override calls `w.push()`, a `&mut self` window method.
//   - FORWARD: `main` hands its own Vale struct to a Rust `fn run<C>(&mut self, cb: &mut C)` by exclusive
//     ref, and calls that `&mut self` method on an owned opaque `Window` local.
// The generated consumer stub must render `&mut self`/`&mut Window` (slice 1) or rustc rejects the impl
// with E0053; then the whole thing links and runs → 7. Proves both `&mut` directions cross the boundary
// together, not just the reverse stub in isolation.
#[test]
fn wrapper_drives_a_reverse_and_forward_mut_callback_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  // The Rust facade: an opaque `Window` with a `&mut self` method (`push`) and a generic `&mut self`
  // caller (`run`) that owns a `Frame` and calls the callback's `&mut self` `on_tick` with `&mut self`
  // (the window) inbound — NobiliaV's `main_loop`/`on_tick` shape after its interior-mutability removal.
  let noblike_rs = out_dir.join("noblike.rs");
  fs::write(
    &noblike_rs,
    "pub struct Window { pub ticks: i32 }\n\
     pub struct Frame {}\n\
     pub trait MainLoop {\n\
     \x20   fn on_tick(&mut self, w: &mut Window, input: &Frame);\n\
     }\n\
     impl Window {\n\
     \x20   pub fn new() -> Window { Window { ticks: 0 } }\n\
     \x20   pub fn push(&mut self) { self.ticks += 1; }\n\
     \x20   pub fn run<C: MainLoop>(&mut self, cb: &mut C) -> i32 {\n\
     \x20       let frame = Frame {};\n\
     \x20       cb.on_tick(self, &frame);\n\
     \x20       7\n\
     \x20   }\n\
     }\n",
  )
  .expect("could not write noblike.rs");
  build_dep_rlib("noblike", &noblike_rs, out_dir);
  let rlib = out_dir.join("libnoblike.rlib");

  let vale = r#"
import rust.noblike.Window;
import rust.noblike.Frame;
import rust.noblike.MainLoop;
struct MyCb { }
impl MainLoop for MyCb;
func on_tick<r', s'>(self &MyCb in s, w &Window in r, input &Frame) mut(r) mut(s) {
  w.push();
}
exported func main() int {
  w = Window.new();
  mmlcb = MyCb();
  return w.run(&mmlcb);
}
"#;

  let exit = wrapper_run_binary(
    out_dir,
    vale,
    vec![format!("--extern=noblike={}", rlib.display()), format!("-L{}", out_dir.display())],
  );
  assert_eq!(exit, 7);
}

// Slice 2 (convo-148) — outbound arg-lowering in isolation: a generic forwarder `MyCb<F>` (holding a
// lambda functor, impl-ing an imported trait so it's projected) is handed to a Rust generic fn `touch<C>`
// that takes it by borrow and *never calls the trait method*. So this exercises only the outbound leaf
// `touch::<MyCb<Lambda>>` — its type arg `MyCb<Lambda>` must lower, with the internal lambda functor
// rewritten to `__ValeOpaque<typeid>` (arch §10.7 Case 2) — with no inbound callback (no `collect_callback`,
// that's slice 3). Before slice 2, lowering the `LambdaCitizen` arg panics in `rust_request_arg_tys`.
#[test]
fn wrapper_lowers_a_forwarder_arg_to_a_noncallback_rust_fn() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  let noblike_rs = out_dir.join("noblike.rs");
  fs::write(
    &noblike_rs,
    "pub struct Window { pub ticks: i32 }\n\
     pub trait MainLoop {\n\
     \x20   fn on_tick(&self, w: &Window);\n\
     }\n\
     pub fn touch<C: MainLoop>(_cb: &C) {}\n",
  )
  .expect("could not write noblike.rs");
  build_dep_rlib("noblike", &noblike_rs, out_dir);
  let rlib = out_dir.join("libnoblike.rlib");

  let vale = r#"
import rust.noblike.Window;
import rust.noblike.MainLoop;
import rust.noblike.touch;
struct MyCb<F> where func drop(F)void, func __call(&F, &Window)void { f F; }
impl<F> MainLoop for MyCb<F>;
func on_tick<F>(self &MyCb<F>, w &Window) {
  (&self.f)(w);
}
exported func main() int {
  cb = MyCb((w2) => { });
  touch(&cb);
  return 7;
}
"#;

  let exit = wrapper_run_binary(
    out_dir,
    vale,
    vec![format!("--extern=noblike={}", rlib.display()), format!("-L{}", out_dir.display())],
  );
  assert_eq!(exit, 7);
}

// PROBE (convo-148): the stateless-lambda forwarder, end to end through the wrapper. A generic forwarder
// `MyCb<F>` holds a *stateless* lambda functor and implements an imported `&self` trait; `main` hands
// `MyCb((w)=>{...})` to a generic Rust `fn run<C>(&self, cb: &C)`. The functor is a ZST, so the opaque-ZST
// stub projection is correct here. This test drives it to find the real next wall past stub-gen (likely
// the functor type arg not being resolvable / `collect_callback` empty-subs); its failure IS the finding.
#[test]
fn wrapper_drives_a_stateless_lambda_forwarder_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  let noblike_rs = out_dir.join("noblike.rs");
  fs::write(
    &noblike_rs,
    "pub struct Window { pub ticks: i32 }\n\
     pub trait MainLoop {\n\
     \x20   fn on_tick(&self, w: &Window);\n\
     }\n\
     impl Window {\n\
     \x20   pub fn new() -> Window { Window { ticks: 0 } }\n\
     \x20   pub fn poke(&self) {}\n\
     \x20   pub fn run<C: MainLoop>(&self, cb: &C) -> i32 {\n\
     \x20       cb.on_tick(self);\n\
     \x20       7\n\
     \x20   }\n\
     }\n",
  )
  .expect("could not write noblike.rs");
  build_dep_rlib("noblike", &noblike_rs, out_dir);
  let rlib = out_dir.join("libnoblike.rlib");

  let vale = r#"
import rust.noblike.Window;
import rust.noblike.MainLoop;
struct MyCb<F> where func drop(F)void, func __call(&F, &Window)void { f F; }
impl<F> MainLoop for MyCb<F>;
func on_tick<F>(self &MyCb<F>, w &Window) {
  (&self.f)(w);
}
exported func main() int {
  w = Window.new();
  cb = MyCb((w2) => { w2.poke(); });
  return w.run(&cb);
}
"#;

  let exit = wrapper_run_binary(
    out_dir,
    vale,
    vec![format!("--extern=noblike={}", rlib.display()), format!("-L{}", out_dir.display())],
  );
  assert_eq!(exit, 7);
}

// Slice 5: the AUTO-GENERATED forwarder, end to end. Same as the stateless-lambda test above, but with
// NO hand-written `struct MyCb / impl MainLoop / func on_tick` — the callsite writes
// `MainLoop((w2) => { w2.poke(); })` and the compiler synthesizes the anon substruct, projects its
// `pub struct MainLoop__anon<F> + impl` through the HinputsT-driven pass-2 stub, and drives it to exit 7.
// This is the whole endeavor's goal (NobiliaV writes the lambda, not the forwarder). Requires the
// two-pass driver (the anon substruct exists only post-typing, so its stub is pass-2).
#[test]
fn wrapper_drives_an_auto_generated_forwarder_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  let noblike_rs = out_dir.join("noblike.rs");
  fs::write(
    &noblike_rs,
    "pub struct Window { pub ticks: i32 }\n\
     pub trait MainLoop {\n\
     \x20   fn on_tick(&self, w: &Window);\n\
     }\n\
     impl Window {\n\
     \x20   pub fn new() -> Window { Window { ticks: 0 } }\n\
     \x20   pub fn poke(&self) {}\n\
     \x20   pub fn run<C: MainLoop>(&self, cb: &C) -> i32 {\n\
     \x20       cb.on_tick(self);\n\
     \x20       7\n\
     \x20   }\n\
     }\n",
  )
  .expect("could not write noblike.rs");
  build_dep_rlib("noblike", &noblike_rs, out_dir);
  let rlib = out_dir.join("libnoblike.rlib");

  let vale = r#"
import rust.noblike.Window;
import rust.noblike.MainLoop;
exported func main() int {
  w = Window.new();
  cb = MainLoop((w2) => { w2.poke(); });
  return w.run(&cb);
}
"#;

  let exit = wrapper_run_binary(
    out_dir,
    vale,
    vec![format!("--extern=noblike={}", rlib.display()), format!("-L{}", out_dir.display())],
  );
  assert_eq!(exit, 7);
}

// Build a `noblike` rlib in `out_dir` shaped like NobiliaV's window: a `&mut self` trait method, a
// `&mut self` window method (`push`), and a generic `&mut self` caller whose return exposes whether
// the callback's churn actually happened (`ticks + 6`, so one `push` → 7).
fn build_churning_noblike_rlib(out_dir: &Path) -> Vec<String> {
  let noblike_rs = out_dir.join("noblike.rs");
  fs::write(
    &noblike_rs,
    "pub struct Window { pub ticks: i32 }\n\
     pub struct Frame {}\n\
     pub trait MainLoop {\n\
     \x20   fn on_tick(&mut self, w: &mut Window, input: &Frame);\n\
     }\n\
     impl Window {\n\
     \x20   pub fn new() -> Window { Window { ticks: 0 } }\n\
     \x20   pub fn push(&mut self) { self.ticks += 1; }\n\
     \x20   pub fn run<C: MainLoop>(&mut self, cb: &mut C) -> i32 {\n\
     \x20       let frame = Frame {};\n\
     \x20       cb.on_tick(self, &frame);\n\
     \x20       self.ticks + 6\n\
     \x20   }\n\
     }\n",
  )
  .expect("could not write noblike.rs");
  build_dep_rlib("noblike", &noblike_rs, out_dir);
  let rlib = out_dir.join("libnoblike.rlib");
  vec![format!("--extern=noblike={}", rlib.display()), format!("-L{}", out_dir.display())]
}

// NobiliaV's target shape: a generic forwarder wrapping a closure that CHURNS the window it is handed.
// The override keeps its named regions (`mut(r) mut(s)`) so the generated stub renders `&mut`, the
// bound writes the churn as the `&Window mut` placeholder, and the closure calls a `&mut self` method.
const CHURNING_FORWARDER_VALE: &str = r#"
import rust.noblike.Window;
import rust.noblike.Frame;
import rust.noblike.MainLoop;
struct MyCb<F> where func drop(F)void, func __call(&F, &Window mut, &Frame)void { f F; }
impl<F> MainLoop for MyCb<F>;
func on_tick<F, r', s'>(self &MyCb<F> in s, w &Window in r, input &Frame) mut(r) mut(s) {
  (&self.f)(w, input);
}
exported func main() int {
  w = Window.new();
  cb = MyCb((w2, input) => { w2.push(); });
  return w.run(&cb);
}
"#;

// With the borrow checker off, the churning forwarder compiles, links, and runs: the closure's `push`
// reaches the real window through the forwarder (exit 7 = one tick + 6). This is the shape NobiliaV
// can build today with `valen build --no-borrow-check`.
#[test]
fn wrapper_with_borrow_check_off_drives_a_churning_lambda_forwarder_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let extra = build_churning_noblike_rlib(out_dir);
  let result = wrapper_drive(out_dir, CHURNING_FORWARDER_VALE, extra, /*borrow_check=*/ false)
    .expect("run_wrapper should succeed with the borrow checker off");
  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert_eq!(run_produced_binary(out_dir), 7);
}

// S2: the AUTO-GENERATED anon substruct for a `&mut`-signature trait runs → 7 with the borrow checker
// off — NobiliaV's real shape with NO hand-written `MyCb<F>` forwarder. `MainLoop((w2, input) => {
// w2.push(); })` is handed straight to a `&mut self` / `&mut Window` trait, and the closure churns the
// window. The pass-2 stub renders `&mut` (S1) so rustc accepts the impl; `--no-borrow-check` steps around
// the unbuilt churn enforcement, exactly as the hand-written churning forwarder above does. `run`'s
// `ticks + 6` return means one `push` → 7, so a 7 proves the churn reached the real window.
#[test]
fn wrapper_drives_an_auto_generated_mut_forwarder_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let extra = build_churning_noblike_rlib(out_dir);
  let vale = r#"
import rust.noblike.Window;
import rust.noblike.Frame;
import rust.noblike.MainLoop;
exported func main() int {
  w = Window.new();
  cb = MainLoop((w2, input) => { w2.push(); });
  return w.run(&cb);
}
"#;
  let result = wrapper_drive(out_dir, vale, extra, /*borrow_check=*/ false)
    .expect("run_wrapper should succeed with the borrow checker off");
  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert_eq!(run_produced_binary(out_dir), 7);
}

// The same program with the borrow checker on is rejected: the closure churns its window param and
// nothing declares that (the `mut` placeholder on the bound is not yet read by the checker). Pins the
// wall `--no-borrow-check` exists to step around.
#[test]
fn wrapper_with_borrow_check_on_rejects_the_churning_lambda_forwarder() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let extra = build_churning_noblike_rlib(out_dir);
  match wrapper_drive(out_dir, CHURNING_FORWARDER_VALE, extra, /*borrow_check=*/ true) {
    Err(err) => assert!(err.contains("BorrowCheckError"), "err:\n{err}"),
    Ok(result) => panic!(
      "the borrow checker should reject the churning closure, but rustc exited {} with firings {:?}",
      result.rustc_exit, result.firings
    ),
  }
}

// Build a `counter` rlib in `out_dir` — an opaque struct with a `&mut self` method — and return the
// `--extern`/`-L` flags that let a driven `.valen` import it.
fn build_counter_rlib(out_dir: &Path) -> Vec<String> {
  let counter_rs = out_dir.join("counter.rs");
  fs::write(
    &counter_rs,
    "pub struct Counter { pub n: i32 }\n\
     impl Counter {\n\
     \x20   pub fn new() -> Counter { Counter { n: 6 } }\n\
     \x20   pub fn bump(&mut self) { self.n += 1; }\n\
     \x20   pub fn get(&self) -> i32 { self.n }\n\
     }\n",
  )
  .expect("could not write counter.rs");
  build_dep_rlib("counter", &counter_rs, out_dir);
  let rlib = out_dir.join("libcounter.rlib");
  vec![format!("--extern=counter={}", rlib.display()), format!("-L{}", out_dir.display())]
}

// A Vale function that churns one of its *parameters'* groups — calling the imported `&mut self`
// `bump` on a `&Counter in g` parameter — without declaring `mut(g)`.
const CHURN_VALE: &str = r#"
import rust.counter.Counter;
func bump_it<g'>(c &Counter in g) {
  c.bump();
}
exported func main() int {
  c = Counter.new();
  bump_it(&c);
  return c.get();
}
"#;

// With the borrow checker on, the undeclared churn is rejected by the producer gate: the importer
// gives `bump` a `mut` effect on its receiver's group, and `bump_it` declares none for `g`. The
// interop twin of producer_gate_tests' `test_undeclared_param_churn_rejected`; the wrapper surfaces
// the typing failure as an `Err`, so nothing is emitted.
#[test]
fn wrapper_rejects_an_undeclared_churn_of_an_imported_mut_method() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let extra = build_counter_rlib(out_dir);
  match wrapper_drive(out_dir, CHURN_VALE, extra, /*borrow_check=*/ true) {
    Err(err) => assert!(err.contains("BorrowCheckError"), "err:\n{err}"),
    Ok(result) => panic!(
      "the borrow checker should reject the undeclared churn, but rustc exited {} with firings {:?}",
      result.rustc_exit, result.firings
    ),
  }
}

// The switch NobiliaV needs: with the borrow checker off, the same program compiles, links and runs,
// so rust interop can be exercised on a program the checker is not yet happy with. The only
// difference from the test above is the flag.
#[test]
fn wrapper_with_borrow_check_off_runs_an_undeclared_churn_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let extra = build_counter_rlib(out_dir);
  let result = wrapper_drive(out_dir, CHURN_VALE, extra, /*borrow_check=*/ false)
    .expect("run_wrapper should succeed with the borrow checker off");
  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert_eq!(run_produced_binary(out_dir), 7);
}
