// Per @DSAUIMZ, all borrow_val() calls in this file borrow from a stack-local
// LocationInDenizenBuilder instead of arena-allocating. The slice is promoted
// to permanent arena storage only inside intern_rune on a miss.

use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::parsing::ast::{BuiltinCallPR, ComponentsPR, EqualsPR, IntPT, IRulexPR, ITypePR, ITemplexPT, OwnershipPT};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::itemplatatype::{
  BooleanTemplataType, CoordTemplataType, ITemplataType, IntegerTemplataType, KindTemplataType,
  LocationTemplataType, MutabilityTemplataType, OwnershipTemplataType, PackTemplataType,
  PrototypeTemplataType, RegionTemplataType, VariabilityTemplataType,
};
use crate::postparsing::names::{CodeRuneS, IImpreciseNameS, IRuneS, IRuneValS, ImplicitRuneValS};
use crate::postparsing::post_parser::{IEnvironmentS, PostParser};
use crate::postparsing::rules::rules::{
  CoordComponentsSR, EqualsSR, IntLiteralSL, IsInterfaceSR, IRulexSR, OneOfSR,
  OwnershipLiteralSL, RuneUsage,
};
use crate::postparsing::rules::rules::ILiteralSL;
use crate::postparsing::rules::templex_scout::translate_templex;
use std::collections::{HashMap, HashSet};
use crate::postparsing::itemplatatype::ImplTemplataType;
use crate::postparsing::rules::rules::DefinitionCoordIsaSR;
use crate::postparsing::rules::rules::CallSiteCoordIsaSR;
use crate::postparsing::rules::rules::PackSR;
use crate::postparsing::rules::rules::KindComponentsSR;
use crate::postparsing::rules::rules::PrototypeComponentsSR;
/*
package dev.vale.postparsing.rules

import dev.vale.lexing.RangeL
import dev.vale.parsing.ast.{BoolTypePR, BuiltinCallPR, ComponentsPR, CoordListTypePR, CoordTypePR, EqualsPR, IRulexPR, ITypePR, IntPT, IntTypePR, KindTypePR, LocationTypePR, MutabilityTypePR, NameP, OrPR, OwnershipPT, OwnershipTypePR, PrototypeTypePR, TemplexPR, TypedPR, VariabilityTypePR}
import dev.vale.postparsing._
import dev.vale._
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing._
import dev.vale.postparsing.rules.RuleScout.translateType

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
*/
/*
class RuleScout(interner: Interner, keywords: Keywords, templexScout: TemplexScout) {
*/

