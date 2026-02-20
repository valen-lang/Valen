// AFTERM: rename to function_post_parser.rs
// AFTERM: review scout_function

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
use crate::StrI;
use crate::parsing::ast::{FunctionP, IAttributeP, INameDeclarationP, ITemplexPT, LoadAsP};
use crate::postparsing::ast::{
  AbstractBodyS, AbstractSP, AdditiveS, CodeBodyS, CoordGenericParameterTypeS, ExportS, ExternBodyS,
  ExternS, FunctionS, GenericParameterS, IBodyS, IFunctionAttributeS, IGenericParameterTypeS,
  LocationInDenizenBuilder, ParameterS, PureS,
};
use crate::postparsing::expressions::{
  BodySE, ConsecutorSE, IExpressionSE,
};
use crate::postparsing::itemplatatype::{
  CoordTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, FunctionNameS, IFunctionDeclarationNameS, IImpreciseNameValS, IRuneS,
  IRuneValS, IVarNameS, ImplicitRuneS, LambdaDeclarationNameS, MagicParamRuneS,
};
use crate::postparsing::post_parser::{
  ExternHasBodyS, FunctionEnvironmentS, ICompileErrorS, IEnvironmentS, InterfaceMethodNeedsSelf, PostParser,
  RangedInternalErrorS, StackFrame,
};
use crate::postparsing::rules::rules::{IRulexSR, MaybeCoercingLookupSR, PlaceholderRuleSR, RuneUsage};
use crate::postparsing::variable_uses::{VariableDeclarationS, VariableDeclarations, VariableUses};
use crate::utils::arena_utils::alloc_slice_from_vec;
use crate::utils::code_hierarchy::FileCoordinate;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum IFunctionParent<'a> {
  FunctionNoParent,
  ParentInterface {
    interface_env: FunctionEnvironmentS<'a>,
    interface_generic_params: Vec<GenericParameterS<'a>>,
    interface_rules: Vec<IRulexSR<'a>>,
    interface_rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType>,
  },
  ParentFunction {
    parent_stack_frame: StackFrame<'a>,
  },
}

