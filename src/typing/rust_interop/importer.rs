// Declares imported Rust types as ordinary Vale citizens.
//
// This is the piece that lets everything downstream stop knowing about Rust. A Rust type gets
// interned under a `rust`-packaged name, declared with `declare_type`, and given an outer
// environment holding its methods — the same sequence `struct_compiler::precompile_struct`
// runs for a Vale struct, using the same public API. After that, method resolution finds Rust
// methods through the ordinary param-environment path, and drop resolves through ordinary
// overload lookup, with no Rust-specific branch in either.
//
// Each imported denizen becomes an id-only env entry (`StructEnvEntry` / `FunctionEnvEntry`),
// and its actual `StructS` / `FunctionS` is seeded into the postparsed caches under the same
// template id — exactly the shape `Compiler::evaluate`'s index loop builds for a Vale denizen.
// After that, method resolution finds a Rust method by ordinary name lookup, and drop resolves
// through ordinary overload lookup, with no Rust-specific branch in either.
//
// Called once from `Compiler::evaluate`, before `CompilerOutputs::new`, while the four
// postparsed caches are still locals there. It seeds two of them (functions and structs), passed
// in by `&mut`.

use crate::interner::StrI;
use crate::postparsing::ast::{FunctionS, InterfaceS, LocationInDenizen, StructS};
use crate::postparsing::names::{FunctionNameS, IFunctionDeclarationNameS};
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::StructDefinitionT;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::{
  make_top_level_environment, CitizenEnvironmentT, GlobalEnvironmentT, IEnvironmentT,
  IInDenizenEnvironmentT, TemplatasStoreBuilder, TemplatasStoreT,
};
use crate::typing::env::environment::{ImportedItemKind, ResolvedName};
use crate::typing::env::i_env_entry::{
  FunctionEnvEntry, IEnvEntryT, InterfaceEnvEntry, StructEnvEntry,
};
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::names::names::*;
use crate::typing::rust_interop::declarations::{
  synthesize_extern_function, synthesize_extern_interface, synthesize_extern_struct,
  SYNTHESIZED_RANGE_OFFSET,
};
use crate::typing::rust_interop::oracle::{RustItemId, RustOracle, ValeSig, ValeSigType};
use crate::typing::rust_interop::reserved::is_rust_backed;
use crate::typing::templata::templata::{ITemplataT, KindTemplataT, PrototypeTemplataT};
use crate::typing::types::types::*;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::fx::IndexMap;
use crate::utils::range::CodeLocationS;

