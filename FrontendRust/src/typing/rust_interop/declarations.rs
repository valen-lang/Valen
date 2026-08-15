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
use crate::scout_arena::ScoutArena;
use crate::parsing::ast::ast::{IMacroInclusionP, SharednessP};
use crate::postparsing::ast::{
    ExternBodyS, ExternS, FunctionS, GenericParameterS, IBodyS, ICitizenAttributeS,
    IFunctionAttributeS, IGenericParameterTypeS, KindGenericParameterTypeS, MacroCallS, ParameterS,
    StructS,
};
use crate::postparsing::itemplatatype::{
    FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::{
    ArgumentRuneS, CodeNameS, CodeRuneS, FunctionNameS, IFunctionDeclarationNameS,
    IImpreciseNameS, IImpreciseNameValS, IRuneValS, IStructDeclarationNameS, IVarNameS,
    ReturnRuneS, TopLevelStructDeclarationNameS,
};
use crate::postparsing::rules::rules::{CallSR, EqualsSR, IRulexSR, LookupSR, RuneUsage};
use crate::typing::compiler::Compiler;
use crate::typing::names::names::{INameT, IStructTemplateNameT};
use crate::typing::rust_interop::oracle::{RustItemId, ValeSig, ValeSigType};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::*;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};

/// The synthetic code-location offset every synthesized Rust denizen shares. Negative because
/// `CodeLocationS::internal` requires it (real source offsets are non-negative). It carries no
/// identity: a denizen's identity is its template id, which is unique without help from the range.
pub const SYNTHESIZED_RANGE_OFFSET: i32 = -1;

/// Synthesize the declaration for one importable Rust free function.
///
/// The synthetic range is a single shared sentinel (`SYNTHESIZED_RANGE_OFFSET`), not a per-item
/// location. It once had to be distinct per item, because `FunctionTemplataT`'s hand-rolled eq/hash
/// keyed on `(range, name)` and ignored the environment, so two synthesized externs named `new`
/// (`Vec::new`, `String::new`) sharing a sentinel would collapse into one overload candidate with no
/// error. That eq/hash is now derived over `{ outer_env, function_template_id }`, so identity is the
/// template id — and a denizen's id is already unique by `(package_coord, init_steps, human_name)`
/// (free functions per package, methods and drops per owner type). The range no longer carries
/// identity, so it needs no per-item content.
///
/// `None` when any type in the signature has no Vale source-level name yet — see `vale_type_name`.
pub fn synthesize_extern_function<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    package_coord: &'s PackageCoordinate<'s>,
    human_name: StrI<'s>,
    sig: &ValeSig<'s, 't>,
) -> Option<&'s FunctionS<'s>>
where
    's: 't,
{
    let scout_arena = compiler.scout_arena;
    let loc = CodeLocationS::internal(scout_arena, SYNTHESIZED_RANGE_OFFSET);
    let range = RangeS::new(loc, loc);

    // The item's own generic parameters, declared before anything refers to them. A generic
    // position then references its rune *directly*, with no rule at all — which is exactly what
    // the postparser emits for a hand-written `func foo<T>(x T) T`, because `templex_scout` uses a
    // locally-declared rune by reference and only reaches for a rule when the name comes from
    // somewhere else. Empty for a concrete function: the degenerate case, not a separate path.
    let mut generic_params: Vec<&'s GenericParameterS<'s>> = Vec::new();
    let mut generic_runes: Vec<RuneUsage<'s>> = Vec::new();
    for name in sig.generic_params.iter() {
        let rune = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: *name }));
        let usage = RuneUsage { range, rune };
        generic_runes.push(usage);
        generic_params.push(scout_arena.alloc(GenericParameterS {
            range,
            rune: usage,
            tyype: IGenericParameterTypeS::KindGenericParameterType(KindGenericParameterTypeS {}),
            default: None,
        }));
    }

    // Rules the *function* owns, which is the return type's and nothing else. A parameter's rules
    // belong to the parameter (@PFVSZ), so they are built per-parameter below.
    let mut header_rules: Vec<IRulexSR<'s>> = Vec::new();

    // Synthetic runes for the intermediate positions a generic citizen needs. `CodeRune` with a
    // reserved-looking name is safe here because the only other code runes in a synthesized
    // declaration are the Rust generic parameters, which carry Rust's own identifiers.
    //
    // Function-scoped, deliberately: it names the runes, so restarting it per parameter would let
    // two parameters mint the same name.
    let mut next_synthetic: u32 = 0;

    let mut params: Vec<ParameterS<'s>> = Vec::new();
    for (index, sig_type) in sig.params.iter().enumerate() {
        let own_rune = RuneUsage {
            range,
            rune: scout_arena.intern_rune(IRuneValS::ArgumentRune(ArgumentRuneS {
                arg_index: index as i32,
            })),
        };
        // The parameter's own bucket, built here and handed straight to `ParameterS::new`. There is
        // no shared list for it to leak into, which is what keeps @PFVSZ's split true by
        // construction rather than by remembering.
        let mut value_type_rules: Vec<IRulexSR<'s>> = Vec::new();
        let rune = bind_sig_type(
            compiler,
            sig_type,
            own_rune,
            range,
            &generic_runes,
            &mut value_type_rules,
            &mut next_synthetic,
        )?;
        params.push(ParameterS::new(
            range,
            None,
            false,
            IVarNameS::CodeVarName(scout_arena.intern_str(&format!("p{}", index))),
            // No outer ref wraps: an extern's params are taken by value at this stage, so the
            // full type and the value type are the same rune. `ParameterS::new` asserts it.
            rune,
            rune,
            scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
            scout_arena.alloc_slice_from_vec(value_type_rules),
        ));
    }

    let ret_own_rune = RuneUsage {
        range,
        rune: scout_arena.intern_rune(IRuneValS::ReturnRune(ReturnRuneS {})),
    };
    let ret_rune = bind_sig_type(
        compiler,
        &sig.ret,
        ret_own_rune,
        range,
        &generic_runes,
        &mut header_rules,
        &mut next_synthetic,
    )?;

    // One template parameter per declared generic, typed as a kind. Empty for a concrete
    // function, which is what makes `make_extern_function` — which reads its template arguments
    // off the *solved* environment — work identically for both.
    let tyype = TemplateTemplataType {
        param_types: scout_arena.alloc_slice_from_vec::<ITemplataType<'s>>(
            generic_params.iter().map(|p| p.tyype.tyype()).collect(),
        ),
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
        scout_arena.alloc_slice_from_vec(generic_params),
        tyype,
        scout_arena.alloc_slice_from_vec(params),
        Some(ret_rune),
        scout_arena.alloc_slice_from_vec(header_rules),
        // No impl bounds, and that is the truth rather than a placeholder. A Rust function's trait
        // obligations are discharged by rustc, never by Vale — and we read no predicates at all,
        // which is why a signature that *needs* one (`first<I: Iterator> -> I::Item`) is declined
        // outright rather than imported with a bound nothing could satisfy.
        &[],
        scout_arena.alloc(IBodyS::ExternBody(ExternBodyS {})),
    )))
}

