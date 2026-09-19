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
use rustc_codegen_ssa::ModuleCodegen;
use rustc_hir::Safety;
use rustc_index::IndexVec;
use rustc_middle::mir::{
  BasicBlock, BasicBlockData, Body, CastKind, ClearCrossCrate, Const, ConstOperand, CoercionSource,
  Local, LocalDecl, MirSource, Operand, Place, Rvalue, SourceInfo, SourceScopeData, Statement,
  StatementKind, Terminator, TerminatorKind,
};
use rustc_middle::middle::deduced_param_attrs::DeducedParamAttrs;
use rustc_hir::attrs::Linkage;
use rustc_middle::mir::mono::{CodegenUnit, MonoItem, MonoItemPartitions, Visibility};
use rustc_abi::{
  AbiAlign, Align, BackendRepr, FieldIdx, FieldsShape, LayoutData, Primitive, RegKind, Size,
  VariantIdx, Variants,
};
use rustc_hashes::Hash64;
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::layout::{LayoutError, TyAndLayout};
use rustc_middle::ty::{self, Instance, PseudoCanonicalInput, Ty, TyCtxt};
use rustc_middle::util::Providers;
use rustc_target::callconv::PassMode;
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::Symbol;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ptr::null;
use std::sync::OnceLock;

use crate::backend_ffi::metal_cache::MetalCache;
use crate::backend_ffi::metal_lowerer::populate_metal_cache;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::backend_ffi::backend_inputs::{BackendInputs, BackendMode, Callback, InteropInputs};
use crate::backend_ffi::metal_lowerer::{Coercion, ExternAbi, StructLayout};
use crate::backend_ffi::{compile, BackendCompileOptions};
use crate::instantiating::instantiated_humanizer::humanize_id;
use crate::instantiating::ast::citizens::StructDefinitionI;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::utils::range::CodeLocationS;
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::names::{IInterfaceTemplateNameI, INameI, IStructTemplateNameI, IdI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::types::KindIT;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::ast::ast::{FunctionExportI, FunctionExternI};
use crate::instantiating::instantiator::{DenizenBoundToDenizenCallerBoundArgI, InstantiatedOutputsI, InstantiatorI};
use crate::keywords::Keywords;
use crate::typing::names::names::{IdT, INameT};
use crate::typing::compiler::Compiler;
use crate::typing::templata_compiler::get_interface_template;
use crate::typing::ast::ast::PrototypeT;
use crate::typing::types::types::RegionT;
use crate::utils::fx::IndexMap;
use crate::scout_arena::ScoutArena;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::rust_interop::reserved::RUST_MODULE;
use crate::typing::rust_interop::tyctxt_oracle::resolve_crate_qualified_path;
use crate::typing::rust_interop::typeid::{anon_substruct_rust_name, typeid};
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
  /// Rust→Vale callbacks the collector reached: a Vale method (a trait-impl override) that *Rust*
  /// calls, not a Vale export. For each, the backend emits a wrapper under the rustc-mangled symbol
  /// that adapts the Rust ABI and forwards to the internal Vale body (single-symbol, arch §5.2).
  /// Accumulated as `per_instance_mir` fires on each such method; read at emit.
  pub callbacks: &'ctx RefCell<Vec<CallbackReq>>,
  /// Per-run log of what the provider did, keyed by nothing — one line per Vale item it fired on.
  /// Lives here (per driven run) rather than in a global so parallel driven tests never race.
  pub firings: &'ctx RefCell<Vec<String>>,
  /// Each Rust leaf's boundary ABI (from `tcx.fn_abi_of_instance`), keyed by the extern's humanized
  /// prototype name. That is the same key the metal prototype gets, so the backend's `buildCallOrSideCall`
  /// finds it. Accumulated as leaves resolve in `collect_new_rust_requests`; read at emit.
  pub extern_abis: &'ctx RefCell<HashMap<String, ExternAbi>>,
  /// The typeid→kind universe (arch §10.9): every Vale struct/interface instantiated so far, keyed by
  /// the content-addressed typeid its `__ValeOpaque<typeid>` crossing carries (`opaque_typeid`).
  /// Appended after every instantiator drain (`register_instantiated_kinds`) and read by the
  /// `layout_of` override to size a Vale struct rustc holds by value, and by `collect_callback` to
  /// confirm a callback's opaque type args are ones Vale instantiated. A separate cell from `monouts`
  /// on purpose: the ABI queries that re-enter `layout_of` fire while the resolve loop still holds
  /// `monouts` mutably. Populate-then-read, in instantiation order; a typeid registers once.
  pub opaque_universe: &'ctx RefCell<IndexMap<u64, OpaqueKindI<'s, 'i>>>,
  /// Whether the `fill_extra_modules` hook should actually lower + emit the Vale bodies into rustc's
  /// borrowed module (Stage 2+), or just record that it fired (Stage 1 / the Milestone-M driven tests
  /// that assert only on resolution, not emission). Off keeps those tests off the backend path.
  pub emit_backend: bool,
}

/// One Rust→Vale callback the backend must emit a wrapper for: `symbol` is the rustc-mangled name
/// Rust's monomorphized call site targets (so the wrapper is the sole definition — single-symbol),
/// and `vale_name` is the humanized name of the internal Vale body to forward to. `vale_name` is
/// also the key its inbound ABI is stored under in `extern_abis`, matching the metal prototype name.
pub struct CallbackReq {
  pub symbol: String,
  pub vale_name: String,
}

/// What a `__ValeOpaque<typeid>` stands for, as recorded in `DriverState.opaque_universe`. A struct
/// carries its definition so the `layout_of` override can size it from its members without touching
/// `monouts`; an interface has no by-value layout (it crosses only by borrow), so only its identity is
/// kept for the callback presence check.
pub enum OpaqueKindI<'s, 'i> {
  Struct(&'i StructDefinitionI<'s, 'i>),
  Interface(IdI<'s, 'i>),
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
    self.register_instantiated_kinds(&monouts);

    self.resolve_new_requests(tcx, &mut monouts, &before)
  }

  /// Bring `opaque_universe` up to date with everything a drain just instantiated. Called right after
  /// each drain and before any rustc query on the new leaves, so by the time `fn_abi_of_instance` asks
  /// the layout of a `__ValeOpaque<typeid>` argument, that typeid is answerable. Idempotent: a kind
  /// already registered (by an earlier export's drain) is left where it is.
  fn register_instantiated_kinds(&self, monouts: &InstantiatedOutputsI<'s, 't, 'i>) {
    let mut universe = self.opaque_universe.borrow_mut();
    for (id, def) in monouts.structs.iter() {
      universe.entry(opaque_typeid(id)).or_insert(OpaqueKindI::Struct(def));
    }
    for id in monouts.interfaces_without_methods.keys() {
      universe.entry(opaque_typeid(id)).or_insert(OpaqueKindI::Interface(*id));
    }
  }