/// The postparsed node a synthesized Rust type hands back for the `evaluate` loop to seed into the
/// right cache: a `StructS` for a struct, an `InterfaceS` for an enum. A function seeds nothing — it
/// synthesizes lazily on first call.
pub enum RustImportSeed<'s, 't> {
  Struct(&'t IdT<'s, 't>, &'s StructS<'s>),
  Interface(&'t IdT<'s, 't>, &'s InterfaceS<'s>),
}

/// Turn one resolved Rust import into its top-level env entry for the reserved `rust` package.
///
/// A **type** becomes an ordinary struct declaration: an eager opaque `StructS` (returned as a seed
/// for the caller to register in the postparsed cache) plus a `StructEnvEntry`. `IEnvEntryT::Struct`
/// rather than a finished `ITemplataT::Kind` is what makes generic Rust types work — the indexing
/// phase converts it into `ITemplataT::StructDefinition`, the one arm `solve_call_rule` can apply type
/// arguments to. A **free function** becomes an id-only lazy `FunctionEnvEntry` (no `fn_sig`, no
/// synthesis, no seed); `create_postparsed_function` builds it on first call. A type's methods and drop
/// are NOT produced here — they are lazy entries in the type's outer environment, added by
/// `rust_method_entries` when `precompile_struct` builds it. `v.get()` resolves via the receiver's
/// outer env; an associated function is called type-prefixed (`Counter.new()`).
///
/// The name is one `resolve_import` returned, so it resolves; a `None` would be a bug and vfails.
pub fn declare_rust_import<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  name: ResolvedName<'s>,
) -> (INameT<'s, 't>, IEnvEntryT<'s, 't>, Option<RustImportSeed<'s, 't>>)
where
  's: 't,
{
  let interner = compiler.typing_interner;
  let oracle = compiler.oracles.rust.expect("declare_rust_import called without a rust oracle");
  let item = oracle
    .resolve(&name)
    .unwrap_or_else(|| panic!("vfail: a resolved rust import does not resolve: {:?}", name));
  let package_coord = oracle
    .item_package(item)
    .unwrap_or_else(|| panic!("vfail: a resolved rust item has no package: {:?}", name));
  // Every Rust denizen is a top-level denizen of its crate's `rust` package, so its template id is
  // that package id plus the denizen's local name. The same id is both the env entry's `template_id`
  // and the postparsed-cache key, so a later lookup can't drift from the seed.
  let package_id = interner.intern_id(IdValT {
    package_coord,
    init_steps: &[],
    local_name: INameT::PackageTopLevel(
      interner.intern_package_top_level_name(PackageTopLevelNameT {}),
    ),
  });
  let human_name = name.importee_name;

  match name.kind {
    ImportedItemKind::Type => {
      let template_name = interner.intern_struct_template_name(StructTemplateNameT { human_name });
      let struct_local_name = INameT::StructTemplate(template_name);
      let struct_s = synthesize_extern_struct(
        compiler,
        package_coord,
        human_name,
        oracle.type_generic_params(item, interner),
      );
      let struct_template_id = package_id.add_step(interner, struct_local_name);
      (
        struct_local_name,
        IEnvEntryT::Struct(StructEnvEntry {
          template_id: struct_template_id,
          tyype: struct_s.tyype,
        }),
        Some(RustImportSeed::Struct(struct_template_id, struct_s)),
      )
    }
    ImportedItemKind::Function => {
      let function_local_name = lazy_extern_function_local_name(compiler, human_name);
      let function_template_id = package_id.add_step(interner, function_local_name);
      (
        function_local_name,
        IEnvEntryT::Function(FunctionEnvEntry { template_id: function_template_id }),
        None,
      )
    }
    ImportedItemKind::Enum => {
      // A Rust enum imports as an opaque sealed interface — the interface analog of the `Type` arm.
      let template_name =
        interner.intern_interface_template_name(InterfaceTemplateNameT { human_namee: human_name });
      let interface_local_name = INameT::InterfaceTemplate(template_name);
      let interface_s = synthesize_extern_interface(
        compiler,
        package_coord,
        human_name,
        oracle.type_generic_params(item, interner),
      );
      let interface_template_id = package_id.add_step(interner, interface_local_name);
      (
        interface_local_name,
        IEnvEntryT::Interface(InterfaceEnvEntry {
          template_id: interface_template_id,
          tyype: interface_s.tyype,
        }),
        Some(RustImportSeed::Interface(interface_template_id, interface_s)),
      )
    }
  }
}

/// The local name a lazily-registered Rust function or method carries, minted WITHOUT synthesizing its
/// `FunctionS`. It is byte-identical to what `synthesize_extern_function`'s output name would translate
/// to (same human name, same shared synthetic location), so the id-only entry's template id and the
/// eventually-built `FunctionS` agree — the same consistency the eager path gets for free. The rustc item
/// is no longer encoded in the location; `create_postparsed_function` recovers it by re-resolving the
/// id's canonical name.
fn lazy_extern_function_local_name<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  human_name: StrI<'s>,
) -> INameT<'s, 't>
where
  's: 't,
{
  let loc = CodeLocationS::internal(compiler.scout_arena, SYNTHESIZED_RANGE_OFFSET);
  let name_s = IFunctionDeclarationNameS::FunctionName(FunctionNameS {
    imprecise_name: compiler.scout_arena.intern_code_name(human_name),
    code_location: loc,
    lid: LocationInDenizen { path: &[] },
  });
  match compiler.translate_generic_function_name(name_s) {
    IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
    other => panic!("lazy extern function got an unexpected template name shape: {:?}", other),
  }
}

