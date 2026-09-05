// Per @DSAUIMZ, all borrow_val() calls in this file borrow from a stack-local
// LocationInDenizenBuilder instead of arena-allocating. The slice is promoted
// to permanent arena storage only inside intern_rune on a miss.

use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parsing::ast::{
  BoolPT, EffectP, FuncPT, GroupP, ITemplexPT, ITemplexPT::NameOrRune, IntPT, NameOrRunePT, NameP,
  RegionP, RegionRunePT, StringPT,
};
use crate::postparsing::ast::{
  AbstractBodyS, FunctionS, IBodyS, LocationInDenizenBuilder, ParameterS,
};
use crate::postparsing::itemplatatype::{
  FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::CodeNameValS;
use crate::postparsing::names::IRuneValS::{CodeRune, ImplicitRune};
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, DesugaredParamNameDeclarationS, FunctionNameS, IFunctionDeclarationNameS,
  IImpreciseNameS, IImpreciseNameValS::CodeName, IRuneS, IVarDeclarationNameS, ImplicitRuneValS,
};
use crate::postparsing::post_parser::{IEnvironmentS, PostParser};
use crate::postparsing::rules::rules::IRulexSR::{Call, Lookup};
use crate::postparsing::rules::rules::{
  BoolLiteralSL, BorrowRefSR, CallSR, ILiteralSL, IRulexSR, IntLiteralSL, KindListSR, LiteralSR,
  LookupSR, OwnRefSR, RegionSR, RuneParentEnvLookupSR, RuneUsage, StringLiteralSL, WeakRefSR,
};
use crate::postparsing::rules::rules::{CallSiteFuncSR, DefinitionFuncSR, ResolveSR};
use crate::postparsing::rules::types::{
  AnonymousRuneST, BoolST, BorrowRefST, CallST, EffectS, GroupS, ITypeST, IntST, NameST, OwnRefST,
  PackST, RegionS, RuneUsageST, RuntimeSizedArrayST, StringST, TupleST, WeakRefST,
};
use crate::scout_arena::ScoutArena;
use crate::utils::range::RangeS;

pub fn add_literal_rule<'s>(
  scout_arena: &ScoutArena<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  value_sr: ILiteralSL<'s>,
) -> RuneUsage<'s> {
  let mut child_lidb = lidb.child();
  let rune_s = RuneUsage {
    range: range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
  };
  rule_builder.push(IRulexSR::Literal(LiteralSR {
    range: range_s,
    rune: rune_s.clone(),
    literal: value_sr,
  }));
  rune_s
}

fn add_rune_parent_env_lookup_rule<'s>(
  _scout_arena: &ScoutArena<'s>,
  _lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  rune_s: IRuneS<'s>,
) -> RuneUsage<'s> {
  let usage = RuneUsage { range: range_s.clone(), rune: rune_s };
  rule_builder.push(IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR {
    range: range_s,
    rune: usage.clone(),
  }));
  usage
}

fn add_lookup_rule<'s>(
  scout_arena: &ScoutArena<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  // Nearest enclosing region marker, see RADTGCA.
  _context_region: IRuneS<'s>,
  name_sn: IImpreciseNameS<'s>,
) -> RuneUsage<'s> {
  let mut child_lidb = lidb.child();
  let rune_s = RuneUsage {
    range: range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
  };
  rule_builder.push(Lookup(LookupSR {
    range: range_s,
    rune: rune_s.clone(),
    parts: scout_arena.alloc_slice_copy(&[name_sn]),
  }));
  rune_s
}

// VCOORD: revisit this
// Emit a zero-arg template application: mint a fresh result rune, push a Call of `template_rune`
// over no args, and return the result rune. This is the bare-name lowering of @TNLTZACZ.
// The ITemplexPT::Call arm builds its own Call instead of calling this: it mints its result rune
// *before* translating the template and args, and that ordering feeds the rune's
// LocationInDenizen path, so sharing this helper would change every applied template's rune.
fn add_zero_arg_call_rule<'s>(
  scout_arena: &ScoutArena<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  template_rune: RuneUsage<'s>,
) -> RuneUsage<'s> {
  let mut child_lidb = lidb.child();
  let result_rune_s = RuneUsage {
    range: range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
  };
  rule_builder.push(Call(CallSR {
    range: range_s,
    result_rune: result_rune_s.clone(),
    template_rune,
    args: &[],
  }));
  result_rune_s
}

pub fn translate_value_templex<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  templex: &ITemplexPT<'p>,
) -> Option<ILiteralSL<'s>> {
  match templex {
    ITemplexPT::Int(IntPT { value, .. }) => {
      Some(ILiteralSL::IntLiteral(IntLiteralSL { value: *value }))
    }
    ITemplexPT::Bool(BoolPT { value, .. }) => {
      Some(ILiteralSL::BoolLiteral(BoolLiteralSL { value: *value }))
    }
    ITemplexPT::String(StringPT { str, .. }) => Some(ILiteralSL::StringLiteral(StringLiteralSL {
      value: scout_arena.intern_str(str.as_str()),
    })),
    _ => None,
  }
}

// Each of these builds one reference layer: mint a fresh result rune, push the layer's rule
// into `builder`, return the result rune. The caller has already translated the inner type
// (and, for a borrow, the region) into runes. Both the normal walk and the signature-position
// split use these. They differ only in which builder they hand over.

fn translate_borrow_ref_templex<'s>(
  scout_arena: &ScoutArena<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  inner_rune: RuneUsage<'s>,
  region: RegionSR<'s>,
) -> RuneUsage<'s> {
  let result_rune = RuneUsage {
    range: range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
  };
  builder.push(IRulexSR::BorrowRef(BorrowRefSR {
    range: range_s,
    result_rune: result_rune.clone(),
    inner_rune,
    region,
  }));
  result_rune
}

fn translate_weak_ref_templex<'s>(
  scout_arena: &ScoutArena<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  inner_rune: RuneUsage<'s>,
) -> RuneUsage<'s> {
  let result_rune = RuneUsage {
    range: range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
  };
  builder.push(IRulexSR::WeakRef(WeakRefSR {
    range: range_s,
    result_rune: result_rune.clone(),
    inner_rune,
  }));
  result_rune
}

fn translate_own_ref_templex<'s>(
  scout_arena: &ScoutArena<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  range_s: RangeS<'s>,
  inner_rune: RuneUsage<'s>,
) -> RuneUsage<'s> {
  let result_rune = RuneUsage {
    range: range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
  };
  builder.push(IRulexSR::OwnRef(OwnRefSR {
    range: range_s,
    result_rune: result_rune.clone(),
    inner_rune,
  }));
  result_rune
}

