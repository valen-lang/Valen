// Declares imported Rust types as ordinary Vale citizens.
//
// This is the piece that lets everything downstream stop knowing about Rust. A Rust type gets
// interned under a `rust`-packaged name, declared with `declare_type`, and given an outer
// environment holding its methods — the same sequence `struct_compiler::precompile_struct`
// runs for a Vale struct, using the same public API. After that, method resolution finds Rust
// methods through the ordinary param-environment path, and drop resolves through ordinary
// overload lookup, with no Rust-specific branch in either.
//
// Two differences from the Vale version, both semantic rather than expedient:
//
//   - The store holds `IEnvEntryT::Templata(ITemplataT::Prototype(..))` rather than
//     `IEnvEntryT::Function(&FunctionS)`. A Rust method has no Vale AST behind it; a
//     prototype is exactly what a call site points at. The `Templata` arm already exists —
//     it is how primitives like `int` and `Array` get into the builtins store.
//   - `sibling_entries` is empty. Vale pulls in the declaring package's siblings because
//     Vale methods are UFCS, so a free `func doSomething(b &Bork)` next to `struct Bork` is
//     callable as `b.doSomething()`. Rust has no UFCS — its methods come from inherent impls
//     and in-scope traits — so there are no siblings to pull in. Simpler, and correct.
//
// Called once from `Compiler::evaluate`, after the global environment exists (a
// `CitizenEnvironmentT` needs one, and a Rust type has no declaring env to inherit it from)
// and after `CompilerOutputs` exists (registering a prototype's instantiation bounds needs
// `&mut`).

use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::StructDefinitionT;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::{
    make_top_level_environment, CitizenEnvironmentT, GlobalEnvironmentT, IEnvironmentT,
    IInDenizenEnvironmentT, TemplatasStoreBuilder, TemplatasStoreT,
};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::names::names::*;
use crate::typing::rust_interop::declarations::synthesize_extern_function;
use crate::typing::rust_interop::oracle::{RustItemId, RustOracle, ValeSig, ValeSigType};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT, PrototypeTemplataT};
use crate::typing::types::types::*;
use crate::utils::code_hierarchy::PackageCoordinate;