  /// Resolve each Rust leaf collected since `before` (a snapshot of `rust_instantiation_requests`'
  /// keys), materialize its `FunctionExternI` with rustc's real mangled symbol — the one place that
  /// symbol is known — and stash its boundary ABI, returning the resolved requests so the caller can
  /// reify each as a `ReifyFnPointer` in its synthetic body. Shared by the export path and the
  /// callback path: a callback body can call out to Rust (`w.get()`) exactly as an export body does.
  fn resolve_new_requests<'tcx>(
    &self,
    tcx: TyCtxt<'tcx>,
    monouts: &mut InstantiatedOutputsI<'s, 't, 'i>,
    before: &HashSet<IdI<'s, 'i>>,
  ) -> Vec<ResolvedRequest<'tcx>> {
    // Collect first (this borrows the requests map immutably), keeping each request's `PrototypeI`.
    // `tcx.symbol_name(Instance)` is a pure read of the same instance the `ReifyFnPointer` reifies, so
    // it matches the symbol rustc actually codegens the leaf under.
    let new_reqs: Vec<_> = monouts
      .rust_instantiation_requests
      .iter()
      .filter(|(id, _)| !before.contains(*id))
      .map(|(_id, proto)| (*proto, resolve_request(tcx, proto)))
      .collect();

    let code_map = |loc: CodeLocationS| format!("{:?}", loc);
    for (proto, req) in &new_reqs {
      if let Some((def_id, args)) = req.dep {
        let instance = ty::Instance::new_raw(def_id, args);
        let symbol = tcx.symbol_name(instance).name;
        let symbol_i: &str = self.interner.bump().alloc_str(symbol);
        // The extern is born here, complete, with rustc's real symbol — the sole registration point
        // for a Rust extern (the instantiator only records the request).
        monouts.function_externs.push(FunctionExternI {
          prototype: *proto,
          num_inherited_generic_parameters: 0,
          link_name: symbol_i,
        });
        // The leaf's boundary ABI, keyed by the same humanized prototype name the metal lowerer uses,
        // so the backend's buildCallOrSideCall finds it.
        if let Some(abi) = compute_extern_abi(tcx, instance) {
          self.extern_abis.borrow_mut().insert(humanize_id(&code_map, &proto.id, None), abi);
        }
      }
    }

    new_reqs.into_iter().map(|(_, req)| req).collect()
  }

  /// Catch a Rust→Vale callback the collector reached: a Vale trait-impl override (`item_name`, e.g.
  /// `on_call`) that *Rust* calls directly, not a Vale export. Instantiate its body into `monouts`
  /// (so the backend emits it as an internal Vale function), compute its inbound ABI (how Rust hands
  /// the receiver/args in), and record the wrapper the backend must emit under the method's
  /// rustc-mangled symbol. The body is instantiated through the ordinary `translate_prototype` +
  /// `drain` — a callback is an ordinary non-generic function whose only distinction is *who* calls
  /// it, so no override/vtable machinery is needed (Rust dispatched statically to the concrete impl).
  ///
  /// Scope — single-level reverse callbacks only. This reads `monouts` (the typeid universe) *before* it
  /// writes to it (the override instantiation + drain below). On a warm rebuild the exports-first phase in
  /// `lang_collect_and_partition_mono_items` guarantees every *export* has populated `monouts` before any
  /// callback reads it. It does NOT order callback-to-callback: a *nested* reverse callback — a callback
  /// whose body hands another lambda to a Rust trait, producing a functor a second callback then reads —
  /// is not covered and could hit the same empty-universe race. No fixture exercises this today (`on_tick`
  /// calls only Rust leaves and constructs no new functor). If one is added, extend the populate-then-read
  /// ordering to callback dependencies (topological), not just exports-before-callbacks.
  fn collect_callback<'tcx>(
    &self,
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    item_name: &str,
  ) -> Vec<ResolvedRequest<'tcx>> {
    let hinputs_ref = self.hinputs.borrow();
    let hinputs = match hinputs_ref.as_ref() {
      Some(h) => h,
      None => return Vec::new(),
    };
    let code_map = |loc: CodeLocationS| format!("{:?}", loc);

    // The concrete `Self` type rustc monomorphized this override at — e.g. `MyCb<__ValeOpaque<HASH>>`
    // for a lambda forwarder, or a plain `MyCb` for a non-generic callback (the degenerate case).
    let impl_def_id = tcx
      .impl_of_assoc(instance.def_id())
      .expect("a trait-impl override method must live in an impl block");
    let self_ty = tcx.type_of(impl_def_id).instantiate(tcx, instance.args);

    // Reverse-decode + identify the concrete Valen impl this callback instance stands for (arch
    // §8.9/§10.9), against the typeid→kind universe (`opaque_universe`) the earlier drains registered,
    // keyed by the same content hash the outbound lowering stamps (`opaque_typeid`, see `opaque_ty`).
    // The concrete override for a *generic* impl is an instantiator product (`translate_override`),
    // never a typing product — so we identify the impl from the collector-driven instance and drive the
    // instantiator's own virtual-dispatch path, exactly as a native `InterfaceFunctionCall` would.
    let (matched_impl_t, matched_impl_i) = {
      let monouts = self.monouts.borrow();
      // Fail loud (not silent-wrong) if the instance carries an opaque type Vale never instantiated:
      // §10.9 recovery must find every crossed Valen type in the universe.
      let universe = self.opaque_universe.borrow();
      for arg in self_ty.walk() {
        if let Some(arg_ty) = arg.as_type() {
          if let Some(tid) = read_opaque_typeid(tcx, arg_ty) {
            assert!(
              universe.contains_key(&tid),
              "rust interop: callback {item_name:?} carries opaque typeid {tid} for a Valen type not in \
               the instantiated universe (a monomorphization Vale never produced)"
            );
          }
        }
      }
      // Match the callback's concrete `Self` against a recorded impl by projecting each candidate's
      // sub-citizen forward through the same converter the outbound path uses and comparing rustc types.
      let mut matched = None;
      for impls in monouts.interface_to_impls.values() {
        for (impl_t, impl_i) in impls.iter() {
          let sub_id = monouts
            .impls
            .get(impl_i)
            .expect("an impl in interface_to_impls must be in impls")
            .0
            .id();
          if citizen_or_opaque_to_rustc_ty(tcx, &sub_id) == Some(self_ty) {
            // Exactly one impl must project to this rustc Self. Keeping the last match (the old
            // behavior) would make the pick order-dependent if two ever matched; assert uniqueness so an
            // ambiguous reverse-callback program fails loud rather than dispatching arbitrarily (@P0
            // no-order-dependent-decisions).
            assert!(
              matched.is_none(),
              "rust interop: callback {item_name:?} Self matches more than one recorded Valen impl \
               (ambiguous reverse-callback dispatch)"
            );
            matched = Some((*impl_t, *impl_i));
          }
        }
      }
      match matched {
        Some(pair) => pair,
        None => return Vec::new(), // no Valen impl for this Self — nothing to emit
      }
    };

    // The typed abstract method header this override implements: reach it through the impl's edge and
    // the interface's blueprint (typing owns the abstract method set + order).
    let impl_template = Compiler::get_impl_template(self.typing_interner, matched_impl_t);
    // `interface_template_to_sub_citizen_to_edge` is a std HashMap (nondeterministic iteration), so a
    // `.find()` that took the first of several matches would be order-dependent. Collect and assert
    // exactly one match: when the pick is unique, iteration order is irrelevant; if it ever isn't, fail
    // loud (@P0 no-order-dependent-decisions) instead of choosing an arbitrary edge.
    let matching_edges: Vec<_> = hinputs
      .interface_template_to_sub_citizen_to_edge
      .values()
      .flat_map(|m| m.values().copied())
      .filter(|edge| Compiler::get_impl_template(self.typing_interner, edge.edge_id) == impl_template)
      .collect();
    assert_eq!(
      matching_edges.len(),
      1,
      "rust interop: expected exactly one edge for the matched impl, found {}",
      matching_edges.len()
    );
    let edge = matching_edges[0];
    let interface_template_id = get_interface_template(self.typing_interner, edge.super_interface);
    let blueprint = hinputs
      .interface_template_to_edge_blueprints
      .get(&interface_template_id)
      .expect("no edge blueprint for the matched impl's interface");
    let abstract_proto_t: PrototypeT = blueprint
      .super_family_root_headers
      .iter()
      .map(|(p, _)| *p)
      .find(|p| {
        matches!(&p.id.local_name, INameT::Function(n) if n.template.human_name.as_str() == item_name)
      })
      .expect("no abstract method by that name in the interface blueprint");

    let instantiator = InstantiatorI {
      opts: self.opts,
      interner: self.interner,
      typing_interner: self.typing_interner,
      scout_arena: self.scout_arena,
      keywords: self.keywords,
      hinputs,
    };
    let mut monouts = self.monouts.borrow_mut();
    // Snapshot the Rust-leaf requests before instantiating the callback body, so `resolve_new_requests`
    // returns only the ones this body added (e.g. an outbound `w.get()`).
    let before: HashSet<_> = monouts.rust_instantiation_requests.keys().copied().collect();

    let empty_bound = DenizenBoundToDenizenCallerBoundArgI {
      func_id_to_bound_arg_prototype: IndexMap::default(),
      bound_param_impl_id_to_bound_arg_impl_id: IndexMap::default(),
    };
    let empty_subs: IndexMap<IdT, ITemplataI> = IndexMap::default();
    // Resolve the concrete override *statically*, the same seam a devirtualized `where implements`
    // call uses (`BoundFunctionCall` in the instantiator). This yields the plain concrete override
    // prototype directly — no vtable, no abstract-method dispatcher instantiation — so the backend
    // never builds an interface fat pointer. The reverse callback is static dispatch (Rust calls the
    // concrete override directly); the interface exists only for typechecking. Non-generic callbacks
    // flow through here unchanged as the degenerate case (@NNGZ).
    let (concrete_impl_id_t, impl_bound_args) = *monouts
      .instantiated_impl_to_typed_impl_and_bounds
      .get(&matched_impl_i)
      .expect("matched impl not recorded in instantiated_impl_to_typed_impl_and_bounds");
    let abstract_bound_args = instantiator.translate_bound_args_for_callee(
      &mut monouts,
      &abstract_proto_t.id,
      &empty_bound,
      &empty_subs,
      &RegionT::Default,
      hinputs.get_instantiation_bound_args(abstract_proto_t.id),
    );
    let override_proto = instantiator.resolve_override_prototype(
      &mut monouts,
      &concrete_impl_id_t,
      &matched_impl_i,
      &abstract_proto_t,
      &abstract_bound_args,
      impl_bound_args,
    );
    instantiator.drain_instantiation_queue(&mut monouts);
    self.register_instantiated_kinds(&monouts);

    // The instantiated body's metal name is `humanize_id(override_proto.id)` (metal_lowerer's
    // `lower_function` keys it that way). That single string is both the extern-ABI key the wrapper's
    // `buildBoundarySignature` looks up and the `vale_name` the wrapper forwards through.
    let vale_name = humanize_id(&code_map, &override_proto.id, None);
    let symbol = tcx.symbol_name(instance).name.to_string();
    // The inbound ABI is role-agnostic: the same `fn_abi_of_instance` read the outbound leaves use
    // gives how Rust passes the `&self` receiver (and args) into the callback.
    if let Some(abi) = compute_extern_abi(tcx, instance) {
      self.extern_abis.borrow_mut().insert(vale_name.clone(), abi);
    }
    self.callbacks.borrow_mut().push(CallbackReq { symbol, vale_name });

    // Reify any Rust leaves the callback body itself calls (e.g. `w.get()`), so rustc's collector
    // queues and codegens them — the same treatment an export body's leaves get.
    self.resolve_new_requests(tcx, &mut monouts, &before)
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

  // A synthesized `deref` (from autoderef): its receiver type implements `Deref`, but `deref` is a
  // trait method, not inherent, so `resolve_method_request` misses it. Resolve it through the `Deref`
  // impl. Checked after the inherent attempt so a real inherent method named `deref` still wins.
  if let Some((def_id, args)) = resolve_deref_request(tcx, proto, &own_arg_tys) {
    return ResolvedRequest {
      log: format!("{path} => {} (deref)", tcx.def_path_str(def_id)),
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

/// Find a struct defined in the crate under compilation (the stub) by name. A Valen struct that
/// implements a Rust trait is projected into the stub, not a dependency, so
/// `resolve_crate_qualified_path` (which walks only loaded dependency crates) cannot see it — this is
/// the type analog of `resolve_local_fn`. It lets a local Valen type be passed as a Rust generic's
/// type argument (`run_callback::<MyCb>`).
fn resolve_local_type(tcx: TyCtxt<'_>, name: &str) -> Option<DefId> {
  tcx
    .module_children_local(rustc_hir::def_id::CRATE_DEF_ID)
    .iter()
    .find(|c| c.ident.name.as_str() == name)
    .and_then(|c| match c.res {
      rustc_hir::def::Res::Def(rustc_hir::def::DefKind::Struct, def_id) => Some(def_id),
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

/// Resolve a synthesized `deref` request to the `Deref::deref` fn of the receiver type's shared `Deref`
/// impl — the instantiator mirror of the oracle's `Deref` discovery. Recognized by the method name
/// `deref` on a rust-backed receiver whose type implements `Deref`; `None` otherwise (so a real
/// inherent method named `deref`, tried first via `resolve_method_request`, still wins).
fn resolve_deref_request<'tcx>(
  tcx: TyCtxt<'tcx>,
  proto: &PrototypeI,
  own_arg_tys: &[Ty<'tcx>],
) -> Option<(DefId, ty::GenericArgsRef<'tcx>)> {
  let (method_name, _, parameters) = request_name_parts(proto)?;
  if method_name.as_str() != "deref" {
    return None;
  }
  let (owner_def_id, receiver_arg_tys) = receiver_owner(tcx, parameters.first()?)?;
  let deref_did = tcx.lang_items().deref_trait()?;
  let receiver_ty = tcx.type_of(owner_def_id).instantiate_identity();
  let mut deref_fn: Option<DefId> = None;
  tcx.for_each_relevant_impl(deref_did, receiver_ty, |impl_did| {
    if deref_fn.is_some() {
      return;
    }
    let self_adt = tcx
      .impl_trait_ref(impl_did)
      .instantiate_identity()
      .self_ty()
      .ty_adt_def()
      .map(|d| d.did());
    if self_adt != Some(owner_def_id) {
      return;
    }
    for assoc in tcx.associated_items(impl_did).in_definition_order() {
      if assoc.as_tag() == ty::AssocTag::Fn {
        deref_fn = Some(assoc.def_id);
      }
    }
  });
  let deref_fn = deref_fn?;
  let mut method_arg_tys = receiver_arg_tys;
  method_arg_tys.extend_from_slice(own_arg_tys);
  Some((deref_fn, build_generic_args(tcx, deref_fn, &method_arg_tys)))
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
  let (name, template_args, _) = request_name_parts(proto)?;
  let mut arg_tys = Vec::with_capacity(template_args.len());
  for t in template_args.iter() {
    match templata_to_rustc_ty(tcx, t) {
      Some(ty) => arg_tys.push(ty),
      // A generic type arg of a Rust callee that won't lower to a rustc `Ty` used to return `None`,
      // silently dropping the whole leaf (no `FunctionExternI`) and aborting the C++ backend far away
      // on a missing extern. That is a real gap — an unprojected/unlowerable type — never a benign
      // skip, so fail loudly here, naming the callee and the offending arg, instead of downstream.
      None => panic!(
        "rust interop: cannot lower generic type argument of Rust callee `{}` to a rustc type: {t:?}",
        name.as_str()
      ),
    }
  }
  Some(arg_tys)
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
    KindIT::StructIT(s) => citizen_or_opaque_to_rustc_ty(tcx, &s.id),
    KindIT::InterfaceIT(i) => citizen_or_opaque_to_rustc_ty(tcx, &i.id),
    _ => None,
  }
}

/// Lower a citizen kind used as a Rust callee's generic argument, choosing between two ways it can
/// cross to rust:
///  - A rust-backed citizen (a dependency-crate type) or a Valen-defined citizen that is *projected*
///    into the stub crate (a struct that implements a Rust trait, so it has a real stub `DefId`) lowers
///    to its own `Adt` via `citizen_to_rustc_ty`.
///  - Any other Valen-internal type — a lambda closure, or a struct not projected into the stub — has no
///    nameable rust identity, so it crosses as an opaque blob `__ValeOpaque<typeid>` (arch §10.7 Case 2):
///    rustc monomorphizes over it without ever inspecting it. A rust-module citizen that fails to resolve
///    is a genuine dependency-resolution failure, not an opaque type, so it stays `None` (fails loudly at
///    the call site rather than being masked).
fn citizen_or_opaque_to_rustc_ty<'tcx>(tcx: TyCtxt<'tcx>, id: &IdI) -> Option<Ty<'tcx>> {
  if let Some(ty) = citizen_to_rustc_ty(tcx, id) {
    return Some(ty);
  }
  if id.package_coord.module.0 == RUST_MODULE {
    return None;
  }
  opaque_ty(tcx, id)
}

/// Build the opaque wrapper type `__ValeOpaque<typeid>` for a Valen-internal type. The typeid is the
/// content-addressed hash of the type's humanized instantiated id, so the inbound callback path
/// (`collect_callback`) can recover which Valen type an opaque instantiation stands for by matching the
/// same hash over the universe. Mirrors Sky's `build_opaque_args` (toylangc/src/oracle.rs:1330).
fn opaque_ty<'tcx>(tcx: TyCtxt<'tcx>, id: &IdI) -> Option<Ty<'tcx>> {
  opaque_ty_for_typeid(tcx, opaque_typeid(id))
}