// Translates a type expression into rules and returns its rune. Every rule goes into
// `rule_builder`. To split the outer reference wrapping (&/weak) from the named
// type it wraps (as a function parameter needs), call translate_signature_templex instead.
/// Translates the template half of an application — the `Opt` of `Opt<int>`.
///
/// A name here yields the `Lookup` alone, never the bare-name lowering that @TNLTZACZ describes.
/// That lowering applies the template to no arguments, which collapses it to its own return type;
/// do it here and the outer application receives a finished kind, with its arguments left nothing
/// to apply to. Every other templex already yields the right rune — a rune name resolves to itself
/// or to a parent-env lookup, and neither applies anything.
fn translate_template_position_templex<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  context_region: IRuneS<'s>,
  templex: &ITemplexPT<'p>,
) -> RuneUsage<'s> {
  let file = env.file();
  if let ITemplexPT::NameOrRune(NameOrRunePT { name: name_or_rune, .. }) = templex {
    let name_str = scout_arena.intern_str(name_or_rune.str().as_str());
    let is_rune_from_env = env
      .all_declared_runes()
      .contains(&scout_arena.intern_rune(CodeRune(CodeRuneS { name: name_str })));
    if !is_rune_from_env {
      let name = scout_arena.intern_imprecise_name(CodeName(CodeNameValS { name: name_str }));
      let range_s = PostParser::eval_range(file, name_or_rune.range());
      let mut child_lidb = lidb.child();
      return add_lookup_rule(
        scout_arena,
        &mut child_lidb,
        rule_builder,
        range_s,
        context_region,
        name,
      );
    }
  }
  translate_templex(scout_arena, keywords, env, lidb, rule_builder, context_region, templex)
}

/// Builds the read-only ITypeST mirror of a written type, alongside the rules translate_templex
/// emits. Per plan-phased-calls §P, later phases read this tree to find each rune mention: a bound
/// like `where exists drop(D)void` searches the *value* D resolves to rather than the parameter
/// type, so it needs the tree to know that `D` is a rune.
///
/// NameOrRune splits into Name (a concrete type like `int`) or Rune (a declared generic like `D`).
/// Ref wraps nest. Unlike the rules, nothing is lowered here: no zero-arg Call for a bare name, no
/// parent-env lookup. Substituting into an ITypeST later produces a KindT, never a new ITypeST.
pub fn translate_templex_into_type_st<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  env: IEnvironmentS<'s>,
  templex: &ITemplexPT<'p>,
) -> ITypeST<'s> {
  let file = env.file();
  let range_s = PostParser::eval_range(file, templex.range());
  match templex {
    ITemplexPT::AnonymousRune(_) => {
      ITypeST::AnonymousRune(scout_arena.alloc(AnonymousRuneST { range: range_s }))
    }

    ITemplexPT::Bool(BoolPT { value, .. }) => {
      ITypeST::Bool(scout_arena.alloc(BoolST { range: range_s, value: *value }))
    }

    ITemplexPT::Int(IntPT { value, .. }) => ITypeST::Int(
      scout_arena.alloc(IntST { range: range_s, value: IntLiteralSL { value: *value } }),
    ),

    ITemplexPT::String(StringPT { str, .. }) => ITypeST::String(
      scout_arena.alloc(StringST { range: range_s, str: scout_arena.intern_str(str.as_str()) }),
    ),

    // A bare name, e.g. `int` (a concrete type) or `D` (a declared generic). The env decides which.
    ITemplexPT::NameOrRune(NameOrRunePT { name: name_or_rune, .. }) => {
      let name_str = scout_arena.intern_str(name_or_rune.str().as_str());
      let is_rune = env
        .all_declared_runes()
        .contains(&scout_arena.intern_rune(CodeRune(CodeRuneS { name: name_str })));
      if is_rune {
        ITypeST::Rune(scout_arena.alloc(RuneUsageST {
          rune: RuneUsage {
            range: range_s,
            rune: scout_arena.intern_rune(CodeRune(CodeRuneS { name: name_str })),
          },
        }))
      } else {
        ITypeST::Name(scout_arena.alloc(NameST {
          range: range_s,
          name: scout_arena.intern_imprecise_name(CodeName(CodeNameValS { name: name_str })),
        }))
      }
    }

    ITemplexPT::Call(call) => {
      let template: &'s ITypeST<'s> =
        scout_arena.alloc(translate_templex_into_type_st(scout_arena, env.clone(), call.template));
      let mut args = Vec::<&'s ITypeST<'s>>::new();
      for arg in call.args {
        args.push(scout_arena.alloc(translate_templex_into_type_st(scout_arena, env.clone(), arg)));
      }
      ITypeST::Call(scout_arena.alloc(CallST {
        range: range_s,
        template,
        args: scout_arena.alloc_slice_from_vec(args),
      }))
    }

    ITemplexPT::BorrowRef(borrow_ref) => {
      let inner: &'s ITypeST<'s> = scout_arena.alloc(translate_templex_into_type_st(
        scout_arena,
        env.clone(),
        borrow_ref.inner,
      ));
      let region = match borrow_ref.region {
        RegionP::Unspecified => RegionS::Unspecified,
        RegionP::Held => RegionS::Held,
        RegionP::Group(group_p) => {
          RegionS::Group(translate_group_p_into_group_s(scout_arena, &env, group_p))
        }
      };
      ITypeST::BorrowRef(scout_arena.alloc(BorrowRefST { range: range_s, inner, region }))
    }

    ITemplexPT::WeakRef(weak_ref) => {
      let inner: &'s ITypeST<'s> =
        scout_arena.alloc(translate_templex_into_type_st(scout_arena, env.clone(), weak_ref.inner));
      ITypeST::WeakRef(scout_arena.alloc(WeakRefST { range: range_s, inner }))
    }

    ITemplexPT::OwnRef(own_ref) => {
      let inner: &'s ITypeST<'s> =
        scout_arena.alloc(translate_templex_into_type_st(scout_arena, env.clone(), own_ref.inner));
      ITypeST::OwnRef(scout_arena.alloc(OwnRefST { range: range_s, inner }))
    }

    ITemplexPT::Pack(pack) => {
      let mut members = Vec::<&'s ITypeST<'s>>::new();
      for member in pack.members {
        members.push(scout_arena.alloc(translate_templex_into_type_st(
          scout_arena,
          env.clone(),
          member,
        )));
      }
      ITypeST::Pack(
        scout_arena
          .alloc(PackST { range: range_s, members: scout_arena.alloc_slice_from_vec(members) }),
      )
    }

    ITemplexPT::Tuple(tuple) => {
      let mut elements = Vec::<&'s ITypeST<'s>>::new();
      for element in tuple.elements {
        elements.push(scout_arena.alloc(translate_templex_into_type_st(
          scout_arena,
          env.clone(),
          element,
        )));
      }
      ITypeST::Tuple(
        scout_arena
          .alloc(TupleST { range: range_s, elements: scout_arena.alloc_slice_from_vec(elements) }),
      )
    }

    ITemplexPT::RuntimeSizedArray(rsa) => {
      let element: &'s ITypeST<'s> =
        scout_arena.alloc(translate_templex_into_type_st(scout_arena, env.clone(), rsa.element));
      ITypeST::RuntimeSizedArray(scout_arena.alloc(RuntimeSizedArrayST { range: range_s, element }))
    }

    // translate_templex itself does not lower these yet, so the ITypeST mirror does not either.
    ITemplexPT::Function(_) => panic!("POSTPARSER_TYPE_ST_FUNCTION_NOT_YET_IMPLEMENTED"),
    ITemplexPT::Func(_) => panic!("POSTPARSER_TYPE_ST_FUNC_NOT_YET_IMPLEMENTED"),
    ITemplexPT::RegionRune(_) => panic!("POSTPARSER_TYPE_ST_REGION_RUNE_IS_NOT_A_TYPE"),
    ITemplexPT::TypedRune(_) => panic!("POSTPARSER_TYPE_ST_TYPED_RUNE_NOT_YET_IMPLEMENTED"),
  }
}