// Returns:
// - new rules produced on the side while translating the given rules
// - the translated versions of the given rules
pub fn translate_rulexes<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
  context_region: IRuneS<'s>,
  rules_p: &[IRulexPR<'p>],
) -> Vec<RuneUsage<'s>> {
  rules_p
    .iter()
    .map(|rule_p| {
      let mut child_lidb = lidb.child();
      translate_rulex(
        scout_arena,
        keywords,
        env.clone(),
        &mut child_lidb,
        builder,
        rune_to_explicit_type,
        context_region.clone(),
        rule_p,
      )
    })
    .collect()
}
/*
  // Returns:
  // - new rules produced on the side while translating the given rules
  // - the translated versions of the given rules
  def translateRulexes(
    env: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    builder: ArrayBuffer[IRulexSR],
    runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
    contextRegion: IRuneS,
    rulesP: Vector[IRulexPR]):
  Vector[RuneUsage] = {
    rulesP.map(translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, _))
  }
*/
fn translate_rulex<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
  context_region: IRuneS<'s>,
  rulex: &IRulexPR<'p>,
) -> RuneUsage<'s> {
  let file = match &env {
    IEnvironmentS::Environment(environment) => environment.file,
    IEnvironmentS::FunctionEnvironment(function_environment) => function_environment.file,
  };
  match rulex {
    IRulexPR::Typed(typed_rule) => {
      let rune = match &typed_rule.rune {
        Some(rune_name) => scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str(rune_name.str().as_str()) })),
        None => {
          let mut child_lidb = lidb.child();
          scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val())))
        }
      };
      let tyype = translate_type(scout_arena, typed_rule.tyype);
      rune_to_explicit_type.push((rune.clone(), tyype));
      RuneUsage {
        range: PostParser::eval_range(file, typed_rule.range),
        rune,
      }
    }
    IRulexPR::Templex(templex) => {
      let mut child_lidb = lidb.child();
      translate_templex(
        scout_arena,
        keywords,
        env,
        &mut child_lidb,
        builder,
        context_region,
        templex,
      )
    }
    IRulexPR::Equals(EqualsPR { range, left, right }) => {
      let mut child_lidb = lidb.child();
      let rune = scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val())));
      let left_usage = {
        let mut child_lidb = lidb.child();
        translate_rulex(scout_arena,
          keywords,
          env.clone(),
          &mut child_lidb,
          builder,
          rune_to_explicit_type,
          context_region.clone(),
          left,
        )
      };
      let right_usage = {
        let mut child_lidb = lidb.child();
        translate_rulex(scout_arena,
          keywords,
          env.clone(),
          &mut child_lidb,
          builder,
          rune_to_explicit_type,
          context_region.clone(),
          right,
        )
      };
      builder.push(IRulexSR::Equals(EqualsSR {
        range: PostParser::eval_range(file, *range),
        left: left_usage,
        right: right_usage,
      }));
      RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune,
      }
    }
    IRulexPR::BuiltinCall(BuiltinCallPR { range, name, args }) => {
      if name.str() == keywords.is_interface {
        assert_eq!(args.len(), 1, "POSTPARSER_IS_INTERFACE_ARGS_LEN");
        let mut child_lidb = lidb.child();
        let arg_rune = translate_rulex(scout_arena,
          keywords,
          env.clone(),
          &mut child_lidb,
          builder,
          rune_to_explicit_type,
          context_region.clone(),
          &args[0],
        );
        // val resultRune = ImplicitRuneS(lidb.child().consume())
        builder.push(IRulexSR::IsInterface(IsInterfaceSR {
          range: PostParser::eval_range(file, *range),
          rune: arg_rune.clone(),
        }));
        // runeToExplicitType.put(resultRune, KindTemplataType())
        rune_to_explicit_type.push((
          arg_rune.rune.clone(),
          ITemplataType::KindTemplataType(KindTemplataType {}),
        ));
        RuneUsage {
          range: PostParser::eval_range(file, *range),
          rune: arg_rune.rune,
        }
      } else if name.str() == keywords.implements {
        assert_eq!(args.len(), 2, "POSTPARSER_IMPLEMENTS_ARGS_LEN");
        let struct_rune = translate_rulex(scout_arena, keywords, env.clone(), &mut lidb.child(), builder, rune_to_explicit_type, context_region.clone(), &args[0]);
        rune_to_explicit_type.push((struct_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})));
        let interface_rune = translate_rulex(scout_arena, keywords, env.clone(), &mut lidb.child(), builder, rune_to_explicit_type, context_region.clone(), &args[1]);
        rune_to_explicit_type.push((interface_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})));

        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: PostParser::eval_range(file, *range),
          rune: scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        rune_to_explicit_type.push((result_rune_s.rune.clone(), ITemplataType::ImplTemplataType(ImplTemplataType {})));

        // Only appears in definition; filtered out when solving call site
        builder.push(IRulexSR::DefinitionCoordIsa(DefinitionCoordIsaSR {
          range: PostParser::eval_range(file, *range),
          result_rune: result_rune_s.clone(),
          sub_rune: struct_rune.clone(),
          super_rune: interface_rune.clone(),
        }));
        // Only appears in call site; filtered out when solving definition
        builder.push(IRulexSR::CallSiteCoordIsa(CallSiteCoordIsaSR {
          range: PostParser::eval_range(file, *range),
          result_rune: Some(result_rune_s),
          sub_rune: struct_rune.clone(),
          super_rune: interface_rune,
        }));

        RuneUsage {
          range: PostParser::eval_range(file, *range),
          rune: struct_rune.rune,
        }
      } else if name.str() == keywords.ref_list_compound_mutability {
        panic!("POSTPARSER_TRANSLATE_RULEX_BUILTINCALL_REF_LIST_COMPOUND_MUTABILITY_NOT_YET_IMPLEMENTED")
      } else if name.str() == keywords.refs {
        let arg_runes: Vec<RuneUsage<'s>> =
          args.iter().map(|arg| {
            translate_rulex(scout_arena, keywords, env.clone(), &mut lidb.child(), builder, rune_to_explicit_type, context_region.clone(), arg)
          }).collect();

        let mut child_lidb = lidb.child();
        let result_rune = RuneUsage {
          range: PostParser::eval_range(file, *range),
          rune: scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        builder.push(IRulexSR::Pack(PackSR {
          range: PostParser::eval_range(file, *range),
          result_rune: result_rune.clone(),
          members: scout_arena.alloc_slice_from_vec(arg_runes),
        }));
        rune_to_explicit_type.push((result_rune.rune.clone(), ITemplataType::PackTemplataType(PackTemplataType { element_type: &*scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})) })));

        result_rune
      } else if name.str() == keywords.any {
        let literals: Vec<ILiteralSL> = args
          .iter()
          .map(|arg| match arg {
            IRulexPR::Templex(templex) => match templex {
              ITemplexPT::Int(IntPT { value, .. }) => {
                ILiteralSL::IntLiteral(IntLiteralSL { value: *value })
              }
              ITemplexPT::Ownership(OwnershipPT(_, ownership)) => {
                ILiteralSL::OwnershipLiteral(OwnershipLiteralSL {
                  ownership: *ownership,
                })
              }
              _ => panic!("POSTPARSER_BUILTINCALL_ANY_ARG_NOT_INT_OR_OWNERSHIP"),
            },
            _ => panic!("POSTPARSER_BUILTINCALL_ANY_ARG_NOT_TEMPLEX"),
          })
          .collect();
        assert!(!literals.is_empty(), "POSTPARSER_ANY_LITERALS_EMPTY");
        let distinct_types: HashSet<_> =
          literals.iter().map(|l| l.get_type()).collect();
        assert_eq!(distinct_types.len(), 1, "POSTPARSER_ANY_LITERALS_MIXED_TYPES");
        let explicit_type = literals.first().unwrap().get_type();
        let mut child_lidb = lidb.child();
        let result_rune = RuneUsage {
          range: PostParser::eval_range(file, *range),
          rune: scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val()))),
        };
        builder.push(IRulexSR::OneOf(OneOfSR {
          range: PostParser::eval_range(file, *range),
          rune: result_rune.clone(),
          literals: scout_arena.alloc_slice_from_vec(literals),
        }));
        rune_to_explicit_type.push((result_rune.rune.clone(), explicit_type));
        result_rune
      } else {
        panic!("POSTPARSER_TRANSLATE_RULEX_BUILTINCALL_NOT_YET_IMPLEMENTED")
      }
    }
    IRulexPR::Components(ComponentsPR {
      range,
      container: tyype,
      components,
    }) => {
      let mut rune_child_lidb = lidb.child();
      let rune = RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune: scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(rune_child_lidb.borrow_val()))),
      };
      rune_to_explicit_type.push((rune.rune.clone(), translate_type(scout_arena, *tyype)));
      match tyype {
        ITypePR::CoordType => {
          // vregionmut() // Put back in with regions
          // if (componentsP.size != 3) {
          //   vfail("Ref rule should have three components! Found: " + componentsP.size)
          // }
          if components.len() != 2 {
            panic!("POSTPARSER_COMPONENTS_REF_SHOULD_HAVE_TWO_COMPONENTS")
          }
          // vregionmut() // Put back in with regions
          // val Vector(ownershipRuneS, regionRuneS, kindRuneS) =
          let mut translate_child_lidb = lidb.child();
          let component_usages = translate_rulexes(
            scout_arena,
            keywords,
            env,
            &mut translate_child_lidb,
            builder,
            rune_to_explicit_type,
            context_region,
            components,
          );
          let ownership_rune = component_usages[0].clone();
          let kind_rune = component_usages[1].clone();
          builder.push(IRulexSR::CoordComponents(CoordComponentsSR {
            range: PostParser::eval_range(file, *range),
            result_rune: rune.clone(),
            ownership_rune,
            kind_rune,
          }));
        }
        ITypePR::KindType => {
          if components.len() != 1 {
            panic!("Kind rule should have one component! Found: {}", components.len())
          }
          let mut translate_child_lidb = lidb.child();
          let component_usages = translate_rulexes(
            scout_arena,
            keywords,
            env,
            &mut translate_child_lidb,
            builder,
            rune_to_explicit_type,
            context_region,
            components,
          );
          let mutability_rune = component_usages[0].clone();
          builder.push(IRulexSR::KindComponents(KindComponentsSR {
            range: PostParser::eval_range(file, *range),
            kind_rune: rune.clone(),
            mutability_rune,
          }));
        }
        ITypePR::PrototypeType => {
          if components.len() != 2 {
            panic!("Prot rule should have two components! Found: {}", components.len())
          }
          let mut translate_child_lidb = lidb.child();
          let component_usages = translate_rulexes(
            scout_arena,
            keywords,
            env,
            &mut translate_child_lidb,
            builder,
            rune_to_explicit_type,
            context_region,
            components,
          );
          let params_rune = component_usages[0].clone();
          let return_rune = component_usages[1].clone();
          builder.push(IRulexSR::PrototypeComponents(PrototypeComponentsSR {
            range: PostParser::eval_range(file, *range),
            result_rune: rune.clone(),
            params_rune,
            return_rune,
          }));
  
        }
        _ => panic!("POSTPARSER_COMPONENTS_INVALID_TYPE_FOR_COMPONENTS_RULE"),
      }
      rune
    }
    _ => panic!("POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED"),
  }
}

