// Synthesizes ordinary Vale declarations from oracle data.
//
// This is the piece that makes the rest of the compiler stop knowing about Rust. A Rust free
// function becomes a `FunctionS` whose body kind is `IBodyS::ExternBody` — the same shape the
// postparser produces for a hand-written `extern func add_two_numbers(a int, b int) int;`. From
// there nothing is Rust-specific: the function-compile phase in `Compiler::evaluate` picks it up
// out of the top-level store like any Vale function, the solver resolves its rules, and
// `make_extern_function` mints the concrete `PrototypeT` and registers its instantiation bounds
// at that point — per instantiation, with real arguments.
//
// That timing is the whole reason for this file. The design it replaces built a finished
// `PrototypeT` at environment-build time from `fn_sig(item, &[])`, which cannot represent a
// generic Rust function (`fn pick<A, B>(a: A, b: B) -> A` has no single signature, only one per
// instantiation) and which @ECSIIOSZ and @BDPFWDZ already forbid.
//
// @SMLRZ: a synthesized declaration must be structurally indistinguishable from what the
// postparser produces for the equivalent hand-written Vale source. If the oracle's knowledge of
// Rust's shape is visible anywhere in the `FunctionS`, Rust's rendering has been baked into the
// typing pass — the mistake ValeRustInterop made and had to roll back wholesale. The `Extern`
// attribute and the `ExternBody` below are exactly what `function_scout` attaches; nothing else
// about the declaration records that rustc was involved.

use crate::interner::StrI;
use crate::postparsing::ast::{
    ExternBodyS, ExternS, FunctionS, IBodyS, IFunctionAttributeS, ParameterS,
};
use crate::postparsing::itemplatatype::{FunctionTemplataType, ITemplataType, TemplateTemplataType};
use crate::postparsing::names::{
    ArgumentRuneS, CodeNameS, FunctionNameS, IFunctionDeclarationNameS, IImpreciseNameValS,
    IRuneValS, IVarNameS, ReturnRuneS,
};
use crate::postparsing::rules::rules::{IRulexSR, LookupSR, RuneUsage};
use crate::typing::compiler::Compiler;
use crate::typing::rust_interop::oracle::{RustItemId, ValeSig};
use crate::typing::types::types::*;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};