/// Bind `own_rune` to one signature position, emitting whatever rules that takes.
///
/// Three shapes, and the split is by *what the name resolves to*, never by argument count:
///
///   - **A generic parameter** references its declared rune directly, with no rule at all — which
///     is what the postparser emits for a hand-written `func foo<T>(x T) T`.
///   - **A primitive** is one `LookupSR`, because the builtins store holds `int` as a bare
///     `ITemplataT::Kind`.
///   - **A citizen is always two rules** — `LookupSR` binding a rune to the *template*, then
///     `CallSR` applying the argument runes to it. A Rust citizen is registered as
///     `IEnvEntryT::Struct`, so its name resolves to a `StructDefinition` template, and turning a
///     template into a kind is what `CallSR` does. **Zero arguments is the degenerate case, not a
///     special one** (@NNGZ): skipping the call for a non-generic citizen was tried and fails
///     loudly, with the parameter rune resolving to a `StructDefinition` where a `Kind` is wanted.
///
/// Recursive, so `Holder<Holder<int>>` and `Holder<T>` fall out rather than needing their own
/// cases — an argument is just another position.
///
/// `None` for anything not nameable, which drops the whole declaration rather than importing it
/// with a hole.
fn bind_sig_type<'s, 't>(
    compiler: &Compiler<'s, '_, 't>,
    sig_type: &ValeSigType<'s, 't>,
    own_rune: RuneUsage<'s>,
    range: RangeS<'s>,
    generic_runes: &[RuneUsage<'s>],
    rules: &mut Vec<IRulexSR<'s>>,
    next_synthetic: &mut u32,
) -> Option<RuneUsage<'s>>
where
    's: 't,
{
    let scout_arena = compiler.scout_arena;
    match sig_type {
        ValeSigType::Generic(index) => {
            // A declared generic parameter *is* its rune — the position denotes it directly, with
            // no rule at all, which is what the postparser emits for a hand-written
            // `func foo<T>(x T) T`. Two parameters of the same type therefore share one rune,
            // which is what `f<T>(a T, b T)` means.
            generic_runes.get(*index as usize).copied()
        }
        ValeSigType::Kind(kind) => {
            let name = vale_type_name(compiler, kind)?;
            rules.push(IRulexSR::Lookup(LookupSR {
                range,
                rune: own_rune,
                // One segment, deliberately. This arm is a *primitive* — `int`, `bool`, `void` —
                // which lives in the builtins store under a bare name. Qualifying it would
                // un-resolve it; only a citizen carries a package path.
                parts: scout_arena.alloc_slice_copy(&[
                    scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name })),
                ]),
            }));
            Some(own_rune)
        }
        ValeSigType::Citizen { name, package, args } => {
            let template_rune = fresh_rune(scout_arena, range, next_synthetic);
            rules.push(IRulexSR::Lookup(LookupSR {
                range,
                rune: template_rune,
                // **The one site in the compiler that emits a multi-segment path.** The citizen is
                // named by its package coordinate followed by its short name — `rust.mycrate.Widget`
                // — so two crates exporting the same short name are reached by different paths and
                // the ambiguity never forms. Both ends are ours: the importer registers the store
                // under this coordinate and this writes the same one, so they agree by construction
                // rather than by a key both sides have to compute identically.
                parts: package_path(scout_arena, package, *name),
            }));

            let mut arg_runes: Vec<RuneUsage<'s>> = Vec::new();
            for arg in args.iter() {
                // An argument is just another position, so it goes through the same call. A generic
                // one comes back as the declared rune, which is what lets the solver run the call
                // backwards from a concrete argument; anything else binds the fresh rune offered
                // here.
                let fresh = fresh_rune(scout_arena, range, next_synthetic);
                let arg_rune = bind_sig_type(
                    compiler,
                    arg,
                    fresh,
                    range,
                    generic_runes,
                    rules,
                    next_synthetic,
                )?;
                arg_runes.push(arg_rune);
            }

            rules.push(IRulexSR::Call(CallSR {
                range,
                result_rune: own_rune,
                template_rune,
                args: scout_arena.alloc_slice_from_vec(arg_runes),
            }));
            Some(own_rune)
        }
    }
}