/// `__ValeOpaque<tid>` as a rustc type, or `None` when the compiled crate declares no `__ValeOpaque`.
/// The type every Valen-internal type crosses as, given its typeid; `opaque_ty` is the id-taking form.
pub(crate) fn opaque_ty_for_typeid<'tcx>(tcx: TyCtxt<'tcx>, tid: u64) -> Option<Ty<'tcx>> {
  let opaque_def_id = resolve_local_type(tcx, "__ValeOpaque")?;
  Some(Ty::new_adt(tcx, tcx.adt_def(opaque_def_id), build_opaque_args(tcx, opaque_def_id, tid)))
}

/// The one definition of the typeid a Valen kind crosses under: the content hash of its humanized
/// instantiated id. Stamped by `opaque_ty` on the way out, and the key `opaque_universe` is registered
/// under, so the inbound decode (`read_opaque_typeid` → universe) recovers the same kind.
fn opaque_typeid(id: &IdI) -> u64 {
  let code_map = |loc: CodeLocationS| format!("{:?}", loc);
  typeid(&humanize_id(&code_map, id, None))
}

/// The `GenericArgs` for `__ValeOpaque<HASH>`: the typeid interned as its single `const T: u64` argument.
fn build_opaque_args<'tcx>(
  tcx: TyCtxt<'tcx>,
  opaque_def_id: DefId,
  tid: u64,
) -> ty::GenericArgsRef<'tcx> {
  let typeid_const =
    ty::Const::from_bits(tcx, tid as u128, ty::TypingEnv::fully_monomorphized(), tcx.types.u64);
  ty::GenericArgs::for_item(tcx, opaque_def_id, |param, _| match param.kind {
    ty::GenericParamDefKind::Const { .. } => typeid_const.into(),
    ty::GenericParamDefKind::Lifetime => tcx.lifetimes.re_erased.into(),
    ty::GenericParamDefKind::Type { .. } => {
      panic!("__ValeOpaque must have only its const param, got a type param")
    }
  })
}

