// The rustc-collector-driven instantiation path (Milestone M).
//
// Under rust_interop, instantiation is driven by rustc rather than by `translate_program`. rustc's
// mono collector walks the stub crate; when it reaches a Vale-defined item (a stub fn carrying
// `#[vale::emit_consumer_body]` in a crate marked with `__VALE_STUBS_MARKER`) it calls our
// `per_instance_mir` provider. The provider drives our monomorphizer for that one exported function,
// collects the Rust functions it transitively calls (the "leaves"), and hands rustc a synthetic MIR
// body: a `ReifyFnPointer` cast per Rust leaf (so the collector queues them) plus `unreachable`
// (the body never runs — our own backend swaps in the real body under the same symbol). The design
// is in `src/instantiating/instantiating-rust-interop-design.md`; the reference is Harmonious's `per_instance.rs`.
//
// This module names rustc's internals (`TyCtxt`/`Instance`/`Body`/MIR), which is why it lives under
// a `rust_interop` directory and behind the feature: the crate root only links the rustc crates
// there.

use rustc_codegen_llvm::ModuleLlvm;
use rustc_codegen_ssa::traits::ExtraModuleAllocator;
use rustc_hir::Safety;
use rustc_index::IndexVec;
use rustc_middle::mir::{
  BasicBlock, BasicBlockData, Body, CastKind, ClearCrossCrate, Const, ConstOperand, CoercionSource,
  Local, LocalDecl, MirSource, Operand, Place, Rvalue, SourceInfo, SourceScopeData, Statement,
  StatementKind, Terminator, TerminatorKind,
};
use rustc_middle::middle::deduced_param_attrs::DeducedParamAttrs;
use rustc_middle::mir::mono::{CodegenUnit, MonoItemPartitions};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{self, Instance, Ty, TyCtxt};
use rustc_middle::util::Providers;
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::Symbol;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::ptr::null;
use std::sync::OnceLock;

use crate::backend_ffi::metal_cache::MetalCache;
use crate::backend_ffi::metal_lowerer::populate_metal_cache;
use crate::backend_ffi::{backend_compile_program_into_safe, BackendCompileOptions};
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::names::{IInterfaceTemplateNameI, INameI, IStructTemplateNameI, IdI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::types::KindIT;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::ast::ast::{FunctionExportI, FunctionExternI};
use crate::instantiating::instantiator::{InstantiatedOutputsI, InstantiatorI};
use crate::keywords::Keywords;
use crate::scout_arena::ScoutArena;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::rust_interop::tyctxt_oracle::resolve_crate_qualified_path;
use crate::typing::typing_interner::TypingInterner;

// The instantiator state the provider drives, reached through a scoped raw pointer rather than a
// `'static` global (Vale has a no-`'static` policy). The state — arenas, interners, the owned
// `HinputsT`, and the accumulating `monouts` — lives as ordinary stack locals in the frame that calls
// `run_compiler`, which encloses both `after_expansion` (where it is armed) and codegen (where the
// provider fires), so it outlives every provider call. This is rustc's own `ty::tls` idiom.
//
// The interior mutability lets the driver build the struct up front (with `&` to slots) and fill
// `hinputs` in `after_expansion`; `monouts` accumulates across calls so a helper shared by two
// exports instantiates once.
pub struct DriverState<'s, 'ctx, 't, 'i> {
  pub opts: &'ctx GlobalOptions,
  pub interner: &'ctx InstantiatingInterner<'s, 'i>,
  pub typing_interner: &'ctx TypingInterner<'s, 't>,
  pub scout_arena: &'ctx ScoutArena<'s>,
  pub keywords: &'ctx Keywords<'s>,
  pub hinputs: &'ctx RefCell<Option<HinputsT<'s, 't>>>,
  pub monouts: &'ctx RefCell<InstantiatedOutputsI<'s, 't, 'i>>,
  /// The `FunctionExportI` for each export the collector actually walked (demand-driven), retained so
  /// the single-instantiation emit path can hand them to `assemble_hinputs` — the driven `monouts` is
  /// the whole program the backend lowers, so nothing seeds exports eagerly.
  pub function_exports: &'ctx RefCell<Vec<FunctionExportI<'s, 'i>>>,
  /// The rustc-mangled symbol of the `__vale_main` stub instance, captured when the collector walks it.
  /// The backend emits the entry (`makeEntryFunction`) under this name — single-symbol (arch §5.2) — so
  /// the stub's `fn main`, which calls the Rust name `__vale_main`, resolves to Vale's real body rather
  /// than rustc's `unreachable!()` placeholder (which the partition filter removes). `None` = no entry.
  pub entry_symbol: &'ctx RefCell<Option<String>>,
  /// Per-run log of what the provider did, keyed by nothing — one line per Vale item it fired on.
  /// Lives here (per driven run) rather than in a global so parallel driven tests never race.
  pub firings: &'ctx RefCell<Vec<String>>,
  /// Whether the `fill_extra_modules` hook should actually lower + emit the Vale bodies into rustc's
  /// borrowed module (Stage 2+), or just record that it fired (Stage 1 / the Milestone-M driven tests
  /// that assert only on resolution, not emission). Off keeps those tests off the backend path.
  pub emit_backend: bool,
}

impl<'s, 'ctx, 't, 'i> DriverState<'s, 'ctx, 't, 'i> {
  /// Drive the monomorphizer for the one exported function named `export_name`: seed it, drain the
  /// queue (instantiating Vale functions, filtering Rust ones into `rust_instantiation_requests`),
  /// and return the Rust requests newly collected by this call. `monouts` persists, so a request
  /// already collected by an earlier export is not returned again (its Rust dep is already queued).
  fn collect_new_rust_requests<'tcx>(
    &self,
    tcx: TyCtxt<'tcx>,
    export_name: &str,
  ) -> Vec<ResolvedRequest<'tcx>> {
    let hinputs_ref = self.hinputs.borrow();
    let hinputs = match hinputs_ref.as_ref() {
      Some(h) => h,
      None => return Vec::new(),
    };
    let export = match hinputs.function_exports.iter().find(|e| e.exported_name.0 == export_name) {
      Some(e) => e,
      None => return Vec::new(),
    };
    let instantiator = InstantiatorI {
      opts: self.opts,
      interner: self.interner,
      typing_interner: self.typing_interner,
      scout_arena: self.scout_arena,
      keywords: self.keywords,
      hinputs,
    };
    let mut monouts = self.monouts.borrow_mut();
    let before: HashSet<_> = monouts.rust_instantiation_requests.keys().copied().collect();
    // Retain the export the collector walked: the single-instantiation emit path finalizes the driven
    // `monouts` with exactly the exports demand reached (rustc memoizes per instance, so once each).
    let export_i = instantiator.instantiate_exported_function(&mut monouts, export);
    self.function_exports.borrow_mut().push(export_i);
    instantiator.drain_instantiation_queue(&mut monouts);

    // Resolve each newly-collected Rust leaf and materialize its `FunctionExternI` here — this is where
    // the leaf's real (rustc-mangled) symbol becomes known, so this is where the extern is defined.
    // Collect first (this borrows the requests map immutably), keeping each request's `PrototypeI`.
    // `tcx.symbol_name(Instance)` is a pure read of the same instance the `ReifyFnPointer` reifies, so it
    // matches the symbol rustc actually codegens the leaf under.
    let new_reqs: Vec<_> = monouts
      .rust_instantiation_requests
      .iter()
      .filter(|(id, _)| !before.contains(*id))
      .map(|(_id, proto)| (*proto, resolve_request(tcx, proto)))
      .collect();

    for (proto, req) in &new_reqs {
      if let Some((def_id, args)) = req.dep {
        let symbol = tcx.symbol_name(ty::Instance::new_raw(def_id, args)).name;
        let symbol_i: &str = self.interner.bump().alloc_str(symbol);
        // The extern is born here, complete, with rustc's real symbol — the one place it is known. This
        // is the sole registration point for Rust externs (the instantiator only records the request).
        monouts.function_externs.push(FunctionExternI {
          prototype: *proto,
          num_inherited_generic_parameters: 0,
          link_name: symbol_i,
        });
      }
    }

    new_reqs.into_iter().map(|(_, req)| req).collect()
  }
}