/// Synthesize the declaration for one importable Rust free function.
///
/// `item` supplies the declaration's identity. Every synthesized denizen must get its **own**
/// `CodeLocationS`, because four separate things key on it and one of them fails silently:
///
///   - `FunctionTemplataT`'s hand-rolled eq/hash is `(function.range, function.name)` and
///     ignores the environment the declaration came from (`templata.rs:162-176`);
///   - overload candidates are deduped through a `HashSet` over exactly that
///     (`overload_resolver.rs:576`, `:191`);
///   - `FunctionNameS` carries the location, and it flows into `FunctionTemplateNameT`;
///   - `ExternTemplateNameT`'s only field is a code location (`names.rs:1236`).
///
/// So two synthesized externs sharing a sentinel location collapse into one candidate with no
/// error at all — and `Vec::new`, `String::new`, `Box::new` all have the human name `new`. The
/// in-tree macros dodge this by borrowing their citizen's *real* range and using sentinels only
/// on rules and params, which are never identity keys; we have no real range to borrow.
///
/// `RustItemId` is an index into the oracle's item table, so it is unique within a compilation by
/// construction and reproducible across identical runs. (Deriving from `tcx.def_path_hash` instead
/// would additionally survive a change in item ordering between builds — worth doing if these
/// locations ever reach a symbol name, which today they do not: ordinary function templates drop
/// the location at the backend.)
///
/// `None` when any type in the signature has no Vale source-level name yet — see `vale_type_name`.
pub fn synthesize_extern_function<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    package_coord: &'s PackageCoordinate<'s>,
    human_name: StrI<'s>,
    item: RustItemId,
    sig: &ValeSig<'s, 't>,
) -> Option<&'s FunctionS<'s>>
where
    's: 't,
{
    let scout_arena = compiler.scout_arena;

    // Negative offsets are the established convention for compiler-synthesized ranges, and
    // `CodeLocationS::internal` asserts on it. One distinct offset per item, per the doc comment.
    let offset = -1_000_000 - (item.0 as i32);
    let loc = CodeLocationS::internal(scout_arena, offset);
    let range = RangeS::new(loc, loc);

    let mut rules: Vec<IRulexSR<'s>> = Vec::new();

    // Each param and the return get a rune bound by an ordinary `LookupSR` to the Vale name of
    // its type. A primitive resolves straight to a kind — the builtins store holds `int` as
    // `IEnvEntryT::Templata(ITemplataT::Kind(..))` — so no `CallSR` is needed to apply a template
    // to arguments, which is the extra rule the citizen-shaped macros have to emit.
    let mut params: Vec<ParameterS<'s>> = Vec::new();
    for (index, param_kind) in sig.params.iter().enumerate() {
        let rune = scout_arena.intern_rune(IRuneValS::ArgumentRune(ArgumentRuneS {
            arg_index: index as i32,
        }));
        let usage = RuneUsage { range, rune };
        rules.push(IRulexSR::Lookup(LookupSR {
            range,
            rune: usage,
            name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
                name: vale_type_name(compiler, param_kind)?,
            })),
        }));
        params.push(ParameterS::new(
            range,
            None,
            false,
            IVarNameS::CodeVarName(scout_arena.intern_str(&format!("p{}", index))),
            // No outer ref wraps: an extern's params are taken by value at this stage, so the
            // full type and the value type are the same rune. `ParameterS::new` asserts it.
            usage,
            usage,
            scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
            scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
        ));
    }

    let ret_rune = scout_arena.intern_rune(IRuneValS::ReturnRune(ReturnRuneS {}));
    let ret_usage = RuneUsage { range, rune: ret_rune };
    rules.push(IRulexSR::Lookup(LookupSR {
        range,
        rune: ret_usage,
        name: scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
            name: vale_type_name(compiler, &sig.ret)?,
        })),
    }));

    // Non-generic for now, so the template takes no arguments. When generics land, these become
    // the Rust item's own type parameters and the rules above refer to them by rune instead of
    // looking up a concrete type by name — which is what lets one declaration serve every
    // instantiation.
    let tyype = TemplateTemplataType {
        param_types: scout_arena.alloc_slice_from_vec::<ITemplataType<'s>>(Vec::new()),
        return_type: scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
    };

    Some(scout_arena.alloc(FunctionS::new(
        range,
        IFunctionDeclarationNameS::FunctionName(FunctionNameS {
            name: human_name,
            code_location: loc,
        }),
        // The same attribute `function_scout` attaches for a source-level `extern func`. It is
        // what `translate_function_attributes` turns into `IFunctionAttributeT::Extern`, and
        // downstream what marks the denizen as foreign.
        scout_arena.alloc_slice_from_vec(vec![IFunctionAttributeS::Extern(ExternS {
            package_coord,
        })]),
        scout_arena.alloc_slice_from_vec(Vec::new()),
        tyype,
        scout_arena.alloc_slice_from_vec(params),
        Some(ret_usage),
        scout_arena.alloc_slice_from_vec(rules),
        scout_arena.alloc(IBodyS::ExternBody(ExternBodyS {})),
    )))
}

/// The Vale source-level name a lowered kind is written as, if it has one yet.
///
/// A declaration's rules name their types the way source does, so a lowered `KindT` has to be
/// mapped back to the name that resolves to it. Primitives are in the builtins store under their
/// keyword, so they resolve directly.
///
/// `None` means "not nameable yet", and it is deliberately narrow. A **Rust-backed citizen** is the
/// live case: `import_rust_types` registers it in a per-type outer env keyed by template id, not in
/// any ambient namespace, so there is no name a `LookupSR` could resolve to it — and inventing a
/// bare `CodeName("Counter")` would resolve against Vale's global namespace, which concatenates
/// every package and hard-panics on two hits with no precedence rule. That is the qualified-name
/// work, and it is deferred; see the plan doc §5.
///
/// The caller drops the whole declaration when this returns `None`, so an unnameable signature
/// means the function is not importable rather than silently importable with a wrong type. The
/// visible cost is that a call to it reports "couldn't find function to call" for a function that
/// does exist — misleading, but strictly better than resolving to a neighbouring type.
fn vale_type_name<'s, 't>(
    compiler: &Compiler<'s, '_, 't>,
    kind: &KindT<'s, 't>,
) -> Option<StrI<'s>>
where
    's: 't,
{
    match kind {
        KindT::Int(i) if i.bits == 32 => Some(compiler.keywords.int),
        KindT::Bool(_) => Some(compiler.keywords.bool),
        KindT::Void(_) => Some(compiler.keywords.void),
        _ => None,
    }
}
