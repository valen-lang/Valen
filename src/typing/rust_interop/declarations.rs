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
use crate::parsing::ast::ast::{IMacroInclusionP, SharednessP};
use crate::postparsing::ast::{
  AbstractBodyS, AbstractSP, ExternBodyS, ExternS, FunctionS, GenericParameterS, IBodyS,
  ICitizenAttributeS, IFunctionAttributeS, IGenericParameterTypeS, InterfaceS,
  KindGenericParameterTypeS, MacroCallS, LocationInDenizen, LocationInDenizenBuilder, ParameterS,
  SealedS, StructS,
};
use crate::postparsing::itemplatatype::{
  FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::{
  ArgumentRuneS, CodeNameS, CodeNameValS, CodeRuneS, FunctionNameS, IFunctionDeclarationNameS,
  IImpreciseNameS,
  CodeVarNameS, IImpreciseNameValS, IRuneValS, IStructDeclarationNameS, IVarDeclarationNameS,
  ReturnRuneS,
  TopLevelInterfaceDeclarationNameS, TopLevelStructDeclarationNameS,
};
use crate::postparsing::rules::rules::{
  BorrowRefSR, CallSR, EqualsSR, IRulexSR, LookupSR, RegionSR, RuneUsage,
};
use crate::postparsing::rules::types::{
  BorrowRefST, EffectS, GroupS, ITypeST, RegionS, RuneUsageST,
};
use crate::scout_arena::ScoutArena;
use crate::typing::compiler::Compiler;
use crate::typing::names::names::{INameT, IStructTemplateNameT};
use crate::typing::rust_interop::oracle::{RustItemId, ValeSig, ValeSigType};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::typing_interner::TypingInterner;
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
  // A `mut(g)` effect per `&mut` parameter, mirroring Rust's mutation so the borrow checker can
  // enforce the callee's aliasing rules at the call site. Empty when nothing is borrowed mutably.
  let mut effects: Vec<EffectS<'s>> = Vec::new();
  // Names the region group parameters, one per reference parameter. Function-scoped so two
  // parameters never mint the same group name — distinct groups are what make Rust's parameters
  // read as independently borrowed (@ELASZ elision gives each borrow its own lifetime).
  let mut next_region: u32 = 0;
  // The synthesized interop function mints its own lid space; each param gets a distinct child.
  let mut lidb = LocationInDenizenBuilder::new(Vec::new());
  for (index, sig_type) in sig.params.iter().enumerate() {
    let own_rune = RuneUsage {
      range,
      rune: scout_arena
        .intern_rune(IRuneValS::ArgumentRune(ArgumentRuneS { arg_index: index as i32 })),
    };
    // The parameter's own bucket, built here and handed straight to `ParameterS::new`. There is
    // no shared list for it to leak into, which is what keeps @PFVSZ's split true by
    // construction rather than by remembering.
    let mut value_type_rules: Vec<IRulexSR<'s>> = Vec::new();
    // Per @PFVSZ a parameter's type splits into its outer ref wraps (the full type) and the value
    // they enclose. A `&self`/`&T` receiver is the one place a synthesized extern has an outer
    // wrap: the borrow chains the full type (the argument rune) down to the value type (a fresh
    // rune bound in the value bucket). Every other position has no wrap, so full == value.
    //
    // The fourth element is the parameter's `tyype`: a `BorrowRefST` carrying the region group for
    // a borrow (which is where the borrow checker reads the group), a bare rune otherwise. It is
    // metadata alongside the binding rules (@PFVSZ), which stay the source of truth for typing.
    let (full_type_rune, value_type_rune, outer_ref_rules, tyype): (_, _, Vec<IRulexSR<'s>>, _) =
      match sig_type {
        ValeSigType::Borrow { inner, is_mut } => {
          // The argument binds to the *value* type: a dot-call peels the receiver's outer
          // borrow and matches the value it encloses. `bind_sig_type` returns the rune that
          // value settled to — `own_rune` (the argument rune) for a concrete inner like
          // `&Counter` (it binds `own_rune` via Lookup/Call), or the generic's own rune for a
          // `&C` inner (a generic references its rune directly, with no rule that would bind
          // `own_rune`). Using that returned rune — rather than always `own_rune` — is what lets a
          // `&C` parameter resolve; wiring the borrow onto an unbound `own_rune` leaves it unsolved.
          let full_type_rune = fresh_rune(scout_arena, range, &mut next_synthetic);
          let value_rune = bind_sig_type(
            compiler,
            inner,
            own_rune,
            range,
            &generic_runes,
            &mut value_type_rules,
            &mut next_synthetic,
          )?;
          // A region group for this borrow: the parameter borrows `in <group>`, and a `&mut` marks
          // that group `mut(g)`. One `CodeRune` is shared by the `in` clause on the `tyype` and the
          // effect, so the borrow checker sees them as one group; each borrow gets its own group.
          //
          // The group rune is deliberately NOT a `generic_params` entry. The real postparser scouts a
          // region generic parameter but then filters every `RegionGenericParameterType` out of the
          // function's `generic_params` (function_scout.rs, the region filter after IRRAE), so a
          // region rune lives only on the parameter `tyype` and the effects — never as an identifying
          // rune. Pushing it into `generic_params` instead makes it an unsolvable identifying rune at
          // every call site, since a region rune appears in no solver rule.
          let region_name = scout_arena.intern_str(&format!("__rust_group{}", next_region));
          next_region += 1;
          let region_rune = RuneUsage {
            range,
            rune: scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: region_name })),
          };
          let group = scout_arena.alloc(GroupS::Rune(scout_arena.alloc(region_rune)));
          if *is_mut {
            effects.push(EffectS::Mut(group));
          }
          let outer = vec![IRulexSR::BorrowRef(BorrowRefSR {
            range,
            result_rune: full_type_rune,
            inner_rune: value_rune,
            // Unspecified on the outer rule: the group lives on the `tyype` below, matching the
            // postparser — `region_s_into_region_sr` collapses a group to `Unspecified` on the rule
            // side, so an `in g` group survives only on the `BorrowRefST`.
            region: RegionSR::Unspecified,
          })];
          let tyype = ITypeST::BorrowRef(scout_arena.alloc(BorrowRefST {
            range,
            inner: scout_arena
              .alloc(ITypeST::Rune(scout_arena.alloc(RuneUsageST { rune: value_rune }))),
            region: RegionS::Group(group),
          }));
          (full_type_rune, value_rune, outer, tyype)
        }
        _ => {
          let rune = bind_sig_type(
            compiler,
            sig_type,
            own_rune,
            range,
            &generic_runes,
            &mut value_type_rules,
            &mut next_synthetic,
          )?;
          // A bare rune, exactly what the postparser hands a closure or magic param
          // (`create_lambda_param`/`create_magic_parameters`). The binding rules stay the source of
          // truth (@PFVSZ); this carries the shape alongside them.
          let tyype = ITypeST::Rune(scout_arena.alloc(RuneUsageST { rune }));
          (rune, rune, Vec::new(), tyype)
        }
      };
    params.push(ParameterS::new(
      range,
      None,
      false,
      IVarDeclarationNameS::CodeVarName(CodeVarNameS {
        imprecise_name: scout_arena
          .intern_code_name(scout_arena.intern_str(&format!("p{}", index))),
        lid: lidb.child().consume_in_arena(scout_arena),
      }),
      tyype,
      full_type_rune,
      value_type_rune,
      scout_arena.alloc_slice_from_vec(outer_ref_rules),
      scout_arena.alloc_slice_from_vec(value_type_rules),
    ));
  }

  let ret_own_rune =
    RuneUsage { range, rune: scout_arena.intern_rune(IRuneValS::ReturnRune(ReturnRuneS {})) };
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
      imprecise_name: scout_arena.intern_code_name(human_name),
      code_location: loc,
      lid: LocationInDenizen { path: &[] },
    }),
    // The same attribute `function_scout` attaches for a source-level `extern func`. It is
    // what `translate_function_attributes` turns into `IFunctionAttributeT::Extern`, and
    // downstream what marks the denizen as foreign.
    scout_arena.alloc_slice_from_vec(vec![IFunctionAttributeS::Extern(ExternS { package_coord })]),
    scout_arena.alloc_slice_from_vec(generic_params),
    tyype,
    scout_arena.alloc_slice_from_vec(params),
    Some(ret_rune),
    // One `mut(g)` per `&mut` parameter — Rust's mutation, mirrored so the borrow checker can hold
    // callers to the callee's aliasing rules. Empty when nothing is borrowed mutably.
    scout_arena.alloc_slice_from_vec(effects),
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
          scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name }))
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
        let arg_rune =
          bind_sig_type(compiler, arg, fresh, range, generic_runes, rules, next_synthetic)?;
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
    ValeSigType::Borrow { inner, .. } => {
      // A borrow in a non-parameter position — a return type, or nested inside a citizen's
      // arguments (`Vec<&T>`) — where the wrap belongs inline in the value rules rather than in
      // a parameter's outer-ref bucket. A parameter's *top-level* borrow is split off by the
      // caller in `synthesize_extern_function` before it reaches here, per @PFVSZ. A nested borrow's
      // mutation is not yet mirrored into a group (only a top-level parameter borrow is), so `is_mut`
      // is not read here.
      let inner_rune = fresh_rune(scout_arena, range, next_synthetic);
      bind_sig_type(compiler, inner, inner_rune, range, generic_runes, rules, next_synthetic)?;
      rules.push(IRulexSR::BorrowRef(BorrowRefSR {
        range,
        result_rune: own_rune,
        inner_rune,
        region: RegionSR::Unspecified,
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
        tyype: IGenericParameterTypeS::KindGenericParameterType(KindGenericParameterTypeS {}),
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

/// The interface analog of `synthesize_extern_struct`: a Rust **enum** becomes an opaque, **sealed**
/// `InterfaceS` with zero variants (Vale is an external consumer; no variant is projected yet, per
/// §8.10), carrying the `Extern` attribute so it lowers to an extern denizen. Its methods and drop
/// attach lazily in the type's outer env exactly like a struct's.
pub fn synthesize_extern_interface<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  package_coord: &'s PackageCoordinate<'s>,
  human_name: StrI<'s>,
  generic_param_names: &[StrI<'s>],
) -> &'s InterfaceS<'s>
where
  's: 't,
{
  let scout_arena = compiler.scout_arena;
  let loc = CodeLocationS::internal(scout_arena, SYNTHESIZED_RANGE_OFFSET);
  let range = RangeS::new(loc, loc);

  let generic_params: Vec<&'s GenericParameterS<'s>> = generic_param_names
    .iter()
    .map(|name| {
      let rune = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: *name }));
      &*scout_arena.alloc(GenericParameterS {
        range,
        rune: RuneUsage { range, rune },
        tyype: IGenericParameterTypeS::KindGenericParameterType(KindGenericParameterTypeS {}),
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

  scout_arena.alloc(InterfaceS::new(
    range,
    scout_arena.alloc(TopLevelInterfaceDeclarationNameS { name: human_name, range }),
    scout_arena.alloc_slice_from_vec(vec![
      // The `extern` attribute the postparser attaches, plus `sealed`: a Rust enum is a closed sum.
      ICitizenAttributeS::Extern(ExternS { package_coord }),
      ICitizenAttributeS::Sealed(SealedS),
    ]),
    false, // not weakable
    scout_arena.alloc_slice_from_vec(generic_params),
    SharednessP::Single,
    tyype,
    &[], // rules
    &[], // internal_methods
    &[], // impl_bounds
  ))
}

/// Rewrite a trait method's signature so the trait's implicit `Self` (generic index 0) becomes the
/// interface itself. A trait method's `&self` receiver reads as `&Self`; the abstract interface
/// method's receiver must be `&Callback`, so every `Generic(0)` position is replaced by the interface
/// citizen. Only the simple case is handled — a method whose sole generic is `Self` — so any other
/// `Generic` here is an unsupported own-generic and is left as-is (its declaration will decline).
fn map_self_to_interface<'s, 't>(
  interner: &TypingInterner<'s, 't>,
  sig_type: &ValeSigType<'s, 't>,
  interface_name: StrI<'s>,
  package: &'s PackageCoordinate<'s>,
) -> ValeSigType<'s, 't>
where
  's: 't,
{
  match sig_type {
    ValeSigType::Generic(0) => ValeSigType::Citizen { name: interface_name, package, args: &[] },
    ValeSigType::Generic(i) => ValeSigType::Generic(*i),
    ValeSigType::Kind(k) => ValeSigType::Kind(*k),
    ValeSigType::Citizen { name, package: p, args } => {
      let mapped: Vec<ValeSigType<'s, 't>> =
        args.iter().map(|a| map_self_to_interface(interner, a, interface_name, package)).collect();
      ValeSigType::Citizen { name: *name, package: p, args: interner.alloc_slice_from_vec(mapped) }
    }
    ValeSigType::Borrow { inner, is_mut } => ValeSigType::Borrow {
      inner: interner.alloc(map_self_to_interface(interner, inner, interface_name, package)),
      is_mut: *is_mut,
    },
  }
}

/// One abstract method of a synthesized trait-interface — the AHT `FunctionS` `function_scout`
/// produces for a native `func on_call(virtual self &Callback) int;`. It is `synthesize_extern_function`
/// with exactly three differences: the receiver (parameter 0) is the virtual dispatch parameter, the
/// body is `AbstractBody`, and it carries no `Extern` attribute (an interface method's abstractness is
/// the parent interface plus the virtual receiver, not an attribute). `sig` must already have `Self`
/// mapped to the interface and carry no generic parameters (they must equal the interface's, which is
/// non-generic here). A borrowed receiver is the `@PFVSZ` outer-ref split, identical to a `&self`
/// extern param — the borrow lives in the parameter's `type_outer_ref_rules` as a `BorrowRefSR`.
fn synthesize_abstract_interface_method<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  human_name: StrI<'s>,
  sig: &ValeSig<'s, 't>,
) -> Option<&'s FunctionS<'s>>
where
  's: 't,
{
  let scout_arena = compiler.scout_arena;
  let loc = CodeLocationS::internal(scout_arena, SYNTHESIZED_RANGE_OFFSET);
  let range = RangeS::new(loc, loc);

  // `InterfaceS::new` asserts each internal method's generic params equal the interface's. The
  // interface is non-generic (Self filtered, generic trait methods unsupported), so the abstract
  // method carries none and there are no generic runes to reference.
  if !sig.generic_params.is_empty() {
    return None;
  }
  let generic_runes: Vec<RuneUsage<'s>> = Vec::new();

  let mut header_rules: Vec<IRulexSR<'s>> = Vec::new();
  let mut next_synthetic: u32 = 0;
  let mut params: Vec<ParameterS<'s>> = Vec::new();
  let mut lidb = LocationInDenizenBuilder::new(Vec::new());
  for (index, sig_type) in sig.params.iter().enumerate() {
    let own_rune = RuneUsage {
      range,
      rune: scout_arena
        .intern_rune(IRuneValS::ArgumentRune(ArgumentRuneS { arg_index: index as i32 })),
    };
    let mut value_type_rules: Vec<IRulexSR<'s>> = Vec::new();
    let (full_type_rune, value_type_rune, outer_ref_rules): (_, _, Vec<IRulexSR<'s>>) =
      match sig_type {
        // A trait's abstract method carries no region groups: its generic params must equal the
        // (non-generic) interface's, so it has no room for a region parameter, and `is_mut` is not
        // mirrored here. This is the reverse direction (Rust calling into a Vale override), separate
        // from mirroring a called Rust function's parameter mutation.
        ValeSigType::Borrow { inner, .. } => {
          let full_type_rune = fresh_rune(scout_arena, range, &mut next_synthetic);
          bind_sig_type(
            compiler,
            inner,
            own_rune,
            range,
            &generic_runes,
            &mut value_type_rules,
            &mut next_synthetic,
          )?;
          let outer = vec![IRulexSR::BorrowRef(BorrowRefSR {
            range,
            result_rune: full_type_rune,
            inner_rune: own_rune,
            region: RegionSR::Unspecified,
          })];
          (full_type_rune, own_rune, outer)
        }
        _ => {
          let rune = bind_sig_type(
            compiler,
            sig_type,
            own_rune,
            range,
            &generic_runes,
            &mut value_type_rules,
            &mut next_synthetic,
          )?;
          (rune, rune, Vec::new())
        }
      };
    // The receiver (parameter 0) is virtual — the interface-compile reads its virtual slot.
    // `is_internal_method` is true because the method lives inside the interface citizen.
    let virtuality =
      if index == 0 { Some(AbstractSP { range, is_internal_method: true }) } else { None };
    params.push(ParameterS::new(
      range,
      virtuality,
      false,
      IVarDeclarationNameS::CodeVarName(CodeVarNameS {
        imprecise_name: scout_arena
          .intern_code_name(scout_arena.intern_str(&format!("p{}", index))),
        lid: lidb.child().consume_in_arena(scout_arena),
      }),
      ITypeST::Rune(scout_arena.alloc(RuneUsageST { rune: full_type_rune })),
      full_type_rune,
      value_type_rune,
      scout_arena.alloc_slice_from_vec(outer_ref_rules),
      scout_arena.alloc_slice_from_vec(value_type_rules),
    ));
  }

  let ret_own_rune =
    RuneUsage { range, rune: scout_arena.intern_rune(IRuneValS::ReturnRune(ReturnRuneS {})) };
  let ret_rune = bind_sig_type(
    compiler,
    &sig.ret,
    ret_own_rune,
    range,
    &generic_runes,
    &mut header_rules,
    &mut next_synthetic,
  )?;

  let tyype = TemplateTemplataType {
    param_types: scout_arena.alloc_slice_from_vec::<ITemplataType<'s>>(Vec::new()),
    return_type: scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
  };

  Some(scout_arena.alloc(FunctionS::new(
    range,
    IFunctionDeclarationNameS::FunctionName(FunctionNameS {
      imprecise_name: scout_arena.intern_code_name(human_name),
      code_location: loc,
      lid: LocationInDenizen { path: &[] },
    }),
    // No attributes: an interface method's abstractness is its parent interface plus the virtual
    // receiver, not an attribute (`function_scout` rejects a redundant `abstract` here).
    scout_arena.alloc_slice_from_vec(Vec::new()),
    // Generic params must equal the interface's — empty here.
    scout_arena.alloc_slice_from_vec(Vec::new()),
    tyype,
    scout_arena.alloc_slice_from_vec(params),
    Some(ret_rune),
    &[],
    scout_arena.alloc_slice_from_vec(header_rules),
    &[],
    scout_arena.alloc(IBodyS::AbstractBody(AbstractBodyS {})),
  )))
}

/// A Rust **trait** becomes an interface carrying its abstract methods, so a Vale struct can `impl`
/// it and Rust can call back in. Like `synthesize_extern_interface` (the enum analog) it is an
/// opaque `Extern` interface; unlike it, each trait method is projected into `internal_methods` as an
/// abstract method whose virtual receiver is the interface itself, so an `impl Callback for MyCb`
/// resolves its `on_call` through the ordinary override machinery. Non-generic only for now (Self is
/// filtered and generic trait methods are unsupported).
pub fn synthesize_extern_trait<'s, 'ctx, 't>(
  compiler: &Compiler<'s, 'ctx, 't>,
  package_coord: &'s PackageCoordinate<'s>,
  human_name: StrI<'s>,
  methods: &[(StrI<'s>, ValeSig<'s, 't>)],
) -> &'s InterfaceS<'s>
where
  's: 't,
{
  let scout_arena = compiler.scout_arena;
  let interner = compiler.typing_interner;
  let loc = CodeLocationS::internal(scout_arena, SYNTHESIZED_RANGE_OFFSET);
  let range = RangeS::new(loc, loc);

  let tyype = TemplateTemplataType {
    param_types: scout_arena.alloc_slice_from_vec::<ITemplataType<'s>>(Vec::new()),
    return_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
  };

  let mut internal_methods: Vec<&'s FunctionS<'s>> = Vec::new();
  for (method_name, sig) in methods {
    let mapped_params: Vec<ValeSigType<'s, 't>> = sig
      .params
      .iter()
      .map(|p| map_self_to_interface(interner, p, human_name, package_coord))
      .collect();
    let mapped_ret = map_self_to_interface(interner, &sig.ret, human_name, package_coord);
    let mapped_sig = ValeSig {
      generic_params: &[],
      params: interner.alloc_slice_from_vec(mapped_params),
      ret: mapped_ret,
    };
    // A method that fails to project (an unsupported generic) is skipped; an override for it then
    // fails to resolve, surfacing the gap at the impl rather than as a silent hole here.
    if let Some(m) = synthesize_abstract_interface_method(compiler, *method_name, &mapped_sig) {
      internal_methods.push(m);
    }
  }

  scout_arena.alloc(InterfaceS::new(
    range,
    scout_arena.alloc(TopLevelInterfaceDeclarationNameS { name: human_name, range }),
    scout_arena.alloc_slice_from_vec(vec![
      ICitizenAttributeS::Extern(ExternS { package_coord }),
      ICitizenAttributeS::Sealed(SealedS),
    ]),
    false, // not weakable
    &[],   // no generic parameters
    SharednessP::Single,
    tyype,
    &[], // rules
    scout_arena.alloc_slice_from_vec(internal_methods),
    &[], // impl_bounds
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
      scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: segment })),
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
fn vale_type_name<'s, 't>(compiler: &Compiler<'s, '_, 't>, kind: &KindT<'s, 't>) -> Option<StrI<'s>>
where
  's: 't,
{
  match kind {
    KindT::Int(i) if i.bits == 32 => Some(compiler.keywords.int),
    KindT::Bool(_) => Some(compiler.keywords.bool),
    KindT::Void(_) => Some(compiler.keywords.void),
    KindT::USize(_) => Some(compiler.keywords.usize),
    _ => None,
  }
}