/// One collected Rust callee request, resolved to what the provider needs: a rustc `(DefId, args)`
/// to reify (`None` if it could not be resolved), plus a human-readable log line. Owned/`'tcx`-bound,
/// so it outlives the `monouts` borrow it was read from.
struct ResolvedRequest<'tcx> {
  log: String,
  dep: Option<(DefId, ty::GenericArgsRef<'tcx>)>,
}

/// Resolve one Rust callee request (a synthesized-extern `PrototypeI`) to a rustc `(DefId, args)`.
/// A free function resolves by its crate-qualified path; a method resolves through its receiver
/// type's inherent impls (the receiver is the request's first parameter).
fn resolve_request<'tcx>(tcx: TyCtxt<'tcx>, proto: &PrototypeI) -> ResolvedRequest<'tcx> {
  let path = match rust_request_path(proto) {
    Some(p) => p,
    None => return ResolvedRequest { log: "<non-function request>".to_string(), dep: None },
  };
  let own_arg_tys = match rust_request_arg_tys(tcx, proto) {
    Some(a) => a,
    None => return ResolvedRequest { log: format!("{path} => ARGS-UNCONVERTIBLE"), dep: None },
  };

  // A free function: the whole path resolves to an item in the crate graph.
  if let Some((def_id, _kind)) = resolve_crate_qualified_path(tcx, &path) {
    let args = build_generic_args(tcx, def_id, &own_arg_tys);
    return ResolvedRequest {
      log: format!("{path}{own_arg_tys:?} => {}", tcx.def_path_str(def_id)),
      dep: Some((def_id, args)),
    };
  }

  // Otherwise a method: resolve it through the receiver type (the first parameter).
  if let Some((def_id, args)) = resolve_method_request(tcx, proto, &own_arg_tys) {
    return ResolvedRequest {
      log: format!("{path} => {} (method)", tcx.def_path_str(def_id)),
      dep: Some((def_id, args)),
    };
  }

  // Otherwise an associated function (e.g. `Type::new`): no receiver, but its owner type is named in
  // the id's init path.
  if let Some((def_id, args)) = resolve_assoc_fn_request(tcx, proto, &own_arg_tys) {
    return ResolvedRequest {
      log: format!("{path} => {} (assoc)", tcx.def_path_str(def_id)),
      dep: Some((def_id, args)),
    };
  }

  // A synthesized drop of an imported type: it has no Rust `Drop` to resolve to, so reify a generic
  // `__vale_drop<T>` shim (arch §15.7 "drop is a function"). Checked after free-fn/method so a real
  // method named `drop` still resolves normally.
  if let Some((def_id, args)) = resolve_drop_request(tcx, proto) {
    return ResolvedRequest {
      log: format!("{path} => {} (drop shim)", tcx.def_path_str(def_id)),
      dep: Some((def_id, args)),
    };
  }

  ResolvedRequest { log: format!("{path} => UNRESOLVED"), dep: None }
}

