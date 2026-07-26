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
use crate::typing::templata::templata::{ITemplataT, PrototypeTemplataT};
use crate::typing::types::types::*;

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

        let mut entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();

        for (method_name, method_item) in oracle.methods(type_item) {
            let Some(sig) = oracle.fn_sig(method_item, &[], interner) else { continue };
            let entry = make_prototype_entry(
                compiler,
                coutputs,
                template_id,
                package_coord,
                compiler.scout_arena.intern_str(&method_name),
                sig.params,
                sig.ret,
            );
            entries.push(entry);
        }

        // Every imported type gets a `drop`, synthesized rather than queried.
        //
        // `Compiler::drop`'s `KindT::Struct` arm always resolves a destructor call — there is
        // no discard path for a struct — so scope-end drop of a Rust value would otherwise
        // fail with `CouldntFindFunctionToCallT`. Asking rustc for a method named `drop`
        // would answer `None` for any type with no `Drop` impl, which is most of them, so
        // this cannot come from the oracle. Returns `Void` because the drop autocall's
        // post-condition requires `Void` or `Never`. At codegen this lowers to rustc's drop
        // glue, which is a no-op for a type that needs no drop (arch §15.7).
        let drop_entry = make_prototype_entry(
            compiler,
            coutputs,
            template_id,
            package_coord,
            compiler.keywords.drop,
            interner.alloc_slice_from_vec(vec![struct_kind]),
            KindT::Void(VoidT),
        );
        entries.push(drop_entry);

        let mut store = TemplatasStoreBuilder::new(template_id);
        store.add_entries(compiler.scout_arena, entries);
        let templatas = store.build_in(interner);

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
        &'s crate::utils::code_hierarchy::PackageCoordinate<'s>,
        Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>,
    )> = Vec::new();

    for (function_name, item) in oracle.importable_functions() {
        let Some(package_coord) = oracle.item_package(item) else { continue };
        let Some(sig) = oracle.fn_sig(item, &[], interner) else { continue };

        // A declaration, not a resolved prototype. The function-compile phase in
        // `Compiler::evaluate` walks the top-level stores and calls
        // `evaluate_generic_function_from_non_call` on every `IEnvEntryT::Function` it finds, so
        // putting the declaration here is all it takes for a Rust function to be compiled by the
        // ordinary path — and `make_extern_function` mints the prototype at the end of that,
        // once the solver has concrete types.
        // Skipped when the signature mentions a type with no Vale source-level name yet — today
        // that means a Rust-backed citizen, which needs the qualified-name work. Dropping the whole
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
        let entry = (local_name, IEnvEntryT::Function(function_s));

        match per_package.iter_mut().find(|(c, _)| std::ptr::eq(*c, package_coord)) {
            Some((_, entries)) => entries.push(entry),
            None => per_package.push((package_coord, vec![entry])),
        }
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

/// Build an interned prototype for a Rust callee and wrap it as an environment entry.
///
/// The params must ride the NAME, not just the signature: `PrototypeT::param_types`
/// reconstructs them by matching on `id.local_name`, so a prototype whose name disagreed with
/// its signature would silently report wrong param types at every call site.
fn make_prototype_entry<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    owner_template_id: &'t IdT<'s, 't>,
    package_coord: &'s crate::utils::code_hierarchy::PackageCoordinate<'s>,
    human_name: crate::interner::StrI<'s>,
    params: &'t [KindT<'s, 't>],
    ret: KindT<'s, 't>,
) -> (INameT<'s, 't>, IEnvEntryT<'s, 't>)
where
    's: 't,
{
    let interner = compiler.typing_interner;
    let local_name = interner.intern_name(INameValT::ExternFunction(ExternFunctionNameValT {
        human_name,
        template_args: &[],
        parameters: params,
    }));
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
        return_type: ret,
    });

    // An absent bounds entry is not the same as an empty one: `get_candidate_banners_inner`
    // asserts `get_instantiation_bounds(..).is_some()` on every Prototype candidate it
    // accepts, and the drop autocall asserts the same. A Rust item carries no Vale bounds —
    // its own bounds are rustc's to discharge — so the entry is empty, but present.
    coutputs.add_instantiation_bounds(
        compiler.opts.global_options.sanity_check,
        interner,
        *owner_template_id,
        prototype.id,
        interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: interner.alloc_index_map(),
            rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map(),
            rune_to_bound_impl: interner.alloc_index_map(),
        }),
    );

    let templata = ITemplataT::Prototype(interner.alloc(PrototypeTemplataT { prototype }));
    (local_name, IEnvEntryT::Templata(templata))
}
