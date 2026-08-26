# Rust Interop — instantiator design

Living design doc for how the instantiator (`src/instantiating/`) takes part in Rust interop. It is
the source of truth for this subsystem. The handoff (`docs/handoffs/rust-interop-handoff.md`) points
here and holds none of what belongs here. The design source of truth for the wider feature is
`docs/architecture/vale-rust-interop-architecture.md`.

## Design (human-only)

Without rust interop, the Valen compiler calls `translate_program` which does the entire instantiation stage all in one go. It loops over all exported functions, and recursively instantiates them and all things they encounter.

With rust interop, instantiation is driven by rustc; rustc has a work queue, and slowly works away at the items. When rustc encounters a Valen function from that queue, it calls into our compiler's `per_instance_mir` function (more details on this below).

The code in src/instantiating **does not** do anything special for rust interop. It is unchanged for the entire feature, except for the queue filter. Apart from the queue filter, the only difference will be what's in per_instance_mir vs what's in translate_program (which per_instance_mir doesn't call).

### per_instance_mir

Our `per_instance_mir` looks like this:
`fn per_instance_mir<'tcx>(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>) -> Option<&'tcx Body<'tcx>>`
It's a hook that we hacked into rustc for our purposes here. Rustc uses it to call out to the Valen compiler. Parts:
 * `tcx` is how we can ask rustc for information about various things.
 * `Instance` is like Valen's KindI or SignatureI. It's a mention of a thing, plus its generic arguments.
 * `Body` is what we'll return to rustc.

That last part is tricky: it does *not* return a MIR representation of the user's Valen code. It returns two things:
 1. MIR code to assemble an array of function pointers to things that we want rustc to compile ("mir-rust-instantiation-request-list").
 2. An `unreachable` instruction... because this function will never be run (it'll be swapped out by our backend).

The "things that we want rustc to compile" are function points to _Rust_ functions that this _Valen_ function indirectly calls. When rustc sees this MIR, it will have to instantiate those Rust functions that the Valen function wants to call.

As said above, rustc calls our instantiator's `per_instance_mir` function.

In a way, you could think of `per_instance_mir` as *continuing* our instantiation, from a specific `exported` Valen function.

Without rust interop, Valen does this:
 1. Adds each exported function to the Valen instantiation queue.
 2. Iterates over everything in the queue.
 3. When it's done, we've instantiated everything the program indirectly calls.

However, with rust interop, we do it slightly differently:
 * DON'T add anything to the instantiation queue.
 * Return control to rustc.
 * Rustc gets to its instantiator.
 * Rustc starts instantiating the functions it knows about.
    * When doing that, rustc encounters a Valen function.
       * It calls our `per_instance_mir`, which:
          1. Adds _only this_ exported function to the Valen instantiation queue.
          2. Iterate over every _Vale_ function in the Valen instantiation queue, instantiating it. Ignore any rust functions in the Valen instantiation queue (this is the "queue filter").
          3. When it's done, take all the _Rust_ functions from the Valen instantiation queue. Generate a list of them in the mir-rust-instantiation-request-list.

Notes:
 * If two exported Valen functions call the same helper, the normal instantiator machinery will detect it and use the memoized value. Our instantiator is **stateful**, its maps persist between per_instance_mir calls.
 * per_instance_mir will live in src/instantiating/rust_interop.
 * per_instance_mir will have its own `Mutex<InstantiatorState>` state with two things:
    * The InstantiatorI instance.
    * A `mut monouts = InstantiatedOutputsI::new()`.

## Design Proposals

S6/S7 (the `Mutex<InstantiatorState>` + ouroboros state) are **superseded** by the as-built mechanism
in Details: the state lives in the driver's stack frame with real lifetimes, reached through a scoped
thread-local raw pointer, so no `'static`-holding struct (and thus no ouroboros, no leak) is needed.
The Design section above still names `per_instance_mir`'s own `Mutex<InstantiatorState>`; that is the
one part of it the code no longer follows (a lock is only needed once rustc's parallel frontend is
enabled, which the harness does not enable). Flagged for a Design-section read pass.

## Details (as built)

The mechanism is built and reaches Milestone M (rustc drives our monomorphizer; the real Vale bodies
are not emitted yet, so nothing runs). Pieces:

**The one `src/instantiating/` change: the queue filter.** `translate_prototype` gained an
`is_rust_backed` guard: a Rust callee is recorded into a new `monouts.rust_instantiation_requests`
(`IndexMap<IdI, &PrototypeI>`) and its prototype returned, instead of being enqueued for body
translation (which would `vassert_one` a body it has no). Two methods were extracted `pub(crate)` so a
driver can run one export: `instantiate_exported_function` (seed) and `drain_instantiation_queue`
(drain). `translate_program` is otherwise unchanged and is still the non-interop entry.

**The provider (`src/instantiating/rust_interop/`).** `lang_per_instance_mir(tcx, instance)` is
installed via `Callbacks::config`'s `override_queries` behind a `Compilation::Continue` driver. Gated on
`#[vale::emit_consumer_body]` in a `__VALE_STUBS_MARKER` crate, it maps the stub item name (`__vale_<n>`
to export `<n>`) to a `FunctionExportT`, seeds and drains the instantiator, resolves each new request,
and returns a synthetic `Body`: one `ReifyFnPointer` cast per Rust leaf (the thing that queues the leaf
in rustc's collector) plus `Unreachable`. The body never runs; the real body is a backend concern
(single-symbol, arch §5.2).

**Resolution (`resolve_request`).** A free function resolves by crate-qualified path
(`resolve_crate_qualified_path`, made `pub(crate)` in `tyctxt_oracle.rs`); a method through the receiver
type's `inherent_impls` (`receiver_owner` peels ref wrappers for `&self`/`&mut self`); an associated
function through the owner named in the id's init path; a synthesized drop (`name == "drop"`) maps to
the generic `__vale_drop<T>` shim in the stub, with `T` the dropped type (arch §15.7).
`build_generic_args` fills type slots from Vale type args (`kind_to_rustc_ty` lowers primitives, and
Rust-backed citizens to `Adt`s recursively) with lifetimes `re_erased`.

**State without `'static` (supersedes S6/S7).** The instantiator state (arenas, interners, owned
`HinputsT`, and the accumulating `monouts`) lives in the driver's stack frame with real lifetimes. The
provider (a bare `fn`) reaches it through a scoped thread-local raw pointer armed in `after_expansion`
on the rustc thread. The callbacks carry `unsafe impl Send`; this is sound because `run_compiler` joins
its spawned thread synchronously (the calling thread is parked for the whole compile), so the rustc
thread has exclusive access, the `std::thread::scope` guarantee. `monouts` persists across provider
calls, so a shared helper instantiates once. A `Mutex` is only needed if rustc's parallel frontend is
enabled.

**The firing log is per-run, not global.** cargo runs the driven tests in parallel; a shared `static`
log races. It lives in `DriverState`.

## Test cases

Both suites live in `src/typing/test/rust_interop/cases.rs`:

- The `*_reaches_the_instantiator` probes (`run_case_instantiated`, the direct `get_monouts` path)
  assert the instantiator no longer panics on a Rust callee. They un-ignored once the queue filter
  landed.
- The `rustc_collector_drives_*` tests (`run_case_rustc_driven` / `_full`) assert the full inversion:
  rustc fires `per_instance_mir` and every Rust leaf resolves to a real `DefId`. They cover free
  functions, generics (primitives, Rust-type args, multi-param), methods (by-value and
  `&self`/`&mut self`), associated functions (including generic and the real-std `Vec::new`), the drop
  shim, and the composed `domino` case (`A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS`), which
  also asserts rustc exits 0 through codegen.

## Background

### Self-evident from the code
- `instantiator::translate` (`src/instantiating/instantiator.rs:263`) is the monomorphizer entry: it
  builds an `InstantiatorI` and calls `translate_program` (`:283`), which loops the exported functions
  and drains the queue (`new_functions` / `new_impls` / `new_abstract_funcs`) to a fixed point.
- `InstantiatedCompilation::get_monouts` (`src/instantiating/instantiated_compilation.rs`) calls
  `translate` and yields `HinputsI`, the instantiated program the backend consumes.
- `translate_function_callsite` and `translate_prototype` (`src/instantiating/instantiator.rs`) look up
  a callee's `FunctionDefinitionT` with `vassert_one`. A Rust callee has no such definition, so they
  panic with "Expected one element, but was empty."
- `FunctionExternI` (`src/instantiating/ast/ast.rs`) and the `function_externs` field of `HinputsI`
  (`src/instantiating/ast/hinputs.rs`) already exist. The instantiator already emits them for callees
  whose id is `INameT::ExternFunction`.
- `is_rust_backed` (`src/typing/rust_interop/reserved.rs`) identifies a Rust-backed id by the reserved
  `rust` package coordinate.

### Documented
- `docs/architecture/vale-rust-interop-architecture.md` (v0.1.0): §2.8 "Vale tells rustc the leaves;
  rustc walks the rest"; §5.2 single-symbol (our backend emits the real body under the same
  rustc-mangled symbol via `fill_extra_modules`, and a partition filter removes rustc's placeholder) is
  the mechanism behind the Design's "swapped out by our backend"; §19 the `per_instance_mir` provider
  and its `ReifyFnPointer` dep discovery; §13.7 "the instantiator's `per_instance_mir` provider"; §9.5
  transitive Rust deps surface through the nearest exported ancestor.
- `docs/handoffs/rust-interop-handoff.md` (current working tree): the leaf and no-wrapper direction;
  the three interop core touch-points; the two ignored probes.

### Undocumented
- Harmonious (the reference implementation, `toylangc/src/toylang/callbacks_impl.rs`) uses a separate
  `collect_rust_deps_recursive` dep-walker because toylang has no instantiator pass. Vale reuses its
  instantiator instead, because that pass runs anyway to feed the backend.

## Open Questions
(none currently open)