/// The inverse of `translate_templex_into_type_st`: walks a read-only ITypeST and emits the same
/// rules `translate_templex` would for the equivalent source, returning the value rune. This is the
/// plan-phased-calls Post-cleanup direction, where rules are derived from the ITypeST rather than
/// produced alongside it, so a caller can hold one tree as the single source of truth and get its
/// rules on demand.
///
/// It reuses translate_templex's own rule helpers, so the emitted rules match it arm-for-arm. The
/// minted structural runes are fresh (their identity follows the lidb path, not the source), so the
/// rules are structurally equivalent to translate_templex's rather than rune-for-rune identical.
/// Bare names split by position per @TNLTZACZ: a value-position name lowers to Lookup plus a zero-arg
/// Call, a template-position name (the template of a Call) to the Lookup alone.
pub fn translate_type_st_into_rune<'s>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  // Nearest enclosing region marker, see RADTGCA.
  context_region: IRuneS<'s>,
  type_st: &ITypeST<'s>,
) -> RuneUsage<'s> {
  let range_s = type_st.range();
  match type_st {
    ITypeST::Bool(b) => {
      let mut child_lidb = lidb.child();
      add_literal_rule(
        scout_arena,
        &mut child_lidb,
        rule_builder,
        range_s,
        ILiteralSL::BoolLiteral(BoolLiteralSL { value: b.value }),
      )
    }

    ITypeST::Int(i) => {
      let mut child_lidb = lidb.child();
      add_literal_rule(
        scout_arena,
        &mut child_lidb,
        rule_builder,
        range_s,
        ILiteralSL::IntLiteral(i.value),
      )
    }

    ITypeST::String(s) => {
      let mut child_lidb = lidb.child();
      add_literal_rule(
        scout_arena,
        &mut child_lidb,
        rule_builder,
        range_s,
        ILiteralSL::StringLiteral(StringLiteralSL { value: s.str }),
      )
    }

    ITypeST::AnonymousRune(_) => RuneUsage {
      range: range_s,
      rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
    },

    ITypeST::Rune(r) => {
      if env.local_declared_runes().contains(&r.rune.rune) {
        RuneUsage { range: range_s, rune: r.rune.rune }
      } else {
        // From a parent env, e.g. a lambda's `__call` mentioning its parent's `T`.
        let mut child_lidb = lidb.child();
        add_rune_parent_env_lookup_rule(
          scout_arena,
          &mut child_lidb,
          rule_builder,
          range_s,
          r.rune.rune,
        )
      }
    }

    ITypeST::Name(n) => {
      // A value-position bare name like `int`. Per @TNLTZACZ it is a zero-arg application: the
      // name's Lookup, then a Call([]) whose result is the value rune.
      let mut child_lidb = lidb.child();
      let template_rune = add_lookup_rule(
        scout_arena,
        &mut child_lidb,
        rule_builder,
        range_s,
        context_region,
        n.name,
      );
      add_zero_arg_call_rule(scout_arena, &mut child_lidb, rule_builder, range_s, template_rune)
    }

    ITypeST::Call(c) => {
      let mut child_lidb = lidb.child();
      let result_rune_s = RuneUsage {
        range: range_s,
        rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
      };
      let template_rune_s = translate_type_st_template_position_into_rune(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        rule_builder,
        context_region,
        c.template,
      );
      let mut arg_runes = Vec::<RuneUsage<'s>>::new();
      for arg in c.args {
        arg_runes.push(translate_type_st_into_rune(
          scout_arena,
          keywords,
          env.clone(),
          &mut lidb.child(),
          rule_builder,
          context_region,
          arg,
        ));
      }
      rule_builder.push(Call(CallSR {
        range: range_s,
        result_rune: result_rune_s.clone(),
        template_rune: template_rune_s,
        args: scout_arena.alloc_slice_from_vec(arg_runes),
      }));
      result_rune_s
    }

    ITypeST::BorrowRef(br) => {
      let inner_rune = translate_type_st_into_rune(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        rule_builder,
        context_region,
        br.inner,
      );
      let region = region_s_into_region_sr(
        scout_arena,
        env.clone(),
        &mut lidb.child(),
        rule_builder,
        br.region,
      );
      translate_borrow_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune, region)
    }

    ITypeST::WeakRef(wr) => {
      let inner_rune = translate_type_st_into_rune(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        rule_builder,
        context_region,
        wr.inner,
      );
      translate_weak_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune)
    }

    ITypeST::OwnRef(or) => {
      let inner_rune = translate_type_st_into_rune(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        rule_builder,
        context_region,
        or.inner,
      );
      translate_own_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune)
    }

    ITypeST::Tuple(tuple) => {
      let tuple_name = scout_arena.intern_imprecise_name(CodeName(CodeNameValS {
        name: keywords.tuple_human_name[tuple.elements.len()],
      }));
      if tuple.elements.is_empty() {
        // Zero-arg tuple `()`: lowers like any bare type-name, per @TNLTZACZ.
        let mut child_lidb = lidb.child();
        let template_rune_s = RuneUsage {
          range: range_s,
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        rule_builder.push(Lookup(LookupSR {
          range: range_s,
          rune: template_rune_s.clone(),
          parts: scout_arena.alloc_slice_copy(&[tuple_name]),
        }));
        add_zero_arg_call_rule(scout_arena, &mut child_lidb, rule_builder, range_s, template_rune_s)
      } else {
        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: range_s,
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        let mut child_lidb = lidb.child();
        let template_rune_s = RuneUsage {
          range: range_s,
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        rule_builder.push(Lookup(LookupSR {
          range: range_s,
          rune: template_rune_s.clone(),
          parts: scout_arena.alloc_slice_copy(&[tuple_name]),
        }));
        let mut element_runes = Vec::<RuneUsage<'s>>::new();
        for element in tuple.elements {
          element_runes.push(translate_type_st_into_rune(
            scout_arena,
            keywords,
            env.clone(),
            &mut lidb.child(),
            rule_builder,
            context_region,
            element,
          ));
        }
        rule_builder.push(Call(CallSR {
          range: range_s,
          result_rune: result_rune_s.clone(),
          template_rune: template_rune_s,
          args: scout_arena.alloc_slice_from_vec(element_runes),
        }));
        result_rune_s
      }
    }

    ITypeST::RuntimeSizedArray(rsa) => {
      let mut child_lidb = lidb.child();
      let result_rune_s = RuneUsage {
        range: range_s,
        rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
      };
      let mut child_lidb = lidb.child();
      let template_rune_s = RuneUsage {
        range: range_s,
        rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
      };
      rule_builder.push(Lookup(LookupSR {
        range: range_s,
        rune: template_rune_s.clone(),
        parts: scout_arena.alloc_slice_copy(&[
          scout_arena.intern_imprecise_name(CodeName(CodeNameValS { name: keywords.array }))
        ]),
      }));
      let element_rune_s = translate_type_st_into_rune(
        scout_arena,
        keywords,
        env,
        &mut lidb.child(),
        rule_builder,
        context_region,
        rsa.element,
      );
      rule_builder.push(Call(CallSR {
        range: range_s,
        result_rune: result_rune_s.clone(),
        template_rune: template_rune_s,
        args: scout_arena.alloc_slice_from_vec(vec![element_rune_s]),
      }));
      result_rune_s
    }

    // translate_templex itself does not lower these, so its ITypeST twin does not either.
    ITypeST::Pack(_) => panic!("POSTPARSER_TYPE_ST_INTO_RUNE_PACK_NOT_YET_IMPLEMENTED"),
    ITypeST::Function(_) => panic!("POSTPARSER_TYPE_ST_INTO_RUNE_FUNCTION_NOT_YET_IMPLEMENTED"),
  }
}