/// Resolve a synthesized drop request to `__vale_drop::<T>`, where `T` is the dropped type (the
/// request's first parameter). Recognized by the function name `drop`; returns `None` for anything
/// else. `__vale_drop` is a generic shim in the stub crate that calls `ptr::drop_in_place`.
fn resolve_drop_request<'tcx>(
  tcx: TyCtxt<'tcx>,
  proto: &PrototypeI,
) -> Option<(DefId, ty::GenericArgsRef<'tcx>)> {
  let (name, _, parameters) = request_name_parts(proto)?;
  if name.as_str() != "drop" {
    return None;
  }
  let dropped_ty = kind_to_rustc_ty(tcx, parameters.first()?)?;
  let drop_def_id = resolve_local_fn(tcx, "__vale_drop")?;
  Some((drop_def_id, build_generic_args(tcx, drop_def_id, &[dropped_ty])))
}

/// Find a free function defined in the crate under compilation (the stub) by name. Used for the
/// `__vale_drop` shim, which lives in the stub rather than a dependency, so `resolve_crate_qualified_path`
/// (which walks only loaded dependency crates) cannot see it.
fn resolve_local_fn(tcx: TyCtxt<'_>, name: &str) -> Option<DefId> {
  tcx
    .module_children_local(rustc_hir::def_id::CRATE_DEF_ID)
    .iter()
    .find(|c| c.ident.name.as_str() == name)
    .and_then(|c| match c.res {
      rustc_hir::def::Res::Def(rustc_hir::def::DefKind::Fn, def_id) => Some(def_id),
      _ => None,
    })
}

/// Resolve a method request through its receiver type's inherent impls. The receiver is the request's
/// first parameter; the method's generic args are the receiver's type args followed by the method's
/// own (matching rustc's parent-inclusive generic order).
fn resolve_method_request<'tcx>(
  tcx: TyCtxt<'tcx>,
  proto: &PrototypeI,
  own_arg_tys: &[Ty<'tcx>],
) -> Option<(DefId, ty::GenericArgsRef<'tcx>)> {
  let (method_name, _, parameters) = request_name_parts(proto)?;
  let (owner_def_id, receiver_arg_tys) = receiver_owner(tcx, parameters.first()?)?;
  let method_def_id = resolve_inherent_method(tcx, owner_def_id, method_name.as_str())?;
  let mut method_arg_tys = receiver_arg_tys;
  method_arg_tys.extend_from_slice(own_arg_tys);
  Some((method_def_id, build_generic_args(tcx, method_def_id, &method_arg_tys)))
}

/// Resolve an associated function (e.g. `Domino::new`) through its owner type. The owner is named in
/// the request id's init path (a struct/interface template segment); the function is then found in the
/// owner's inherent impls. Generic-owner args (e.g. `Boxed<int>::new`) are not reconstructed here yet.
fn resolve_assoc_fn_request<'tcx>(
  tcx: TyCtxt<'tcx>,
  proto: &PrototypeI,
  own_arg_tys: &[Ty<'tcx>],
) -> Option<(DefId, ty::GenericArgsRef<'tcx>)> {
  let (fn_name, _, _) = request_name_parts(proto)?;
  let fn_name = fn_name.as_str();
  let owner_human = proto.id.init_steps.iter().rev().find_map(|step| match step {
    INameI::StructTemplate(t) => Some(t.human_name.as_str()),
    INameI::InterfaceTemplate(t) => Some(t.human_namee.as_str()),
    _ => None,
  })?;
  let mut segments: Vec<&str> =
    proto.id.package_coord.packages.as_slice().iter().map(|s| s.as_str()).collect();
  segments.push(owner_human);
  let (owner_def_id, _kind) = resolve_crate_qualified_path(tcx, &segments.join("."))?;
  let fn_def_id = resolve_inherent_method(tcx, owner_def_id, fn_name)?;
  Some((fn_def_id, build_generic_args(tcx, fn_def_id, own_arg_tys)))
}

