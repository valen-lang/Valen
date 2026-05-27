// AFTERM: rename to function_post_parser.rs
// AFTERM: review scout_function
// Per @DSAUIMZ, all borrow_val() calls in this file borrow from a stack-local
// LocationInDenizenBuilder instead of arena-allocating. The slice is promoted
// to permanent arena storage only inside intern_rune on a miss.

use crate::parsing::ast::{FunctionP, GenericParameterP, IAttributeP, ITemplexPT, LoadAsP};
use crate::parsing::ast::rules::get_ordered_rune_declarations_from_rulexes_with_duplicates;
use crate::postparsing::ast::{
  AbstractBodyS, AbstractSP, AdditiveS, BuiltinS, CodeBodyS, CoordGenericParameterTypeS, ExportS,
  ExternBodyS, ExternS, FunctionS, GeneratedBodyS, GenericParameterS, IBodyS, IFunctionAttributeS,
  IGenericParameterTypeS, IRegionMutabilityS, LocationInDenizenBuilder, ParameterS, PureS,
  RegionGenericParameterTypeS,
};
use crate::postparsing::expressions::{
  BlockSE, BodySE, ConsecutorSE, IExpressionSE,
};
use crate::postparsing::itemplatatype::{
  CoordTemplataType, FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::lexing::ast::RangeL;
use crate::postparsing::names::{
  ClosureParamNameS, CodeNameS, CodeRuneS, DenizenDefaultRegionRuneS, FunctionNameS,
  IFunctionDeclarationNameS, IFunctionDeclarationNameValS, IImpreciseNameValS, INameS, INameValS,
  IRuneS, IRuneValS, IVarNameS, IVarNameValS, ImplicitRegionRuneValS, ImplicitRuneValS, LambdaDeclarationNameS,
  LambdaStructDeclarationNameS, MagicParamRuneValS,
};
use crate::postparsing::post_parser::{
  CouldntFindRuneS, ExternHasBodyS, FunctionEnvironmentS, ICompileErrorS, IEnvironmentS,
  InterfaceMethodNeedsSelf, PostParser, RangedInternalErrorS, StackFrame, VirtualAndAbstractGoTogether,
};
use crate::postparsing::patterns::pattern_scout::{get_parameter_captures, translate_pattern};
use crate::postparsing::rules::rule_scout::translate_rulexes;
use crate::postparsing::rules::templex_scout::translate_maybe_type_into_maybe_rune;
use crate::parsing::ast::OwnershipP;
use crate::postparsing::rules::rules::{
  AugmentSR, CoerceToCoordSR, IRulexSR, LookupSR, MaybeCoercingLookupSR, RuneUsage,
};
use crate::postparsing::variable_uses::{VariableDeclarationS, VariableDeclarations, VariableUses};
use crate::utils::range::RangeS;
use crate::utils::code_hierarchy::FileCoordinate;
use std::collections::HashMap;
use crate::utils::arena_index_map::ArenaIndexMap;
use indexmap::IndexSet;
use crate::parsing::ast::BlockPE;
use crate::postparsing::expressions::LocalS;
/*
package dev.vale.postparsing

import dev.vale.postparsing.rules.{AugmentSR, IRulexSR, MaybeCoercingLookupSR, RuleScout, RuneUsage, TemplexScout}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import PostParser.{evalRange, noDeclarations, noVariableUses}
import dev.vale
import dev.vale.lexing.RangeL
import dev.vale.{FileCoordinate, Interner, Keywords, RangeS, postparsing, vassertSome, vcurious, vimpl, vwat}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS, PatternScout}
import dev.vale.postparsing.patterns._
//import dev.vale.postparsing.predictor.{Conclusions, PredictorEvaluator}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
//import dev.vale.postparsing.predictor.Conclusions
import dev.vale.postparsing.rules._
//import dev.vale.postparsing.templatepredictor.PredictorEvaluator
import dev.vale._

import scala.collection.immutable.{List, Range}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IFunctionParent<'s>

{
  FunctionNoParent,
  ParentCitizen {
    citizen_is_interface: bool,
    citizen_env: FunctionEnvironmentS<'s>,
    citizen_generic_params: &'s [&'s GenericParameterS<'s>],
    citizen_rules: Vec<IRulexSR<'s>>,
    citizen_rune_to_explicit_type: HashMap<IRuneS<'s>, ITemplataType<'s>>,
  },
  ParentFunction {
    parent_stack_frame: StackFrame<'s>,
  },
}

/*
sealed trait IFunctionParent
*/
/*
case class FunctionNoParent() extends IFunctionParent
*/
/*
case class ParentCitizen(
  citizenIsInterface: Boolean,
  citizenEnv: EnvironmentS,
  citizenGenericParams: Vector[GenericParameterS],
  citizenRules: Vector[IRulexSR],
  citizenRuneToExplicitType: Map[IRuneS, ITemplataType]
) extends IFunctionParent
*/
/*
case class ParentFunction(
  parentStackFrame: StackFrame
) extends IFunctionParent
*/
/*
class FunctionScout(
    postParser: PostParser,
    interner: Interner,
    keywords: Keywords,
    templexScout: TemplexScout,
    ruleScout: RuleScout) {
  val patternScout = new PatternScout(interner, templexScout)
  val expressionScout =
    new ExpressionScout(
      new IExpressionScoutDelegate {
        override def scoutLambda(parentStackFrame: StackFrame, lambdaFunction0: FunctionP): (FunctionS, VariableUses) = {
          FunctionScout.this.scoutLambda(parentStackFrame, lambdaFunction0)
        }
      },
      templexScout,
      ruleScout,
      patternScout,
      interner,
      keywords
    )
*/
impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  pub(crate) fn scout_function(
    &self,
    file_coordinate: &'s FileCoordinate<'s>,
    function: &FunctionP<'p>,
    maybe_parent: IFunctionParent<'s>,
  ) -> Result<(&'s FunctionS<'s>, VariableUses<'s>), ICompileErrorS<'s>>
  {
    // AFTERM: check the order of these various chunks of logic

    let is_parent_function = matches!(&maybe_parent, IFunctionParent::ParentFunction { .. });
    let is_parent_interface = matches!(&maybe_parent, IFunctionParent::ParentCitizen { citizen_is_interface: true, .. });
    let function_name = function.header.name.as_ref();
    let generic_parameters_p: &[GenericParameterP<'p>] = function
      .header
      .generic_parameters
      .as_ref()
      .map(|generic_parameters| generic_parameters.params)
      .unwrap_or(&[]);
    // See: Must Scan For Declared Runes First (MSFDRF)
    let user_specified_identifying_runes: Vec<RuneUsage<'s>> = generic_parameters_p
      .iter()
      .map(|generic_parameter| RuneUsage {
        range: Self::eval_range(file_coordinate, function.range),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(generic_parameter.name.str().as_str()),
        })),
      })
      .collect();
    let user_runes_from_rules: Vec<RuneUsage<'s>> = function
      .header
      .template_rules
      .as_ref()
      .map(|template_rules| {
        get_ordered_rune_declarations_from_rulexes_with_duplicates(template_rules.rules)
          .into_iter()
          .map(|name_p| RuneUsage {
            range: Self::eval_range(file_coordinate, function.range),
            rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
              name: self.scout_arena.intern_str(name_p.str().as_str()),
            })),
          })
          .collect()
      })
      .unwrap_or_default();
    let mut user_declared_runes = user_specified_identifying_runes.clone();
    for rune_usage in user_runes_from_rules {
      if !user_declared_runes
        .iter()
        .any(|existing_rune_usage| existing_rune_usage.rune == rune_usage.rune)
      {
        user_declared_runes.push(rune_usage);
      }
    }
    match &maybe_parent {
      IFunctionParent::FunctionNoParent => {
        if function.header.attributes.iter().any(|a| matches!(a, IAttributeP::AbstractAttribute(_))) {
          match &function.header.params {
            None => {
              return Err(ICompileErrorS::VirtualAndAbstractGoTogether(
                VirtualAndAbstractGoTogether {
                  range: Self::eval_range(file_coordinate, function.range),
                },
              ));
            }
            Some(params) => {
              if !params.params.iter().any(|param| param.virtuality.is_some()) {
                return Err(ICompileErrorS::VirtualAndAbstractGoTogether(
                  VirtualAndAbstractGoTogether {
                    range: Self::eval_range(file_coordinate, function.range),
                  },
                ));
              }
            }
          }
        }
      }
      IFunctionParent::ParentFunction { .. } => {}
      IFunctionParent::ParentCitizen { citizen_is_interface, .. } => {
        // When we have traits that can have static methods, this check might need to go away
        if *citizen_is_interface {
          if let Some(params) = &function.header.params {
            if !params.params.iter().any(|param| param.virtuality.is_some()) {
              return Err(ICompileErrorS::InterfaceMethodNeedsSelf(
                InterfaceMethodNeedsSelf {
                  range: Self::eval_range(file_coordinate, function.range),
                },
              ));
            }
          }
        }
      }
    }
    let mut lidb = LocationInDenizenBuilder::new(vec![]);
    let mut rules: Vec<IRulexSR<'s>> = Vec::new();
    let mut rune_to_explicit_type: Vec<(IRuneS<'s>, ITemplataType)> = Vec::new();
    let function_declaration_name = match (&maybe_parent, function_name) {
      (IFunctionParent::ParentFunction { .. }, Some(_)) => {
        panic!("POSTPARSER_SCOUT_LAMBDA_WITH_NAME_NOT_YET_IMPLEMENTED");
      }
      (_, Some(function_name)) => self.scout_arena.intern_name(INameValS::FunctionDeclaration(
        IFunctionDeclarationNameValS::FunctionName(FunctionNameS {
          name: self.scout_arena.intern_str(function_name.str().as_str()),
          code_location: Self::eval_pos(file_coordinate, function.range.begin()),
        }),
      )),
      (IFunctionParent::ParentFunction { .. }, None) => self.scout_arena.intern_name(
        INameValS::FunctionDeclaration(IFunctionDeclarationNameValS::LambdaDeclarationName(
          LambdaDeclarationNameS {
            code_location: Self::eval_pos(file_coordinate, function.range.begin()),
          },
        )),
      ),
      _ => panic!("POSTPARSER_SCOUT_FUNCTION_WITHOUT_NAME"),
    };
    let function_declaration_name_for_env = match &function_declaration_name {
      INameS::FunctionDeclaration(r) => (*r).clone(),
      _ => panic!("POSTPARSER_INTERN_FUNCTION_NAME_EXPECTED_FUNCTION_DECLARATION"),
    };
    let extra_generic_params_from_parent: Vec<&'s GenericParameterS<'s>> = match &maybe_parent {
      IFunctionParent::ParentCitizen {
        citizen_generic_params,
        ..
      } => citizen_generic_params.to_vec(),
      _ => Vec::new(),
    };
    for gp in &extra_generic_params_from_parent {
      rune_to_explicit_type.push((gp.rune.rune.clone(), gp.tyype.tyype()));
    }
    let parent_env: Option<Box<IEnvironmentS<'s>>> = match &maybe_parent {
      IFunctionParent::FunctionNoParent => None,
      IFunctionParent::ParentFunction { parent_stack_frame } => {
        Some(Box::new(IEnvironmentS::FunctionEnvironment(
          parent_stack_frame.parent_env.clone(),
        )))
      }
      IFunctionParent::ParentCitizen { citizen_env: interface_env, .. } => {
        Some(Box::new(IEnvironmentS::FunctionEnvironment(interface_env.clone())))
      }
    };
    let declared_runes: IndexSet<IRuneS<'s>> = match &maybe_parent {
      IFunctionParent::ParentCitizen { .. } => IndexSet::new(),
      _ => user_declared_runes
        .iter()
        .map(|rune_usage| rune_usage.rune.clone())
        .collect(),
    };
    let function_environment = FunctionEnvironmentS {
      file: file_coordinate,
      name: function_declaration_name_for_env.clone(),
      parent_env,
      declared_runes,
      num_explicit_params: function
        .header
        .params
        .as_ref()
        .map(|params| params.params.len() as i32)
        .unwrap_or(0),
      is_interface_internal_method: matches!(&maybe_parent, IFunctionParent::ParentCitizen { .. }),
    };
    let header_range_s = Self::eval_range(file_coordinate, function.header.range);
    let (default_region_rune, _maybe_region_generic_param): (IRuneS<'s>, _) = match function
      .body
      .as_ref()
      .and_then(|body| body.maybe_default_region.as_ref())
    {
      None => {
        let region_range = RangeS::new(
          header_range_s.end.clone(),
          header_range_s.end.clone(),
        );
        let rune = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
          DenizenDefaultRegionRuneS {
            denizen_name: function_declaration_name.clone(),
          },
        ));
        let implicit_region_generic_param = GenericParameterS {
          range: region_range.clone(),
          rune: RuneUsage {
            range: region_range,
            rune: rune.clone(),
          },
          tyype: IGenericParameterTypeS::RegionGenericParameterType(
            RegionGenericParameterTypeS {
              mutability: IRegionMutabilityS::ReadWriteRegion,
            },
          ),
          default: None,
        };
        (rune, Some(implicit_region_generic_param))
      }
      Some(region_rune_pt) => {
        let region_name = region_rune_pt
          .name
          .as_ref()
          .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_FUNCTION_DEFAULT_REGION_NAME_MISSING"));
        let rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(region_name.str().as_str()),
        }));
        if !function_environment.all_declared_runes().contains(&rune) {
          return Err(ICompileErrorS::CouldntFindRuneS(CouldntFindRuneS {
            range: Self::eval_range(file_coordinate, function.range),
            name: region_name.str().as_str().to_string(),
          }));
        }
        (rune, None)
      }
    };
    let template_rules_p = function
      .header
      .template_rules
      .as_ref()
      .map(|template_rules| template_rules.rules)
      .unwrap_or(&[]);
    match &maybe_parent {
      IFunctionParent::FunctionNoParent => {
        let mut child_lidb = lidb.child();
        translate_rulexes(
          self.scout_arena,
          self.keywords,
          IEnvironmentS::FunctionEnvironment(function_environment.clone()),
          &mut child_lidb,
          &mut rules,
          &mut rune_to_explicit_type,
          default_region_rune.clone(),
          template_rules_p,
        );
      }
      IFunctionParent::ParentFunction { .. } => {
        assert!(
          template_rules_p.is_empty(),
          "POSTPARSER_SCOUT_FUNCTION_TEMPLATE_RULES_NOT_YET_IMPLEMENTED"
        );
      }
      IFunctionParent::ParentCitizen { citizen_env: interface_env, .. } => {
        let mut child_lidb = lidb.child();
        translate_rulexes(
          self.scout_arena,
          self.keywords,
          IEnvironmentS::FunctionEnvironment(interface_env.clone()),
          &mut child_lidb,
          &mut rules,
          &mut rune_to_explicit_type,
          default_region_rune.clone(),
          template_rules_p,
        );
      }
    }
    // We'll add the implicit runes to the end, see IRRAE.
    let function_user_specified_generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
      .iter()
      .zip(user_specified_identifying_runes.iter())
      .map(|(generic_parameter_p, identifying_rune_s)| {
        let mut child_lidb = lidb.child();
        &*self.scout_arena.alloc(self.scout_generic_parameter(
          IEnvironmentS::FunctionEnvironment(function_environment.clone()),
          &mut child_lidb,
          &mut rune_to_explicit_type,
          &mut rules,
          default_region_rune.clone(),
          generic_parameter_p,
          identifying_rune_s.clone(),
        ))
      })
      .collect::<Vec<_>>();
    let params_p: Vec<_> = function
      .header
      .params
      .as_ref()
      .map(|params| {
        params
          .params
          .iter()
          .map(|param| {
            match &maybe_parent {
              IFunctionParent::FunctionNoParent | IFunctionParent::ParentCitizen { .. } => {
                assert!(
                  param.pattern.as_ref().map(|pattern| pattern.templex.is_some()).unwrap_or(false),
                  "POSTPARSER_SCOUT_FUNCTION_PARAM_TYPE_REQUIRED_NOT_YET_IMPLEMENTED"
                );
              }
              IFunctionParent::ParentFunction { .. } => {}
            }
            param
          })
          .collect()
      })
      .unwrap_or_default();
    // We say PerhapsTypeless because we're in a lambda, they might be anonymous params.
    // For lambdas, untyped explicit params (like `(a, b) => ...` or `(_) => ...`) get a
    // synthesized coord rune here that will be added to the function's identifying runes.
    let explicit_params_s_and_synthesized_runes: Vec<(ParameterS<'s>, Option<RuneUsage<'s>>)> = params_p
      .iter()
      .map(|param| {
            let param_range = PostParser::eval_range(file_coordinate, param.range);
            let virtuality = param.virtuality.as_ref().map(|abstract_p| AbstractSP {
              range: PostParser::eval_range(file_coordinate, abstract_p.range),
              is_internal_method: matches!(&maybe_parent, IFunctionParent::ParentCitizen { .. }),
            });
            let (pattern, synthesized_rune): (AtomSP<'s>, Option<RuneUsage<'s>>) = match (&param.self_borrow, &param.pattern) {
              (Some(_), None) => {
                let coord_rune = RuneUsage {
                  range: param_range.clone(),
                  rune: self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
                };
                rune_to_explicit_type.push((
                  coord_rune.rune.clone(),
                  ITemplataType::CoordTemplataType(CoordTemplataType {}),
                ));
                let pattern_s = AtomSP {
                  range: param_range.clone(),
                  name: Some(CaptureS {
                    name: IVarNameS::CodeVarName(self.keywords.self_),
                    mutate: false,
                  }),
                  coord_rune: Some(coord_rune),
                  destructure: None,
                };
                (pattern_s, None)
              }
              (None, Some(pattern)) => {
                let mut pattern_lidb = lidb.child();
                let mut rune_to_explicit_type_for_pattern: HashMap<IRuneS<'s>, ITemplataType> =
                  rune_to_explicit_type.iter().cloned().collect();
                let mut pattern_s = translate_pattern(
                  self.scout_arena,
                  self.keywords,
                  StackFrame {
                    file: file_coordinate,
                    name: function_declaration_name_for_env.clone(),
                    parent_env: function_environment.clone(),
                    maybe_parent: None,
                    context_region: default_region_rune.clone(),
                    pure_height: 0,
                    locals: VariableDeclarations { vars: Vec::new() },
                  },
                  &mut pattern_lidb,
                  &mut rules,
                  &mut rune_to_explicit_type_for_pattern,
                  pattern,
                );
                for (rune, tyype) in rune_to_explicit_type_for_pattern {
                  if !rune_to_explicit_type
                    .iter()
                    .any(|(existing_rune, _)| *existing_rune == rune)
                  {
                    rune_to_explicit_type.push((rune, tyype));
                  }
                }
                match pattern_s.coord_rune {
                  None => {
                    // Untyped param (like in `(a) => a`) so make a rune that will be added to
                    // genericParams and to the identifying runes. This only happens for lambdas,
                    // top level functions can't have these (enforced elsewhere).
                    let coord_rune = RuneUsage {
                      range: param_range.clone(),
                      rune: self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
                    };
                    rune_to_explicit_type.push((
                      coord_rune.rune.clone(),
                      ITemplataType::CoordTemplataType(CoordTemplataType {}),
                    ));
                    pattern_s.coord_rune = Some(coord_rune.clone());
                    (pattern_s, Some(coord_rune))
                  }
                  Some(_) => (pattern_s, None),
                }
              }
              _ => panic!("POSTPARSER_SCOUT_FUNCTION_PARAM_FORM_NOT_YET_IMPLEMENTED"),
            };
            (
              ParameterS::new(
                param_range.clone(),
                virtuality,
                param.maybe_pre_checked.is_some(),
                pattern,
              ),
              synthesized_rune,
            )
      })
      .collect::<Vec<(ParameterS<'s>, Option<RuneUsage<'s>>)>>();
    let mut explicit_params_s: Vec<ParameterS<'s>> = Vec::new();
    let mut explicit_params_synthesized_runes: Vec<RuneUsage<'s>> = Vec::new();
    for (param_s, maybe_rune) in explicit_params_s_and_synthesized_runes {
      explicit_params_s.push(param_s);
      if let Some(rune) = maybe_rune {
        explicit_params_synthesized_runes.push(rune);
      }
    }
    // Untyped lambda params (from `(_) =>` or `(a, b) =>`) contribute their synthesized coord runes here so later
    // passes see a uniform FunctionS shape regardless of whether the user wrote `<T>` or an untyped param.
    let extra_generic_params_from_explicit_params_s: Vec<&'s GenericParameterS<'s>> =
      explicit_params_synthesized_runes.into_iter()
        .map(|rune| {
          &*self.scout_arena.alloc(GenericParameterS {
            range: rune.range,
            rune,
            tyype: IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS {
              coord_region: None,
              kind_mutable: true,
              region_mutable: false,
            }),
            default: None,
          })
        })
        .collect();
    let maybe_capture_declarations = match function.body {
      None => None,
      Some(_) => {
        let mut first_params = match &maybe_parent {
          IFunctionParent::FunctionNoParent | IFunctionParent::ParentCitizen { .. } => {
            VariableDeclarations { vars: Vec::new() }
          }
          IFunctionParent::ParentFunction { .. } => {
            let closure_param_pos = Self::eval_pos(file_coordinate, function.range.begin());
            let closure_param_name = match self.scout_arena.intern_name(INameValS::VarName(
              IVarNameValS::ClosureParamName(ClosureParamNameS {
                code_location: closure_param_pos,
              }),
            )) {
              INameS::VarName(r) => (*r).clone(),
              _ => panic!("POSTPARSER_INTERN_VAR_NAME_EXPECTED_VAR_NAME"),
            };
            VariableDeclarations {
              vars: vec![VariableDeclarationS {
                name: closure_param_name,
              }],
            }
          }
        };
        for explicit_param_s in &explicit_params_s {
          let param_declarations = VariableDeclarations {
            vars: get_parameter_captures(&explicit_param_s.pattern),
          };
          first_params = first_params.plus_plus(&param_declarations);
        }
        Some(first_params)
      }
    };
    let maybe_ret_coord_rune = match &function.header.ret.ret_type {
      None | Some(ITemplexPT::RegionRune(_)) => {
        if is_parent_function {
          None
        } else {
          let ret_range_s = Self::eval_range(file_coordinate, function.header.ret.range);
          let ret_rune = RuneUsage {
            range: ret_range_s.clone(),
            rune: self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
          };
          rules.push(IRulexSR::MaybeCoercingLookup(MaybeCoercingLookupSR {
            range: ret_range_s.clone(),
            rune: ret_rune.clone(),
            name: self
              .scout_arena
              .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
                name: self.keywords.void,
              })),
          }));
          rune_to_explicit_type.push((
            ret_rune.rune.clone(),
            ITemplataType::CoordTemplataType(CoordTemplataType {}),
          ));
          Some(ret_rune)
        }
      }
      Some(ret_type_p) => {
        let mut ret_lidb = lidb.child();
        let mut rune_to_explicit_type_for_ret: HashMap<IRuneS<'s>, ITemplataType> =
          rune_to_explicit_type.iter().cloned().collect();
        let ret_rune = translate_maybe_type_into_maybe_rune(
          self.scout_arena,
          self.keywords,
          IEnvironmentS::FunctionEnvironment(function_environment.clone()),
          &mut ret_lidb,
          Self::eval_range(file_coordinate, function.header.ret.range),
          &mut rules,
          &mut rune_to_explicit_type_for_ret,
          default_region_rune.clone(),
          Some(ret_type_p),
        );
        for (rune, tyype) in rune_to_explicit_type_for_ret {
          if !rune_to_explicit_type
            .iter()
            .any(|(existing_rune, _)| *existing_rune == rune)
          {
            rune_to_explicit_type.push((rune, tyype));
          }
        }
        if let Some(ret_rune) = &ret_rune {
          rune_to_explicit_type.push((
            ret_rune.rune.clone(),
            ITemplataType::CoordTemplataType(CoordTemplataType {}),
          ));
        }
        ret_rune
      }
    };
    let has_extern_attr = function
      .header
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::ExternAttribute(_)));
    let has_abstract_attr = function
      .header
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::AbstractAttribute(_)));
    let has_builtin_attr = function
      .header
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::BuiltinAttribute(_)));
    if is_parent_interface && has_abstract_attr {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        range: Self::eval_range(file_coordinate, function.range),
        message: "Dont need abstract here".to_string(),
      }));
    }
    let (body_s, variable_uses, total_params_s, extra_generic_params_from_body) = if is_parent_interface {
      (
        &*self.scout_arena.alloc(IBodyS::AbstractBody(AbstractBodyS {})),
        VariableUses::<'s>::empty(),
        explicit_params_s,
        Vec::<&'s GenericParameterS<'s>>::new(),
      )
    } else if has_abstract_attr {
      (
        &*self.scout_arena.alloc(IBodyS::AbstractBody(AbstractBodyS {})),
        VariableUses::<'s>::empty(),
        explicit_params_s,
        Vec::<&'s GenericParameterS<'s>>::new(),
      )
    } else if has_extern_attr {
      if function.body.is_some() {
        return Err(ICompileErrorS::ExternHasBodyS(ExternHasBodyS {
          range: Self::eval_range(file_coordinate, function.range),
        }));
      }
      (
        &*self.scout_arena.alloc(IBodyS::ExternBody(ExternBodyS {})),
        VariableUses::<'s>::empty(),
        explicit_params_s,
        Vec::<&'s GenericParameterS<'s>>::new(),
      )
    } else if has_builtin_attr {
      let generator_name = function
        .header
        .attributes
        .iter()
        .find_map(|attr| match attr {
          IAttributeP::BuiltinAttribute(builtin_attr) => Some(self.scout_arena.intern_str(builtin_attr.generator_name.str().as_str())),
          _ => None,
        })
        .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_FUNCTION_BUILTIN_ATTR_NOT_FOUND"));
      (
        &*self.scout_arena.alloc(IBodyS::GeneratedBody(GeneratedBodyS { generator_id: generator_name })),
        VariableUses::<'s>::empty(),
        explicit_params_s,
        Vec::<&'s GenericParameterS<'s>>::new(),
      )
    } else {
      let body = function
        .body
        .as_ref()
        .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_FUNCTION_WITHOUT_BODY"));
      if body.maybe_pure.is_some() {
        panic!("POSTPARSER_SCOUT_PURE_BLOCKS_NOT_YET_IMPLEMENTED");
      }
      if body.maybe_default_region.is_some() {
        panic!("POSTPARSER_SCOUT_BLOCK_DEFAULT_REGION_NOT_YET_IMPLEMENTED");
      }
      let parent_stack_frame = match &maybe_parent {
        IFunctionParent::ParentFunction { parent_stack_frame } => Some(parent_stack_frame.clone()),
        _ => None,
      };
      let (body_s, variable_uses, magic_param_names): (&'s BodySE<'s>, VariableUses<'s>, Vec<IVarNameS<'s>>) = self.scout_body(
        function_environment,
        parent_stack_frame,
        &mut lidb,
        default_region_rune,
        body,
        maybe_capture_declarations
          .clone()
          .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_FUNCTION_CAPTURE_DECLARATIONS_EXPECTED")),
      )?;
      if !is_parent_function && !magic_param_names.is_empty() {
        panic!("POSTPARSER_SCOUT_FUNCTION_MAGIC_PARAMS_NOT_YET_IMPLEMENTED");
      }
      if !is_parent_function && !body_s.closured_names.is_empty() {
        panic!(
          "POSTPARSER_SCOUT_FUNCTION_BODY_CLOSURED_NAMES_NOT_EMPTY_NOT_YET_IMPLEMENTED: {:?}",
          body_s.closured_names
        );
      }
      if is_parent_function && !magic_param_names.is_empty() && !explicit_params_s.is_empty() {
        return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
          range: Self::eval_range(file_coordinate, function.range),
          message: "Cant have a lambda with _ and params".to_string(),
        }));
      }
      let mut total_params_s: Vec<ParameterS<'s>> = Vec::new();
      let mut extra_generic_params_from_body = Vec::<&'s GenericParameterS<'s>>::new();
      if is_parent_function {
        let IFunctionParent::ParentFunction { parent_stack_frame } = &maybe_parent else {
          panic!("POSTPARSER_SCOUT_FUNCTION_EXPECTED_PARENT_FUNCTION");
        };
        let closure_struct_kind_rune = self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val())));
        let closure_struct_region_rune = self.scout_arena.intern_rune(IRuneValS::ImplicitRegionRune(ImplicitRegionRuneValS { original_rune: closure_struct_kind_rune }));
        let closure_struct_coord_rune = self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val())));
        let closure_param_s = self.create_closure_param(
          function.range,
          function_declaration_name_for_env.clone(),
          &mut lidb,
          &mut rules,
          &mut rune_to_explicit_type,
          parent_stack_frame,
          closure_struct_region_rune,
          closure_struct_kind_rune,
          closure_struct_coord_rune,
        );
        total_params_s.push(closure_param_s);
      }
      total_params_s.extend(explicit_params_s);
      if is_parent_function {
        let magic_params: Vec<ParameterS<'s>> =
          self.create_magic_parameters(&mut lidb, magic_param_names, &mut rune_to_explicit_type);
        // Lambdas identifying runes are determined by their magic params.
        // See: Lambdas Dont Need Explicit Identifying Runes (LDNEIR)
        extra_generic_params_from_body.extend(magic_params.iter().map(|magic_param| {
          let coord_rune = magic_param
            .pattern
            .coord_rune
            .as_ref()
            .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_MAGIC_PARAM_WITHOUT_COORD_RUNE"))
            .clone();
          &*self.scout_arena.alloc(GenericParameterS {
            range: magic_param.pattern.range.clone(),
            rune: coord_rune,
            tyype: IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS {
              coord_region: None,
              kind_mutable: true,
              region_mutable: false,
            }),
            default: None,
          })
        }));
        total_params_s.extend(magic_params);
      }
      (
        &*self.scout_arena.alloc(IBodyS::CodeBody(CodeBodyS { body: body_s })),
        variable_uses,
        total_params_s,
        extra_generic_params_from_body,
      )
    };
    // Per @PRIIROZ, parent ones go on the end.
    let mut generic_params: Vec<&'s GenericParameterS<'s>> = function_user_specified_generic_parameters_s;
    generic_params.extend(extra_generic_params_from_explicit_params_s);
    generic_params.extend(extra_generic_params_from_body);
    generic_params.extend(extra_generic_params_from_parent);
    generic_params = generic_params
      .into_iter()
      .filter(|generic_param| {
        !matches!(
          generic_param.tyype,
          IGenericParameterTypeS::RegionGenericParameterType(_)
        )
      })
      .collect();

    let unfiltered_rules_array: Vec<IRulexSR<'s>> = rules;
    let rules_array = match &maybe_parent {
      IFunctionParent::ParentCitizen { .. } => unfiltered_rules_array
        .into_iter()
        .filter(|rule| !matches!(rule, IRulexSR::RuneParentEnvLookup(_)))
        .collect::<Vec<_>>(),
      _ => unfiltered_rules_array,
    };

    let unfiltered_attrs_p = function.header.attributes;
    let filtered_attrs: Vec<&IAttributeP<'p>> = match &maybe_parent {
      IFunctionParent::FunctionNoParent => unfiltered_attrs_p
        .iter()
        .filter(|a| !matches!(a, IAttributeP::AbstractAttribute(_)))
        .collect(),
      IFunctionParent::ParentCitizen { .. } => unfiltered_attrs_p.iter().collect(),
      IFunctionParent::ParentFunction { .. } => unfiltered_attrs_p.iter().collect(),
    };
    let func_attrs_s: Vec<IFunctionAttributeS<'s>> = filtered_attrs
      .into_iter()
      .map(|attr| match attr {
        IAttributeP::ExportAttribute(_) => IFunctionAttributeS::Export(ExportS {
          package_coordinate: file_coordinate.package_coord,
        }),
        IAttributeP::ExternAttribute(_) => IFunctionAttributeS::Extern(ExternS {
          package_coord: file_coordinate.package_coord,
        }),
        IAttributeP::PureAttribute(_) => IFunctionAttributeS::Pure(PureS),
        IAttributeP::AdditiveAttribute(_) => IFunctionAttributeS::Additive(AdditiveS),
        IAttributeP::BuiltinAttribute(builtin_attr) => IFunctionAttributeS::Builtin(BuiltinS {
          generator_name: self.scout_arena.intern_str(builtin_attr.generator_name.str().as_str()),
        }),
        IAttributeP::AbstractAttribute(_) => panic!("AbstractAttribute should have been filtered"),
        other => panic!("POSTPARSER_SCOUT_FUNCTION_ATTRIBUTE_NOT_YET_IMPLEMENTED: {:?}", other),
      })
      .collect();

    let range_s = Self::eval_range(file_coordinate, function.range);
    let mut rune_to_predicted_type = Self::predict_rune_types(
      self.scout_arena,
      range_s.clone(),
      &user_specified_identifying_runes
        .iter()
        .map(|rune_usage| rune_usage.rune.clone())
        .collect::<Vec<_>>(),
      &mut rune_to_explicit_type,
      &rules_array,
    )?;
    rune_to_predicted_type.retain(|_, tyype| !matches!(tyype, ITemplataType::RegionTemplataType(_)));
    let rules_array: &'s [IRulexSR<'s>] = self.scout_arena.alloc_slice_from_vec(rules_array);
    self.check_identifiability(
      range_s,
      &generic_params
        .iter()
        .map(|generic_param| generic_param.rune.rune.clone())
        .collect::<Vec<_>>(),
      rules_array,
    )?;

    let param_types_vec: Vec<ITemplataType<'s>> = generic_params
        .iter()
        .map(|generic_param| generic_param.tyype.tyype())
        .collect();
    let tyype = TemplateTemplataType {
      param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
      return_type: self.scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
    };
    let function_name_ref: &'s IFunctionDeclarationNameS<'s> = match function_declaration_name {
      INameS::FunctionDeclaration(r) => r,
      _ => panic!("POSTPARSER_FUNCTION_NAME_EXPECTED_FUNCTION_DECLARATION"),
    };
    Ok((
      &*self.scout_arena.alloc(
        FunctionS::new(
          Self::eval_range(file_coordinate, function.range),
          function_name_ref,
          self.scout_arena.alloc_slice_from_vec(func_attrs_s),
          self.scout_arena.alloc_slice_from_vec(generic_params),
          rune_to_predicted_type,
          tyype,
          self.scout_arena.alloc_slice_from_vec(total_params_s),
          maybe_ret_coord_rune,
          rules_array,
          body_s,
        )),
      variable_uses,
    ))
  }