// The template half of an application yields the name's Lookup alone, never the value-position
// zero-arg Call (@TNLTZACZ). The tree encodes template position structurally: only a CallST's
// `template` reaches here. Everything but a bare Name lowers normally.
fn translate_type_st_template_position_into_rune<'s>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  context_region: IRuneS<'s>,
  type_st: &ITypeST<'s>,
) -> RuneUsage<'s> {
  if let ITypeST::Name(n) = type_st {
    let mut child_lidb = lidb.child();
    add_lookup_rule(
      scout_arena,
      &mut child_lidb,
      rule_builder,
      type_st.range(),
      context_region,
      n.name,
    )
  } else {
    translate_type_st_into_rune(
      scout_arena,
      keywords,
      env,
      lidb,
      rule_builder,
      context_region,
      type_st,
    )
  }
}

// The one way to lower an ITypeST for a slot that needs the @PFVSZ split (params, members): returns the
// full-type and value-type runes plus the outer ref-wrap rules and the value-type rules, separated.
// Callers needing the split (params) use both lists; callers wanting a single type concat them and take
// full_rune. The ITypeST twin of translate_signature_templex.
//
// The outer function owns both lists. The inner value translator, translate_type_st_into_rune, only
// ever sees value_rules; it never touches outer_ref_rules. A wrap-less type produces an empty
// outer_ref_rules with full_rune == value_rune, so it lowers byte-identically to a plain
// translate_type_st_into_rune call.
pub fn translate_signature_type_st<'s>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  context_region: IRuneS<'s>,
  type_st: &ITypeST<'s>,
) -> (RuneUsage<'s>, RuneUsage<'s>, Vec<IRulexSR<'s>>, Vec<IRulexSR<'s>>) {
  let mut value_rules = Vec::new();
  let mut outer_ref_rules = Vec::new();
  let (full_rune, value_rune) = split_type_st_into(
    scout_arena,
    keywords,
    env,
    lidb,
    &mut value_rules,
    &mut outer_ref_rules,
    context_region,
    type_st,
  );
  (full_rune, value_rune, outer_ref_rules, value_rules)
}

// The recursive outer-wrap peeler. Each ref layer recurses to peel the next, then emits its wrap rule
// into outer_ref_builder (and its region into value_builder, matching the flat core). The value root
// found at the bottom goes through translate_type_st_into_rune into value_builder — the only place the
// inner value translator runs, and it only ever sees value_builder. Build order is forced: a wrap needs
// its inner rune, so the value (the innermost rune) is built first, then the wraps outward.
fn split_type_st_into<'s>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  value_builder: &mut Vec<IRulexSR<'s>>,
  outer_ref_builder: &mut Vec<IRulexSR<'s>>,
  context_region: IRuneS<'s>,
  type_st: &ITypeST<'s>,
) -> (RuneUsage<'s>, RuneUsage<'s>) {
  let range_s = type_st.range();
  match type_st {
    ITypeST::BorrowRef(br) => {
      let (inner_full, value) = split_type_st_into(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        value_builder,
        outer_ref_builder,
        context_region,
        br.inner,
      );
      let region = region_s_into_region_sr(
        scout_arena,
        env.clone(),
        &mut lidb.child(),
        value_builder,
        br.region,
      );
      let full = translate_borrow_ref_templex(
        scout_arena,
        lidb,
        outer_ref_builder,
        range_s,
        inner_full,
        region,
      );
      (full, value)
    }
    ITypeST::WeakRef(wr) => {
      let (inner_full, value) = split_type_st_into(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        value_builder,
        outer_ref_builder,
        context_region,
        wr.inner,
      );
      let full =
        translate_weak_ref_templex(scout_arena, lidb, outer_ref_builder, range_s, inner_full);
      (full, value)
    }
    ITypeST::OwnRef(or) => {
      let (inner_full, value) = split_type_st_into(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        value_builder,
        outer_ref_builder,
        context_region,
        or.inner,
      );
      let full =
        translate_own_ref_templex(scout_arena, lidb, outer_ref_builder, range_s, inner_full);
      (full, value)
    }
    // The value root, past the outer wraps: translate it flat into the value list (nested wraps, e.g.
    // the `&` inside `Opt<&Spaceship>`, correctly stay in value position). full == value here.
    _ => {
      let value = translate_type_st_into_rune(
        scout_arena,
        keywords,
        env,
        lidb,
        value_builder,
        context_region,
        type_st,
      );
      (value, value)
    }
  }
}