/// Declare every importable Rust type. A no-op when no Rust oracle is present, which is
/// every ordinary compilation.
pub fn import_rust_types<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    global_env: &'t GlobalEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
) where
    's: 't,
{
    let Some(oracle) = compiler.oracles.rust else { return };
    let interner = compiler.typing_interner;

    for (type_name, type_item) in oracle.importable_types() {
        let human_name = compiler.scout_arena.intern_str(&type_name);

        // Five interner calls and a Rust struct is an ordinary Vale struct-kind. No new
        // `KindT` arm, no new name type — "Rust-backed" is carried entirely by the reserved
        // `rust` package coordinate on the id.
        let template_name =
            interner.intern_struct_template_name(StructTemplateNameT { human_name });
        let struct_name = interner.intern_struct_name(StructNameValT {
            template: IStructTemplateNameT::StructTemplate(template_name),
            template_args: &[],
        });
        let Some(package_coord) = oracle.item_package(type_item) else { continue };
        let struct_id = interner.intern_id(IdValT {
            package_coord,
            init_steps: &[],
            local_name: INameT::Struct(struct_name),
        });
        let struct_kind = KindT::Struct(interner.intern_struct_tt(StructTTValT { id: *struct_id }));

        // The env is keyed by the *template* id, because that is what `get_struct_template`
        // derives from an instance id and what `get_param_environments` looks up.
        let template_id = interner.intern_id(IdValT {
            package_coord,
            init_steps: &[],
            local_name: INameT::StructTemplate(template_name),
        });

        coutputs.declare_type(template_id);
        coutputs.declare_type_sharedness(template_id, SharednessT::Single);

        // A real definition, with zero members and an `Extern` attribute.
        //
        // The attribute is the honest way to say "this type is foreign": every site that wants to
        // know reads it off the definition, the same as for a hand-written `extern struct`. The
        // alternative — teaching each of those sites to recognise a `rust`-packaged id — spreads a
        // Rust-specific special case across the core one site at a time, which is precisely what
        // this design exists to avoid.
        //
        // Zero members is not a stub, it is the truth: Vale is an external consumer of a Rust
        // type, so its layout is opaque and its private fields are none of Vale's business.
        // `struct_hammer`'s `translate_opaque_i` asserts exactly that emptiness downstream, and
        // both sibling implementations converged on the same shape after trying a synthetic
        // blob-member and abandoning it.
        let empty_bounds = interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: interner.alloc_index_map(),
            rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map(),
            rune_to_bound_impl: interner.alloc_index_map(),
        });
        coutputs.add_struct(interner.alloc(StructDefinitionT {
            template_name: *template_id,
            instantiated_citizen: *interner.intern_struct_tt(StructTTValT { id: *struct_id }),
            attributes: interner.alloc_slice_from_vec(vec![ICitizenAttributeT::Extern(ExternT {
                package_coord: *package_coord,
            })]),
            weakable: false,
            sharedness: SharednessT::Single,
            members: interner.alloc_slice_from_vec(Vec::new()),
            is_closure: false,
            instantiation_bound_params: empty_bounds,
        }));

        // The kind itself needs bounds registered, not just its methods. Substituting into a
        // struct rebuilds its `StructTT` and unwraps the original's bounds to translate them
        // (`substitute_templatas_in_struct`), and that runs even for a non-generic callee. A
        // Rust type carries no Vale bounds — rustc discharges its own — but an absent entry
        // is not the same as an empty one.
        for id in [*struct_id, *template_id] {
            coutputs.add_instantiation_bounds(
                compiler.opts.global_options.sanity_check,
                interner,
                *template_id,
                id,
                interner.alloc(InstantiationBoundArgumentsT {
                    rune_to_bound_prototype: interner.alloc_index_map(),
                    rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map(),
                    rune_to_bound_impl: interner.alloc_index_map(),
                }),
            );
        }

        // The outer env is empty, and that is the point.
        //
        // A citizen's outer env exists so `get_param_environments` can find the methods declared
        // inside its braces. Rust methods are not declared there — they are ordinary top-level
        // declarations taking the receiver as their first parameter (`rust_package_stores`), the
        // way Vale's own UFCS wants. Putting them here as well produced two candidates for every
        // call and a `CouldntNarrowDownCandidates` on the first method call, which is how the
        // duplication announced itself.
        //
        // The env still has to *exist*: `get_param_environments` reaches for it unconditionally
        // for a `KindT::Struct` argument and `get_outer_env_for_type` panics on absence.
        let templatas = TemplatasStoreBuilder::new(template_id).build_in(interner);

        // The parent is the reserved `rust` package's top-level env. A Vale citizen inherits
        // its declaring env; a Rust type has none, so we build the one it would have had.
        let package_top_level = INameT::PackageTopLevel(
            interner.intern_package_top_level_name(PackageTopLevelNameT {}),
        );
        let package_id = interner.intern_id(IdValT {
            package_coord,
            init_steps: &[],
            local_name: package_top_level,
        });
        let parent_env = make_top_level_environment(global_env, *package_id, interner);

        let outer_env = interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: IEnvironmentT::Package(parent_env),
            template_id: *template_id,
            id: *template_id,
            templatas,
        });
        coutputs.declare_type_outer_env(template_id, IInDenizenEnvironmentT::Citizen(outer_env));

        // An inner env too, empty.
        //
        // For a Vale citizen the inner env holds every rune its definition solve concluded, and
        // `check_defining_conclusions_and_resolve` (`infer_compiler.rs:491`) walks it to harvest
        // reachable bound prototypes from any citizen a signature mentions. It reaches for the
        // inner env of *every* such citizen, unconditionally — so the moment a synthesized
        // declaration names a Rust type, that type needs one or the lookup unwraps `None`.
        //
        // Empty is the honest content: a Rust citizen has no Vale-side runes to conclude, and no
        // Vale bounds — rustc discharges its own. As with instantiation bounds, an absent entry
        // and an empty one mean different things, and only the empty one is true here.
        let inner_store = TemplatasStoreBuilder::new(template_id).build_in(interner);
        let inner_env = interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: IEnvironmentT::Citizen(outer_env),
            template_id: *template_id,
            id: *template_id,
            templatas: inner_store,
        });
        coutputs.declare_type_inner_env(template_id, IInDenizenEnvironmentT::Citizen(inner_env));
    }
}