/// A distinct rune for one of the intermediate positions a generic citizen needs.
fn fresh_rune<'s>(
    scout_arena: &ScoutArena<'s>,
    range: RangeS<'s>,
    next_synthetic: &mut u32,
) -> RuneUsage<'s> {
    let name = scout_arena.intern_str(&format!("__rust_arg{}", next_synthetic));
    *next_synthetic += 1;
    RuneUsage { range, rune: scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name })) }
}

/// The template arguments of a **citizen** kind — `Some(&[])` for a non-generic one, `None` for
/// anything that is not a citizen at all.
///
/// The `Some(&[]) != None` distinction is load-bearing: it separates "a template that takes no
/// arguments" from "not a template," which is exactly the difference between needing a `CallSR`
/// and not. Collapsing the two is what @NNGZ warns about.
fn citizen_template_args<'s, 't>(kind: &KindT<'s, 't>) -> Option<&'t [ITemplataT<'s, 't>]> {
    match kind {
        KindT::Struct(struct_tt) => match struct_tt.id.local_name {
            INameT::Struct(name) => Some(name.template_args),
            _ => None,
        },
        _ => None,
    }
}

/// Synthesize the declaration for one importable Rust type.
///
/// The counterpart to `synthesize_extern_function`, and the same idea: hand the ordinary machinery
/// a *declaration* and let it produce the definition, rather than hand-building the definition and
/// registering the result. `precompile_struct` and `compile_struct` then do the `declare_type` /
/// `add_struct` / environment work that this module used to do by hand.
///
/// **`generic_params` is the payload.** It is what makes `Holder` a *template* rather than a
/// finished type — and therefore what gives a `CallSR` something to apply `[int]` to. Without it,
/// `Holder<i32>` and `Holder<bool>` intern to one argument-less kind and Vale gives the same answer
/// for two different Rust types.
///
/// Three fields say something worth stating:
///
///   - **`members: &[]`** — zero members is the truth, not a stub. Vale is an external consumer of
///     a Rust type; its layout is opaque and its private fields are none of Vale's business.
///     Synthesizing a declaration does not mean fabricating fields.
///   - **`internal_methods: &[]`** — a Rust method is an ordinary top-level declaration whose first
///     parameter is the receiver, not something declared inside the citizen's braces. The sibling
///     implementation puts its extern functions *inside* the extern struct; that is the
///     method-shaped path this arc deliberately collapsed, so we do not.
///   - **both derive macros suppressed** — see below.
///
/// **Why both `DeriveStructConstructor` and `DeriveStructDrop` are turned off**, using the
/// language's own `DontCallMacro` attribute rather than any Rust-specific special case:
///
///   - The **constructor** would be a field constructor over zero members, which claims knowledge
///     of a layout and invariants Vale does not have. A Rust type is constructed by calling a Rust
///     associated function (`Counter::new`), never by a Vale struct literal.
///   - The **drop** is the subtler one. The derived drop's body is `GeneratedBody(drop_generator)`,
///     which destructures the struct and drops its *members* — so for a zero-member Rust citizen it
///     is an **empty destructor that never reaches rustc**. That is indistinguishable from correct
///     for a type with no `Drop` impl and silently skips the destructor for a type that has one.
///     We synthesize our own `drop` instead, with an `ExternBody` that becomes `__vale_drop<T>` →
///     `drop_in_place::<T>` at codegen, letting rustc resolve its own drop glue.
pub fn synthesize_extern_struct<'s, 'ctx, 't>(
    compiler: &Compiler<'s, 'ctx, 't>,
    package_coord: &'s PackageCoordinate<'s>,
    human_name: StrI<'s>,
    generic_param_names: &[StrI<'s>],
) -> &'s StructS<'s>
where
    's: 't,
{
    let scout_arena = compiler.scout_arena;

    // The shared synthesized sentinel; see `synthesize_extern_function`. A struct's identity is its
    // template id (`StructTemplate(human_name)` under its package), which carries no code location, so
    // the range is purely cosmetic here and needs no per-item content.
    let loc = CodeLocationS::internal(scout_arena, SYNTHESIZED_RANGE_OFFSET);
    let range = RangeS::new(loc, loc);

    let generic_params: Vec<&'s GenericParameterS<'s>> = generic_param_names
        .iter()
        .map(|name| {
            let rune = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: *name }));
            &*scout_arena.alloc(GenericParameterS {
                range,
                rune: RuneUsage { range, rune },
                tyype: IGenericParameterTypeS::KindGenericParameterType(
                    KindGenericParameterTypeS {},
                ),
                default: None,
            })
        })
        .collect();

    let tyype = TemplateTemplataType {
        param_types: scout_arena.alloc_slice_from_vec::<ITemplataType<'s>>(
            generic_params.iter().map(|p| p.tyype.tyype()).collect(),
        ),
        return_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    };

    let dont_call = |macro_name: StrI<'s>| {
        ICitizenAttributeS::MacroCall(MacroCallS {
            range,
            include: IMacroInclusionP::DontCallMacro,
            macro_name,
        })
    };

    scout_arena.alloc(StructS::new(
        range,
        IStructDeclarationNameS::TopLevelStructDeclarationName(TopLevelStructDeclarationNameS {
            name: human_name,
            range,
        }),
        scout_arena.alloc_slice_from_vec(vec![
            // The same attribute the postparser attaches for a hand-written `extern struct`.
            ICitizenAttributeS::Extern(ExternS { package_coord }),
            dont_call(compiler.keywords.derive_struct_constructor),
            dont_call(compiler.keywords.derive_struct_drop),
        ]),
        // Rust will never support either, so these are permanent rather than provisional.
        false,
        scout_arena.alloc_slice_from_vec(generic_params),
        SharednessP::Single,
        tyype,
        // No bounds: rustc discharges a Rust type's own obligations, and we read no predicates.
        &[],
        &[],
        &[],
        &[],
        &[],
    ))
}