/// The canonical `ResolvedName` a top-level Rust id carries, or `None` if its local name is neither a
/// struct template nor a function template. `rust_interop` hands this to `oracle.resolve` to recover the
/// rustc item, which is what lets the offset-encoding trick go away: the id already names the item.
fn resolved_name_of<'s, 't>(
  package_coord: &'s PackageCoordinate<'s>,
  local_name: INameT<'s, 't>,
) -> Option<ResolvedName<'s>>
where
  's: 't,
{
  let (importee_name, kind) = match local_name {
    INameT::StructTemplate(t) => (t.human_name, ImportedItemKind::Type),
    INameT::InterfaceTemplate(t) => (t.human_namee, ImportedItemKind::Enum),
    INameT::FunctionTemplate(t) => (t.human_name, ImportedItemKind::Function),
    _ => return None,
  };
  Some(ResolvedName {
    module_name: package_coord.module,
    package_names: package_coord.packages.as_slice(),
    importee_name,
    kind,
  })
}

/// Build a lazily-registered Rust function's `FunctionS` on its first lookup, called by
/// `Compiler::get_or_create_postparsed_function` on a cache miss. Recovers the rustc item by
/// re-resolving the canonical name the template id carries (no offset decoding), queries its signature,
/// synthesizes the declaration, and registers it under the same id.
///
/// A free function's own id resolves directly. A method nests under its owner type, so its owner is
/// resolved first and the method found among that type's `methods` by name.
///
/// Returns `None` when the id is not a Rust-backed function template (a genuine bug, surfaced by the
/// caller's vfail) or when the signature declines — a called function whose type Vale cannot name. That
/// decline path is out of scope for now (Vale2's callsite/overload rework owns graceful errors), so the
/// caller's vfail is the interim behavior.
pub fn create_postparsed_function<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  // Read-only handle: rust_interop is a pure producer of the postparsed `FunctionS`; core
  // (`get_or_create_postparsed_function`) owns registering it and queuing its deferred compile.
  _coutputs: &CompilerOutputs<'s, 't>,
  template_id: &'t IdT<'s, 't>,
) -> Option<&'s FunctionS<'s>>
where
  's: 't,
{
  if !is_rust_backed(template_id) {
    return None;
  }
  let oracle = compiler.oracles.rust?;
  let interner = compiler.typing_interner;
  let function_template_name = match template_id.local_name {
    INameT::FunctionTemplate(r) => r,
    _ => return None,
  };
  let human_name = function_template_name.human_name;

  let sig = if template_id.init_steps.is_empty() {
    // A top-level free function: its own canonical name resolves directly to the item.
    let name = resolved_name_of(template_id.package_coord, template_id.local_name)?;
    let item = oracle.resolve(&name)?;
    oracle.fn_sig(item, interner)?
  } else {
    // A denizen nested under its owner type (`OwnerTemplate.add_step(name)`): resolve the owner.
    let owner_local_name = *template_id.init_steps.last()?;
    let owner_name = resolved_name_of(template_id.package_coord, owner_local_name)?;
    let owner_item = oracle.resolve(&owner_name)?;
    // ataflbz-allow: not Rust-item identity — dispatching on a Vale keyword (`drop`) to pick the
    // synthesis path, since rustc has no drop method to resolve. The owner's identity is its `DefId`.
    let is_drop = human_name == compiler.keywords.drop; // ataflbz-allow: keyword dispatch
    if is_drop {
      // A drop is a method with no rustc signature to query: manufacture `drop(self Owner<T…>)
      // void`. The receiver is the owner at its own generic parameters (`ValeSigType::Citizen`),
      // resolved from the owner type item, exactly as the old eager drop built it.
      let owner_human_name = match owner_local_name {
        INameT::StructTemplate(t) => t.human_name,
        INameT::InterfaceTemplate(t) => t.human_namee,
        _ => return None,
      };
      let generic_params = oracle.type_generic_params(owner_item, interner);
      let receiver = ValeSigType::Citizen {
        name: owner_human_name,
        package: template_id.package_coord,
        args: interner.alloc_slice_from_vec(
          (0..generic_params.len()).map(|i| ValeSigType::Generic(i as u32)).collect(),
        ),
      };
      ValeSig {
        generic_params,
        params: interner.alloc_slice_from_vec(vec![receiver]),
        ret: ValeSigType::Kind(KindT::Void(VoidT)),
      }
    } else {
      // A regular method: find it among the owner's `methods` by name.
      let method_item = oracle
        .methods(owner_item)
        .into_iter()
        // ataflbz-allow: selection, not identity — one type's methods have unique names, so this
        // picks the right method; its identity is still the `DefId` behind the returned item.
        .find(|(n, _)| n.as_str() == human_name.0) // ataflbz-allow: selection
        .map(|(_, item)| item)?;
      oracle.fn_sig(method_item, interner)?
    }
  };

  let function_s =
    synthesize_extern_function(compiler, template_id.package_coord, human_name, &sig)?;
  Some(function_s)
}