/// The owning type of a method receiver: its `DefId` and its own type arguments. Peels any reference
/// wrappers first, so a `&self`/`&mut self` receiver (a borrow-wrapped citizen) resolves the same as
/// a by-value one.
fn receiver_owner<'tcx>(tcx: TyCtxt<'tcx>, kind: &KindIT) -> Option<(DefId, Vec<Ty<'tcx>>)> {
  let mut current = kind;
  loop {
    current = match current {
      KindIT::BorrowRefIT(r) => &r.inner,
      KindIT::OwnRefIT(r) => &r.inner,
      KindIT::ShareRefIT(r) => &r.inner,
      KindIT::WeakRefIT(r) => &r.inner,
      KindIT::StructIT(s) => return citizen_def_id_and_args(tcx, &s.id),
      KindIT::InterfaceIT(i) => return citizen_def_id_and_args(tcx, &i.id),
      _ => return None,
    };
  }
}

/// Find an inherent method by name on a type, returning its `DefId`. Mirrors the oracle's
/// `inherent_impls` → `associated_items` walk.
fn resolve_inherent_method(tcx: TyCtxt<'_>, owner_def_id: DefId, method_name: &str) -> Option<DefId> {
  for impl_def_id in tcx.inherent_impls(owner_def_id).iter() {
    for assoc in tcx.associated_items(*impl_def_id).in_definition_order() {
      if assoc.as_tag() == ty::AssocTag::Fn && assoc.name().to_string() == method_name {
        return Some(assoc.def_id);
      }
    }
  }
  None
}

/// The crate-qualified dotted path (`crate.module….item`) of a Rust callee request, reconstructed
/// from its instantiated id: the package coordinate holds the crate and module segments, and the
/// function name's `human_name` is the item. `resolve_crate_qualified_path` turns this back into a
/// rustc `DefId`. Returns `None` for a request whose name is not a plain function (e.g. a method),
/// which the non-generic free-function bridge does not handle yet.
fn rust_request_path(proto: &PrototypeI) -> Option<String> {
  let (human_name, _, _) = request_name_parts(proto)?;
  let mut segments: Vec<&str> =
    proto.id.package_coord.packages.as_slice().iter().map(|s| s.as_str()).collect();
  segments.push(human_name.as_str());
  Some(segments.join("."))
}

/// The name, generic type-args, and parameter types of a Rust callee request. The request is the
/// synthesized *extern* prototype recorded at the `ExternFunctionCall` node, so its name is normally
/// `INameI::ExternFunction`; a plain `FunctionNameIX` is accepted too. `None` for any other shape.
fn request_name_parts<'s, 'i>(
  proto: &PrototypeI<'s, 'i>,
) -> Option<(StrI<'s>, &'i [ITemplataI<'s, 'i>], &'i [KindIT<'s, 'i>])> {
  match proto.id.local_name {
    INameI::ExternFunction(e) => Some((e.human_name, e.template_args, e.parameters)),
    INameI::FunctionNameIX(fnx) => {
      Some((fnx.template.human_name, fnx.template_args, fnx.parameters))
    }
    _ => None,
  }
}

/// The rustc type arguments of a Rust callee request, converted from the Vale template args on its
/// instantiated name. `None` if the request is not a plain function, or if any type arg is one we
/// cannot yet lower (so the caller drops the request rather than mis-instantiate it).
fn rust_request_arg_tys<'tcx>(tcx: TyCtxt<'tcx>, proto: &PrototypeI) -> Option<Vec<Ty<'tcx>>> {
  let (_, template_args, _) = request_name_parts(proto)?;
  template_args.iter().map(|t| templata_to_rustc_ty(tcx, t)).collect()
}

/// Convert one Vale type templata to a rustc `Ty`. Only type (`Kind`) templatas participate in a
/// callee's generic args; other templata kinds (function/impl bounds, integer/bool *values*) return
/// `None`. Widens as more type shapes are needed.
fn templata_to_rustc_ty<'tcx>(tcx: TyCtxt<'tcx>, templata: &ITemplataI) -> Option<Ty<'tcx>> {
  match templata {
    ITemplataI::Kind(k) => kind_to_rustc_ty(tcx, &k.kind),
    _ => None,
  }
}

/// Convert a Vale instantiated kind to a rustc `Ty`. Primitives map to their rustc counterparts;
/// other kinds (Rust-backed structs/enums, arrays, refs) are not lowered yet and return `None`.
fn kind_to_rustc_ty<'tcx>(tcx: TyCtxt<'tcx>, kind: &KindIT) -> Option<Ty<'tcx>> {
  match kind {
    KindIT::IntIT(i) => match i.bits {
      32 => Some(tcx.types.i32),
      64 => Some(tcx.types.i64),
      _ => None,
    },
    KindIT::BoolIT(_) => Some(tcx.types.bool),
    KindIT::USizeIT(_) => Some(tcx.types.usize),
    KindIT::StructIT(s) => citizen_to_rustc_ty(tcx, &s.id),
    KindIT::InterfaceIT(i) => citizen_to_rustc_ty(tcx, &i.id),
    _ => None,
  }
}