/// Lowers a parse-side group expression (`GroupP`) into the symbolic scout-side `GroupS`. A bare
/// group name resolves to `GroupS::Rune` when it is a declared group/region rune, else
/// `GroupS::Local`, using the same name-vs-rune test the NameOrRune type arm uses. `x.items` becomes
/// a `Member` step and `x.items[]` an `Elements` step, recursing on the base. Reused by borrow
/// regions and effect clauses. Union group expressions are deferred.
fn translate_group_p_into_group_s<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  env: &IEnvironmentS<'s>,
  group_p: &GroupP<'p>,
) -> &'s GroupS<'s> {
  match group_p {
    GroupP::Name(name) => {
      let group_name_str = scout_arena.intern_str(name.str().as_str());
      let group_range = PostParser::eval_range(env.file(), name.0);
      let is_rune = env
        .all_declared_runes()
        .contains(&scout_arena.intern_rune(CodeRune(CodeRuneS { name: group_name_str })));
      if is_rune {
        scout_arena.alloc(GroupS::Rune(scout_arena.alloc(RuneUsage {
          range: group_range,
          rune: scout_arena.intern_rune(CodeRune(CodeRuneS { name: group_name_str })),
        })))
      } else {
        scout_arena.alloc(GroupS::Local(scout_arena.intern_imprecise_name(
          CodeName(CodeNameValS { name: group_name_str }),
        )))
      }
    }
    GroupP::Member { base, member } => scout_arena.alloc(GroupS::Member {
      base: translate_group_p_into_group_s(scout_arena, env, base),
      member_name: scout_arena.intern_str(member.str().as_str()),
    }),
    GroupP::Elements { base } => scout_arena.alloc(GroupS::Elements {
      base: translate_group_p_into_group_s(scout_arena, env, base),
    }),
    GroupP::Ellipsis { base } => scout_arena.alloc(GroupS::Ellipsis {
      base: translate_group_p_into_group_s(scout_arena, env, base),
    }),
    GroupP::Union { .. } => panic!("POSTPARSER_GROUP_UNION_NOT_YET_IMPLEMENTED"),
  }
}

/// Lowers a function's parse-side effect clauses (`EffectP`) into the symbolic scout-side `EffectS`,
/// resolving each group via `translate_group_p_into_group_s`. These land (borrowed from `'s`) in the
/// per-`FunctionT` side table later; the durable `FunctionHeaderT` never carries them.
pub(crate) fn translate_effects_p_into_effects_s<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  env: &IEnvironmentS<'s>,
  effects_p: &[EffectP<'p>],
) -> Vec<EffectS<'s>> {
  effects_p
    .iter()
    .map(|effect_p| match effect_p {
      EffectP::Mut(group_p) => {
        EffectS::Mut(translate_group_p_into_group_s(scout_arena, env, group_p))
      }
      EffectP::NotMut(group_p) => {
        EffectS::NotMut(translate_group_p_into_group_s(scout_arena, env, group_p))
      }
    })
    .collect()
}

// Lowers a borrow's region (the ITypeST RegionS) into the rule-side RegionSR, resolving a named
// region rune the way a value rune resolves: local runes pass straight through, a parent-env rune
// gets a RuneParentEnvLookup rule.
fn region_s_into_region_sr<'s>(
  scout_arena: &ScoutArena<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  region: RegionS<'s>,
) -> RegionSR<'s> {
  match region {
    RegionS::Unspecified => RegionSR::Unspecified,
    RegionS::Held => RegionSR::Held,
    RegionS::Group(_group_s) => RegionSR::Unspecified,
  }
}