/// The id-only method entries that belong in a Rust type's outer environment (Vale's home for a type's
/// methods and associated functions). Chained into `precompile_struct`'s outer store for every citizen;
/// empty for a Vale struct. Each is lazy — no `fn_sig`, no synthesis — like a lazily-imported free
/// function, and synthesizes on first call through `create_postparsed_function`. Its template id nests
/// under the type's (`struct_template_id.add_step(method_name)`), the shape a Vale internal method uses;
/// the citizen-compile loop skips these Rust-backed entries so they are not force-compiled.
///
/// The type's rustc item is recovered by re-resolving the type's own canonical name (carried by
/// `struct_template_id`), so the synthesized `StructS`'s range no longer needs to encode it.
pub fn rust_method_entries<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  struct_template_id: &'t IdT<'s, 't>,
) -> Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>
where
  's: 't,
{
  if !is_rust_backed(struct_template_id) {
    return Vec::new();
  }
  let Some(oracle) = compiler.oracles.rust else { return Vec::new() };
  let interner = compiler.typing_interner;
  let Some(owner_name) =
    resolved_name_of(struct_template_id.package_coord, struct_template_id.local_name)
  else {
    return Vec::new();
  };
  let Some(type_item) = oracle.resolve(&owner_name) else { return Vec::new() };
  let mut entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = oracle
    .methods(type_item)
    .into_iter()
    .map(|(method_name, method_item)| {
      let human_name = compiler.scout_arena.intern_str(&method_name);
      let local_name = lazy_extern_function_local_name(compiler, human_name);
      let method_id = struct_template_id.add_step(interner, local_name);
      (local_name, IEnvEntryT::Function(FunctionEnvEntry { template_id: method_id }))
    })
    .collect();

  // Every imported type gets a `drop`, an id-only lazy entry nested in the type's env exactly like a
  // method (drop is just a method — no special case in the store). rustc exposes no drop signature, so
  // `create_postparsed_function` manufactures `drop(self Owner<T…>) void` on force. The minted name
  // carries the *type* (owner) item, which the create hook resolves the receiver's generics from.
  let drop_local_name = lazy_extern_function_local_name(compiler, compiler.keywords.drop);
  let drop_id = struct_template_id.add_step(interner, drop_local_name);
  entries.push((drop_local_name, IEnvEntryT::Function(FunctionEnvEntry { template_id: drop_id })));
  entries
}