/// Lower a Rust-backed citizen (struct or enum) kind to its rustc `Adt` `Ty`: reconstruct the type's
/// crate-qualified path from its id, resolve the `DefId`, convert the citizen's own type arguments
/// (recursively, so `Holder<int>` lowers through this same path), and build the `Adt`.
fn citizen_to_rustc_ty<'tcx>(tcx: TyCtxt<'tcx>, id: &IdI) -> Option<Ty<'tcx>> {
  let (def_id, arg_tys) = citizen_def_id_and_args(tcx, id)?;
  let args = build_generic_args(tcx, def_id, &arg_tys);
  Some(Ty::new_adt(tcx, tcx.adt_def(def_id), args))
}

/// A Rust-backed citizen's rustc `DefId` and its converted type arguments, from its instantiated id.
/// The package coordinate holds the crate + module path; the citizen name holds the item name and its
/// own template args.
fn citizen_def_id_and_args<'tcx>(
  tcx: TyCtxt<'tcx>,
  id: &IdI,
) -> Option<(DefId, Vec<Ty<'tcx>>)> {
  let (human_name, template_args) = match id.local_name {
    INameI::StructName(sn) => match sn.template {
      IStructTemplateNameI::StructTemplate(t) => (t.human_name.as_str(), sn.template_args),
      _ => return None,
    },
    INameI::InterfaceName(inm) => {
      let IInterfaceTemplateNameI::InterfaceTemplate(t) = inm.template;
      (t.human_namee.as_str(), inm.template_args)
    }
    _ => return None,
  };
  let mut segments: Vec<&str> =
    id.package_coord.packages.as_slice().iter().map(|s| s.as_str()).collect();
  segments.push(human_name);
  let (def_id, _kind) = resolve_crate_qualified_path(tcx, &segments.join("."))?;
  let arg_tys: Vec<Ty<'tcx>> =
    template_args.iter().map(|t| templata_to_rustc_ty(tcx, t)).collect::<Option<_>>()?;
  Some((def_id, arg_tys))
}

/// The full rustc `GenericArgs` for a Rust callee, filling type slots from `arg_tys` (already
/// converted, in declaration order) and lifetime slots with `re_erased` (borrowck ran on the stub;
/// lifetimes are irrelevant post-borrowck). A non-generic callee has no slots, so `arg_tys` is empty
/// and the callback never fires. Panics on a const-generic slot or a type-slot shortfall — both are
/// "not supported yet" rather than something to guess.
fn build_generic_args<'tcx>(
  tcx: TyCtxt<'tcx>,
  def_id: DefId,
  arg_tys: &[Ty<'tcx>],
) -> ty::GenericArgsRef<'tcx> {
  let mut types = arg_tys.iter().copied();
  ty::GenericArgs::for_item(tcx, def_id, |param, _| match param.kind {
    ty::GenericParamDefKind::Lifetime => tcx.lifetimes.re_erased.into(),
    ty::GenericParamDefKind::Type { .. } => types
      .next()
      .unwrap_or_else(|| panic!("too few type args for Rust callee {def_id:?}"))
      .into(),
    ty::GenericParamDefKind::Const { .. } => {
      panic!("const-generic Rust callee args not supported: {def_id:?}")
    }
  })
}

thread_local! {
  // A raw pointer to the current `DriverState`, armed for the duration of one driven `run_compiler`.
  // Null when no driven compile is active (so the provider returns None and rustc uses its defaults).
  // Thread-local because rustc runs the compilation — `after_expansion` and the provider both — on
  // one spawned thread; single-threaded, so no lock (add one under the parallel frontend).
  static DRIVER_STATE: Cell<*const ()> = const { Cell::new(null()) };
}

/// Arm the scoped pointer at `state` for the current thread. Call from `after_expansion`, on the
/// rustc thread the provider will fire on. `state` must outlive every subsequent provider call this
/// compile makes (it lives in the `run_compiler`-calling frame, which does).
pub fn arm_driver_state(state: *const ()) {
  DRIVER_STATE.with(|c| c.set(state));
}

/// Disarm the scoped pointer. Call after `run_compiler` returns.
pub fn disarm_driver_state() {
  DRIVER_STATE.with(|c| c.set(null()));
}

/// rustc's default `collect_and_partition_mono_items` / `deduced_param_attrs`, saved before we override
/// them so the overrides can delegate for non-Vale items. Set once per process in `vale_override_queries`.
static DEFAULT_COLLECT_AND_PARTITION: OnceLock<
  for<'tcx> fn(TyCtxt<'tcx>, ()) -> MonoItemPartitions<'tcx>,
> = OnceLock::new();
static DEFAULT_DEDUCED_PARAM_ATTRS: OnceLock<
  for<'tcx> fn(TyCtxt<'tcx>, LocalDefId) -> &'tcx [DeducedParamAttrs],
> = OnceLock::new();