/// Recover the content-addressed typeid from an `__ValeOpaque<HASH>` type — the inverse of
/// `opaque_ty`/`build_opaque_args`. This is the inbound (`collect_callback`) decode: rustc hands us a
/// callback `Instance` whose `Self` type carries `__ValeOpaque<HASH>` for each Valen-internal type
/// arg (e.g. the lambda functor), and the HASH is `typeid(humanize_id(kind.id))` — so this reader plus
/// the typeid→kind universe (`DriverState.opaque_universe`) recovers which Valen kind the opaque blob
/// stands for (arch §10.9). Returns `None` for any other type (a primitive, a real Rust `Adt`, a
/// borrow) so a caller can fall through / fail loud rather than mis-decode.
fn read_opaque_typeid<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<u64> {
  let ty::TyKind::Adt(adt_def, args) = ty.kind() else {
    return None;
  };
  let opaque_def_id = resolve_local_type(tcx, "__ValeOpaque")?;
  if adt_def.did() != opaque_def_id {
    return None;
  }
  args.const_at(0).try_to_leaf().map(|scalar| scalar.to_u64())
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
  let (human_name, template_args): (String, _) = match id.local_name {
    INameI::StructName(sn) => match sn.template {
      IStructTemplateNameI::StructTemplate(t) => (t.human_name.as_str().to_string(), sn.template_args),
      _ => return None,
    },
    INameI::InterfaceName(inm) => {
      let IInterfaceTemplateNameI::InterfaceTemplate(t) = inm.template;
      (t.human_namee.as_str().to_string(), inm.template_args)
    }
    // The anonymous substruct auto-generated for an imported trait — a lambda handed to
    // `SomeTrait((..) => {..})`. Its Vale name (`<interface>.anonymous`) is not a Rust identifier, so it
    // crosses under the mangled `<interface>__anon` the pass-2 stub generator also emits (the one
    // name-agreement seam, `anon_substruct_rust_name`), resolved locally like a hand-written forwarder
    // struct rather than as an opaque blob — so only the functor it wraps is opaque, not the whole
    // substruct.
    INameI::AnonymousSubstruct(asn) => {
      let IInterfaceTemplateNameI::InterfaceTemplate(t) = asn.template.interface;
      (anon_substruct_rust_name(t.human_namee.as_str()), asn.template_args)
    }
    _ => return None,
  };
  // A rust-backed citizen (reserved `rust` module) lives in a loaded dependency crate; a Valen-defined
  // citizen — e.g. a struct that implements a Rust trait, or an auto-generated anon substruct — is
  // projected into the stub crate under compilation, which `resolve_crate_qualified_path` (dependency
  // crates only) can't see, so resolve it locally by name.
  let def_id = if id.package_coord.module.0 == RUST_MODULE {
    let mut segments: Vec<&str> =
      id.package_coord.packages.as_slice().iter().map(|s| s.as_str()).collect();
    segments.push(&human_name);
    resolve_crate_qualified_path(tcx, &segments.join("."))?.0
  } else {
    resolve_local_type(tcx, &human_name)?
  };
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
static DEFAULT_LAYOUT_OF: OnceLock<
  for<'tcx> fn(
    TyCtxt<'tcx>,
    PseudoCanonicalInput<'tcx, Ty<'tcx>>,
  ) -> Result<TyAndLayout<'tcx>, &'tcx LayoutError<'tcx>>,