/// A citizen's `LookupSR` path: its package coordinate's segments, then its short name.
///
/// The coordinate is `{ module, packages }`, so `rust.["mycrate"]` yields `[rust, mycrate, Widget]`
/// — module first, exactly the order `GlobalEnvironmentT::find_package_store` matches against.
/// The two must stay in step; they are the two ends of the same handshake.
fn package_path<'s>(
    scout_arena: &ScoutArena<'s>,
    package: &'s PackageCoordinate<'s>,
    name: StrI<'s>,
) -> &'s [IImpreciseNameS<'s>] {
    let mut parts: Vec<IImpreciseNameS<'s>> = Vec::new();
    let mut push = |segment: StrI<'s>| {
        parts.push(
            scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: segment })),
        );
    };
    push(package.module);
    for segment in package.packages.iter() {
        push(*segment);
    }
    push(name);
    scout_arena.alloc_slice_from_vec(parts)
}

/// The Vale keyword naming a **builtin**, for the one-segment `LookupSR` a primitive needs.
///
/// Only builtins reach here. A citizen never does: `lower_sig_ty` catches `TyKind::Adt` ahead of
/// the fallthrough that produces `ValeSigType::Kind`, so a struct arrives as
/// `ValeSigType::Citizen` — carrying its package coordinate — and is named by a path instead.
///
/// It used to have a `KindT::Struct` arm, from before `Citizen` existed, which turned a citizen
/// into a bare human name. **Measured dead 2026-07-27** (arm replaced with a `panic!`, both configs
/// re-run unchanged with zero hits) and deleted rather than parked: it was the last thing in this
/// file that could reduce a citizen to an unqualified name, which is precisely the shape the
/// package path exists to prevent.
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