/// The `override_queries` hook: a bare `fn` (rustc query providers cannot capture state), installed
/// from the driver's `config()`.
///
/// - `per_instance_mir` → our provider (drives Vale's instantiator).
/// - `collect_and_partition_mono_items` → strips Vale's `#[vale::emit_consumer_body]` stub bodies from
///   rustc's codegen, because Vale emits the real bodies under the *same* rustc-mangled names via
///   `fill_extra_modules` (single-symbol, arch §5.2); without this, rustc's `unreachable!()` placeholder
///   and Vale's body collide as a duplicate symbol at link.
/// - `deduced_param_attrs` → `&[]` for those same items, so rustc infers no `readonly`/`captures(none)`
///   from the `unreachable!()` body (which would be silent UB against Vale's real body; arch §22.4).
///
/// A non-Vale crate is byte-identical: the overrides only diverge for `is_vale_codegen_target` items,
/// which exist solely in stub crates.
pub fn vale_override_queries(_session: &rustc_session::Session, providers: &mut Providers) {
  providers.queries.per_instance_mir = lang_per_instance_mir;
  let _ = DEFAULT_COLLECT_AND_PARTITION.set(providers.queries.collect_and_partition_mono_items);
  let _ = DEFAULT_DEDUCED_PARAM_ATTRS.set(providers.queries.deduced_param_attrs);
  providers.queries.collect_and_partition_mono_items = lang_collect_and_partition_mono_items;
  providers.queries.deduced_param_attrs = lang_deduced_param_attrs;
}

/// Rebuild rustc's CGUs with Vale's stub-body items removed, so rustc emits no `.o` symbol for a body
/// Vale itself emits under the same mangled name. Delegates to the saved default partitioner, then
/// drops `is_vale_codegen_target` items from each CGU (leaving `all_mono_items` untouched — downstream
/// queries inspect it). Mirrors Harmonious's `lang_collect_and_partition_mono_items` (arch §5.3).
fn lang_collect_and_partition_mono_items<'tcx>(
  tcx: TyCtxt<'tcx>,
  key: (),
) -> MonoItemPartitions<'tcx> {
  let upstream = DEFAULT_COLLECT_AND_PARTITION
    .get()
    .expect("default collect_and_partition_mono_items not saved");
  let MonoItemPartitions { codegen_units: upstream_cgus, all_mono_items: reachable, .. } =
    upstream(tcx, key);

  let mut filtered_cgus: Vec<CodegenUnit<'tcx>> = Vec::with_capacity(upstream_cgus.len());
  for cgu in upstream_cgus.iter() {
    let mut new_cgu = CodegenUnit::new(cgu.name());
    for (&mono_item, &data) in cgu.items() {
      if is_vale_codegen_target(tcx, mono_item.def_id()) {
        continue;
      }
      new_cgu.items_mut().insert(mono_item, data);
    }
    if cgu.is_primary() {
      new_cgu.make_primary();
    }
    if cgu.is_code_coverage_dead_code_cgu() {
      new_cgu.make_code_coverage_dead_code_cgu();
    }
    new_cgu.compute_size_estimate();
    filtered_cgus.push(new_cgu);
  }

  MonoItemPartitions {
    codegen_units: tcx.arena.alloc_from_iter(filtered_cgus),
    all_mono_items: reachable,
  }
}

/// Claim no deduced param attrs for a Vale stub item: its `unreachable!()` MIR touches no params, so
/// rustc would infer `readonly`/`captures(none)` and stamp them at every call site — a lie against
/// Vale's real body. `&[]` is the conservative safe default. Delegates otherwise (arch §22.4).
fn lang_deduced_param_attrs<'tcx>(
  tcx: TyCtxt<'tcx>,
  def_id: LocalDefId,
) -> &'tcx [DeducedParamAttrs] {
  if is_vale_codegen_target(tcx, def_id.to_def_id()) {
    return &[];
  }
  let default = DEFAULT_DEDUCED_PARAM_ATTRS.get().expect("default deduced_param_attrs not saved");
  default(tcx, def_id)
}

/// Is this item one whose body Vale supplies? Gate: it carries `#[vale::emit_consumer_body]`. (The
/// full design also checks the defining crate carries `__VALE_STUBS_MARKER`; the per-item attribute
/// is enough to identify our stub fns in the driven test.)
fn is_vale_codegen_target(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
  tcx.has_attrs_with_path(
    def_id,
    &[Symbol::intern("vale"), Symbol::intern("emit_consumer_body")],
  )
}