> = OnceLock::new();

/// The `override_queries` hook: a bare `fn` (rustc query providers cannot capture state), installed
/// from the driver's `config()`.
///
/// - `per_instance_mir` → our provider (drives Vale's instantiator).
/// - `collect_and_partition_mono_items` → two interop fixups: (a) strips Vale's
///   `#[vale::emit_consumer_body]` stub bodies from rustc's codegen, because Vale emits the real bodies
///   under the *same* rustc-mangled names via `fill_extra_modules` (single-symbol, arch §5.2) —
///   without this, rustc's `unreachable!()` placeholder and Vale's body collide as a duplicate symbol
///   at link; and (b) force-promotes each reified Rust leaf to `External` linkage so rustc's release
///   `internalize_symbols` pass can't strand Vale's out-of-band `vale_cgu` reference to it (see the fn).
/// - `deduced_param_attrs` → `&[]` for those same items, so rustc infers no `readonly`/`captures(none)`
///   from the `unreachable!()` body (which would be silent UB against Vale's real body; arch §22.4).
/// - `layout_of` → Vale's real size and alignment for a `__ValeOpaque<typeid>` naming an instantiated
///   Vale struct (arch §10.3–§10.5), so a Vale struct crossing by value is neither zero-sized nor
///   decomposed; every other type is rustc's.
///
/// A non-Vale crate is byte-identical: the pure-Rust passthrough (`NoopCallbacks`) installs no
/// overrides at all, and each override diverges only for Vale's own stub items / reified leaves.
pub fn vale_override_queries(_session: &rustc_session::Session, providers: &mut Providers) {
  providers.queries.per_instance_mir = lang_per_instance_mir;
  let _ = DEFAULT_COLLECT_AND_PARTITION.set(providers.queries.collect_and_partition_mono_items);
  let _ = DEFAULT_DEDUCED_PARAM_ATTRS.set(providers.queries.deduced_param_attrs);
  let _ = DEFAULT_LAYOUT_OF.set(providers.queries.layout_of);
  providers.queries.collect_and_partition_mono_items = lang_collect_and_partition_mono_items;
  providers.queries.deduced_param_attrs = lang_deduced_param_attrs;
  providers.queries.layout_of = lang_layout_of;
}

/// The `layout_of` override (arch §10.3–§10.5; Sky's `rustc-lang-facade/src/queries/layout.rs`). A Vale
/// type in a Rust slot crosses as `__ValeOpaque<typeid>`, whose own fields are zero-sized markers — so
/// rustc's layout would make a Vale struct passed by value `PassMode::Ignore` (nothing crosses) and give
/// a `Vec<Ship>` no stride. This answers with Vale's real size and alignment instead: the typeid names an
/// instantiated Vale struct in `opaque_universe`, each member lowers to a rustc type that rustc sizes
/// itself, and a C-offset walk over them yields the same layout LLVM gives the backend's struct of those
/// members. The result is `BackendRepr::Memory` (rustc never splits it into scalars), with one offset per
/// marker field so rustc's debuginfo walker, which visits one layout field per source field, stays
/// consistent (§10.4.5).
///
/// Only the wrapper ADT itself is intercepted; `&__ValeOpaque<..>`, `*mut`, `Option<..>` and every other
/// type is rustc's (Sky's hard-won lesson: intercepting derived types corrupts their layouts). A typeid
/// naming nothing in the universe is rustc's too: the projection of a trait-implementing Vale struct
/// carries a `__ValeOpaque<typeid(name)>` *field* keyed on the struct's name, not an instantiated id, and
/// that wrapper crosses only by borrow, where its size is never read.
fn lang_layout_of<'tcx>(
  tcx: TyCtxt<'tcx>,
  query: PseudoCanonicalInput<'tcx, Ty<'tcx>>,
) -> Result<TyAndLayout<'tcx>, &'tcx LayoutError<'tcx>> {
  let default = DEFAULT_LAYOUT_OF
    .get()
    .expect("vale_override_queries saves rustc's layout_of before installing the override");
  let ty = query.value;
  let Some(tid) = read_opaque_typeid(tcx, ty) else {
    return default(tcx, query);
  };
  let state_ptr = DRIVER_STATE.with(|c| c.get());
  // `__ValeOpaque` exists only in a driven stub crate, and the driver arms its state in
  // `after_expansion`, before any layout is asked — so answering from rustc's zero-sized view here
  // would be silently wrong, never benign.
  assert!(
    !state_ptr.is_null(),
    "rust interop: layout_of asked for {ty:?} with no driven run armed"
  );
  // SAFETY: as in `lang_per_instance_mir` — the `DriverState` lives in the `run_compiler`-calling
  // frame, which outlives every provider call; borrowed only for this call, nothing escapes.
  let state: &DriverState = unsafe { &*(state_ptr as *const DriverState) };
  let universe = state.opaque_universe.borrow();
  let def = match universe.get(&tid) {
    None => return default(tcx, query),
    Some(OpaqueKindI::Struct(def)) => def,
    Some(OpaqueKindI::Interface(id)) => panic!(
      "rust interop: Vale interface {id:?} crosses to Rust by value ({ty:?}); only a struct has a \
       by-value layout"
    ),
  };
  let struct_id = &def.instantiated_citizen.id;
  // Each member as rustc sizes it. A member no rustc type can stand for (a string, a float, a Vale
  // reference) has no honest size here, and a wrong size is a silent memory error — so fail loud.
  let mut member_tys = Vec::with_capacity(def.members.len());
  for member in def.members.iter() {
    match kind_to_rustc_ty(tcx, &member.tyype) {
      Some(member_ty) => member_tys.push(member_ty),
      None => panic!(
        "rust interop: Vale struct {struct_id:?} crosses to Rust by value ({ty:?}) but its member \
         {:?} of type {:?} has no rustc type to size it by",
        member.name, member.tyype
      ),
    }
  }
  // One layout field per source field of `__ValeOpaque` itself — its marker fields — so rustc's
  // debuginfo walker, which visits `layout.field(i)` for each source field, always finds one.
  let ty::TyKind::Adt(opaque_adt, _) = ty.kind() else {
    unreachable!("read_opaque_typeid accepted a non-ADT: {ty:?}")
  };
  let marker_field_count = opaque_adt.non_enum_variant().fields.len();
  Ok(TyAndLayout { ty, layout: tcx.mk_layout(c_layout_over(tcx, &member_tys, marker_field_count)?) })
}