/*
sealed trait IFunctionParent
*/
/*
case class FunctionNoParent() extends IFunctionParent
*/
/*
case class ParentInterface(
  interfaceEnv: EnvironmentS,
  interfaceGenericParams: Vector[GenericParameterS],
  interfaceRules: Vector[IRulexSR],
  interfaceRuneToExplicitType: Map[IRuneS, ITemplataType]
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
impl<'a, 'p, 'ctx, 's> PostParser<'a, 'p, 'ctx, 's>
where
  'a: 'ctx,
  'a: 'p,
  'a: 's,
{
  pub(crate) fn scout_function(
    &self,
    file_coordinate: &'a FileCoordinate<'a>,
    function: &FunctionP<'a, 'p>,
    maybe_parent: IFunctionParent<'a>,
  ) -> Result<(FunctionS<'a, 's>, VariableUses<'a>), ICompileErrorS<'a>>
  where
    'a: 'p,
  {
    // AFTERM: check the order of these various chunks of logic

    let is_parent_function = matches!(&maybe_parent, IFunctionParent::ParentFunction { .. });
    let is_parent_interface = matches!(&maybe_parent, IFunctionParent::ParentInterface { .. });
    let function_name = function.header.name.as_ref();
    let func_attrs_s: Vec<IFunctionAttributeS<'a>> = {
      let unfiltered_attrs_p = function.header.attributes;
      let filtered_attrs: Vec<&IAttributeP<'a>> = match maybe_parent {
        IFunctionParent::FunctionNoParent => unfiltered_attrs_p
          .iter()
          .filter(|a| !matches!(a, IAttributeP::AbstractAttribute(_)))
          .collect(),
        IFunctionParent::ParentInterface { .. } => unfiltered_attrs_p.iter().collect(),
        IFunctionParent::ParentFunction { .. } => unfiltered_attrs_p.iter().collect(),
      };
      filtered_attrs
        .into_iter()
        .map(|attr| match attr {
          IAttributeP::ExportAttribute(_) => {
            IFunctionAttributeS::Export(ExportS {
              package_coordinate: file_coordinate.package_coord,
            })
          }
          IAttributeP::ExternAttribute(_) => {
            IFunctionAttributeS::Extern(ExternS {
              package_coord: file_coordinate.package_coord,
            })
          }
          IAttributeP::PureAttribute(_) => IFunctionAttributeS::Pure(PureS),
          IAttributeP::AdditiveAttribute(_) => IFunctionAttributeS::Additive(AdditiveS),
          IAttributeP::BuiltinAttribute(builtin_attr) => {
            IFunctionAttributeS::Builtin(crate::postparsing::ast::BuiltinS {
              generator_name: builtin_attr.generator_name.str(),
            })
          }
          IAttributeP::AbstractAttribute(_) => panic!("AbstractAttribute should have been filtered"),
          other => panic!("POSTPARSER_SCOUT_FUNCTION_ATTRIBUTE_NOT_YET_IMPLEMENTED: {:?}", other),
        })
        .collect()
    };
    if function.header.generic_parameters.is_some() {
      panic!("POSTPARSER_SCOUT_FUNCTION_GENERICS_NOT_YET_IMPLEMENTED");
    }
    if function.header.template_rules.is_some() {
      panic!("POSTPARSER_SCOUT_FUNCTION_TEMPLATE_RULES_NOT_YET_IMPLEMENTED");
    }
    let explicit_self_name: Option<StrI<'a>> = if is_parent_function || is_parent_interface {
      None
    } else if let Some(params) = &function.header.params {
      if params.params.is_empty() {
        None
      } else if params.params.len() == 1 {
        let only_param = params.params.first().unwrap();
        assert!(
          only_param.maybe_pre_checked.is_none(),
          "POSTPARSER_SCOUT_FUNCTION_PARAM_PRECHECKED_NOT_YET_IMPLEMENTED"
        );
        match (&only_param.self_borrow, &only_param.pattern) {
          (Some(_), None) => None,
          (None, Some(pattern)) => {
            let destination = pattern
              .destination
              .as_ref()
              .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_FUNCTION_PARAM_DESTINATION_NOT_YET_IMPLEMENTED"));
            assert!(
              destination.mutate.is_none(),
              "POSTPARSER_SCOUT_FUNCTION_PARAM_MUTATE_NOT_YET_IMPLEMENTED"
            );
            let INameDeclarationP::LocalNameDeclaration(local_name) = &destination.decl else {
              panic!("POSTPARSER_SCOUT_FUNCTION_PARAM_DECL_NOT_YET_IMPLEMENTED");
            };
            assert!(
              pattern.templex.is_some(),
              "POSTPARSER_SCOUT_FUNCTION_SELF_PARAM_WITHOUT_TYPE_NOT_YET_IMPLEMENTED"
            );
            assert!(
              pattern.destructure.is_none(),
              "POSTPARSER_SCOUT_FUNCTION_PARAM_DESTRUCTURE_NOT_YET_IMPLEMENTED"
            );
            Some(local_name.str())
          }
          _ => panic!("POSTPARSER_SCOUT_FUNCTION_PARAM_FORM_NOT_YET_IMPLEMENTED"),
        }
      } else {
        // For multi-parameter functions, "self" can only be one capture among many,
        // and explicit_params_s will populate declarations for all captures.
        None
      }
    } else {
      None
    };
    let mut lidb = LocationInDenizenBuilder::new(vec![]);
    let mut rules: Vec<IRulexSR<'a>> = match &maybe_parent {
      IFunctionParent::ParentInterface { interface_rules, .. } => interface_rules.to_vec(),
      _ => Vec::new(),
    };
    let mut rune_to_predicted_type: HashMap<IRuneS<'a>, ITemplataType> = match &maybe_parent {
      IFunctionParent::ParentInterface {
        interface_rune_to_explicit_type,
        ..
      } => interface_rune_to_explicit_type.clone(),
      _ => HashMap::new(),
    };
    let maybe_ret_coord_rune = match &function.header.ret.ret_type {
      None => None,
      Some(ret_type) => {
        let ITemplexPT::NameOrRune(name_or_rune) = ret_type else {
          panic!("POSTPARSER_SCOUT_FUNCTION_RETURN_TYPE_NOT_YET_IMPLEMENTED");
        };
        let ret_range = PostParser::eval_range(file_coordinate, name_or_rune.0.range());
        let ret_coord_rune = RuneUsage {
          range: ret_range.clone(),
          rune: self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
            lid: lidb.child().consume(),
          })),
        };
        rules.push(IRulexSR::MaybeCoercingLookup(MaybeCoercingLookupSR {
          range: ret_range,
          rune: ret_coord_rune.clone(),
          name: self.interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
            name: name_or_rune.0.str(),
          })),
        }));
        rune_to_predicted_type.insert(
          ret_coord_rune.rune.clone(),
          ITemplataType::CoordTemplataType(CoordTemplataType {}),
        );
        Some(ret_coord_rune)
      }
    };
    let function_declaration_name = match (&maybe_parent, function_name) {
      (IFunctionParent::ParentFunction { .. }, Some(_)) => {
        panic!("POSTPARSER_SCOUT_LAMBDA_WITH_NAME_NOT_YET_IMPLEMENTED");
      }
      (_, Some(function_name)) => IFunctionDeclarationNameS::FunctionName(FunctionNameS {
        name: function_name.str(),
        code_location: Self::eval_pos(file_coordinate, function_name.range().begin()),
      }),
      (IFunctionParent::ParentFunction { .. }, None) => {
        IFunctionDeclarationNameS::LambdaDeclarationName(LambdaDeclarationNameS {
          code_location: Self::eval_pos(file_coordinate, function.range.begin()),
        })
      }
      _ => panic!("POSTPARSER_SCOUT_FUNCTION_WITHOUT_NAME"),
    };
    let extra_generic_params_from_parent: Vec<GenericParameterS<'a>> = match &maybe_parent {
      IFunctionParent::ParentInterface {
        interface_generic_params,
        ..
      } => interface_generic_params.to_vec(),
      _ => Vec::new(),
    };
    let parent_env: Option<Box<IEnvironmentS<'a>>> = match &maybe_parent {
      IFunctionParent::FunctionNoParent => None,
      IFunctionParent::ParentFunction { parent_stack_frame } => {
        Some(Box::new(IEnvironmentS::FunctionEnvironment(
          parent_stack_frame.parent_env.clone(),
        )))
      }
      IFunctionParent::ParentInterface { interface_env, .. } => {
        Some(Box::new(IEnvironmentS::FunctionEnvironment(interface_env.clone())))
      }
    };
    let declared_runes: Vec<IRuneS<'a>> = match &maybe_parent {
      IFunctionParent::ParentInterface {
        interface_rune_to_explicit_type,
        ..
      } => interface_rune_to_explicit_type.keys().cloned().collect(),
      _ => Vec::new(),
    };
    let function_environment = FunctionEnvironmentS {
      file: file_coordinate,
      name: function_declaration_name.clone(),
      parent_env,
      declared_runes,
      num_explicit_params: function
        .header
        .params
        .as_ref()
        .map(|params| params.params.len() as i32)
        .unwrap_or(0),
      is_interface_internal_method: matches!(&maybe_parent, IFunctionParent::ParentInterface { .. }),
    };
    let default_region_rune = if let Some(function_name) = function_name {
      self.interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
        name: function_name.str(),
      }))
    } else {
      self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
        lid: lidb.child().consume(),
      }))
    };
    let mut initial_declarations = {
      let mut vars = Vec::<VariableDeclarationS>::new();
      if let Some(explicit_self_name) = &explicit_self_name {
        vars.push(VariableDeclarationS {
          name: IVarNameS::CodeVarName(*explicit_self_name),
        });
      }
      VariableDeclarations { vars }
    };
    let explicit_params_s: Vec<ParameterS<'a>> = function
      .header
      .params
      .as_ref()
      .map(|params| {
        params
          .params
          .iter()
          .map(|param| {
            let param_range = PostParser::eval_range(file_coordinate, param.range);
            let virtuality = param.virtuality.as_ref().map(|abstract_p| AbstractSP {
              range: PostParser::eval_range(file_coordinate, abstract_p.range),
              is_internal_method: matches!(maybe_parent, IFunctionParent::ParentInterface { .. }),
            });
            let coord_rune = RuneUsage {
              range: param_range.clone(),
              rune: self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
                lid: lidb.child().consume(),
              })),
            };
            rune_to_predicted_type.insert(
              coord_rune.rune.clone(),
              ITemplataType::CoordTemplataType(CoordTemplataType {}),
            );
            let capture = match (&param.self_borrow, &param.pattern) {
              (Some(_), None) => Some(CaptureS {
                name: IVarNameS::CodeVarName(self.keywords.self_),
                mutate: false,
              }),
              (None, Some(pattern)) => {
                if !is_parent_function {
                  assert!(
                    pattern.templex.is_some(),
                    "POSTPARSER_SCOUT_FUNCTION_NON_LAMBDA_PARAM_WITHOUT_TYPE_NOT_YET_IMPLEMENTED"
                  );
                }
                assert!(
                  pattern.destructure.is_none(),
                  "POSTPARSER_SCOUT_FUNCTION_PARAM_DESTRUCTURE_NOT_YET_IMPLEMENTED"
                );
                pattern.destination.as_ref().map(|destination| CaptureS {
                  name: match &destination.decl {
                    INameDeclarationP::LocalNameDeclaration(local_name) => {
                      IVarNameS::CodeVarName(local_name.str())
                    }
                    _ => panic!(
                      "POSTPARSER_SCOUT_FUNCTION_PARAM_DECL_NOT_YET_IMPLEMENTED: {:?}",
                      destination.decl
                    ),
                  },
                  mutate: destination.mutate.is_some(),
                })
              }
              _ => panic!("POSTPARSER_SCOUT_FUNCTION_PARAM_FORM_NOT_YET_IMPLEMENTED"),
            };
            ParameterS {
              range: param_range.clone(),
              virtuality,
              pre_checked: param.maybe_pre_checked.is_some(),
              pattern: AtomSP {
                range: param_range,
                name: capture,
                coord_rune: Some(coord_rune),
                destructure: None,
              },
            }
          })
          .collect()
      })
      .unwrap_or_default();
    for param_s in &explicit_params_s {
      if let Some(capture) = &param_s.pattern.name {
        if !initial_declarations.vars.iter().any(|decl| decl.name == capture.name) {
          initial_declarations.vars.push(VariableDeclarationS {
            name: capture.name.clone(),
          });
        }
      }
    }
    let has_extern_attr = function
      .header
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::ExternAttribute(_)));
    let (body_s, variable_uses, total_params_s, extra_generic_params_from_body) = if is_parent_interface {
      (
        IBodyS::AbstractBody(AbstractBodyS {}),
        VariableUses::empty(),
        explicit_params_s,
        Vec::new(),
      )
    } else if has_extern_attr {
      if function.body.is_some() {
        return Err(ICompileErrorS::ExternHasBodyS(ExternHasBodyS {
          range: Self::eval_range(file_coordinate, function.range),
        }));
      }
      (
        IBodyS::ExternBody(ExternBodyS {}),
        VariableUses::empty(),
        explicit_params_s,
        Vec::new(),
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
      let (body_s, variable_uses, magic_param_names) = self.scout_body(
        function_environment,
        None,
        &mut lidb,
        default_region_rune,
        body,
        initial_declarations,
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
      if is_parent_function && !body_s.closured_names.is_empty() {
        panic!(
          "POSTPARSER_SCOUT_LAMBDA_CLOSURE_CAPTURES_NOT_YET_IMPLEMENTED: {:?}",
          body_s.closured_names
        );
      }
      if is_parent_function && !magic_param_names.is_empty() && !explicit_params_s.is_empty() {
        return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
          range: Self::eval_range(file_coordinate, function.range),
          message: "Cant have a lambda with _ and params".to_string(),
        }));
      }
      let mut total_params_s: Vec<ParameterS<'a>> = Vec::new();
      let mut extra_generic_params_from_body = Vec::<GenericParameterS<'a>>::new();
      if is_parent_function {
        let IFunctionParent::ParentFunction { parent_stack_frame } = &maybe_parent else {
          panic!("POSTPARSER_SCOUT_FUNCTION_EXPECTED_PARENT_FUNCTION");
        };
        let closure_struct_region_rune = self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
          lid: lidb.child().consume(),
        }));
        let closure_struct_kind_rune = self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
          lid: lidb.child().consume(),
        }));
        let closure_struct_coord_rune = self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
          lid: lidb.child().consume(),
        }));
        let closure_param_s = self.create_closure_param(
          function.range,
          function_declaration_name.clone(),
          &mut lidb,
          &mut rules,
          &mut rune_to_predicted_type,
          parent_stack_frame,
          closure_struct_region_rune,
          closure_struct_kind_rune,
          closure_struct_coord_rune,
        );
        total_params_s.push(closure_param_s);
      }
      total_params_s.extend(explicit_params_s.clone());
      if is_parent_function {
        let magic_params =
          self.create_magic_parameters(&mut lidb, magic_param_names, &mut rune_to_predicted_type);
        // Lambdas identifying runes are determined by their magic params.
        // See: Lambdas Dont Need Explicit Identifying Runes (LDNEIR)
        extra_generic_params_from_body.extend(magic_params.iter().map(|magic_param| {
          let coord_rune = magic_param
            .pattern
            .coord_rune
            .as_ref()
            .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_MAGIC_PARAM_WITHOUT_COORD_RUNE"))
            .clone();
          GenericParameterS {
            range: magic_param.pattern.range.clone(),
            rune: coord_rune,
            tyype: IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS {
              coord_region: None,
              kind_mutable: true,
              region_mutable: false,
            }),
            default: None,
          }
        }));
        total_params_s.extend(magic_params);
      }
      (
        IBodyS::CodeBody(CodeBodyS { body: body_s }),
        variable_uses,
        total_params_s,
        extra_generic_params_from_body,
      )
    };
    let mut generic_params = extra_generic_params_from_parent;
    generic_params.extend(extra_generic_params_from_body);
    Ok((FunctionS {
      range: Self::eval_range(file_coordinate, function.range),
      name: function_declaration_name,
      attributes: alloc_slice_from_vec(self.scout_arena, func_attrs_s),
      generic_params: alloc_slice_from_vec(self.scout_arena, generic_params),
      rune_to_predicted_type: rune_to_predicted_type,
      tyype: TemplateTemplataType {
        param_types: Vec::new(),
        return_type: Box::new(ITemplataType::CoordTemplataType(CoordTemplataType {})),
      },
      params: alloc_slice_from_vec(self.scout_arena, total_params_s),
      maybe_ret_coord_rune,
      rules: alloc_slice_from_vec(self.scout_arena, rules),
      body: body_s,
    }, variable_uses))
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
      case ParentInterface(_, _, _, _) =>
      case ParentFunction(_) => {
        vcurious(maybeGenericParametersP.isEmpty)
      }
    }

    val funcName =
      maybeParent match {
        case FunctionNoParent() | ParentInterface(_, _, _, _) => {
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

    maybeParent match {
      case ParentInterface(_, _, _, _) => vassert(userDeclaredRunes.isEmpty)
      case _ =>
    }

    val lidb = new LocationInDenizenBuilder(Vector())

    maybeParent match {
      case FunctionNoParent() =>
      case ParentFunction(_) =>
      case ParentInterface(_, _, _, _) => {
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

    val parentEnv =
      maybeParent match {
        case FunctionNoParent() => None
        case ParentFunction(parentStackFrame) => Some(parentStackFrame.parentEnv)
        case ParentInterface(interfaceEnv, _, _, _) => Some(interfaceEnv)
      }
    val isInterfaceInternalMethod =
      maybeParent match {
        case FunctionNoParent() => false
        case ParentFunction(parentStackFrame) => false
        case ParentInterface(_, _, _, _) => true
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
      case ParentInterface(interfaceEnv, _, interfaceRules, interfaceRuneToExplicitType) => {
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
          case FunctionNoParent() | ParentInterface(_, _, _, _) => {
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

    val explicitParamsS =
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
                ParameterS(rangeS, maybeAbstractS, maybePreChecked.nonEmpty, patternS)
              }
              case (None, Some(patternP)) => {
                val patternPerhapsWithoutCoordRuneS =
                  patternScout.translatePattern(
                    myStackFrameWithoutParams,
                    lidb.child(),
                    ruleBuilder,
                    runeToExplicitType,
                    patternP)
                val patternS =
                  patternPerhapsWithoutCoordRuneS.coordRune match {
                    case None => {
                      val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                      runeToExplicitType += ((rune.rune, CoordTemplataType()))
                      patternPerhapsWithoutCoordRuneS.copy(coordRune = Some(rune))
                    }
                    case Some(_) => patternPerhapsWithoutCoordRuneS
                  }
                ParameterS(rangeS, maybeAbstractS, maybePreChecked.nonEmpty, patternS)
              }
            }
          }
        })
//    val explicitParamsS = explicitParamPatternsAndIdentifyingRunes.map(_._1).map(ParameterS)
//    val identifyingRunesFromExplicitParams = explicitParamPatternsAndIdentifyingRunes.flatMap(_._2)

    // Only if the function actually has a body
    val maybeCaptureDeclarations =
      maybeBody0 match {
        case None => None
        case Some(_) => {
          val firstParams =
            maybeParent match {
              case FunctionNoParent() => noDeclarations
              case ParentInterface(_, _, _, _) => noDeclarations
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
            case FunctionNoParent() | ParentInterface(_, _, _, _) => {
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
            maybeParent match {
              case FunctionNoParent() => functionEnv
              case ParentFunction(_) => functionEnv
              case ParentInterface(interfaceEnv, _, _, _) => interfaceEnv
            },
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
      case ParentInterface(_, _, _, _) => {
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
        case ParentInterface(_, interfaceGenericParams, _, _) => interfaceGenericParams
      })

    val (maybeBody1, variableUses, extraGenericParamsFromBodyS, maybeClosureParam, magicParams) =
      if (maybeParent match { case ParentInterface(_, _, _, _) => true case _ => false }) {
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
            case FunctionNoParent() | ParentInterface(_, _, _, _) => None
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
            case FunctionNoParent() | ParentInterface(_, _, _, _) => {
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
      (extraGenericParamsFromParentS ++
        functionUserSpecifiedGenericParametersS ++
        extraGenericParamsFromBodyS)
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
        case ParentInterface(_, _, _, _) => unfilteredAttrsP
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
        case ParentInterface(_, _, _, _) => {
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
  range: crate::lexing::ast::RangeL,
  func_name: IFunctionDeclarationNameS<'a>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  rune_to_predicted_type: &mut HashMap<IRuneS<'a>, ITemplataType>,
  parent_stack_frame: &StackFrame<'a>,
  _closure_struct_region_rune: IRuneS<'a>,
  closure_struct_kind_rune: IRuneS<'a>,
  closure_struct_coord_rune: IRuneS<'a>,
) -> crate::postparsing::ast::ParameterS<'a> {
  let closure_param_pos = PostParser::eval_pos(parent_stack_frame.file, range.begin());
  let closure_param_range = crate::utils::range::RangeS {
    begin: closure_param_pos.clone(),
    end: closure_param_pos.clone(),
  };
  let IFunctionDeclarationNameS::LambdaDeclarationName(_lambda_name) = func_name else {
    panic!("POSTPARSER_SCOUT_CREATE_CLOSURE_PARAM_NON_LAMBDA_NAME");
  };
  rune_to_predicted_type.insert(
    closure_struct_kind_rune,
    ITemplataType::KindTemplataType(KindTemplataType {}),
  );
  rune_to_predicted_type.insert(
    closure_struct_coord_rune.clone(),
    ITemplataType::CoordTemplataType(CoordTemplataType {}),
  );
  let closure_param_type_rune = RuneUsage {
    range: closure_param_range.clone(),
    rune: self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
      lid: lidb.child().consume(),
    })),
  };
  rune_to_predicted_type.insert(
    closure_param_type_rune.rune.clone(),
    ITemplataType::CoordTemplataType(CoordTemplataType {}),
  );
  // Scala emits Lookup/CoerceToCoord/Augment rules here. We do not have
  // those rule variants in Rust yet, so we leave explicit placeholders
  // instead of silently dropping this part of the shape.
  rule_builder.push(IRulexSR::Placeholder(PlaceholderRuleSR {
    range: closure_param_range.clone(),
  }));
  rule_builder.push(IRulexSR::Placeholder(PlaceholderRuleSR {
    range: closure_param_range.clone(),
  }));
  rule_builder.push(IRulexSR::Placeholder(PlaceholderRuleSR {
    range: closure_param_range.clone(),
  }));
  ParameterS {
    range: closure_param_range.clone(),
    virtuality: None,
    pre_checked: false,
    pattern: AtomSP {
      range: closure_param_range,
      name: Some(CaptureS {
        name: IVarNameS::ClosureParamName(closure_param_pos),
        mutate: false,
      }),
      coord_rune: Some(closure_param_type_rune),
      destructure: None,
    },
  }
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
  lambda_magic_param_names: Vec<crate::postparsing::names::MagicParamNameS<'a>>,
  rune_to_predicted_type: &mut HashMap<IRuneS<'a>, ITemplataType>,
) -> Vec<crate::postparsing::ast::ParameterS<'a>> {
  lambda_magic_param_names
    .into_iter()
    .map(|magic_param_name| {
      let magic_param_range = crate::utils::range::RangeS {
        begin: magic_param_name.code_location.clone(),
        end: magic_param_name.code_location.clone(),
      };
      let magic_param_rune = self.interner.intern_rune(IRuneValS::MagicParamRune(MagicParamRuneS {
        lid: lidb.child().consume(),
      }));
      rune_to_predicted_type.insert(
        magic_param_rune.clone(),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      );
      ParameterS {
        range: magic_param_range.clone(),
        virtuality: None,
        pre_checked: false,
        pattern: AtomSP {
          range: magic_param_range.clone(),
          name: Some(CaptureS {
            name: IVarNameS::MagicParamName(magic_param_name.code_location),
            mutate: false,
          }),
          coord_rune: Some(RuneUsage {
            range: magic_param_range,
            rune: magic_param_rune,
          }),
          destructure: None,
        },
      }
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
    parent_stack_frame: StackFrame<'a>,
    function: &FunctionP<'a, 'p>,
  ) -> Result<(FunctionS<'a, 's>, VariableUses<'a>), ICompileErrorS<'a>>
  where
    'a: 'p,
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
    function_env: FunctionEnvironmentS<'a>,
    parent_stack_frame: Option<StackFrame<'a>>,
    lidb: &mut LocationInDenizenBuilder,
    context_region: IRuneS<'a>,
    body0: &crate::parsing::ast::BlockPE<'a, 'p>,
    initial_declarations: VariableDeclarations<'a>,
  ) -> Result<
    (
      crate::postparsing::expressions::BodySE<'a, 's>,
      VariableUses<'a>,
      Vec<crate::postparsing::names::MagicParamNameS<'a>>,
    ),
    ICompileErrorS<'a>,
  > {
    let function_body_env = function_env.child();
    let body_range_s = PostParser::eval_range(function_body_env.file, body0.range);
    let mut new_block_lidb = lidb.child();
    let (block1, self_uses, child_uses) = self.new_block(
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
        let expr_without_constructing_without_void: &'s IExpressionSE<'a, 's> = match inner_expr {
          IExpressionSE::Consecutor(consecutor) => {
            let exprs: Vec<&'s IExpressionSE<'a, 's>> = {
              let mut v: Vec<_> = consecutor.exprs.iter().copied().collect();
              while matches!(v.last(), Some(IExpressionSE::Void(_))) {
                v.pop();
              }
              v
            };
            assert!(
              !exprs.is_empty(),
              "POSTPARSER_SCOUT_BODY_CONSECUTOR_EMPTY_AFTER_VOID_STRIP"
            );
            if exprs.len() == 1 {
              exprs.into_iter().next().unwrap()
            } else {
              &*self.scout_arena.alloc(IExpressionSE::Consecutor(ConsecutorSE {
                exprs: alloc_slice_from_vec(self.scout_arena, exprs),
              }))
            }
          }
          other => other,
        };
        Ok((
          stack_frame2,
          expr_without_constructing_without_void,
          inner_self_uses,
          inner_child_uses,
        ))
      },
    )?;

    let magic_param_names: Vec<crate::postparsing::names::MagicParamNameS<'a>> = self_uses
      .uses
      .iter()
      .filter_map(|use_| match &use_.name {
        IVarNameS::MagicParamName(code_location) => {
          Some(crate::postparsing::names::MagicParamNameS {
            code_location: code_location.clone(),
          })
        }
        _ => None,
      })
      .collect();
    let magic_param_vars: Vec<VariableDeclarationS<'a>> = magic_param_names
      .iter()
      .map(|magic_param_name| VariableDeclarationS {
        name: IVarNameS::MagicParamName(magic_param_name.code_location.clone()),
      })
      .collect();
    let magic_param_locals: Vec<crate::postparsing::expressions::LocalS<'a>> = magic_param_vars
      .iter()
      .map(|declared| crate::postparsing::expressions::LocalS {
        var_name: declared.name.clone(),
        self_borrowed: self_uses.is_borrowed(&declared.name),
        self_moved: self_uses.is_moved(&declared.name),
        self_mutated: self_uses.is_mutated(&declared.name),
        child_borrowed: child_uses.is_borrowed(&declared.name),
        child_moved: child_uses.is_moved(&declared.name),
        child_mutated: child_uses.is_mutated(&declared.name),
      })
      .collect();
    let mut block1_with_magic_param_locals = block1.clone();
    block1_with_magic_param_locals.locals.extend(magic_param_locals);
    let block1 = &*self.scout_arena.alloc(block1_with_magic_param_locals);
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
    let closured_names: Vec<IVarNameS<'a>> = uses_of_parent_variables
      .iter()
      .map(|use_| use_.name.clone())
      .collect();
    let body_s = BodySE {
      range: PostParser::eval_range(function_body_env.file, body0.range),
      closured_names,
      block: block1,
    };
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
    file_coordinate: &'a FileCoordinate<'a>,
    function_p: &crate::parsing::ast::FunctionP<'a, 'p>,
    interface_generic_params: &[GenericParameterS<'a>],
    interface_rules: &[IRulexSR<'a>],
    interface_rune_to_explicit_type: &HashMap<IRuneS<'a>, ITemplataType>,
  ) -> Result<FunctionS<'a, 's>, ICompileErrorS<'a>>
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
    assert!(
      function_p.header.template_rules.is_none(),
      "POSTPARSER_SCOUT_INTERFACE_MEMBER_TEMPLATE_RULES_NOT_YET_IMPLEMENTED"
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
    let interface_env = FunctionEnvironmentS {
      file: file_coordinate,
      name: IFunctionDeclarationNameS::FunctionName(FunctionNameS {
        name: method_name_p.str(),
        code_location: Self::eval_pos(file_coordinate, method_name_p.range().begin()),
      }),
      parent_env: None,
      declared_runes: Vec::new(),
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
      IFunctionParent::ParentInterface {
        interface_env,
        interface_generic_params: interface_generic_params.to_vec(),
        interface_rules: interface_rules.to_vec(),
        interface_rune_to_explicit_type: interface_rune_to_explicit_type.clone(),
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
    parentInterface: ParentInterface,
    functionP: FunctionP):
  FunctionS = {
    val file = parentInterface.interfaceEnv.file
    val (functionS, variableUses) = scoutFunction(file, functionP, parentInterface)
    vassert(variableUses.uses.isEmpty)
    functionS
  }
*/
}
/*
  }
*/