/*
  def translateRulex(
    env: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    builder: ArrayBuffer[IRulexSR],
    runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
    contextRegion: IRuneS,
    rulex: IRulexPR):
  RuneUsage = {
    val evalRange = (range: RangeL) => PostParser.evalRange(env.file, range)

    rulex match {
      case EqualsPR(range, leftP, rightP) => {
        val rune = ImplicitRuneS(lidb.child().consume())
        builder +=
          rules.EqualsSR(
            evalRange(range),
            translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, leftP),
            translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, rightP))
        rules.RuneUsage(evalRange(range), rune)
      }
      case OrPR(range, possibilitiesP) => {
        val rune = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))

        val values =
          possibilitiesP
            .map({
              case TemplexPR(templex) => {
                templexScout.translateValueTemplex(templex) match {
                  case None => vfail("Or rules can only contain values for their possibilities.")
                  case Some(x) => x
                }
              }
              case _ => vfail("Or rules can only contain values for their possibilities.")
            })

        builder += rules.OneOfSR(evalRange(range), rune, values.toVector)
        rune
      }
      case ComponentsPR(range, tyype, componentsP) => {
        val rune = RuneUsage(PostParser.evalRange(env.file, range), ImplicitRuneS(lidb.child().consume()))
        runeToExplicitType += ((rune.rune, translateType(tyype)))
        tyype match {
          case CoordTypePR => {
            vregionmut() // Put back in with regions
            // if (componentsP.size != 3) {
            //   vfail("Ref rule should have three components! Found: " + componentsP.size)
            // }
            if (componentsP.size != 2) {
              vfail("Ref rule should have two components! Found: " + componentsP.size)
            }
            vregionmut() // Put back in with regions
            // val Vector(ownershipRuneS, regionRuneS, kindRuneS) =
            val Vector(ownershipRuneS, kindRuneS) =
              translateRulexes(env, lidb.child(), builder, runeToExplicitType, contextRegion, componentsP)
            builder +=
              CoordComponentsSR(
                PostParser.evalRange(env.file, range),
                rune,
                ownershipRuneS,
                kindRuneS)
          }
          case KindTypePR => {
            if (componentsP.size != 1) {
              vfail("Kind rule should have one component! Found: " + componentsP.size)
            }
            val Vector(mutabilityRuneS) =
              translateRulexes(
                env, lidb.child(), builder, runeToExplicitType, contextRegion, componentsP)
            builder +=
              KindComponentsSR(
                PostParser.evalRange(env.file, range),
                rune,
                mutabilityRuneS)
          }
          case PrototypeTypePR => {
            if (componentsP.size != 2) {
              vfail("Prot rule should have two components! Found: " + componentsP.size)
            }
            val Vector(paramsRuneS, returnRuneS) =
              translateRulexes(
                env, lidb.child(), builder, runeToExplicitType, contextRegion, componentsP)
            builder +=
              PrototypeComponentsSR(
                PostParser.evalRange(env.file, range),
                rune,
                paramsRuneS,
                returnRuneS)
          }
          case _ => {
            vfail("Invalid type for compnents rule: " + tyype)
          }
        }
        rune
      }
      case TypedPR(range, None, tyype) => {
        val rune = ImplicitRuneS(lidb.child().consume())
        runeToExplicitType += ((rune, translateType(tyype)))
        rules.RuneUsage(evalRange(range), rune)
      }
      case TypedPR(range, Some(NameP(_, runeName)), tyype) => {
        val rune = CodeRuneS(runeName)
        runeToExplicitType += ((rune, translateType(tyype)))
        rules.RuneUsage(evalRange(range), rune)
      }
      case TemplexPR(templex) => {
        templexScout.translateTemplex(env, lidb.child(), builder, contextRegion, templex)
      }
      case BuiltinCallPR(range, name, args) => {
        if (name.str == keywords.IS_INTERFACE) {
          vassert(args.length == 1)
          val argRune = translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, args.head)

  //        val resultRune = ImplicitRuneS(lidb.child().consume())
          builder += IsInterfaceSR(evalRange(range), argRune)
  //        runeToExplicitType.put(resultRune, KindTemplataType())
          runeToExplicitType += ((argRune.rune, KindTemplataType()))

          rules.RuneUsage(evalRange(range), argRune.rune)
        } else if (name.str == keywords.IMPLEMENTS) {
          vassert(args.length == 2)
          val Vector(structRule, interfaceRule) = args
          val structRune = translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, structRule)
          runeToExplicitType += ((structRune.rune, CoordTemplataType()))
          val interfaceRune = translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, interfaceRule)
          runeToExplicitType += ((interfaceRune.rune, CoordTemplataType()))

          val resultRuneS = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))
          runeToExplicitType += ((resultRuneS.rune, ImplTemplataType()))

          // Only appears in definition; filtered out when solving call site
          builder += rules.DefinitionCoordIsaSR(evalRange(range), resultRuneS, structRune, interfaceRune)
          // Only appears in call site; filtered out when solving definition
          builder += rules.CallSiteCoordIsaSR(evalRange(range), Some(resultRuneS), structRune, interfaceRune)

          rules.RuneUsage(evalRange(range), structRune.rune)
        } else if (name.str == keywords.REF_LIST_COMPOUND_MUTABILITY) {
          vassert(args.length == 1)
          val argRune = translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, args.head)

          val resultRune = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))
          builder += RefListCompoundMutabilitySR(evalRange(range), resultRune, argRune)
          runeToExplicitType += ((resultRune.rune, MutabilityTemplataType()))
          runeToExplicitType += ((argRune.rune, PackTemplataType(CoordTemplataType())))

          rules.RuneUsage(evalRange(range), resultRune.rune)
        } else if (name.str == keywords.Refs) {
          val argRunes =
            args.map(arg => {
              translateRulex(env, lidb.child(), builder, runeToExplicitType, contextRegion, arg)
            })

          val resultRune = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))
          builder += rules.PackSR(evalRange(range), resultRune, argRunes.toVector)
          runeToExplicitType += ((resultRune.rune, PackTemplataType(CoordTemplataType())))

          rules.RuneUsage(evalRange(range), resultRune.rune)
        } else if (name.str == keywords.ANY) {
          val literals: Vector[ILiteralSL] =
            args.map({
              case TemplexPR(IntPT(_, i)) => IntLiteralSL(i)
              case TemplexPR(OwnershipPT(_, i)) => OwnershipLiteralSL(i)
              case other => vimpl(other)
            }).toVector

          val resultRune = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))
          builder += rules.OneOfSR(evalRange(range), resultRune, literals)
          runeToExplicitType += ((resultRune.rune, vassertOne(literals.map(_.getType()).distinct)))

          rules.RuneUsage(evalRange(range), resultRune.rune)
        } else {
          throw new CompileErrorExceptionS(UnknownRuleFunctionS(evalRange(range), name.str.str))
        }
      }
    }
  }
*/
/*
}

object RuleScout {
*/
pub fn translate_type<'s>(scout_arena: &ScoutArena<'s>, tyype: ITypePR) -> ITemplataType<'s> {
  match tyype {
    ITypePR::PrototypeType => ITemplataType::PrototypeTemplataType(PrototypeTemplataType {}),
    ITypePR::IntType => ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
    ITypePR::BoolType => ITemplataType::BooleanTemplataType(BooleanTemplataType {}),
    ITypePR::OwnershipType => ITemplataType::OwnershipTemplataType(OwnershipTemplataType {}),
    ITypePR::MutabilityType => ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
    ITypePR::VariabilityType => ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}),
    ITypePR::LocationType => ITemplataType::LocationTemplataType(LocationTemplataType {}),
    ITypePR::CoordType => ITemplataType::CoordTemplataType(CoordTemplataType {}),
    ITypePR::CoordListType => ITemplataType::PackTemplataType(PackTemplataType {
      element_type: scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})),
    }),
    ITypePR::KindType => ITemplataType::KindTemplataType(KindTemplataType {}),
    ITypePR::RegionType => ITemplataType::RegionTemplataType(RegionTemplataType {}),
    ITypePR::CitizenTemplateType => {
      panic!("POSTPARSER_TRANSLATE_TYPE_CITIZEN_TEMPLATE_NOT_YET_IMPLEMENTED")
    }
  }
}
/*
  def translateType(tyype: ITypePR): ITemplataType = {
    tyype match {
      case PrototypeTypePR => PrototypeTemplataType()
      case IntTypePR => IntegerTemplataType()
      case BoolTypePR => BooleanTemplataType()
      case OwnershipTypePR => OwnershipTemplataType()
      case MutabilityTypePR => MutabilityTemplataType()
      case VariabilityTypePR => VariabilityTemplataType()
      case LocationTypePR => LocationTemplataType()
      case CoordTypePR => CoordTemplataType()
      case CoordListTypePR => PackTemplataType(CoordTemplataType())
      case KindTypePR => KindTemplataType()
      case RegionTypePR => RegionTemplataType()
    }
  }
*/
fn get_rune_kind_template<'s>(
  _rules_s: &[IRulexSR<'s>],
  _rune: IRuneS<'s>,
) -> IImpreciseNameS<'s> {
  panic!("Unimplemented get_rune_kind_template");
}
/*
  // Gets the template name (or the kind name if not template)
  def getRuneKindTemplate(rulesS: IndexedSeq[IRulexSR], rune: IRuneS) = {
    val equivalencies = new Equivalencies(rulesS)
    val structKindEquivalentRunes = equivalencies.getKindEquivalentRunes(rune)
    val templateRunes =
      rulesS.collect({
        case MaybeCoercingCallSR(_, resultRune, templateRune, _)
          if structKindEquivalentRunes.contains(resultRune.rune) => templateRune.rune
      })
    val runesToLookFor = structKindEquivalentRunes ++ templateRunes
    val templateNames =
      rulesS.collect({
        case MaybeCoercingLookupSR(_, rune, name) if runesToLookFor.contains(rune.rune) => name
      })
    vassert(templateNames.nonEmpty)
    vcurious(templateNames.size == 1)
    val templateName = templateNames.head
    templateName
  }
*/
/*
}
*/
struct Equivalencies<'s> {
  rune_to_kind_equivalent_runes: HashMap<IRuneS<'s>, HashSet<IRuneS<'s>>>,
}
/*
class Equivalencies(rules: IndexedSeq[IRulexSR]) {
  val runeToKindEquivalentRunes: mutable.HashMap[IRuneS, mutable.HashSet[IRuneS]] = mutable.HashMap()
*/
impl<'s> Equivalencies<'s> {
  fn mark_kind_equivalent(&mut self, rune_a: IRuneS<'s>, rune_b: IRuneS<'s>) {
    self.rune_to_kind_equivalent_runes.entry(rune_a).or_default().insert(rune_b);
    self.rune_to_kind_equivalent_runes.entry(rune_b).or_default().insert(rune_a);
  }
/*
  def markKindEquivalent(runeA: IRuneS, runeB: IRuneS): Unit = {
    runeToKindEquivalentRunes.getOrElseUpdate(runeA, mutable.HashSet()) += runeB
    runeToKindEquivalentRunes.getOrElseUpdate(runeB, mutable.HashSet()) += runeA
  }
*/
  fn new(rules_s: &[IRulexSR<'s>]) -> Self {
    let mut this = Self { rune_to_kind_equivalent_runes: HashMap::new() };
    for rule in rules_s {
      match rule {
        IRulexSR::CoordComponents(r) => this.mark_kind_equivalent(r.result_rune.rune, r.kind_rune.rune),
        IRulexSR::KindComponents(_) => {}
        IRulexSR::Equals(r) => this.mark_kind_equivalent(r.left.rune, r.right.rune),
        IRulexSR::Call(_) => {}
        IRulexSR::MaybeCoercingCall(_) => {}
        IRulexSR::CallSiteCoordIsa(_) => {}
        IRulexSR::DefinitionCoordIsa(_) => {}
        IRulexSR::CoordSend(_) => {}
        IRulexSR::Augment(r) => this.mark_kind_equivalent(r.result_rune.rune, r.inner_rune.rune),
        IRulexSR::Literal(_) => {}
        IRulexSR::MaybeCoercingLookup(_) => {}
        IRulexSR::CoerceToCoord(r) => this.mark_kind_equivalent(r.coord_rune.rune, r.kind_rune.rune),
        IRulexSR::OneOf(_) => {}
        IRulexSR::CallSiteFunc(_) => {}
        IRulexSR::DefinitionFunc(_) => {}
        IRulexSR::Resolve(_) => {}
        IRulexSR::Pack(_) => {}
        IRulexSR::PrototypeComponents(_) => {}
        IRulexSR::RefListCompoundMutability(_) => {}
        _ => panic!("implement: Equivalencies::new unhandled rule"),
      }
    }
    this
  }
/*
  rules.foreach({
    case CoordComponentsSR(_, resultRune, _, kindRune) => markKindEquivalent(resultRune.rune, kindRune.rune)
    case KindComponentsSR(_, resultRune, _) =>
    case EqualsSR(_, left, right) => markKindEquivalent(left.rune, right.rune)
    case CallSR(range, resultRune, templateRune, args) =>
    case MaybeCoercingCallSR(range, resultRune, templateRune, args) =>
    case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) =>
    case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) =>
    case CoordSendSR(range, senderRune, receiverRune) =>
    case AugmentSR(range, resultRune, ownership, innerRune) => markKindEquivalent(resultRune.rune, innerRune.rune)
    case LiteralSR(range, rune, literal) =>
    case MaybeCoercingLookupSR(range, rune, name) =>
    case CoerceToCoordSR(range, coordRune, kindRune) => markKindEquivalent(coordRune.rune, kindRune.rune)
//    case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) =>
//    case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) =>
    case OneOfSR(range, rune, literals) =>
    case CallSiteFuncSR(range, resultRune, nameRune, paramsListRune, returnRune) =>
    case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) =>
    case ResolveSR(range, resultRune, name, paramsListRune, returnRune) =>
    case PackSR(range, resultRune, members) =>
    case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) =>
    case RefListCompoundMutabilitySR(range, resultRune, coordListRune) =>
    case other => vimpl(other)
  })
*/
  fn find_transitively_equivalent_into(
    &self,
    found_so_far: &mut HashSet<IRuneS<'s>>,
    rune: IRuneS<'s>,
  ) {
    let equivalents: Vec<IRuneS<'s>> = self.rune_to_kind_equivalent_runes
      .get(&rune)
      .map(|s| s.iter().copied().collect())
      .unwrap_or_default();
    for r in equivalents {
      if !found_so_far.contains(&r) {
        found_so_far.insert(r);
        self.find_transitively_equivalent_into(found_so_far, r);
      }
    }
  }
/*
  private def findTransitivelyEquivalentInto(foundSoFar: mutable.HashSet[IRuneS], rune: IRuneS): Unit = {
    runeToKindEquivalentRunes.getOrElse(rune, Vector()).foreach(r => {
      if (!foundSoFar.contains(r)) {
        foundSoFar += r
        findTransitivelyEquivalentInto(foundSoFar, r)
      }
    })
  }
*/
  fn get_kind_equivalent_runes(&self, rune: IRuneS<'s>) -> HashSet<IRuneS<'s>> {
    let mut set = HashSet::new();
    set.insert(rune);
    self.find_transitively_equivalent_into(&mut set, rune);
    set
  }
/*
  // MIGALLOW: getKindEquivalentRunes -> get_kind_equivalent_runes
  def getKindEquivalentRunes(rune: IRuneS): Set[IRuneS] = {
    val set = mutable.HashSet[IRuneS]()
    set += rune
    findTransitivelyEquivalentInto(set, rune)
    set.toSet
  }
*/
  fn get_kind_equivalent_runes_iter<I>(&self, runes: I) -> HashSet<IRuneS<'s>>
  where
    I: Iterator<Item = IRuneS<'s>>,
  {
    runes.flat_map(|r| self.get_kind_equivalent_runes(r)).collect()
  }
}
/*
  // MIGALLOW: getKindEquivalentRunes -> get_kind_equivalent_runes_iter
  def getKindEquivalentRunes(runes: Iterable[IRuneS]): Set[IRuneS] = {
    runes
      .map(getKindEquivalentRunes)
      .foldLeft(Set[IRuneS]())(_ ++ _)
  }
*/
/*
}
*/

// Rust adaptation: callers in the typing pass have `&[IRulexSR<'s>]` but no
// `Equivalencies` instance (Scala constructed one ad-hoc at each call site:
// `new Equivalencies(rulesS).getKindEquivalentRunes(runes)`). This free fn
// preserves the call-site shape; it constructs the Equivalencies internally
// and delegates.
pub fn get_kind_equivalent_runes_iter<'s, I>(
  rules_s: &[IRulexSR<'s>],
  runes: I,
) -> HashSet<IRuneS<'s>>
where
  I: Iterator<Item = IRuneS<'s>>,
{
  Equivalencies::new(rules_s).get_kind_equivalent_runes_iter(runes)
}
/* */