/// The `per_instance_mir` provider. rustc's mono collector calls this for every `Instance` it walks;
/// we answer `Some(synthetic_body)` for Vale items and `None` for everything else (so the collector
/// falls through to rustc's own `instance_mir`).
fn lang_per_instance_mir<'tcx>(
  tcx: TyCtxt<'tcx>,
  instance: Instance<'tcx>,
) -> Option<&'tcx Body<'tcx>> {
  let def_id = instance.def_id();
  if !is_vale_codegen_target(tcx, def_id) {
    return None;
  }

  // The stub root `__vale_<name>` names the Vale export `<name>` (the stub mirrors each exported
  // Vale function as a Rust fn rustc's collector can walk).
  let stub_name = tcx.item_name(def_id).to_string();
  let export_name = stub_name.strip_prefix("__vale_").unwrap_or(&stub_name).to_string();

  let state_ptr = DRIVER_STATE.with(|c| c.get());
  if state_ptr.is_null() {
    // No driven run active (the override is only installed on the driven path), so there is nothing
    // to drive. Fall through to rustc's own MIR.
    return None;
  }
  // SAFETY: the `DriverState` lives in the `run_compiler`-calling frame, which outlives every
  // provider call; we only borrow it for the duration of this call and return nothing borrowing it.
  // Lifetimes are erased through the raw pointer and chosen fresh here (rustc's `ty::tls` pattern).
  // Armed on this same thread in `after_expansion`.
  let state: &DriverState = unsafe { &*(state_ptr as *const DriverState) };
  // Capture the entry symbol: `__vale_main` is the Vale binary's entry, and the backend must emit its
  // body under rustc's own mangled name for this stub instance so the stub's `fn main` (which calls the
  // Rust name `__vale_main`) links to Vale's body. A pure read of the same instance rustc codegens.
  if stub_name == "__vale_main" {
    *state.entry_symbol.borrow_mut() = Some(tcx.symbol_name(instance).name.to_string());
  }
  let requests = state.collect_new_rust_requests(tcx, &export_name);

  // Each resolved request's `(DefId, args)` becomes a `ReifyFnPointer` cast in the body, which is
  // what puts that Rust `Instance` into the collector's queue.
  let rust_deps: Vec<(DefId, ty::GenericArgsRef<'tcx>)> =
    requests.iter().filter_map(|r| r.dep).collect();
  let log = requests.iter().map(|r| r.log.as_str()).collect::<Vec<_>>().join(", ");
  state.firings.borrow_mut().push(format!("{stub_name} -> [{log}]"));

  let body = build_dependency_body(tcx, instance, &rust_deps);
  Some(tcx.arena.alloc(body))
}

/// The `fill_extra_modules` hook handler, installed via `set_fill_extra_modules_hook` from the driven
/// `config()`. rustc calls it once per `codegen_crate`, synchronously on the main thread, before
/// `start_async_codegen` (arch §5.1) — by which point every `per_instance_mir` call has run, so the
/// instantiator state reached through `DRIVER_STATE` is complete. When `emit_backend` is set it lowers
/// the Vale program and emits its bodies into a rustc-lent module (Stage 2+); otherwise it only
/// records that it fired (Stage 1 / the Milestone-M driven tests that assert on resolution, not
/// emission).
pub fn consumer_fill_modules<'tcx>(
  _tcx: TyCtxt<'tcx>,
  allocator: &ExtraModuleAllocator<ModuleLlvm>,
) {
  let state_ptr = DRIVER_STATE.with(|c| c.get());
  if state_ptr.is_null() {
    // No driven run active (the hook is a process-global `OnceLock`, but only does anything when a
    // driven `DriverState` is armed); nothing to do.
    return;
  }
  // SAFETY: same scoped-pointer contract as the provider (see `lang_per_instance_mir`) — the
  // `DriverState` lives in the `run_compiler`-calling frame and this fires on the same thread within
  // the same compile, so the pointer is live and exclusively ours for this call.
  let state: &DriverState = unsafe { &*(state_ptr as *const DriverState) };

  if !state.emit_backend {
    state.firings.borrow_mut().push("consumer_fill_modules fired".to_string());
    return;
  }

  let rc = emit_vale_into_borrowed_module(state, allocator);
  state.firings.borrow_mut().push(format!("consumer_fill_modules emitted rc={rc}"));
  // A nonzero rc means the C++ backend rejected its own emission (e.g. LLVMVerifyModule failed on the
  // Vale IR in rustc's module). Fail loudly rather than let rustc link a silently-broken module.
  assert_eq!(rc, 0, "backend_compile_program_into returned {rc}");
}