/// Rewrites every rune an ITypeST mentions through `func`, returning a fresh tree. This is the
/// ITypeST twin of the rune-remapping the anonymous-interface macro does over rules: when that macro
/// renames a denizen's runes, the ITypeST it carries must be renamed the same way, or its rune
/// mentions go stale against the remapped rules. Leaves that name no rune pass through unchanged.
pub fn map_runes_in_type_st<'s, F>(
  scout_arena: &ScoutArena<'s>,
  func: &F,
  type_st: &ITypeST<'s>,
) -> ITypeST<'s>
where
  F: Fn(IRuneS<'s>) -> IRuneS<'s>,
{
  match type_st {
    ITypeST::AnonymousRune(_)
    | ITypeST::Bool(_)
    | ITypeST::Int(_)
    | ITypeST::String(_)
    | ITypeST::Name(_) => *type_st,

    ITypeST::Rune(r) => ITypeST::Rune(
      scout_arena
        .alloc(RuneUsageST { rune: RuneUsage { range: r.rune.range, rune: func(r.rune.rune) } }),
    ),

    ITypeST::Call(c) => {
      let template: &'s ITypeST<'s> =
        scout_arena.alloc(map_runes_in_type_st(scout_arena, func, c.template));
      let mut args = Vec::<&'s ITypeST<'s>>::new();
      for arg in c.args {
        args.push(scout_arena.alloc(map_runes_in_type_st(scout_arena, func, arg)));
      }
      ITypeST::Call(scout_arena.alloc(CallST {
        range: c.range,
        template,
        args: scout_arena.alloc_slice_from_vec(args),
      }))
    }

    ITypeST::BorrowRef(br) => {
      let inner: &'s ITypeST<'s> =
        scout_arena.alloc(map_runes_in_type_st(scout_arena, func, br.inner));
      let region = br.region;
      ITypeST::BorrowRef(scout_arena.alloc(BorrowRefST { range: br.range, inner, region }))
    }

    ITypeST::WeakRef(wr) => {
      let inner: &'s ITypeST<'s> =
        scout_arena.alloc(map_runes_in_type_st(scout_arena, func, wr.inner));
      ITypeST::WeakRef(scout_arena.alloc(WeakRefST { range: wr.range, inner }))
    }

    ITypeST::OwnRef(or) => {
      let inner: &'s ITypeST<'s> =
        scout_arena.alloc(map_runes_in_type_st(scout_arena, func, or.inner));
      ITypeST::OwnRef(scout_arena.alloc(OwnRefST { range: or.range, inner }))
    }

    ITypeST::Pack(p) => {
      let mut members = Vec::<&'s ITypeST<'s>>::new();
      for member in p.members {
        members.push(scout_arena.alloc(map_runes_in_type_st(scout_arena, func, member)));
      }
      ITypeST::Pack(
        scout_arena
          .alloc(PackST { range: p.range, members: scout_arena.alloc_slice_from_vec(members) }),
      )
    }

    ITypeST::Tuple(t) => {
      let mut elements = Vec::<&'s ITypeST<'s>>::new();
      for element in t.elements {
        elements.push(scout_arena.alloc(map_runes_in_type_st(scout_arena, func, element)));
      }
      ITypeST::Tuple(
        scout_arena
          .alloc(TupleST { range: t.range, elements: scout_arena.alloc_slice_from_vec(elements) }),
      )
    }

    ITypeST::RuntimeSizedArray(rsa) => {
      let element: &'s ITypeST<'s> =
        scout_arena.alloc(map_runes_in_type_st(scout_arena, func, rsa.element));
      ITypeST::RuntimeSizedArray(
        scout_arena.alloc(RuntimeSizedArrayST { range: rsa.range, element }),
      )
    }

    // The builder never produces a Function node, so its remap is unneeded until it does.
    ITypeST::Function(_) => panic!("POSTPARSER_MAP_RUNES_IN_TYPE_ST_FUNCTION_NOT_YET_IMPLEMENTED"),
  }
}

// Lowers a `func NAME(TYPES)RET` templex — a where-clause function bound — into its rule stream
// (CallSiteFunc/DefinitionFunc/Resolve, unchanged from before). When `maybe_func_bounds` is Some
// (only the top-level where-clause path passes it; a nested func-typed param passes None), it also
// captures the bound as a synthesized abstract-function `FunctionS`.
pub fn translate_func_templex<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  context_region: IRuneS<'s>,
  func: &FuncPT<'p>,
  maybe_func_bounds: Option<&mut Vec<(RuneUsage<'s>, FunctionS<'s>)>>,
) -> RuneUsage<'s> {
  let file = env.file();
  let range_s = PostParser::eval_range(file, func.range);
  let params_range_s = PostParser::eval_range(file, func.params_range);
  let NameP(_, name_p) = &func.name;
  let name: StrI<'s> = scout_arena.intern_str(name_p.as_str());
  let mut params_types = Vec::<ITypeST<'s>>::new();
  for param_p in func.parameters {
    params_types.push(translate_templex_into_type_st(scout_arena, env.clone(), param_p));
  }
  let return_type = translate_templex_into_type_st(scout_arena, env.clone(), func.return_type);

  // Retain each param's @PFVSZ rune split + type (all Copy) so the synthesized bound `FunctionS` can
  // build a `ParameterS` from the same data; the rule stream produced below is unchanged.
  let mut retained_params: Vec<(
    RuneUsage<'s>,
    RuneUsage<'s>,
    &'s [IRulexSR<'s>],
    &'s [IRulexSR<'s>],
    ITypeST<'s>,
  )> = Vec::new();
  let params_s: Vec<RuneUsage<'s>> = params_types
    .iter()
    .map(|param_type| {
      let (full, value, outer_vec, value_vec) = translate_signature_type_st(
        scout_arena,
        keywords,
        env.clone(),
        &mut lidb.child(),
        context_region.clone(),
        param_type,
      );
      let value_slice = scout_arena.alloc_slice_copy(&value_vec);
      let outer_slice = scout_arena.alloc_slice_copy(&outer_vec);
      rule_builder.extend(value_vec);
      rule_builder.extend(outer_vec);
      retained_params.push((full, value, outer_slice, value_slice, *param_type));
      full
    })
    .collect();
  let param_list_rune_s = RuneUsage {
    range: params_range_s.clone(),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
  };
  rule_builder.push(IRulexSR::KindList(KindListSR {
    range: params_range_s,
    result_rune: param_list_rune_s.clone(),
    members: scout_arena.alloc_slice_from_vec(params_s),
  }));

  let (return_rune_s, _rt_value, rt_outer_vec, rt_value_vec) = translate_signature_type_st(
    scout_arena,
    keywords,
    env.clone(),
    &mut lidb.child(),
    context_region.clone(),
    &return_type,
  );
  // The return-type rules become the synthesized bound's own header_rules (param-type rules live on
  // its ParameterS, per @PFVSZ). Copy them out before they are moved into the outer rule stream.
  let mut bound_header_rules_vec: Vec<IRulexSR<'s>> = Vec::new();
  bound_header_rules_vec.extend(rt_value_vec.iter().copied());
  bound_header_rules_vec.extend(rt_outer_vec.iter().copied());
  rule_builder.extend(rt_value_vec);
  rule_builder.extend(rt_outer_vec);

  let result_rune_s = RuneUsage {
    range: PostParser::eval_range(file, func.range),
    rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
  };

  // Only appears in call site; filtered out when solving definition
  rule_builder.push(IRulexSR::CallSiteFunc(CallSiteFuncSR {
    range: range_s.clone(),
    prototype_rune: result_rune_s.clone(),
    name: name.clone(),
    params_list_rune: param_list_rune_s.clone(),
    return_rune: return_rune_s.clone(),
  }));
  // Only appears in definition; filtered out when solving call site
  rule_builder.push(IRulexSR::DefinitionFunc(DefinitionFuncSR {
    range: range_s.clone(),
    result_rune: result_rune_s.clone(),
    name: name.clone(),
    params_list_rune: param_list_rune_s.clone(),
    return_rune: return_rune_s.clone(),
  }));

  // Only appears in call site; filtered out when solving definition
  rule_builder.push(IRulexSR::Resolve(ResolveSR {
    range: range_s,
    result_rune: result_rune_s.clone(),
    name: name.clone(),
    params_list_rune: param_list_rune_s,
    params_types: scout_arena.alloc_slice_from_vec(params_types),
    return_rune: return_rune_s,
    return_type,
  }));

  // A function bound is an abstract-function declaration; when collecting bounds, capture one.
  // Done last, so the implicit-rune LIDs consumed by the rules above are unchanged. Nameless bound
  // params get a synthetic DesugaredParamName; an anonymous bound flows through as `__call`.
  if let Some(func_bounds) = maybe_func_bounds {
    let params_vec: Vec<ParameterS<'s>> = retained_params
      .iter()
      .enumerate()
      .map(|(i, (full, value, outer_slice, value_slice, param_type))| {
        ParameterS::new(
          PostParser::eval_range(file, func.parameters[i].range()),
          None,
          false,
          IVarDeclarationNameS::DesugaredParamName(DesugaredParamNameDeclarationS {
            imprecise_name: scout_arena
              .intern_desugared_param_name(PostParser::eval_pos(file, func.parameters[i].range().begin())),
            lid: lidb.child().consume_in_arena(scout_arena),
          }),
          *param_type,
          *full,
          *value,
          outer_slice,
          value_slice,
        )
      })
      .collect();
    let bound_name = IFunctionDeclarationNameS::FunctionName(FunctionNameS {
      imprecise_name: scout_arena.intern_code_name(name),
      code_location: PostParser::eval_pos(file, func.range.begin()),
      lid: lidb.child().consume_in_arena(scout_arena),
    });
    func_bounds.push((result_rune_s, FunctionS::new(
      PostParser::eval_range(file, func.range),
      bound_name,
      &[],
      &[],
      TemplateTemplataType {
        param_types: &[],
        return_type: scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
      },
      scout_arena.alloc_slice_from_vec(params_vec),
      Some(return_rune_s),
      Some(return_type),
      &[],
      scout_arena.alloc_slice_from_vec(bound_header_rules_vec),
      &[],
      &[],
      scout_arena.alloc(IBodyS::AbstractBody(AbstractBodyS {})),
    )));
  }

  result_rune_s
}