/// The C struct layout of `member_tys` in declaration order — each member at the next offset aligned
/// to its own alignment, the whole padded to the largest alignment — which is also LLVM's layout of a
/// non-packed struct of those members, i.e. what the backend emits. Reported over `__ValeOpaque`'s own
/// `marker_field_count` zero-sized fields: the first at offset 0, where the payload starts, and every
/// other one at the end, past the payload.
fn c_layout_over<'tcx>(
  tcx: TyCtxt<'tcx>,
  member_tys: &[Ty<'tcx>],
  marker_field_count: usize,
) -> Result<LayoutData<FieldIdx, VariantIdx>, &'tcx LayoutError<'tcx>> {
  let mut offset = 0u64;
  let mut max_align = 1u64;
  for member_ty in member_tys {
    let member_layout = tcx.layout_of(PseudoCanonicalInput {
      value: *member_ty,
      typing_env: ty::TypingEnv::fully_monomorphized(),
    })?;
    let member_align = member_layout.align.abi.bytes();
    max_align = max_align.max(member_align);
    offset = align_up(offset, member_align) + member_layout.size.bytes();
  }
  let total_size = align_up(offset, max_align);
  let align = Align::from_bytes(max_align).expect("a member alignment is a power of two");
  Ok(LayoutData {
    fields: FieldsShape::Arbitrary {
      offsets: IndexVec::from_iter(
        (0..marker_field_count).map(|i| if i == 0 { Size::ZERO } else { Size::from_bytes(total_size) }),
      ),
      in_memory_order: IndexVec::from_iter((0..marker_field_count).map(FieldIdx::from_usize)),
    },
    variants: Variants::Single { index: VariantIdx::from_u32(0) },
    backend_repr: BackendRepr::Memory { sized: true },
    largest_niche: None,
    uninhabited: false,
    align: AbiAlign::new(align),
    size: Size::from_bytes(total_size),
    max_repr_align: None,
    unadjusted_abi_align: align,
    randomization_seed: Hash64::ZERO,
  })
}

fn align_up(offset: u64, align: u64) -> u64 {
  (offset + align - 1) & !(align - 1)
}