/// Lower the instantiated Vale program and emit its bodies into a module rustc lends us. Reached only
/// with `emit_backend` set. Returns the C++ backend's rc (0 = emitted + verified).
///
/// The Vale bodies come purely from `hinputs` (the typing output) via the ordinary `translate_program`
/// — the same finalized `HinputsI` owned-mode produces. rustc's `per_instance_mir` drive is a separate
/// concern (it makes rustc reify/codegen the *Rust* leaves); it does not feed the Vale bodies here.
fn emit_vale_into_borrowed_module(
  state: &DriverState,
  allocator: &ExtraModuleAllocator<ModuleLlvm>,
) -> i32 {
  let hinputs_ref = state.hinputs.borrow();
  let hinputs = match hinputs_ref.as_ref() {
    Some(h) => h,
    None => return 0, // nothing typed (shouldn't happen on the driven path) — nothing to emit.
  };
  let instantiator = InstantiatorI {
    opts: state.opts,
    interner: state.interner,
    typing_interner: state.typing_interner,
    scout_arena: state.scout_arena,
    keywords: state.keywords,
    hinputs,
  };
  // Single instantiation: finalize the accumulator the driven `per_instance_mir` already built, rather
  // than re-instantiating the whole program. The drive recorded exactly the functions/externs rustc
  // demanded and retained the exports it walked, so `assemble_hinputs` packages that demand-driven
  // `monouts` directly. No eager kind-export / function-extern loops, no `-Clink-dead-code`-style
  // over-instantiation: we emit only what was actually used.
  let mut monouts = state.monouts.borrow_mut();
  let function_exports: Vec<_> = state.function_exports.borrow_mut().drain(..).collect();
  let hinputs_i =
    instantiator.assemble_hinputs(&mut *monouts, Vec::new(), function_exports, Vec::new());

  let cache = MetalCache::new();
  let program = populate_metal_cache(&cache, &hinputs_i);

  // Ask rustc for one fresh module (a fresh LLVMContext + LLVMModule) and take its raw handles. Only
  // one CGU for now; the realloc caveat (fill before requesting the next) is moot with a single call.
  let name = "vale_cgu";
  // SAFETY: `allocator.allocate` is rustc's `#[repr(C)]` fn pointer; `allocator.state` is its paired
  // state; both are valid for this synchronous hook call. It returns a `*mut ModuleLlvm` owned by
  // rustc (pushed into its extras vec) — we borrow it, never free it.
  let module_ptr = unsafe { (allocator.allocate)(allocator.state, name.as_ptr(), name.len()) };
  assert!(!module_ptr.is_null(), "rustc's extra-module allocator returned null");
  let module: &mut ModuleLlvm = unsafe { &mut *module_ptr };
  let llcx = module.llcx_raw_mut();
  let llmod = module.llmod_raw();

  let opts = BackendCompileOptions::default();
  let entry_symbol = state.entry_symbol.borrow();
  // SAFETY: `llcx`/`llmod` are rustc's live borrowed handles for this call; the C++ side emits into
  // the module and disposes nothing (rustc owns their lifecycle and disposes them after the hook).
  unsafe {
    backend_compile_program_into_safe(&cache, &program, &opts, llcx, llmod, entry_symbol.as_deref())
  }
}

/// Build the synthetic MIR body rustc gets for a Vale item: one `ReifyFnPointer` cast per Rust leaf
/// (which is what puts each Rust `Instance` into the collector's queue) followed by `unreachable`.
/// The body never executes — our backend emits the real body under the same rustc-mangled symbol
/// (single-symbol; arch §5.2). Mirrors Harmonious's `build_dependency_body`.
fn build_dependency_body<'tcx>(
  tcx: TyCtxt<'tcx>,
  instance: Instance<'tcx>,
  rust_deps: &[(DefId, ty::GenericArgsRef<'tcx>)],
) -> Body<'tcx> {
  let def_id = instance.def_id();

  // Shape the locals from the host item's signature: _0 return, _1.._n args.
  let sig = tcx.fn_sig(def_id).instantiate(tcx, instance.args);
  let sig = tcx.normalize_erasing_late_bound_regions(ty::TypingEnv::fully_monomorphized(), sig);

  let span = tcx.def_span(def_id);
  let source_info = SourceInfo::outermost(span);

  let mut local_decls: IndexVec<Local, LocalDecl<'tcx>> = IndexVec::new();
  local_decls.push(LocalDecl::new(sig.output(), span)); // _0: return
  for &input_ty in sig.inputs() {
    local_decls.push(LocalDecl::new(input_ty, span));
  }

  let mut blocks: IndexVec<BasicBlock, BasicBlockData<'tcx>> = IndexVec::new();
  let mut stmts = Vec::new();

  for &(dep_def_id, dep_args) in rust_deps {
    // Each Rust leaf becomes `_k = <dep as fn(...)> as fn(...)` — a ReifyFnPointer cast of the
    // zero-sized FnDef const. The collector queues the FnDef's Instance; the value is never used.
    let fn_def_ty = Ty::new_fn_def(tcx, dep_def_id, dep_args);
    let fn_sig = tcx.fn_sig(dep_def_id).instantiate(tcx, dep_args);
    let fn_ptr_ty = Ty::new_fn_ptr(tcx, fn_sig);
    let fn_ptr_local = local_decls.push(LocalDecl::new(fn_ptr_ty, span));

    stmts.push(Statement::new(
      source_info,
      StatementKind::Assign(Box::new((
        Place::from(fn_ptr_local),
        Rvalue::Cast(
          CastKind::PointerCoercion(
            PointerCoercion::ReifyFnPointer(Safety::Safe),
            CoercionSource::Implicit,
          ),
          Operand::Constant(Box::new(ConstOperand {
            span,
            user_ty: None,
            const_: Const::zero_sized(fn_def_ty),
          })),
          fn_ptr_ty,
        ),
      ))),
    ));
  }

  blocks.push(BasicBlockData::new_stmts(
    stmts,
    Some(Terminator { source_info, kind: TerminatorKind::Unreachable }),
    false,
  ));

  let source_scopes = IndexVec::from_elem_n(
    SourceScopeData {
      span,
      parent_scope: None,
      inlined: None,
      inlined_parent_scope: None,
      local_data: ClearCrossCrate::Clear,
    },
    1,
  );

  let mut body = Body::new(
    MirSource::item(def_id),
    blocks,
    source_scopes,
    local_decls,
    IndexVec::new(),
    sig.inputs().len(),
    vec![],
    span,
    None,
    None,
  );
  // The collector reads these; our body has neither.
  body.set_required_consts(vec![]);
  body.set_mentioned_items(vec![]);
  body
}