pub fn translate_templex<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  // Nearest enclosing region marker, see RADTGCA.
  context_region: IRuneS<'s>,
  templex: &ITemplexPT<'p>,
) -> RuneUsage<'s> {
  let file = env.file();

  match translate_value_templex(scout_arena, templex) {
    Some(x) => {
      let mut child_lidb = lidb.child();
      add_literal_rule(
        scout_arena,
        &mut child_lidb,
        rule_builder,
        PostParser::eval_range(file, templex.range()),
        x,
      )
    }

    None => match templex {
      ITemplexPT::AnonymousRune(anonymous_rune) => {
        let mut child_lidb = lidb.child();
        let rune = RuneUsage {
          range: PostParser::eval_range(file, anonymous_rune.range),
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        rune
      }

      ITemplexPT::RegionRune(RegionRunePT { range: _, name: None }) => {
        panic!("POSTPARSER_TRANSLATE_TEMPLEX_REGION_RUNE_NONE_NOT_YET_IMPLEMENTED")
      }

      ITemplexPT::RegionRune(RegionRunePT { range, name: Some(name) }) => {
        let name_s = scout_arena.intern_str(name.str().as_str());
        let is_rune_from_local_env = env
          .local_declared_runes()
          .contains(&scout_arena.intern_rune(CodeRune(CodeRuneS { name: name_s })));
        if is_rune_from_local_env {
          RuneUsage {
            range: PostParser::eval_range(file, *range),
            rune: scout_arena.intern_rune(CodeRune(CodeRuneS { name: name_s })),
          }
        } else {
          // It's from a parent env
          let mut child_lidb = lidb.child();
          add_rune_parent_env_lookup_rule(
            scout_arena,
            &mut child_lidb,
            rule_builder,
            PostParser::eval_range(file, *range),
            scout_arena.intern_rune(CodeRune(CodeRuneS { name: name_s })),
          )
        }
      }

      ITemplexPT::NameOrRune(NameOrRunePT { name: name_or_rune, .. }) => {
        let is_rune_from_env =
          env.all_declared_runes().contains(&scout_arena.intern_rune(CodeRune(CodeRuneS {
            name: scout_arena.intern_str(name_or_rune.str().as_str()),
          })));
        if is_rune_from_env {
          let is_rune_from_local_env =
            env.local_declared_runes().contains(&scout_arena.intern_rune(CodeRune(CodeRuneS {
              name: scout_arena.intern_str(name_or_rune.str().as_str()),
            })));
          if is_rune_from_local_env {
            RuneUsage {
              range: PostParser::eval_range(file, name_or_rune.range()),
              rune: scout_arena.intern_rune(CodeRune(CodeRuneS {
                name: scout_arena.intern_str(name_or_rune.str().as_str()),
              })),
            }
          } else {
            // It's from a parent env
            let mut child_lidb = lidb.child();
            add_rune_parent_env_lookup_rule(
              scout_arena,
              &mut child_lidb,
              rule_builder,
              PostParser::eval_range(file, name_or_rune.range()),
              scout_arena.intern_rune(CodeRune(CodeRuneS {
                name: scout_arena.intern_str(name_or_rune.str().as_str()),
              })),
            )
          }
        } else {
          // e.g. "int", or a citizen like "Moo". Per @TNLTZACZ, a bare type-name is a zero-arg
          // application: the name's Lookup, then a Call([]) whose result is the rune we return.
          let name = scout_arena.intern_imprecise_name(CodeName(CodeNameValS {
            name: scout_arena.intern_str(name_or_rune.str().as_str()),
          }));
          let range_s = PostParser::eval_range(file, name_or_rune.range());
          let mut child_lidb = lidb.child();
          let template_rune = add_lookup_rule(
            scout_arena,
            &mut child_lidb,
            rule_builder,
            range_s.clone(),
            context_region,
            name,
          );
          // For lookups like these, we bring them into the current region.
          add_zero_arg_call_rule(scout_arena, &mut child_lidb, rule_builder, range_s, template_rune)
        }
      }

      ITemplexPT::BorrowRef(borrow_ref) => {
        let range_s = PostParser::eval_range(file, borrow_ref.range);
        let inner_rune = translate_templex(
          scout_arena,
          keywords,
          env.clone(),
          &mut lidb.child(),
          rule_builder,
          context_region.clone(),
          borrow_ref.inner,
        );
        let region = match borrow_ref.region {
          RegionP::Unspecified => RegionSR::Unspecified,
          RegionP::Held => RegionSR::Held,
          // The rules/solver side carries no group (`RegionSR` is `Unspecified`/`Held`/`Rune` only);
          // the borrow checker reads a written `in g` off the `ITypeST`, not off the rules. So a group
          // annotation here becomes `Unspecified` for the solve, exactly as the `ITypeST`→`RegionSR`
          // conversion does elsewhere in this file.
          RegionP::Group(_group_p) => RegionSR::Unspecified,
        };
        translate_borrow_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune, region)
      }

      ITemplexPT::WeakRef(weak_ref) => {
        let range_s = PostParser::eval_range(file, weak_ref.range);
        let inner_rune = translate_templex(
          scout_arena,
          keywords,
          env.clone(),
          &mut lidb.child(),
          rule_builder,
          context_region.clone(),
          weak_ref.inner,
        );
        translate_weak_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune)
      }

      ITemplexPT::OwnRef(own_ref) => {
        let range_s = PostParser::eval_range(file, own_ref.range);
        let inner_rune = translate_templex(
          scout_arena,
          keywords,
          env.clone(),
          &mut lidb.child(),
          rule_builder,
          context_region.clone(),
          own_ref.inner,
        );
        translate_own_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune)
      }

      ITemplexPT::Call(call) => {
        let range_s = PostParser::eval_range(file, call.range);
        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        let mut child_lidb = lidb.child();
        let template_rune_s = translate_template_position_templex(
          scout_arena,
          keywords,
          env.clone(),
          &mut child_lidb,
          rule_builder,
          context_region.clone(),
          call.template,
        );
        let mut arg_runes = Vec::<RuneUsage<'s>>::new();
        for arg in call.args {
          let mut child_lidb = lidb.child();
          arg_runes.push(translate_templex(
            scout_arena,
            keywords,
            env.clone(),
            &mut child_lidb,
            rule_builder,
            context_region.clone(),
            arg,
          ));
        }
        rule_builder.push(Call(CallSR {
          range: range_s,
          result_rune: result_rune_s.clone(),
          template_rune: template_rune_s,
          args: scout_arena.alloc_slice_from_vec(arg_runes),
        }));
        result_rune_s
      }

      ITemplexPT::Function(_function) => {
        panic!("POSTPARSER_TRANSLATE_TEMPLEX_FUNCTION_NOT_YET_IMPLEMENTED")
      }

      ITemplexPT::Func(func) => translate_func_templex(
        scout_arena,
        keywords,
        env,
        lidb,
        rule_builder,
        context_region,
        func,
        None,
      ),

      ITemplexPT::Pack(_pack) => panic!("POSTPARSER_TRANSLATE_TEMPLEX_PACK_NOT_YET_IMPLEMENTED"),

      ITemplexPT::RuntimeSizedArray(runtime_sized_array) => {
        let range_s = PostParser::eval_range(file, runtime_sized_array.range);
        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        let mut child_lidb = lidb.child();
        let template_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: scout_arena
            .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        rule_builder.push(Lookup(LookupSR {
          range: range_s.clone(),
          rune: template_rune_s.clone(),
          parts: scout_arena.alloc_slice_copy(&[
            scout_arena.intern_imprecise_name(CodeName(CodeNameValS { name: keywords.array }))
          ]),
        }));
        let mut child_lidb = lidb.child();
        let element_rune_s = translate_templex(
          scout_arena,
          keywords,
          env,
          &mut child_lidb,
          rule_builder,
          context_region,
          runtime_sized_array.element,
        );
        rule_builder.push(Call(CallSR {
          range: range_s,
          result_rune: result_rune_s.clone(),
          template_rune: template_rune_s,
          args: scout_arena.alloc_slice_from_vec(vec![element_rune_s]),
        }));
        result_rune_s
      }

      ITemplexPT::Tuple(tuple) => {
        let range_s = PostParser::eval_range(file, tuple.range);
        let tuple_name = scout_arena.intern_imprecise_name(CodeName(CodeNameValS {
          name: keywords.tuple_human_name[tuple.elements.len()],
        }));
        if tuple.elements.is_empty() {
          // Zero-arg tuple `()`: lowers like any bare type-name, per @TNLTZACZ.
          let mut child_lidb = lidb.child();
          let template_rune_s = RuneUsage {
            range: range_s.clone(),
            rune: scout_arena
              .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
          };
          rule_builder.push(Lookup(LookupSR {
            range: range_s.clone(),
            rune: template_rune_s.clone(),
            parts: scout_arena.alloc_slice_copy(&[tuple_name]),
          }));
          add_zero_arg_call_rule(
            scout_arena,
            &mut child_lidb,
            rule_builder,
            range_s,
            template_rune_s,
          )
        } else {
          let mut child_lidb = lidb.child();
          let result_rune_s = RuneUsage {
            range: range_s.clone(),
            rune: scout_arena
              .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
          };
          let mut child_lidb = lidb.child();
          let template_rune_s = RuneUsage {
            range: range_s.clone(),
            rune: scout_arena
              .intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
          };
          rule_builder.push(Lookup(LookupSR {
            range: range_s.clone(),
            rune: template_rune_s.clone(),
            parts: scout_arena.alloc_slice_copy(&[tuple_name]),
          }));
          let mut element_runes = Vec::<RuneUsage<'s>>::new();
          for element in tuple.elements {
            let mut child_lidb = lidb.child();
            element_runes.push(translate_templex(
              scout_arena,
              keywords,
              env.clone(),
              &mut child_lidb,
              rule_builder,
              context_region.clone(),
              element,
            ));
          }
          rule_builder.push(Call(CallSR {
            range: range_s,
            result_rune: result_rune_s.clone(),
            template_rune: template_rune_s,
            args: scout_arena.alloc_slice_from_vec(element_runes),
          }));
          result_rune_s
        }
      }

      _ => panic!("POSTPARSER_TRANSLATE_TEMPLEX_NOT_YET_IMPLEMENTED"),
    },
  }
}