/*
  def scoutFunction(
    file: FileCoordinate,
    functionP: FunctionP,
    maybeParent: IFunctionParent):
  (FunctionS, VariableUses) = {
    val FunctionP(range, headerP, maybeBody0) = functionP;
    val FunctionHeaderP(headerRange, maybeName, attrsP, maybeGenericParametersP, templateRulesP, maybeParamsP, returnP) = headerP
    val FunctionReturnP(retRange, maybeRetType) = returnP

    val headerRangeS = PostParser.evalRange(file, headerRange)
    val rangeS = PostParser.evalRange(file, range)
    val codeLocation = rangeS.begin
    val retRangeS = PostParser.evalRange(file, retRange)


    maybeParent match {
      case FunctionNoParent() =>
      case ParentCitizen(_, _, _, _, _) =>
      case ParentFunction(_) => {
        vcurious(maybeGenericParametersP.isEmpty)
      }
    }

    val funcName =
      maybeParent match {
        case FunctionNoParent() | ParentCitizen(_, _, _, _, _) => {
          val NameP(_, codeName) = vassertSome(maybeName)
          interner.intern(FunctionNameS(codeName, codeLocation))
        }
        case ParentFunction(_) => {
          vassert(maybeName.isEmpty)
          interner.intern(LambdaDeclarationNameS(codeLocation))
        }
      }

    val genericParametersP =
      maybeGenericParametersP
        .toVector
        .flatMap(_.params)

    val userSpecifiedIdentifyingRunes =
      genericParametersP
        .map({ case GenericParameterP(_, NameP(range, identifyingRuneName), _, _, _, _) =>
          rules.RuneUsage(rangeS, CodeRuneS(identifyingRuneName))
        })

    // See: Must Scan For Declared Runes First (MSFDRF)
    val userRunesFromRules =
      templateRulesP
        .toVector
        .flatMap(rules => RulePUtils.getOrderedRuneDeclarationsFromRulexesWithDuplicates(rules.rules))
        .map({ case NameP(range, identifyingRuneName) => rules.RuneUsage(rangeS, CodeRuneS(identifyingRuneName)) })
    val userDeclaredRunes = (userSpecifiedIdentifyingRunes ++ userRunesFromRules).distinct

    val lidb = new LocationInDenizenBuilder(Vector())

    maybeParent match {
      case FunctionNoParent() => {
        if (attrsP.collectFirst({ case AbstractAttributeP(_) => }).nonEmpty) {
          maybeParamsP match {
            case None =>
              throw CompileErrorExceptionS(VirtualAndAbstractGoTogether(rangeS))
            case Some(paramsP) =>
              if (!paramsP.params.exists(_.virtuality match { case Some(AbstractP(_)) => true case _ => false })) {
                throw CompileErrorExceptionS(VirtualAndAbstractGoTogether(rangeS))
              }
          }
        }
      }
      case ParentFunction(_) =>
      case ParentCitizen(citizenIsInterface, _, _, _, _) => {
        // When we have traits that can have static methods, this check might need to go away
        if (citizenIsInterface) {
          maybeParamsP match {
            case None =>
            case Some(paramsP) => {
              if (!paramsP.params.exists(_.virtuality match { case Some(AbstractP(_)) => true case _ => false })) {
                throw CompileErrorExceptionS(InterfaceMethodNeedsSelf(rangeS))
              }
            }
          }
        }
      }
    }

    val parentEnv =
      maybeParent match {
        case FunctionNoParent() => None
        case ParentFunction(parentStackFrame) => Some(parentStackFrame.parentEnv)
        case ParentCitizen(_, citizenEnv, _, _, _) => Some(citizenEnv)
      }
    val isInterfaceInternalMethod =
      maybeParent match {
        case FunctionNoParent() => false
        case ParentFunction(parentStackFrame) => false
        case ParentCitizen(_, _, _, _, _) => true
      }
    val functionEnv =
      postparsing.FunctionEnvironmentS(
        file, funcName, parentEnv, userDeclaredRunes.map(_.rune).toSet, maybeParamsP.size, isInterfaceInternalMethod)


    val ruleBuilder = ArrayBuffer[IRulexSR]()
    val runeToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

    val (defaultRegionRuneRangeS, defaultRegionRuneS, maybeRegionGenericParam) =
      maybeBody0.flatMap(_.maybeDefaultRegion) match {
        case None => {
          val regionRange = RangeS(headerRangeS.end, headerRangeS.end)
          val rune = DenizenDefaultRegionRuneS(funcName)
          vassert(!runeToExplicitType.exists(_._1 == rune))
          vregionmut() // Put this back in with regions
          // runeToExplicitType += ((rune, RegionTemplataType()))
          val implicitRegionGenericParam =
            GenericParameterS(
              regionRange, RuneUsage(regionRange, rune), RegionGenericParameterTypeS(ReadWriteRegionS), None)
          (regionRange, rune, Some(implicitRegionGenericParam))
        }
        case Some(RegionRunePT(regionRange, regionName)) => {
          val rune = CodeRuneS(vassertSome(regionName).str) // impl isolates
          if (!functionEnv.allDeclaredRunes().contains(rune)) {
            throw CompileErrorExceptionS(CouldntFindRuneS(PostParser.evalRange(file, range), rune.name.str))
          }
          (evalRange(file, regionRange), rune, None)
        }
      }

    maybeParent match {
      case FunctionNoParent() => {
        ruleScout.translateRulexes(
          functionEnv,
          lidb.child(),
          ruleBuilder,
          runeToExplicitType,
          defaultRegionRuneS,
          templateRulesP.toVector.flatMap(_.rules))
      }
      case ParentFunction(_) => {
        vassert(templateRulesP.isEmpty)
      }
      case ParentCitizen(_, interfaceEnv, _, interfaceRules, interfaceRuneToExplicitType) => {
        // ruleBuilder ++= interfaceRules
        // runeToExplicitType ++= interfaceRuneToExplicitType
        ruleScout.translateRulexes(
          interfaceEnv,
          lidb.child(),
          ruleBuilder,
          runeToExplicitType,
          defaultRegionRuneS,
          templateRulesP.toVector.flatMap(_.rules))
      }
    }

//    val isPure =
//      functionP.header.attributes
//        .exists({ case PureAttributeP(_) => true case _ => false })
//    val isAdditive =
//      functionP.header.attributes
//        .exists({ case AdditiveAttributeP(_) => true case _ => false })

    // We'll add the implicit runes to the end, see IRRAE.
    val functionUserSpecifiedGenericParametersS =
      genericParametersP.zip(userSpecifiedIdentifyingRunes)
        .map({ case (g, r) =>
          PostParser.scoutGenericParameter(
            templexScout, functionEnv, lidb.child(), runeToExplicitType, ruleBuilder, defaultRegionRuneS, g, r)
        })

    val myStackFrameWithoutParams =
      StackFrame(file, funcName, functionEnv, None, defaultRegionRuneS, 0, noDeclarations)

    val paramsP =
      maybeParamsP.toVector.flatMap(_.params).map(param => {
        maybeParent match {
          case FunctionNoParent() | ParentCitizen(_, _, _, _, _) => {
            // Should have been caught by LightFunctionMustHaveParamTypes error in parser,
            vassert(vassertSome(param.pattern).templex.nonEmpty)
          }
          case ParentFunction(_) =>
        }
        param
      })

    // We say PerhapsTypeless because we're in a lambda, they might be anonymous params
    // like in `(_) => { true }`
    // Later on, we'll make identifying runes for these.

    // For lambdas, untyped explicit params (like `(a, b) => ...` or `(_) => ...`) get a
    // synthesized coord rune here that will be added to the function's identifying runes.
    val explicitParamsSAndSynthesizedRunes: Vector[(ParameterS, Option[RuneUsage])] =
      paramsP
        .map({
          case ParameterP(rangeL, maybeAbstractP, maybePreChecked, maybeSelfBorrow, maybePattern) => {
            val rangeS = evalRange(file, rangeL)
            val maybeAbstractS =
              maybeAbstractP match {
                case None => None
                case Some(AbstractP(range)) => {
                  Some(AbstractSP(PostParser.evalRange(file, range), myStackFrameWithoutParams.parentEnv.isInterfaceInternalMethod))
                }
              }
            (maybeSelfBorrow, maybePattern) match {
              case (Some(selfBorrow), None) => {
                val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                runeToExplicitType += ((rune.rune, CoordTemplataType()))
                val patternS =
                  AtomSP(rangeS, Some(CaptureS(CodeVarNameS(keywords.self), false)), Some(rune), None)
                (ParameterS(rangeS, maybeAbstractS, maybePreChecked.nonEmpty, patternS), None)
              }
              case (None, Some(patternP)) => {
                val patternPerhapsWithoutCoordRuneS =
                  patternScout.translatePattern(
                    myStackFrameWithoutParams,
                    lidb.child(),
                    ruleBuilder,
                    runeToExplicitType,
                    patternP)
                patternPerhapsWithoutCoordRuneS.coordRune match {
                  case None => {
                    // Untyped param (like in `(a) => a`) so make a rune that will be added to
                    // genericParams and to the identifying runes. This only happens for lambdas,
                    // top level functions can't have these (enforced elsewhere).
                    val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                    runeToExplicitType += ((rune.rune, CoordTemplataType()))
                    val patternS = patternPerhapsWithoutCoordRuneS.copy(coordRune = Some(rune))
                    (ParameterS(rangeS, maybeAbstractS, maybePreChecked.nonEmpty, patternS), Some(rune))
                  }
                  case Some(_) =>
                    (ParameterS(rangeS, maybeAbstractS, maybePreChecked.nonEmpty, patternPerhapsWithoutCoordRuneS), None)
                }
              }
            }
          }
        })
    val explicitParamsS = explicitParamsSAndSynthesizedRunes.map(_._1)
    // Untyped lambda params (from `(_) =>` or `(a, b) =>`) contribute their synthesized coord runes here so later
    // passes see a uniform FunctionS shape regardless of whether the user wrote `<T>` or an untyped param.
    val extraGenericParamsFromExplicitParamsS =
      explicitParamsSAndSynthesizedRunes
          .flatMap(_._2)
          .map(rune =>
            GenericParameterS(
              rune.range,
              rune,
              CoordGenericParameterTypeS(None, true, false),
              None))

    // Only if the function actually has a body
    val maybeCaptureDeclarations =
      maybeBody0 match {
        case None => None
        case Some(_) => {
          val firstParams =
            maybeParent match {
              case FunctionNoParent() => noDeclarations
              case ParentCitizen(_, _, _, _, _) => noDeclarations
              case ParentFunction(_) => {
                // Every lambda has a closure as its first arg, even if its empty
                val closureParamName = interner.intern(ClosureParamNameS(rangeS.begin))
                val closureDeclaration =
                  VariableDeclarations(Vector(VariableDeclaration(closureParamName)))
                closureDeclaration
              }
            }
          Some(
            explicitParamsS
              .map(_.pattern)
              .map(pattern1 => {
                postparsing.VariableDeclarations(patternScout.getParameterCaptures(pattern1))
              })
              .foldLeft(firstParams)(_ ++ _))
        }
      }

    val maybeRetCoordRune =
      maybeRetType match {
        case None | Some(RegionRunePT(_, _)) => {
          maybeParent match {
            case ParentFunction(_) => {
              None // Infer the return
            }
            case FunctionNoParent() | ParentCitizen(_, _, _, _, _) => {
              // If nothing's present, assume void
              val rangeS = PostParser.evalRange(file, retRange)
              val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder +=
                MaybeCoercingLookupSR(
                  rangeS,
                  rune,
                  interner.intern(CodeNameS(keywords.void)))
              Some(rune)
            }
          }
        }
        case Some(retTypePT) => {
          templexScout.translateMaybeTypeIntoMaybeRune(
            functionEnv,
            lidb.child(),
            PostParser.evalRange(myStackFrameWithoutParams.file, retRange),
            ruleBuilder,
            runeToExplicitType,
            defaultRegionRuneS,
            Some(retTypePT))
        }
      }

    maybeRetCoordRune.foreach(retCoordRune => runeToExplicitType += ((retCoordRune.rune, CoordTemplataType())))

    maybeParent match {
      case FunctionNoParent() =>
      case ParentFunction(_) =>
      case ParentCitizen(_, _, _, _, _) => {
        if (attrsP.collect({ case AbstractAttributeP(_) => true }).nonEmpty) {
          throw CompileErrorExceptionS(
            RangedInternalErrorS(rangeS, "Dont need abstract here"))
        }
      }
    }

    val extraGenericParamsFromParentS =
      (maybeParent match {
        case FunctionNoParent() => Vector()
        case ParentFunction(_) => Vector()
        case ParentCitizen(_, _, interfaceGenericParams, _, _) => interfaceGenericParams
      })
    extraGenericParamsFromParentS.foreach(gp => {
      runeToExplicitType += ((gp.rune.rune, gp.tyype.tyype))
    })

    val (maybeBody1, variableUses, extraGenericParamsFromBodyS, maybeClosureParam, magicParams) =
      if (maybeParent match { case ParentCitizen(true, _, _, _, _) => true case _ => false }) {
        // Only true interface members get an implicit abstract body — struct internal methods
        // with no body would have been rejected earlier as a parse error.
        val bodyS = AbstractBodyS
        (bodyS, noVariableUses, Vector(), None, Vector())
      } else if (attrsP.collectFirst({ case AbstractAttributeP(_) => }).nonEmpty) {
        val bodyS = AbstractBodyS
        (bodyS, noVariableUses, Vector(), None, Vector())
      } else if (attrsP.collectFirst({ case ExternAttributeP(_) => }).nonEmpty) {
        if (maybeBody0.nonEmpty) {
          throw CompileErrorExceptionS(ExternHasBody(PostParser.evalRange(file, range)))
        }
        val bodyS = ExternBodyS
        (bodyS, noVariableUses, Vector(), None, Vector())
      } else if (attrsP.collectFirst({ case BuiltinAttributeP(_, _) => }).nonEmpty) {
        val bodyS =
          GeneratedBodyS(
            attrsP.collectFirst({ case BuiltinAttributeP(_, generatorId) => generatorId }).head.str)
        (bodyS, noVariableUses, Vector(), None, Vector())
      } else {
        val captureDeclarations = vassertSome(maybeCaptureDeclarations)
        val bodyP =
          maybeBody0 match {
            case None => {
              throw CompileErrorExceptionS(
                postparsing.RangedInternalErrorS(rangeS, "Error: function has no body."))
            }
            case Some(x) => x
          }

        val parentStackFrame =
          maybeParent match {
            case FunctionNoParent() | ParentCitizen(_, _, _, _, _) => None
            case ParentFunction(parentStackFrame) => Some(parentStackFrame)
          }
        val (body1, variableUses, lambdaMagicParamNames) =
          scoutBody(
            functionEnv,
            parentStackFrame,
            lidb.child(),
            defaultRegionRuneS,
            bodyP,
            // We hand these into scoutBody instead of assembling a StackFrame on our own
            // because we want StackFrame's to be made in one place, where we can centralize the
            // logic for tracking variable uses and so on.
            captureDeclarations)

        val (extraGenericParamsFromBodyS, maybeClosureParam, magicParams) =
          maybeParent match {
            case FunctionNoParent() | ParentCitizen(_, _, _, _, _) => {
              if (lambdaMagicParamNames.nonEmpty) {
                throw CompileErrorExceptionS(postparsing.RangedInternalErrorS(rangeS, "Magic param (underscore) in a normal block!"))
              }
              if (body1.closuredNames.nonEmpty) {
                throw CompileErrorExceptionS(postparsing.RangedInternalErrorS(rangeS, "Internal error: Body closured names not empty?:\n" + body1.closuredNames))
              }
              (Vector(), None, Vector())
            }
            case ParentFunction(parentStackFrame) => {
              if (lambdaMagicParamNames.nonEmpty && (explicitParamsS.nonEmpty)) {
                throw CompileErrorExceptionS(
                  RangedInternalErrorS(rangeS, "Cant have a lambda with _ and params"))
              }

              val closureStructKindRune = ImplicitRuneS(lidb.child().consume())
              val closureStructRegionRune =
                ImplicitRegionRuneS(closureStructKindRune)
              val closureStructCoordRune = ImplicitRuneS(lidb.child().consume())

              val closureParamS =
                createClosureParam(
                  range,
                  funcName,
                  lidb,
                  ruleBuilder,
                  runeToExplicitType,
                  parentStackFrame,
                  closureStructRegionRune,
                  closureStructKindRune,
                  closureStructCoordRune)

              val magicParams =
                createMagicParameters(lidb, lambdaMagicParamNames, runeToExplicitType)

              val extraGenericParamsFromBodyS =
                // Lambdas identifying runes are determined by their magic params.
                // See: Lambdas Dont Need Explicit Identifying Runes (LDNEIR)
                magicParams.flatMap(param => {
                  val coordRune = vassertSome(param.pattern.coordRune)
//                  val implicitRegionRune =
//                    RuneUsage(
//                      param.pattern.range,
//                      ImplicitRegionRuneS(vassertSome(param.pattern.coordRune).rune))
                  List(
//                    GenericParameterS(
//                      param.pattern.range,
//                      implicitRegionRune,
//                      RegionTemplataType(),
//                      None,
//                      Vector(),
//                      None),
                    GenericParameterS(
                      param.pattern.range,
                      coordRune,
                      CoordGenericParameterTypeS(None, true, false),
                      None))
                })
              (extraGenericParamsFromBodyS, Some(closureParamS), magicParams)
            }
          }
        (CodeBodyS(body1), variableUses, extraGenericParamsFromBodyS, maybeClosureParam, magicParams)
      }

    val totalParamsS = maybeClosureParam.toVector ++ explicitParamsS ++ magicParams;

    vregionmut() // Put back in regions
    val genericParametersS =
      (functionUserSpecifiedGenericParametersS ++
        extraGenericParamsFromExplicitParamsS ++
        extraGenericParamsFromBodyS ++
        // Parent ones go on the end, see @PRIIROZ.
        extraGenericParamsFromParentS)
          .filter({
            case GenericParameterS(_, _, RegionGenericParameterTypeS(_), _) => false
            case _ => true
          })

    // ++

        //maybeRegionGenericParam
        //++ userSpecifiedRunesImplicitRegionRunesS

    val unfilteredRulesArray = ruleBuilder.toVector

    val unfilteredAttrsP = attrsP

    val filteredAttrs =
      (maybeParent match {
        case FunctionNoParent() => {
          unfilteredAttrsP.filter({ case AbstractAttributeP(_) => false case _ => true })
        }
        case ParentCitizen(_, _, _, _, _) => unfilteredAttrsP
        case ParentFunction(_) => unfilteredAttrsP
      })
        //.filter({ case AdditiveAttributeP(_) => false case _ => true })

    val funcAttrsS =
      filteredAttrs.map({
        case AbstractAttributeP(_) => vwat() // Should have been filtered out, typingpass cares about abstract directly
        case AdditiveAttributeP(_) => AdditiveS
        case ExportAttributeP(_) => ExportS(file.packageCoordinate)
        case ExternAttributeP(_) => ExternS(file.packageCoordinate)
        case PureAttributeP(_) => PureS
        case BuiltinAttributeP(_, generatorName) => BuiltinS(generatorName.str)
        case x => vimpl(x.toString)
      })

    // Filter out any RuneParentEnvLookupSR rules, we don't want these methods to look up these runes
    // from the environment. See MKRFA.
    val rulesArray =
      maybeParent match {
        case FunctionNoParent() => unfilteredRulesArray
        case ParentFunction(_) => unfilteredRulesArray
        case ParentCitizen(_, _, _, _, _) => {
          unfilteredRulesArray.filter({
            case RuneParentEnvLookupSR(_, _) => false
            case _ => true
          })
        }
      }

    val runeToPredictedType =
      postParser.predictRuneTypes(
        rangeS,
        userSpecifiedIdentifyingRunes.map(_.rune),
        runeToExplicitType,
        rulesArray)
          .filter({
            case (key, RegionTemplataType()) => false
            case _ => true
          })
    vregionmut() // dont filter regions out

    postParser.checkIdentifiability(
      rangeS,
      genericParametersS.map(_.rune.rune),
      rulesArray)

    val tyype = TemplateTemplataType(genericParametersS.map(_.tyype.tyype), FunctionTemplataType())

    val functionS =
      FunctionS(
        PostParser.evalRange(file, range),
        funcName,
        funcAttrsS,
        genericParametersS,
        runeToPredictedType,
        tyype,
        totalParamsS,
        maybeRetCoordRune,
        rulesArray,
        maybeBody1)
    (functionS, variableUses)
  }
*/
fn create_closure_param(
  &self,
  range: RangeL,
  func_name: IFunctionDeclarationNameS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType)>,
  parent_stack_frame: &StackFrame<'s>,
  _closure_struct_region_rune: IRuneS<'s>,
  closure_struct_kind_rune: IRuneS<'s>,
  closure_struct_coord_rune: IRuneS<'s>,
) -> ParameterS<'s> {
  let closure_param_pos = PostParser::eval_pos(parent_stack_frame.file, range.begin());
  let closure_param_range = RangeS::new(
    closure_param_pos.clone(),
    closure_param_pos.clone(),
  );
  let closure_param_name = match self.scout_arena.intern_name(INameValS::VarName(
    IVarNameValS::ClosureParamName(ClosureParamNameS {
      code_location: closure_param_range.begin.clone(),
    }),
  )) {
    INameS::VarName(r) => (*r).clone(),
    _ => panic!("POSTPARSER_INTERN_VAR_NAME_EXPECTED_VAR_NAME"),
  };
  rune_to_explicit_type.push((
    closure_struct_kind_rune.clone(),
    ITemplataType::KindTemplataType(KindTemplataType {}),
  ));
  let IFunctionDeclarationNameS::LambdaDeclarationName(lambda_name) = func_name else {
    panic!("POSTPARSER_SCOUT_CREATE_CLOSURE_PARAM_NON_LAMBDA_NAME");
  };
  let closure_struct_name =
    self.scout_arena.intern_name(INameValS::LambdaStructDeclaration(LambdaStructDeclarationNameS {
      lambda_name: lambda_name.clone(),
    }));
  let closure_struct_imprecise_name = match &closure_struct_name {
    INameS::LambdaStructDeclaration(r) => (*r).get_imprecise_name(self.scout_arena),
    _ => panic!("POSTPARSER_INTERN_LAMBDA_STRUCT_NAME_EXPECTED_LAMBDA_STRUCT"),
  };
  rule_builder.push(IRulexSR::Lookup(LookupSR {
    range: closure_param_range.clone(),
    rune: RuneUsage {
      range: closure_param_range.clone(),
      rune: closure_struct_kind_rune.clone(),
    },
    name: closure_struct_imprecise_name.clone(),
  }));
  rune_to_explicit_type.push((
    closure_struct_coord_rune.clone(),
    ITemplataType::CoordTemplataType(CoordTemplataType {}),
  ));
  rule_builder.push(IRulexSR::CoerceToCoord(CoerceToCoordSR {
    range: closure_param_range.clone(),
    coord_rune: RuneUsage {
      range: closure_param_range.clone(),
      rune: closure_struct_coord_rune.clone(),
    },
    kind_rune: RuneUsage {
      range: closure_param_range.clone(),
      rune: closure_struct_kind_rune.clone(),
    },
  }));
  let closure_param_type_rune = RuneUsage {
    range: closure_param_range.clone(),
    rune: self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val()))),
  };
  rule_builder.push(IRulexSR::Augment(AugmentSR {
    range: closure_param_range.clone(),
    result_rune: closure_param_type_rune.clone(),
    ownership: Some(OwnershipP::Borrow),
    inner_rune: RuneUsage {
      range: closure_param_range.clone(),
      rune: closure_struct_coord_rune,
    },
  }));
  let capture: CaptureS<'s> = CaptureS {
    name: closure_param_name,
    mutate: false,
  };
  let closure_pattern = AtomSP::<'s> {
    range: closure_param_range.clone(),
    name: Some(capture),
    coord_rune: Some(closure_param_type_rune),
    destructure: None,
  };
  return ParameterS::<'s> {
    range: closure_param_range.clone(),
    virtuality: None,
    pre_checked: false,
    pattern: closure_pattern,
  };
}
/*
  private def createClosureParam(
    range: RangeL,
    funcName: IFunctionDeclarationNameS,
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
    parentStackFrame: StackFrame,
    closureStructRegionRune: IRuneS,
    closureStructKindRune: IRuneS,
    closureStructCoordRune: IRuneS):
  ParameterS = {
    val closureParamPos = PostParser.evalPos(parentStackFrame.file, range.begin)
    val closureParamRange = RangeS(closureParamPos, closureParamPos)
    val closureParamName = interner.intern(ClosureParamNameS(closureParamRange.begin))

    runeToExplicitType += ((closureStructKindRune, KindTemplataType()))
    val closureStructName =
      interner.intern(LambdaStructDeclarationNameS(
        funcName match { case x @ LambdaDeclarationNameS(_) => x }))
    ruleBuilder +=
      LookupSR(
        closureParamRange,
        rules.RuneUsage(closureParamRange, closureStructKindRune),
        closureStructName.getImpreciseName(interner))

    runeToExplicitType += ((closureStructCoordRune, CoordTemplataType()))
    ruleBuilder +=
      CoerceToCoordSR(
        closureParamRange,
        RuneUsage(closureParamRange, closureStructCoordRune),
        RuneUsage(closureParamRange, closureStructKindRune))

    val closureParamTypeRune =
      rules.RuneUsage(closureParamRange, ImplicitRuneS(lidb.child().consume()))
    ruleBuilder +=
      AugmentSR(
        closureParamRange,
        closureParamTypeRune,
        Some(BorrowP),
        RuneUsage(closureParamRange, closureStructCoordRune))

    val capture = CaptureS(closureParamName, false)
    val closurePattern =
      AtomSP(closureParamRange, Some(capture), Some(closureParamTypeRune), None)
    ParameterS(closureParamRange, None, false, closurePattern)
  }
*/
fn create_magic_parameters(
  &self,
  lidb: &mut LocationInDenizenBuilder,
  lambda_magic_param_names: Vec<IVarNameS<'s>>,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType)>,
) -> Vec<ParameterS<'s>> {
  lambda_magic_param_names
    .into_iter()
    .map(|magic_param_name| {
      let code_location = match &magic_param_name {
        IVarNameS::MagicParamName(c) => c.clone(),
        _ => panic!("POSTPARSER_CREATE_MAGIC_PARAMS_EXPECTED_MAGIC_PARAM_NAME"),
      };
      let magic_param_range = RangeS::new(
        code_location.clone(),
        code_location.clone(),
      );
      let magic_param_rune = self.scout_arena.intern_rune(IRuneValS::MagicParamRune(MagicParamRuneValS::new(lidb.child().borrow_val())));
      rune_to_explicit_type.push((
        magic_param_rune.clone(),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      ));
      ParameterS::new(
        magic_param_range.clone(),
        None,
        false,
        AtomSP {
          range: magic_param_range.clone(),
          name: Some(CaptureS {
            name: magic_param_name,
            mutate: false,
          }),
          coord_rune: Some(RuneUsage {
            range: magic_param_range,
            rune: magic_param_rune,
          }),
          destructure: None,
        },
      )
    })
    .collect()
}
/*
  private def createMagicParameters(
    lidb: LocationInDenizenBuilder,
    lambdaMagicParamNames: Vector[MagicParamNameS],
    runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)]):
  Vector[ParameterS] = {
    lambdaMagicParamNames.map({
      case mpn@MagicParamNameS(codeLocation) => {
        val magicParamRange = vale.RangeS(codeLocation, codeLocation)
        val magicParamRune =
          rules.RuneUsage(magicParamRange, MagicParamRuneS(lidb.child().consume()))
        runeToExplicitType += ((magicParamRune.rune, CoordTemplataType()))
        val paramS =
          ParameterS(
            magicParamRange,
            None,
            false,
            AtomSP(
              magicParamRange,
              Some(patterns.CaptureS(mpn, false)), Some(magicParamRune), None))
        paramS
      }
    })
  }
*/
  #[allow(dead_code)]
  pub(crate) fn scout_lambda(
    &self,
    parent_stack_frame: StackFrame<'s>,
    function: &FunctionP<'p>,
  ) -> Result<(&'s FunctionS<'s>, VariableUses<'s>), ICompileErrorS<'s>>
  {
    let file_coordinate = parent_stack_frame.file;
    self.scout_function(
      file_coordinate,
      function,
      IFunctionParent::ParentFunction { parent_stack_frame },
    )
  }
/*
  def scoutLambda(
    parentStackFrame: StackFrame,
    functionP: FunctionP):
  (FunctionS, VariableUses) = {
    val file = parentStackFrame.file
    scoutFunction(file, functionP, ParentFunction(parentStackFrame))
  }
*/
  fn scout_body(
    &self,
    function_env: FunctionEnvironmentS<'s>,
    parent_stack_frame: Option<StackFrame<'s>>,
    lidb: &mut LocationInDenizenBuilder,
    context_region: IRuneS<'s>,
    body0: &BlockPE<'p>,
    initial_declarations: VariableDeclarations<'s>,
  ) -> Result<
    (
      &'s BodySE<'s>,
      VariableUses<'s>,
      Vec<IVarNameS<'s>>,
    ),
    ICompileErrorS<'s>,
  > {
    let function_body_env: FunctionEnvironmentS<'s> = function_env.child();
    let body_range_s = PostParser::eval_range(function_body_env.file, body0.range);
    let mut new_block_lidb = lidb.child();
    let (block1, self_uses, child_uses): (&'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>) = self.new_block(
      function_body_env.clone(),
      parent_stack_frame,
      &mut new_block_lidb,
      body_range_s,
      context_region,
      initial_declarations,
      |stack_frame1, scout_contents_lidb| {
        let (stack_frame2, inner_expr, inner_self_uses, inner_child_uses) =
          self.scout_expression_and_coerce(
            stack_frame1,
            scout_contents_lidb,
            body0.inner,
            LoadAsP::Use,
          )?;
        Ok((
          stack_frame2,
          inner_expr,
          inner_self_uses,
          inner_child_uses,
        ))
      },
    )?;

    let magic_param_names: Vec<IVarNameS<'s>> = self_uses
      .uses
      .iter()
      .filter_map(|use_| match &use_.name {
        IVarNameS::MagicParamName(code_location) => {
          Some(
            match self.scout_arena.intern_name(INameValS::VarName(IVarNameValS::MagicParamName(
              code_location.clone(),
            ))) {
              INameS::VarName(r) => (*r).clone(),
              _ => panic!("POSTPARSER_INTERN_MAGIC_PARAM_EXPECTED_VAR_NAME"),
            },
          )
        }
        _ => None,
      })
      .collect();
    let magic_param_vars: Vec<VariableDeclarationS<'s>> = magic_param_names
      .iter()
      .map(|magic_param_name| VariableDeclarationS {
        name: magic_param_name.clone(),
      })
      .collect();
    let magic_param_locals: Vec<LocalS<'s>> = magic_param_vars
      .iter()
      .map(|declared| LocalS {
        var_name: declared.name.clone(),
        self_borrowed: self_uses.is_borrowed(&declared.name),
        self_moved: self_uses.is_moved(&declared.name),
        self_mutated: self_uses.is_mutated(&declared.name),
        child_borrowed: child_uses.is_borrowed(&declared.name),
        child_moved: child_uses.is_moved(&declared.name),
        child_mutated: child_uses.is_mutated(&declared.name),
      })
      .collect();
    let mut combined_locals: Vec<_> = block1.locals.to_vec();
    combined_locals.extend(magic_param_locals);
    let block1 = &*self.scout_arena.alloc(BlockSE {
      range: block1.range.clone(),
      locals: self.scout_arena.alloc_slice_from_vec(combined_locals),
      expr: block1.expr,
    });
    // V: tell me about the above change?
    // VA: This is a faithful translation of Scala's `BlockSE(bodyRangeS, block1.locals ++ magicParamLocals, block1.expr)`.
    // VA: It re-allocates BlockSE with combined locals (original + magic params) into the arena. Not novel logic.
    // VA: One minor divergence: Rust uses block1.range (already computed) while Scala uses bodyRangeS
    // VA: (a fresh evalRange(body0.range)). They likely resolve to the same value but the source differs.
    let all_uses = self_uses.then_merge(&child_uses);
    let uses_of_parent_variables = all_uses
      .uses
      .iter()
      .filter(|use_| {
        if block1.locals.iter().any(|local| local.var_name == use_.name) {
          false
        } else {
          !matches!(use_.name, IVarNameS::MagicParamName(_))
        }
      })
      .cloned()
      .collect::<Vec<_>>();
    let closured_names: Vec<IVarNameS<'s>> = uses_of_parent_variables
      .iter()
      .map(|use_| use_.name.clone())
      .collect();
    let body_s = &*self.scout_arena.alloc(BodySE {
      range: PostParser::eval_range(function_body_env.file, body0.range),
      closured_names: self.scout_arena.alloc_slice_from_vec(closured_names),
      block: block1,
    });
    Ok((body_s, VariableUses { uses: uses_of_parent_variables }, magic_param_names))
  }
/*
  // Returns:
  // - Body.
  // - Uses of parent variables.
  // - Magic params made/used inside.
  private def scoutBody(
    functionEnv: FunctionEnvironmentS,
    // This might be the block containing the lambda that we're evaluating now.
    parentStackFrame: Option[StackFrame],
    lidb: LocationInDenizenBuilder,
    contextRegion: IRuneS,
    body0: BlockPE,
    initialDeclarations: VariableDeclarations):
  (BodySE, VariableUses, Vector[MagicParamNameS]) = {
    val functionBodyEnv = functionEnv.child()

    // There's an interesting consequence of calling this function here...
    // If we have a lone lookup node, like "m = Marine(); m;" then that
    // 'm' will be turned into an expression, which means that's how it's
    // destroyed. So, thats how we destroy things before their time.
    val (block1, selfUses, childUses) =
    expressionScout.newBlock(
      functionBodyEnv,
      parentStackFrame,
      lidb.child(),
      PostParser.evalRange(functionBodyEnv.file, body0.range),
      contextRegion,
      initialDeclarations,
      (stackFrame1, lidb) => {
        expressionScout.scoutExpressionAndCoerce(
          stackFrame1, lidb, body0.inner, UseP)
      })

    vcurious(
      childUses.uses.map(_.name).collect({ case mpn@MagicParamNameS(_) => mpn }).isEmpty)
    val magicParamNames =
      selfUses.uses.map(_.name).collect({ case mpn@MagicParamNameS(_) => mpn })
    val magicParamVars = magicParamNames.map(n => VariableDeclaration(n))

    val magicParamLocals =
      magicParamVars.map({ declared =>
        LocalS(
          declared.name,
          selfUses.isBorrowed(declared.name),
          selfUses.isMoved(declared.name),
          selfUses.isMutated(declared.name),
          childUses.isBorrowed(declared.name),
          childUses.isMoved(declared.name),
          childUses.isMutated(declared.name))
      })
    val bodyRangeS = PostParser.evalRange(functionBodyEnv.file, body0.range)
    val block1WithParamLocals =
      BlockSE(bodyRangeS, block1.locals ++ magicParamLocals, block1.expr)

    val allUses =
      selfUses.combine(childUses, {
        case (None, other) => other
        case (other, None) => other
        case (Some(NotUsed), other) => other
        case (other, Some(NotUsed)) => other
        case (Some(Used), Some(Used)) => Some(Used)
      })

    // We're trying to figure out which variables from parent environments
    // we're using.
    // This is so we can remember in BodySE which variables we're using from
    // containing functions (so we can define the struct which we take in as
    // an implicit first parameter), and also so we can report those upward
    // so if we're using variables from our grandparent, our parent can know
    // that it needs to capture them for us.
    val usesOfParentVariables =
    allUses.uses.filter(use => {
      if (block1WithParamLocals.locals.exists(_.varName == use.name)) {
        // This is a use of a variable declared in this function.
        false
      } else {
        use.name match {
          case MagicParamNameS(_) => {
            // We're using a magic param, which we'll have in this function's params.
            false
          }
          case _ => {
            // This is a use of a variable from somewhere above.
            true
          }
        }
      }
    })

    val bodySE = postparsing.BodySE(PostParser.evalRange(functionBodyEnv.file, body0.range), usesOfParentVariables.map(_.name), block1WithParamLocals)
    (bodySE, VariableUses(usesOfParentVariables), magicParamNames)
  }
*/
  pub(crate) fn scout_interface_member(
    &self,
    file_coordinate: &'s FileCoordinate<'s>,
    function_p: &FunctionP<'p>,
    parent_interface_env: &IEnvironmentS<'s>,
    interface_generic_params: &'s [&'s GenericParameterS<'s>],
    interface_rules: &[IRulexSR<'s>],
    interface_rune_to_explicit_type: &ArenaIndexMap<'s, IRuneS<'s>, ITemplataType>,
  ) -> Result<&'s FunctionS<'s>, ICompileErrorS<'s>>
  {
    assert!(
      function_p.body.is_none(),
      "POSTPARSER_SCOUT_INTERFACE_MEMBER_BODY_NOT_YET_IMPLEMENTED"
    );
    assert!(
      function_p.header.attributes.is_empty(),
      "POSTPARSER_SCOUT_INTERFACE_MEMBER_ATTRIBUTES_NOT_YET_IMPLEMENTED"
    );
    assert!(
      function_p.header.generic_parameters.is_none(),
      "POSTPARSER_SCOUT_INTERFACE_MEMBER_GENERIC_PARAMETERS_NOT_YET_IMPLEMENTED"
    );
    if let Some(params) = &function_p.header.params {
      if !params.params.iter().any(|param| param.virtuality.is_some()) {
        return Err(ICompileErrorS::InterfaceMethodNeedsSelf(
          InterfaceMethodNeedsSelf {
            range: Self::eval_range(file_coordinate, function_p.range),
          },
        ));
      }
    }
    let Some(method_name_p) = function_p.header.name.as_ref() else {
      panic!("POSTPARSER_INTERFACE_MEMBER_WITHOUT_NAME");
    };
    let method_name = self.scout_arena.intern_name(INameValS::FunctionDeclaration(
      IFunctionDeclarationNameValS::FunctionName(FunctionNameS {
        name: self.scout_arena.intern_str(method_name_p.str().as_str()),
        code_location: Self::eval_pos(file_coordinate, method_name_p.range().begin()),
      }),
    ));
    let parent_declared_runes = match parent_interface_env {
      IEnvironmentS::Environment(env) => env.user_declared_runes.clone(),
      _ => panic!("Expected EnvironmentS for interface env"),
    };
    let interface_env = FunctionEnvironmentS {
      file: file_coordinate,
      name: match &method_name {
        INameS::FunctionDeclaration(r) => (*r).clone(),
        _ => panic!("POSTPARSER_INTERN_INTERFACE_METHOD_NAME_EXPECTED_FUNCTION_DECLARATION"),
      },
      parent_env: None,
      declared_runes: parent_declared_runes,
      num_explicit_params: function_p
        .header
        .params
        .as_ref()
        .map(|params| params.params.len() as i32)
        .unwrap_or(0),
      is_interface_internal_method: true,
    };
    let (function_s, variable_uses) = self.scout_function(
      file_coordinate,
      function_p,
      IFunctionParent::ParentCitizen {
        citizen_is_interface: true,
        citizen_env: interface_env,
        citizen_generic_params: interface_generic_params,
        citizen_rules: interface_rules.to_vec(),
        citizen_rune_to_explicit_type: interface_rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
      },
    )?;
    assert!(
      variable_uses.uses.is_empty(),
      "POSTPARSER_SCOUT_INTERFACE_MEMBER_VARIABLE_USES_NOT_EMPTY_NOT_YET_IMPLEMENTED: {:?}",
      variable_uses.uses
    );
    Ok(function_s)
  }
/*
  def scoutInterfaceMember(
    parentInterface: ParentCitizen,
    functionP: FunctionP):
  FunctionS = {
    val file = parentInterface.citizenEnv.file
    val (functionS, variableUses) = scoutFunction(file, functionP, parentInterface)
    vassert(variableUses.uses.isEmpty)
    functionS
  }
*/
}
/*
  }
*/