/// Build the reserved `rust` package's top-level store: every importable free function, as a
/// prototype entry.
///
/// This is what retires the overload-resolution hook. With these names in
/// `name_to_top_level_environment`, a Rust free function is found by ordinary ambient name
/// lookup — the same path that finds any Vale function — instead of by a Rust-specific
/// candidate source. Nothing asks the oracle per call site any more: the store either has the
/// name or it doesn't, at ordinary lookup cost.
///
/// Scoping is membership in this store. That is not a weaker guarantee than an import check —
/// it *is* the import list, materialized.
///
/// Returns one store per distinct package coordinate, since imported items may come from more
/// than one Rust crate. Empty when there is no oracle, which is every ordinary compilation.
pub fn rust_package_stores<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
) -> Vec<(&'t IdT<'s, 't>, &'t TemplatasStoreT<'s, 't>)>
where
    's: 't,
{
    let Some(oracle) = compiler.oracles.rust else { return Vec::new() };
    let interner = compiler.typing_interner;

    // Group by package coord: one top-level store per Rust crate.
    let mut per_package: Vec<(
        &'s PackageCoordinate<'s>,
        Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>,
    )> = Vec::new();

    let push = |per_package: &mut Vec<(&'s PackageCoordinate<'s>, Vec<_>)>,
                    package_coord: &'s PackageCoordinate<'s>,
                    entry: (INameT<'s, 't>, IEnvEntryT<'s, 't>)| {
        match per_package.iter_mut().find(|(c, _)| std::ptr::eq(*c, package_coord)) {
            Some((_, entries)) => entries.push(entry),
            None => per_package.push((package_coord, vec![entry])),
        }
    };

    // Every imported Rust type, under its own human name.
    //
    // This is what makes a Rust type nameable, and it is the same shape the builtins store uses to
    // put `int` and `bool` in scope: a bare `Kind` templata, resolved by an ordinary `LookupSR`
    // with no `CallSR` to apply arguments to. Being in a *top-level* store means the name resolves
    // ambiently, which is what a top-level declaration requires — a per-type environment would only
    // be reachable from inside that type's own methods, and under this design there are none.
    for (type_name, type_item) in oracle.importable_types() {
        let Some(package_coord) = oracle.item_package(type_item) else { continue };
        let human_name = compiler.scout_arena.intern_str(&type_name);
        let template_name = interner.intern_struct_template_name(StructTemplateNameT { human_name });
        let struct_name = interner.intern_struct_name(StructNameValT {
            template: IStructTemplateNameT::StructTemplate(template_name),
            template_args: &[],
        });
        let struct_id = interner.intern_id(IdValT {
            package_coord,
            init_steps: &[],
            local_name: INameT::Struct(struct_name),
        });
        let kind = KindT::Struct(interner.intern_struct_tt(StructTTValT { id: *struct_id }));
        push(
            &mut per_package,
            package_coord,
            (
                INameT::StructTemplate(template_name),
                IEnvEntryT::Templata(ITemplataT::Kind(interner.alloc(KindTemplataT { kind }))),
            ),
        );

        // Every imported type gets a `drop`, synthesized rather than queried — and as an ordinary
        // top-level declaration, exactly like its methods.
        //
        // `Compiler::drop`'s `KindT::Struct` arm always resolves a destructor call; there is no
        // discard path for an owned struct, so a Rust value going out of scope needs a `drop` to
        // resolve or the program fails with `CouldntFindFunctionToCallT`. It cannot come from the
        // oracle: rustc would answer `None` for any type with no `Drop` impl, which is most of
        // them. Returns `Void` because the drop autocall requires `Void` or `Never`.
        //
        // At codegen this becomes a call to the `__vale_drop<T>` wrapper (arch §1.7), where
        // `drop_in_place::<T>` lets rustc resolve its own drop glue — a no-op for a type that
        // needs no drop. Nothing here has to name that glue, which is the point of the wrapper.
        //
        // The type's own item id supplies the declaration's identity. Nothing else uses it — the
        // type itself is a `Kind` entry, which carries no location — so it is free and unique.
        let drop_sig = ValeSig {
            generic_params: interner.alloc_slice_from_vec(Vec::new()),
            params: interner.alloc_slice_from_vec(vec![ValeSigType::Kind(kind)]),
            ret: ValeSigType::Kind(KindT::Void(VoidT)),
        };
        if let Some(drop_s) = synthesize_extern_function(
            compiler,
            package_coord,
            compiler.keywords.drop,
            type_item,
            &drop_sig,
        ) {
            let local_name = match compiler.translate_generic_function_name(drop_s.name) {
                IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
                other => panic!("synthesized drop got an unexpected name shape: {:?}", other),
            };
            push(&mut per_package, package_coord, (local_name, IEnvEntryT::Function(drop_s)));
        }
    }

    // Free functions and methods become the same thing: an ordinary top-level declaration whose
    // first parameter is the receiver, if it has one.
    //
    // Vale erases method syntax in the postparser — `v.get()` becomes an overload call with the
    // subject spliced in as argument zero — so a method-shaped declaration would buy nothing and
    // cost an asymmetry. One code path, for that reason.
    let mut declarations: Vec<(String, RustItemId)> = oracle.importable_functions();
    for (_, type_item) in oracle.importable_types() {
        declarations.extend(oracle.methods(type_item));
    }

    for (function_name, item) in declarations {
        let Some(package_coord) = oracle.item_package(item) else { continue };
        let Some(sig) = oracle.fn_sig(item, interner) else { continue };

        // A declaration, not a resolved prototype. The function-compile phase in
        // `Compiler::evaluate` walks the top-level stores and calls
        // `evaluate_generic_function_from_non_call` on every `IEnvEntryT::Function` it finds, so
        // putting the declaration here is all it takes for a Rust function to be compiled by the
        // ordinary path — and `make_extern_function` mints the prototype at the end of that,
        // once the solver has concrete types.
        //
        // Skipped when the signature mentions something with no Vale source-level name — an
        // associated-type projection, or a type Vale's IR cannot express. Dropping the whole
        // declaration is deliberate: it makes the function un-importable rather than importable
        // with a wrong type.
        let Some(function_s) = synthesize_extern_function(
            compiler,
            package_coord,
            compiler.scout_arena.intern_str(&function_name),
            item,
            &sig,
        ) else {
            continue;
        };
        let local_name = match compiler.translate_generic_function_name(function_s.name) {
            IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
            other => panic!(
                "synthesized extern declaration got an unexpected template name shape: {:?}",
                other
            ),
        };
        push(&mut per_package, package_coord, (local_name, IEnvEntryT::Function(function_s)));
    }

    per_package
        .into_iter()
        .map(|(package_coord, entries)| {
            let package_id = interner.intern_id(IdValT {
                package_coord,
                init_steps: &[],
                local_name: INameT::PackageTopLevel(
                    interner.intern_package_top_level_name(PackageTopLevelNameT {}),
                ),
            });
            let mut store = TemplatasStoreBuilder::new(package_id);
            store.add_entries(compiler.scout_arena, entries);
            (package_id, store.build_in(interner))
        })
        .collect()
}

