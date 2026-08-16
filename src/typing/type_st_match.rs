use crate::utils::range::RangeS;

use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::infer_compiler::InferEnv;

use crate::typing::env::environment::{IEnvironmentT, IInDenizenEnvironmentT};
use crate::typing::names::names::*; // IdT
use crate::typing::templata::templata::*; // ITemplataT, StructDefinitionTemplataT, InterfaceDefinitionTemplataT
use crate::typing::types::types::*; // KindT, ISubKindTT, ISuperKindTT, RegionT

use crate::postparsing::ast::{LocationInDenizen, ParameterS};
use crate::postparsing::names::{IImpreciseNameS, IRuneS};
use crate::postparsing::rules::rules::IRulexSR;

// ---- §2A dyn-upcast: reuse get_parents to find the arg's super, keep the one the param wants ----

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  /// Phase 2A dyn upcastability
  /// VCOORD: check this against ~/.claude/plans/true-phase-2a-please-validated-walrus.md
  /// Some(coerced value kind) when `peeled_arg`'s template differs from the param's expected
  /// value-type template AND the arg implements that expected interface; else None. Reuses
  /// get_parents (the same super-finding the compiler already uses for upcasts elsewhere) rather
  /// than re-matching impls by hand: it solves each relating impl and hands back the concrete
  /// super, e.g. Opt<int> for a Some<int> arg.
  pub fn compute_upcast_coerced_arg(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range_t: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    context_region: RegionT,
    peeled_arg: KindT<'s, 't>,
    param: &ParameterS<'s>,
  ) -> Option<KindT<'s, 't>> {
    // A concrete citizen OR a generic placeholder can upcast: get_parents finds a citizen's impls
    // and a placeholder's where-clause bound parents alike (via get_sub_kind_template, not the
    // citizen-only get_citizen_template). Primitives and refs have no parents here.
    let arg_sub_kind = match peeled_arg {
      KindT::Struct(s) => ISubKindTT::Struct(s),
      KindT::Interface(i) => ISubKindTT::Interface(i),
      KindT::KindPlaceholder(kp) => ISubKindTT::KindPlaceholder(kp),
      _ => return None,
    };

    // The template the param wants (static scan; None for a bare generic param `m T`).
    let expected_template_id = self.param_expected_value_type_template(
      coutputs,
      calling_env,
      call_range_t,
      call_location,
      context_region,
      param,
    )?;

    if let KindT::Struct(_) | KindT::Interface(_) = peeled_arg {
      if self.get_citizen_template(arg_sub_kind.id()) == expected_template_id {
        return None;
      }
    }

    // Pick the arg's interface super whose template is the param's expected interface. Skip a
    // KindPlaceholder super (a generic bound's parent), which has no citizen template to name.
    // (No matching super means None, preserving the existing reject for unrelated types; more
    // than one takes the first, with as<> disambiguation deferred.)
    self
      .get_parents(coutputs, call_range_t, call_location, calling_env, arg_sub_kind)
      .into_iter()
      .find(|s| {
        matches!(s, ISuperKindTT::Interface(_))
          && self.get_citizen_template(s.id()) == expected_template_id
      })
      .map(KindT::from)
  }

  /// The imprecise citizen name heading the param's value type, resolved to a template id.
  /// None when the value type is a bare generic-param rune.
  /// TODO: rename to get_param_value_type_template
  fn param_expected_value_type_template(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range_t: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    context_region: RegionT,
    param: &ParameterS<'s>,
  ) -> Option<IdT<'s, 't>> {
    let name = value_type_root_name(param.value_type_rules, param.value_type_rune.rune)?;
    let envs = InferEnv {
      original_calling_env: calling_env,
      parent_ranges: call_range_t,
      call_location,
      self_env: IEnvironmentT::from(calling_env),
      context_region,
    };
    match self.lookup_templata_imprecise(envs, coutputs, call_range_t, name)? {
      ITemplataT::StructDefinition(sd) => Some(*sd.struct_template_id),
      ITemplataT::InterfaceDefinition(idf) => Some(*idf.interface_template_id),
      _ => None,
    }
  }
}

/// Pure: if a Call produces value_type_rune, follow its template_rune; then take the imprecise name
/// from the Lookup that binds that rune.
fn value_type_root_name<'s>(
  rules: &[IRulexSR<'s>],
  value_type_rune: IRuneS<'s>,
) -> Option<IImpreciseNameS<'s>> {
  let mut target = value_type_rune;
  if let Some(c) = rules.iter().find_map(|r| match r {
    IRulexSR::Call(c) if c.result_rune.rune == value_type_rune => Some(c),
    _ => None,
  }) {
    target = c.template_rune.rune;
  }
  rules.iter().find_map(|r| match r {
    IRulexSR::Lookup(l) if l.rune.rune == target => l.parts.first().copied(),
    _ => None,
  })
}