// Returns:
// - Rune for this type
fn translate_type_into_rune<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  // Nearest enclosing region marker, see RADTGCA.
  context_region: IRuneS<'s>,
  type_p: &ITemplexPT<'p>,
) -> RuneUsage<'s> {
  let file = env.file();
  match type_p {
    NameOrRune(NameOrRunePT { name: NameP(range, name_or_rune), .. })
      if env.all_declared_runes().contains(&scout_arena.intern_rune(CodeRune(CodeRuneS {
        name: scout_arena.intern_str(name_or_rune.as_str()),
      }))) =>
    {
      let result_rune_s = RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune: scout_arena
          .intern_rune(CodeRune(CodeRuneS { name: scout_arena.intern_str(name_or_rune.as_str()) })),
      };
      result_rune_s
    }
    non_rune_templex_p => {
      let mut child_lidb = lidb.child();
      translate_templex(
        scout_arena,
        keywords,
        env,
        &mut child_lidb,
        rule_builder,
        context_region,
        non_rune_templex_p,
      )
    }
  }
}

// Returns:
// - Rune for this type
pub fn translate_maybe_type_into_rune<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeS<'s>,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  context_region: IRuneS<'s>,
  maybe_type_p: Option<&ITemplexPT<'p>>,
) -> RuneUsage<'s> {
  match maybe_type_p {
    None => {
      let mut child_lidb = lidb.child();
      let result_rune_s = RuneUsage {
        range,
        rune: scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
      };
      result_rune_s
    }
    Some(type_p) => translate_type_into_rune(
      scout_arena,
      keywords,
      env,
      lidb,
      rule_builder,
      context_region,
      type_p,
    ),
  }
}

pub(crate) fn translate_maybe_type_into_maybe_rune<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeS<'s>,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  context_region: IRuneS<'s>,
  maybe_type_p: Option<&ITemplexPT<'p>>,
) -> Option<RuneUsage<'s>> {
  if maybe_type_p.is_none() {
    None
  } else {
    let mut child_lidb = lidb.child();
    let result_rune = translate_maybe_type_into_rune(
      scout_arena,
      keywords,
      env,
      &mut child_lidb,
      range,
      rule_builder,
      context_region,
      maybe_type_p,
    );
    Some(result_rune)
  }
}