/// Rebuild rustc's CGUs with two interop fixups. (1) Drop Vale's `#[vale::emit_consumer_body]` stub
/// items, so rustc emits no `.o` for a body Vale itself emits under the same mangled name. (2) Force
/// each surviving reified Rust leaf to `External` linkage.
///
/// Why (2): rustc's release partitioner internalizes those leaves — for an `Executable` crate
/// `local_crate_exports_generics()` is false, so the item is `Hidden`/`can_be_internalized`, and
/// `internalize_symbols` demotes it to `Internal` because its only *apparent* user shares its CGU. But
/// the real user is Vale's out-of-band `vale_cgu` object, which references the leaf as an external
/// symbol, so `Internal` (object-local) strands it as an undefined symbol at link. Debug leaves them
/// `External` and links; we reproduce that. Setting `(External, Default)` here — after the delegated
/// `upstream()` already ran internalize — wins because the LLVM backend reads `data.linkage` directly.
/// The leaves are exactly the `FunctionExternI` the provider materialized, so we match each item's
/// `symbol_name` against their `link_name`s (never `def_path_str`, which ICEs in this non-diagnostic
/// context). Mirrors Harmonious's fork-free "Outcome A" (arch §5.3).
fn lang_collect_and_partition_mono_items<'tcx>(
  tcx: TyCtxt<'tcx>,
  key: (),
) -> MonoItemPartitions<'tcx> {
  let upstream = DEFAULT_COLLECT_AND_PARTITION
    .get()
    .expect("default collect_and_partition_mono_items not saved");
  let MonoItemPartitions { codegen_units: upstream_cgus, all_mono_items: reachable, .. } =
    upstream(tcx, key);

  // The reified Rust leaves' mangled symbols, from the `FunctionExternI` the provider materialized as
  // each leaf resolved. Empty when no driven run is active (the override is installed only on the
  // driven path), which leaves the partition a pure stub-body filter.
  let leaf_symbols: HashSet<String> = {
    let state_ptr = DRIVER_STATE.with(|c| c.get());
    if state_ptr.is_null() {
      HashSet::new()
    } else {
      // SAFETY: same scoped-pointer contract as `lang_per_instance_mir` — `DriverState` outlives every
      // provider call on this thread, and we only borrow it for this read.
      let state: &DriverState = unsafe { &*(state_ptr as *const DriverState) };
      state.monouts.borrow().function_externs.iter().map(|e| e.link_name.to_string()).collect()
    }
  };

  // Exports-first (warm-rebuild determinism). Re-fire `per_instance_mir` for every Vale EXPORT before the
  // CGU loop below re-fires any callback. An export's provider instantiates its body and drains, which
  // POPULATES `monouts` (structs/interfaces/impls); a callback's provider (`collect_callback`) READS
  // `monouts` to build the typeid universe and asserts every crossed type is present. On a warm rebuild
  // the upstream partitioner served `items_of_instance` from the incremental cache and never called
  // `per_instance_mir`, so `monouts` starts empty and this fn is its sole re-populator — but the CGU
  // order rustc hands us does not guarantee the export precedes the callback, so a callback-first order
  // reads an empty universe and panics (~half of warm builds, per the baked cold-build layout). Cold gets
  // populate-then-read for free: the collector discovers a callback only after the export's body reifies
  // the caller that reaches it. We restore that invariant by seeding from the authoritative export list
  // (sorted for a byte-stable order among exports), resolving each `__vale_<name>` stub to its instance
  // directly — which also covers a non-`main` export that has no Rust caller and so is in no CGU. The
  // later CGU-loop re-fires of these same exports are memoized no-ops (`per_instance_mir` is never
  // disk-cached and is in-memory-cached, so at most once per instance per build).
  {
    let state_ptr = DRIVER_STATE.with(|c| c.get());
    if !state_ptr.is_null() {
      // SAFETY: same scoped-pointer contract as `lang_per_instance_mir` — `DriverState` outlives every
      // provider call on this thread. We drop the `hinputs` borrow (collecting owned names) before any
      // `per_instance_mir` call, which itself borrows `DriverState`'s `monouts`/`hinputs`.
      let state: &DriverState = unsafe { &*(state_ptr as *const DriverState) };
      let mut export_names: Vec<String> = state
        .hinputs
        .borrow()
        .as_ref()
        .map(|h| h.function_exports.iter().map(|e| e.exported_name.0.to_string()).collect())
        .unwrap_or_default();
      export_names.sort();
      for name in export_names {
        if let Some(def_id) = resolve_local_fn(tcx, &format!("__vale_{name}")) {
          let instance = ty::Instance::new_raw(def_id, build_generic_args(tcx, def_id, &[]));
          let _ = tcx.per_instance_mir(instance);
        }
      }
    }
  }

  let mut filtered_cgus: Vec<CodegenUnit<'tcx>> = Vec::with_capacity(upstream_cgus.len());
  for cgu in upstream_cgus.iter() {
    let mut new_cgu = CodegenUnit::new(cgu.name());
    for (&mono_item, &data) in cgu.items() {
      if is_vale_codegen_target(tcx, mono_item.def_id()) {
        // Re-fire `per_instance_mir` for our stub instances (incremental fix, Part A). Its provider's
        // side effects (populating `DriverState` with the entry symbol, the instantiated bodies, the
        // callbacks) are the emit's only inputs, but rustc reaches it only through the disk-cached
        // `items_of_instance` — so on a *warm* rebuild the collector never calls it and the emit comes
        // out empty (undefined `__vale_main` at link). This query (`collect_and_partition_mono_items`)
        // is `eval_always`, so it runs every build; calling `per_instance_mir` here forces the side
        // effect to fire. Safe against double-firing: `per_instance_mir` declares no disk-cache
        // modifier, so it is never served from disk, and rustc's in-memory query cache runs its
        // provider at most once per instance per build — so on a cold build, where the default
        // partitioner above already called it, this is a cache-hit no-op. We discard the body; only
        // the side effect matters here.
        if let MonoItem::Fn(instance) = mono_item {
          let _ = tcx.per_instance_mir(instance);
        }
        continue;
      }
      let mut data = data;
      if leaf_symbols.contains(mono_item.symbol_name(tcx).name) {
        // Undo the release internalize so Vale's out-of-band `vale_cgu` reference resolves at link.
        data.linkage = Linkage::External;
        data.visibility = Visibility::Default;
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
  // Is this fired item a Vale *export* that Vale drives (`__vale_main`, or a library export), or a
  // trait-impl *callback* that rustc's collector reached because Rust calls it directly? An export is
  // named in `hinputs.function_exports`; a callback (e.g. `on_call`) is not, so it takes the branch
  // that instantiates its body and records an inbound wrapper.
  let is_export = {
    let h = state.hinputs.borrow();
    h.as_ref()
      .map_or(false, |h| h.function_exports.iter().any(|e| e.exported_name.0 == export_name))
  };

  // Each resolved request's `(DefId, args)` becomes a `ReifyFnPointer` cast in the body, which is
  // what puts that Rust `Instance` into the collector's queue.
  let rust_deps: Vec<(DefId, ty::GenericArgsRef<'tcx>)> = if is_export {
    if stub_name == "__vale_main" {
      *state.entry_symbol.borrow_mut() = Some(tcx.symbol_name(instance).name.to_string());
    }
    let requests = state.collect_new_rust_requests(tcx, &export_name);
    let log = requests.iter().map(|r| r.log.as_str()).collect::<Vec<_>>().join(", ");
    state.firings.borrow_mut().push(format!("{stub_name} -> [{log}]"));
    requests.iter().filter_map(|r| r.dep).collect()
  } else {
    // A Rust→Vale callback: instantiate its body, record the wrapper the backend emits under the
    // rustc-mangled symbol, and reify any Rust leaves the callback body itself calls (e.g. an
    // outbound `w.get()`) so rustc's collector queues them — just like an export body's leaves.
    let requests = state.collect_callback(tcx, instance, &stub_name);
    let log = requests.iter().map(|r| r.log.as_str()).collect::<Vec<_>>().join(", ");
    state.firings.borrow_mut().push(format!("{stub_name} -> [callback: {log}]"));
    requests.iter().filter_map(|r| r.dep).collect()
  };

  let body = build_dependency_body(tcx, instance, &rust_deps);
  Some(tcx.arena.alloc(body))
}

/// The `fill_extra_modules` hook handler, installed via `set_fill_extra_modules_hook` from the driven
/// `config()`. rustc calls it once per `codegen_crate`, synchronously on the main thread, before
/// `start_async_codegen` (arch §5.1) — by which point every `per_instance_mir` call has run, so the
/// instantiator state reached through `DRIVER_STATE` is complete. When `emit_backend` is set it lowers
/// the Vale program, emits its bodies into a fresh module, and returns that module for rustc to own
/// (Stage 2+); otherwise it only records that it fired and returns no modules (Stage 1 / the
/// Milestone-M driven tests that assert on resolution, not emission).
pub fn consumer_fill_modules<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<ModuleCodegen<ModuleLlvm>> {
  let state_ptr = DRIVER_STATE.with(|c| c.get());
  if state_ptr.is_null() {
    // No driven run active (the hook is a process-global `OnceLock`, but only does anything when a
    // driven `DriverState` is armed); nothing to contribute.
    return Vec::new();
  }
  // SAFETY: same scoped-pointer contract as the provider (see `lang_per_instance_mir`) — the
  // `DriverState` lives in the `run_compiler`-calling frame and this fires on the same thread within
  // the same compile, so the pointer is live and exclusively ours for this call.
  let state: &DriverState = unsafe { &*(state_ptr as *const DriverState) };

  if !state.emit_backend {
    state.firings.borrow_mut().push("consumer_fill_modules fired".to_string());
    return Vec::new();
  }

  let (rc, module) = emit_vale_into_fresh_module(state, tcx);
  state.firings.borrow_mut().push(format!("consumer_fill_modules emitted rc={rc}"));
  // A nonzero rc means the C++ backend rejected its own emission (e.g. LLVMVerifyModule failed on the
  // Vale IR in the module). Fail loudly rather than let rustc link a silently-broken module.
  assert_eq!(rc, 0, "backend_compile_program_into returned {rc}");
  match module {
    Some(m) => vec![m],
    None => Vec::new(),
  }
}

/// Lower the instantiated Vale program and emit its bodies into a fresh module, which the caller hands
/// to rustc. Reached only with `emit_backend` set. Returns the C++ backend's rc (0 = emitted +
/// verified) and the filled module (`None` only if nothing was typed, which the driven path never hits).
///
/// The Vale bodies come purely from `hinputs` (the typing output) via the ordinary `translate_program`
/// — the same finalized `HinputsI` owned-mode produces. rustc's `per_instance_mir` drive is a separate
/// concern (it makes rustc reify/codegen the *Rust* leaves); it does not feed the Vale bodies here.
fn emit_vale_into_fresh_module<'tcx>(
  state: &DriverState,
  tcx: TyCtxt<'tcx>,
) -> (i32, Option<ModuleCodegen<ModuleLlvm>>) {
  let hinputs_ref = state.hinputs.borrow();
  let hinputs = match hinputs_ref.as_ref() {
    Some(h) => h,
    None => return (0, None), // nothing typed (shouldn't happen on the driven path) — nothing to emit.
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

  // Ask rustc (tcx.layout_of / tcx.fn_abi_of_instance) for the size of each imported struct and the ABI
  // of each extern leaf, so the backend sizes opaque structs and coerces extern calls. Keyed by the same
  // humanized names the metal lowerer gives its struct kinds / prototypes (see populate_metal_cache).
  let struct_layouts = compute_struct_layouts(tcx, &hinputs_i);
  let extern_abis = state.extern_abis.borrow();
  let cache = MetalCache::new();
  // The rust-interop (cargo-driven) path has no source code map on hand, and interop debugging is
  // out of scope for the debugger arc, so lower with an empty map (source ranges resolve to a
  // no-op location, which Backend discards anyway).
  let empty_code_map: FileCoordinateMap<String> = FileCoordinateMap::new();
  let program = populate_metal_cache(&cache, &hinputs_i, &empty_code_map, &struct_layouts, &extern_abis);

  // Mint one fresh module (a fresh LLVMContext + LLVMModule + TargetMachine) the way rustc mints its
  // own per-CGU modules, and take its raw handles for the C++ backend. Only one CGU for now. The
  // module is returned to rustc, which owns it from then on and disposes it after codegen.
  let name = "vale_cgu";
  let mut module = ModuleLlvm::new(tcx, name);
  let llcx = module.llcx_raw_mut();
  let llmod = module.llmod_raw();

  let opts = BackendCompileOptions { verify: true, ..BackendCompileOptions::default() };
  let entry_symbol = state.entry_symbol.borrow();
  // The Rust→Vale callbacks, sorted deterministically by their rustc symbol (which encodes
  // self/args/trait/method) so the emitted module is byte-stable regardless of collector walk order.
  let callback_reqs = state.callbacks.borrow();
  let mut callbacks: Vec<Callback> = callback_reqs
    .iter()
    .map(|c| Callback { symbol: c.symbol.as_str(), vale_name: c.vale_name.as_str() })
    .collect();
  callbacks.sort_by(|a, b| a.symbol.cmp(b.symbol));
  // `llcx`/`llmod` are the module's live handles for this call; the C++ side emits into the module
  // and disposes nothing (`ModuleLlvm`'s own Drop disposes them once rustc is done with the module).
  let rc = compile(BackendInputs {
    cache: &cache,
    program: &program,
    options: opts,
    mode: BackendMode::Interop(InteropInputs {
      context: llcx,
      module: llmod,
      entry_symbol: entry_symbol.as_deref(),
      callbacks,
    }),
    // Interop (rustc) debugging is out of scope; the rust-interop lowerer passes an empty code map.
    absolute_source_paths: vec![],
  });
  (rc, Some(ModuleCodegen::new_regular(name, module)))
}

/// Ask rustc (`tcx.layout_of`) for the size/align of each imported Rust struct the program uses, keyed
/// by the humanized name of the struct's instantiated id. That name is identical to the one the metal
/// lowerer gives the struct kind, so `Unsafe::defineStruct` finds the layout by `structKind->fullName->name`. A struct
/// that doesn't resolve to a rustc type (Vale's own, or a still-generic one) is skipped, leaving the
/// backend to size it from its members as usual.
fn compute_struct_layouts<'s, 'i, 'tcx>(
  tcx: TyCtxt<'tcx>,
  hinputs: &HinputsI<'s, 'i>,
) -> HashMap<String, StructLayout> {
  let typing_env = ty::TypingEnv::fully_monomorphized();
  let code_map = |loc: CodeLocationS| format!("{:?}", loc);
  let mut out = HashMap::new();
  for s in hinputs.structs.iter() {
    let id = &s.instantiated_citizen.id;
    let ty = match citizen_to_rustc_ty(tcx, id) {
      Some(t) => t,
      None => continue, // not a Rust-backed struct
    };
    let layout = match tcx.layout_of(typing_env.as_query_input(ty)) {
      Ok(l) => l,
      Err(_) => continue,
    };
    out.insert(
      humanize_id(&code_map, id, None),
      StructLayout { size: layout.size.bytes(), align: layout.align.abi.bytes() },
    );
  }
  out
}

/// Ask rustc (`tcx.fn_abi_of_instance`) how a Rust leaf's return and each argument cross the boundary,
/// mapping each `PassMode` to a `Coercion` the backend obeys. `None` if rustc can't compute the ABI. A
/// `Pair`/`Cast` (a struct split across two registers, or a float/mixed-register class) `panic!`s;
/// nothing in domino hits one.
fn compute_extern_abi<'tcx>(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>) -> Option<ExternAbi> {
  let typing_env = ty::TypingEnv::fully_monomorphized();
  let fn_abi = tcx
    .fn_abi_of_instance(typing_env.as_query_input((instance, ty::List::empty())))
    .ok()?;
  let coercion_of = |arg: &rustc_target::callconv::ArgAbi<'tcx, Ty<'tcx>>| -> Coercion {
    match &arg.mode {
      // A unit return `()`, e.g. the `__vale_drop` shim's return.
      PassMode::Ignore => Coercion::Ignore,
      // Per @EACBIPZ, a large aggregate crosses as an indirect pointer to a caller-made copy, not byval.
      // Only on_stack: false (AArch64 AAPCS) is handled here; on_stack: true (x86 byval) is unsupported,
      // like Pair/Cast.
      PassMode::Indirect { on_stack: false, .. } => Coercion::Indirect,
      // A value small enough for a register.
      PassMode::Direct(_) => {
        // A reference (`&self`, `&mut self`, `*mut T`) is a pointer-scalar and crosses as a real pointer.
        // A small integer-classed aggregate (`Counter`, `Glyph`) crosses as its register integer.
        if let BackendRepr::Scalar(scalar) = arg.layout.backend_repr {
          if matches!(scalar.primitive(), Primitive::Pointer(_)) {
            return Coercion::DirectPtr;
          }
        }
        Coercion::DirectInt(arg.layout.size.bits() as u32)
      }
      // A small struct rustc `Cast`s to registers. We handle the single-integer case only: no leading
      // prefix registers, and one integer unit covering the whole value (count 1) — an 8-byte struct
      // crossing as a bare `i64` (PieceId). Multi-piece (`[2 x i64]`), float, or prefixed casts are
      // deferred, like Pair/HFA.
      PassMode::Cast { cast, pad_i32: false } => {
        if cast.prefix.iter().any(|p| p.is_some())
          || cast.rest_offset.is_some()
          || cast.rest.unit.kind != RegKind::Integer
          || cast.rest.total != cast.rest.unit.size
        {
          panic!("unsupported Cast {cast:?} for interop extern (only a single integer unit is handled)");
        }
        Coercion::Cast(cast.rest.unit.size.bits() as u32)
      }
      // A small struct rustc passes as two register scalars (`ScalarPair`), e.g. `{i32, i32}`. Each
      // component crosses as its own integer; the struct is reassembled from the two on the far side.
      // Only integer components are handled — a pointer component (a fat pointer / slice) is deferred.
      PassMode::Pair(_, _) => match arg.layout.backend_repr {
        BackendRepr::ScalarPair(s0, s1) => {
          if matches!(s0.primitive(), Primitive::Pointer(_))
            || matches!(s1.primitive(), Primitive::Pointer(_))
          {
            panic!(
              "unsupported Pair with a pointer component for interop extern (fat pointer not handled)"
            );
          }
          Coercion::Pair(s0.size(&tcx).bits() as u32, s1.size(&tcx).bits() as u32)
        }
        other => panic!("PassMode::Pair without a ScalarPair backend_repr: {other:?}"),
      },
      other => panic!("unsupported PassMode {other:?} for interop extern (on-stack byval / multi-piece Cast / HFA not yet handled)"),
    }
  };
  let ret = coercion_of(&fn_abi.ret);
  let mut args: Vec<Coercion> = fn_abi.args.iter().map(|a| coercion_of(a)).collect();
  // Per @TCHAPZ, a `#[track_caller]` fn's `fn_abi` has a hidden trailing `&Location` arg that its
  // `fn_sig`, and so the Vale prototype, lacks. Detect it via `requires_caller_location` and mark that
  // trailing coercion `LocationPtr`, so the backend declares a ptr param and passes null for it.
  if instance.def.requires_caller_location(tcx) {
    match args.last_mut() {
      Some(last) => *last = Coercion::LocationPtr,
      None => panic!("requires_caller_location is true but fn_abi has no args"),
    }
  }
  Some(ExternAbi { ret, args })
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
