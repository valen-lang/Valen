// The per-question seam helpers that the `#[cfg(feature = "rust_interop")]` hooks in the core
// typing files delegate to.
//
// Each returns `Option`: `None` means "not a Rust question, carry on as before", so
// the core hook is a single delegating line and all the interop logic lives here.

use crate::interner::StrI;
use crate::postparsing::names::IImpreciseNameS;
use crate::typing::ast::ast::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::names::names::*;
use crate::typing::overload_resolver::AttemptedCandidate;
use crate::typing::rust_interop::oracle::RustFieldInfo;
use crate::typing::rust_interop::reserved::{citizen_id, is_rust_backed};
use crate::typing::types::types::*;

/// Contribute a matching Rust function, if any, as an ordinary overload candidate.
///
/// This is a **candidate source**, sitting alongside the calling env, the param
/// envs, and the placeholder extra-call envs in `get_candidate_banners`. It is
/// deliberately not a fallback on resolution failure: a Rust callee should compete
/// with same-named Vale functions on equal footing and go through `params_match`,
/// scoring, and `narrow_down_callable_overloads` like any other candidate. A
/// failure-branch fallback would make a Rust callee invisible whenever any Vale
/// function of the same name matched loosely.
///
/// Two triggers, because a Rust callee reaches us two different ways:
///
/// - **A method on a Rust-backed receiver** (`my_vec.push(x)`). Vale methods are UFCS,
///   so this arrives as `push(my_vec, x)` and the receiver is `param_filters[0]`.
/// - **A free function** (`add_two_numbers(3, 4)`). There is no Rust-backed argument at
///   all — both args are plain ints — so nothing about the *types* signals Rust. The
///   name is the only signal, and which names are in scope is the oracle's to know.
///
/// What we contribute is a *prototype* — the thing a call site points at — never a
/// definition. Vale holds no `FunctionDefinitionT` for a Rust callee; there is no body
/// on this side of the boundary.
pub fn push_rust_call_candidates<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    function_name: IImpreciseNameS<'s>,
    param_filters: &[KindT<'s, 't>],
    results: &mut Vec<ICalleeCandidate<'s, 't>>,
) {
    let IImpreciseNameS::CodeName(code_name) = function_name else { return };
    let callee_name: StrI<'s> = code_name.name;

    // A method on a Rust-backed receiver is NOT contributed here. The importer gives every
    // imported type an outer environment holding its methods, so `get_param_environments`
    // already finds them through the ordinary path — and contributing them here as well
    // produces two candidates for one method, which overload resolution cannot choose
    // between (`CouldntNarrowDownCandidates`). Free functions have no receiver to hang an
    // environment off, so they still need this.
    //
    // citizen_id looks through the reference onion to the receiver's kind.
    let receiver_is_rust_backed = param_filters
        .first()
        .and_then(|kind| citizen_id(*kind))
        .is_some_and(|id| is_rust_backed(&id));
    if receiver_is_rust_backed {
        return;
    }

    // No Rust oracle means nothing to ask — a compilation with no Rust dependencies, or a
    // test about Vale semantics.
    let Some(oracle) = compiler.oracles.rust else { return };

    let Some(item) = oracle.resolve_function(callee_name.0) else { return };

    // @EarlyBinder: fn_sig instantiates at the call's args before lowering. A signature
    // lowered pre-instantiation would carry the wrong substitution into every later use.
    let Some(sig) = oracle.fn_sig(item, param_filters, compiler.typing_interner)
        else { return };

    let interner = compiler.typing_interner;

    // The params must ride the NAME, not just the signature: PrototypeT::param_types
    // (ast.rs:417) reconstructs them by matching on id.local_name. A prototype whose
    // name disagreed with its signature would silently report the wrong param types at
    // every call site, so both are built from one `sig.params` here.
    //
    // ExternFunction is the right existing variant — from Vale's side a Rust method
    // *is* a function defined elsewhere with no Vale body, which is exactly what the
    // C-extern path already models (function_compiler_core.rs:336). Two known
    // consequences: IFunctionNameT::template() panics for this variant (names.rs:472),
    // and template_args() returns &[] (names.rs:488), so a Rust method cannot yet carry
    // generic args of its own — only those already on the receiver kind.
    let local_name = interner.intern_name(INameValT::ExternFunction(ExternFunctionNameValT {
        human_name: callee_name,
        template_args: &[],
        parameters: sig.params,
    }));

    // A free function has no owning type to nest under, so it sits directly in the package
    // the oracle reports — `rust.mycrate` / [] / add_two_numbers. The id is in the reserved
    // `rust` package, so is_rust_backed holds for the prototype too.
    let Some(package_coord) = oracle.item_package(item) else { return };
    let id = interner.intern_id(IdValT {
        package_coord,
        init_steps: &[],
        local_name,
    });

    let prototype = interner.intern_prototype(PrototypeValT {
        id: IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: id.local_name,
        },
        return_type: sig.ret,
    });

    // A Rust item carries no Vale bounds — its own bounds are rustc's to discharge.
    // But an absent entry is not the same as an empty one: get_candidate_banners_inner
    // asserts get_instantiation_bounds(..).is_some() on every Prototype candidate it
    // accepts (overload_resolver.rs:220), and ~9 more sites assert the same downstream.
    // Registering here is why this is a candidate source rather than an environment:
    // env lookup has no &mut CompilerOutputs, and get_outer_env_for_type takes &self.
    coutputs.add_instantiation_bounds(
        compiler.opts.global_options.sanity_check,
        interner,
        calling_env.denizen_template_id(),
        prototype.id,
        interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: interner.alloc_index_map(),
            rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map(),
            rune_to_bound_impl: interner.alloc_index_map(),
        }),
    );

    results.push(ICalleeCandidate::PrototypeTemplata(
        PrototypeTemplataCalleeCandidate { prototype_t: *prototype }));
}

/// Read a `pub` field off a Rust-backed struct.
///
/// `None` means either "not Rust-backed" (carry on with the Vale definition lookup)
/// or "no such public field". Distinguishing those two — so a private Rust field gets
/// its own diagnostic rather than falling through to `CouldntFindMemberT` — needs a
/// dedicated error variant and lands with the rest of the field seam.
pub fn maybe_rust_field<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    owner: &IdT<'s, 't>,
    field_name: &str,
) -> Option<RustFieldInfo<'s, 't>> {
    if !is_rust_backed(owner) {
        return None;
    }
    compiler.oracles.rust?.field(owner, field_name)
}
