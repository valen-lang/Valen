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
use crate::typing::rust_interop::declarations::{
    synthesize_extern_function, synthesize_extern_struct,
};
use crate::typing::rust_interop::oracle::{RustItemId, RustOracle, ValeSig, ValeSigType};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT, PrototypeTemplataT};
use crate::typing::types::types::*;
use crate::utils::code_hierarchy::PackageCoordinate;

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

    // Every imported Rust type, as an ordinary struct **declaration**.
    //
    // `IEnvEntryT::Struct` rather than a finished `ITemplataT::Kind`, and that is the whole of what
    // makes generic Rust types work. The indexing phase in `Compiler::evaluate` converts a `Struct`
    // entry into `ITemplataT::StructDefinition`, which is the one arm `solve_call_rule` can apply
    // type arguments to; a `Kind` entry hits a different arm that binds the result and **ignores
    // the arguments entirely**, so two instantiations silently become one type.
    //
    // Registering the declaration also means the ordinary machinery produces the definition:
    // `precompile_struct` and `compile_struct` do the `declare_type` / `add_struct` /
    // environment work this module used to do by hand. That is why `import_rust_types` is gone
    // rather than reduced — synthesized is the degenerate case of parsed, for types exactly as for
    // functions.
    for (type_name, type_item) in oracle.importable_types() {
        let Some(package_coord) = oracle.item_package(type_item) else { continue };
        let human_name = compiler.scout_arena.intern_str(&type_name);
        let template_name = interner.intern_struct_template_name(StructTemplateNameT { human_name });
        let struct_s = synthesize_extern_struct(
            compiler,
            package_coord,
            human_name,
            type_item,
            oracle.type_generic_params(type_item, interner),
        );
        push(
            &mut per_package,
            package_coord,
            (INameT::StructTemplate(template_name), IEnvEntryT::Struct(struct_s)),
        );

        // Every imported type gets a `drop`, synthesized rather than queried, and as an ordinary
        // top-level declaration exactly like its methods.
        //
        // The receiver is the citizen **at its own generic parameters** — `drop<T>(self Holder<T>)`
        // — which is precisely what `ValeSigType::Citizen` exists to express. Before that variant
        // the receiver could only be a settled kind, so a generic type's drop named `Holder` with no
        // arguments and `predict_struct` zipped one parameter against zero arguments.
        //
        // It cannot come from the oracle: rustc answers `None` for any type with no `Drop` impl,
        // which is most of them. And it cannot come from `DeriveStructDrop`, which we suppress —
        // that macro's generated body destructures *members*, and a Rust citizen truthfully has
        // none, so its drop would be an empty destructor that never reaches rustc. `ExternBody`
        // becomes `__vale_drop<T>` → `drop_in_place::<T>` at codegen instead, letting rustc resolve
        // its own drop glue. Returns `Void` because the drop autocall requires `Void` or `Never`.
        let generic_params = oracle.type_generic_params(type_item, interner);
        let receiver = ValeSigType::Citizen {
            name: human_name,
            args: interner.alloc_slice_from_vec(
                (0..generic_params.len()).map(|i| ValeSigType::Generic(i as u32)).collect(),
            ),
        };
        let drop_sig = ValeSig {
            generic_params,
            params: interner.alloc_slice_from_vec(vec![receiver]),
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

