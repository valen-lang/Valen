/*
package dev.vale.postparsing.rules

import dev.vale.lexing.RangeL
import dev.vale.parsing.ast._
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, vassert, vassertSome, vimpl}
import dev.vale.postparsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing._

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
*/
/*
class TemplexScout(
    interner: Interner,
  keywords: Keywords) {
*/

use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::{
  BoolPT, IntPT, ITemplexPT, ITemplexPT::NameOrRune, LocationPT, MutabilityPT, NameOrRunePT,
  NameP, OwnershipPT, RegionRunePT, StringPT, VariabilityPT,
};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, IImpreciseNameS, IImpreciseNameValS::CodeName, ImplicitRuneS, IRuneS,
};
use crate::postparsing::names::IRuneValS::{CodeRune, ImplicitRune};
use crate::postparsing::post_parser::{IEnvironmentS, PostParser};
use crate::postparsing::rules::rules::IRulexSR::{Lookup, MaybeCoercingCall, MaybeCoercingLookup};
use crate::postparsing::rules::rules::{
  BoolLiteralSL, ILiteralSL, IntLiteralSL, IRulexSR, LiteralSR, LocationLiteralSL, LookupSR,
  MaybeCoercingCallSR, MaybeCoercingLookupSR, MutabilityLiteralSL, OwnershipLiteralSL,
  RuneParentEnvLookupSR, RuneUsage, StringLiteralSL, VariabilityLiteralSL,
};
use crate::utils::range::RangeS;
use std::collections::HashMap;

fn add_literal_rule<'a>(
  interner: &Interner<'a>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  range_s: RangeS<'a>,
  value_sr: ILiteralSL,
) -> RuneUsage<'a> {
  let mut child_lidb = lidb.child();
  let rune_s = RuneUsage {
    range: range_s.clone(),
    rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
      lid: child_lidb.consume(),
    })),
  };
  rule_builder.push(IRulexSR::Literal(LiteralSR {
    range: range_s,
    rune: rune_s.clone(),
    literal: value_sr,
  }));
  rune_s
}
/*
  def addLiteralRule(
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    rangeS: RangeS,
    valueSR: ILiteralSL):
  RuneUsage = {
    val runeS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
    ruleBuilder += LiteralSR(rangeS, runeS, valueSR)
    runeS
  }
*/
fn add_rune_parent_env_lookup_rule<'a>(
  _lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  range_s: RangeS<'a>,
  rune_s: IRuneS<'a>,
) -> RuneUsage<'a> {
  let usage = RuneUsage {
    range: range_s.clone(),
    rune: rune_s,
  };
  rule_builder.push(IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR {
      range: range_s,
      rune: usage.clone(),
    },
  ));
  usage
}
/*
  def addRuneParentEnvLookupRule(
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    rangeS: RangeS,
    runeS: IRuneS):
  RuneUsage = {
    val usage = rules.RuneUsage(rangeS, runeS)
    ruleBuilder += RuneParentEnvLookupSR(rangeS, usage)
    usage
  }
*/
fn add_lookup_rule<'a>(
  interner: &Interner<'a>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  range_s: RangeS<'a>,
  // Nearest enclosing region marker, see RADTGCA.
  _context_region: IRuneS<'a>,
  name_sn: IImpreciseNameS<'a>,
) -> RuneUsage<'a> {
  let mut child_lidb = lidb.child();
  let rune_s = RuneUsage {
    range: range_s.clone(),
    rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
      lid: child_lidb.consume(),
    })),
  };
  rule_builder.push(MaybeCoercingLookup(MaybeCoercingLookupSR {
    range: range_s,
    rune: rune_s.clone(),
    name: name_sn,
  }));
  rune_s
}
/*
  def addLookupRule(
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    rangeS: RangeS,
    contextRegion: IRuneS, // Nearest enclosing region marker, see RADTGCA.
    nameSN: IImpreciseNameS):
  RuneUsage = {
    val runeS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
    ruleBuilder += rules.MaybeCoercingLookupSR(rangeS, runeS, nameSN)
    runeS
  }
*/
fn translate_value_templex<'a, 'p>(
  templex: &ITemplexPT<'a, 'p>,
) -> Option<ILiteralSL> {
  match templex {
    ITemplexPT::Int(IntPT { value, .. }) => Some(ILiteralSL::IntLiteral(IntLiteralSL {
      value: *value,
    })),
    ITemplexPT::Bool(BoolPT { value, .. }) => Some(ILiteralSL::BoolLiteral(BoolLiteralSL {
      value: *value,
    })),
    ITemplexPT::Mutability(MutabilityPT(_, mutability)) => Some(ILiteralSL::MutabilityLiteral(
      MutabilityLiteralSL {
        mutability: *mutability,
      },
    )),
    ITemplexPT::Variability(VariabilityPT(_, variability)) => Some(ILiteralSL::VariabilityLiteral(
      VariabilityLiteralSL {
        variability: *variability,
      },
    )),
    ITemplexPT::String(StringPT { str, .. }) => Some(ILiteralSL::StringLiteral(
      StringLiteralSL {
        value: str.clone(),
      },
    )),
    ITemplexPT::Location(LocationPT { location, .. }) => Some(ILiteralSL::LocationLiteral(
      LocationLiteralSL {
        location: *location,
      },
    )),
    ITemplexPT::Ownership(OwnershipPT(_, ownership)) => Some(ILiteralSL::OwnershipLiteral(
      OwnershipLiteralSL {
        ownership: *ownership,
      },
    )),
    _ => None,
  }
}
/*
  def translateValueTemplex(templex: ITemplexPT): Option[ILiteralSL] = {
    templex match {
      case IntPT(_, value) => Some(IntLiteralSL(value))
      case BoolPT(_, value) => Some(BoolLiteralSL(value))
      case MutabilityPT(_, mutability) => Some(MutabilityLiteralSL(mutability))
      case VariabilityPT(_, variability) => Some(VariabilityLiteralSL(variability))
      case StringPT(_, value) => Some(StringLiteralSL(value))
      case LocationPT(_, location) => Some(LocationLiteralSL(location))
      case OwnershipPT(_, ownership) => Some(OwnershipLiteralSL(ownership))
      case _ => None
    }
  }
*/
// Returns:
// - Rune for this type
pub fn translate_templex<'a, 'p>(
  interner: &Interner<'a>,
  keywords: &Keywords<'a>,
  env: IEnvironmentS<'a>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  // Nearest enclosing region marker, see RADTGCA.
  context_region: IRuneS<'a>,
  templex: &ITemplexPT<'a, 'p>,
) -> RuneUsage<'a> {
  let file = env.file();
  match translate_value_templex(templex) {
    Some(x) => {
      let mut child_lidb = lidb.child();
      add_literal_rule(
        interner,
        &mut child_lidb,
        rule_builder,
        PostParser::eval_range(file, templex.range()),
        x,
      )
    }
    None => match templex {
      ITemplexPT::Inline(inline) => translate_templex(
        interner,
        keywords,
        env,
        lidb,
        rule_builder,
        context_region,
        inline.inner,
      ),
      ITemplexPT::AnonymousRune(anonymous_rune) => {
        let mut child_lidb = lidb.child();
        let rune = RuneUsage {
          range: PostParser::eval_range(file, anonymous_rune.range),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        rune
      }
      ITemplexPT::RegionRune(RegionRunePT {
        range: _,
        name: None,
      }) => panic!("POSTPARSER_TRANSLATE_TEMPLEX_REGION_RUNE_NONE_NOT_YET_IMPLEMENTED"),
      ITemplexPT::RegionRune(RegionRunePT {
        range,
        name: Some(name),
      }) => {
        let is_rune_from_local_env = env.local_declared_runes().contains(
          &interner.intern_rune(CodeRune(CodeRuneS { name: name.str() })),
        );
        if is_rune_from_local_env {
          RuneUsage {
            range: PostParser::eval_range(file, *range),
            rune: interner.intern_rune(CodeRune(CodeRuneS { name: name.str() })),
          }
        } else {
          // It's from a parent env
          let mut child_lidb = lidb.child();
          add_rune_parent_env_lookup_rule(
            &mut child_lidb,
            rule_builder,
            PostParser::eval_range(file, *range),
            interner.intern_rune(CodeRune(CodeRuneS { name: name.str() })),
          )
        }
      }
      ITemplexPT::NameOrRune(NameOrRunePT(name_or_rune)) => {
        let is_rune_from_env = env.all_declared_runes().contains(&interner.intern_rune(CodeRune(
          CodeRuneS {
            name: name_or_rune.str(),
          },
        )));
        if is_rune_from_env {
          let is_rune_from_local_env = env.local_declared_runes().contains(
            &interner.intern_rune(CodeRune(CodeRuneS {
              name: name_or_rune.str(),
            })),
          );
          if is_rune_from_local_env {
            RuneUsage {
              range: PostParser::eval_range(file, name_or_rune.range()),
              rune: interner.intern_rune(CodeRune(CodeRuneS {
                name: name_or_rune.str(),
              })),
            }
          } else {
            // It's from a parent env
            let mut child_lidb = lidb.child();
            add_rune_parent_env_lookup_rule(
              &mut child_lidb,
              rule_builder,
              PostParser::eval_range(file, name_or_rune.range()),
              interner.intern_rune(CodeRune(CodeRuneS {
                name: name_or_rune.str(),
              })),
            )
          }
        } else {
          // e.g. "int"
          let name = interner.intern_imprecise_name(CodeName(CodeNameS {
            name: name_or_rune.str(),
          }));
          let mut child_lidb = lidb.child();
          add_lookup_rule(
            interner,
            &mut child_lidb,
            rule_builder,
            PostParser::eval_range(file, name_or_rune.range()),
            context_region,
            name,
          )
          // For lookups like these, we bring them into the current region.
        }
      }
      ITemplexPT::Interpreted(_interpreted) => {
        panic!("POSTPARSER_TRANSLATE_TEMPLEX_INTERPRETED_NOT_YET_IMPLEMENTED")
      }
      ITemplexPT::Call(_call) => panic!("POSTPARSER_TRANSLATE_TEMPLEX_CALL_NOT_YET_IMPLEMENTED"),
      ITemplexPT::Function(_function) => {
        panic!("POSTPARSER_TRANSLATE_TEMPLEX_FUNCTION_NOT_YET_IMPLEMENTED")
      }
      ITemplexPT::Func(_func) => panic!("POSTPARSER_TRANSLATE_TEMPLEX_FUNC_NOT_YET_IMPLEMENTED"),
      ITemplexPT::Pack(_pack) => panic!("POSTPARSER_TRANSLATE_TEMPLEX_PACK_NOT_YET_IMPLEMENTED"),
      ITemplexPT::StaticSizedArray(static_sized_array) => {
        let range_s = PostParser::eval_range(file, static_sized_array.range);
        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        let mut child_lidb = lidb.child();
        let template_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        rule_builder.push(Lookup(LookupSR {
          range: range_s.clone(),
          rune: template_rune_s.clone(),
          name: interner.intern_imprecise_name(CodeName(CodeNameS {
            name: keywords.static_array,
          })),
        }));
        let mut child_lidb = lidb.child();
        let size_rune_s = translate_templex(
          interner,
          keywords,
          env.clone(),
          &mut child_lidb,
          rule_builder,
          context_region.clone(),
          static_sized_array.size,
        );
        let mut child_lidb = lidb.child();
        let mutability_rune_s = translate_templex(
          interner,
          keywords,
          env.clone(),
          &mut child_lidb,
          rule_builder,
          context_region.clone(),
          static_sized_array.mutability,
        );
        let mut child_lidb = lidb.child();
        let variability_rune_s = translate_templex(
          interner,
          keywords,
          env.clone(),
          &mut child_lidb,
          rule_builder,
          context_region.clone(),
          static_sized_array.variability,
        );
        let mut child_lidb = lidb.child();
        let element_rune_s = translate_templex(
          interner,
          keywords,
          env,
          &mut child_lidb,
          rule_builder,
          context_region,
          static_sized_array.element,
        );
        rule_builder.push(MaybeCoercingCall(MaybeCoercingCallSR {
          range: range_s,
          result_rune: result_rune_s.clone(),
          template_rune: template_rune_s,
          args: vec![size_rune_s, mutability_rune_s, variability_rune_s, element_rune_s],
        }));
        result_rune_s
      }
      ITemplexPT::RuntimeSizedArray(runtime_sized_array) => {
        let range_s = PostParser::eval_range(file, runtime_sized_array.range);
        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        let mut child_lidb = lidb.child();
        let template_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        rule_builder.push(Lookup(LookupSR {
          range: range_s.clone(),
          rune: template_rune_s.clone(),
          name: interner.intern_imprecise_name(CodeName(CodeNameS {
            name: keywords.array,
          })),
        }));
        let mut child_lidb = lidb.child();
        let mutability_rune_s = translate_templex(
          interner,
          keywords,
          env.clone(),
          &mut child_lidb,
          rule_builder,
          context_region.clone(),
          runtime_sized_array.mutability,
        );
        let mut child_lidb = lidb.child();
        let element_rune_s = translate_templex(
          interner,
          keywords,
          env,
          &mut child_lidb,
          rule_builder,
          context_region,
          runtime_sized_array.element,
        );
        rule_builder.push(MaybeCoercingCall(MaybeCoercingCallSR {
          range: range_s,
          result_rune: result_rune_s.clone(),
          template_rune: template_rune_s,
          args: vec![mutability_rune_s, element_rune_s],
        }));
        result_rune_s
      }
      ITemplexPT::Tuple(tuple) => {
        let range_s = PostParser::eval_range(file, tuple.range);
        let mut child_lidb = lidb.child();
        let result_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        let mut child_lidb = lidb.child();
        let template_rune_s = RuneUsage {
          range: range_s.clone(),
          rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
            lid: child_lidb.consume(),
          })),
        };
        rule_builder.push(MaybeCoercingLookup(MaybeCoercingLookupSR {
          range: range_s.clone(),
          rune: template_rune_s.clone(),
          name: interner.intern_imprecise_name(CodeName(CodeNameS {
            name: keywords.tuple_human_name[tuple.elements.len()],
          })),
        }));
        let mut element_runes = Vec::<RuneUsage<'a>>::new();
        for element in tuple.elements {
          let mut child_lidb = lidb.child();
          element_runes.push(translate_templex(
            interner,
            keywords,
            env.clone(),
            &mut child_lidb,
            rule_builder,
            context_region.clone(),
            element,
          ));
        }
        rule_builder.push(MaybeCoercingCall(MaybeCoercingCallSR {
          range: range_s,
          result_rune: result_rune_s.clone(),
          template_rune: template_rune_s,
          args: element_runes,
        }));
        result_rune_s
      }
      _ => panic!("POSTPARSER_TRANSLATE_TEMPLEX_NOT_YET_IMPLEMENTED"),
    },
  }
}
/*
  // Returns:
  // - Rune for this type
  def translateTemplex(
    env: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    contextRegion: IRuneS, // Nearest enclosing region marker, see RADTGCA.
    templex: ITemplexPT):
  RuneUsage = {
    Profiler.frame(() => {
      val evalRange = (range: RangeL) => PostParser.evalRange(env.file, range)

      translateValueTemplex(templex) match {
        case Some(x) => {
          val rune = addLiteralRule(lidb.child(), ruleBuilder, evalRange(templex.range), x)
          rune
        }
        case None => {
          templex match {
            case InlinePT(range, inner) => translateTemplex(env, lidb, ruleBuilder, contextRegion, inner)
            case AnonymousRunePT(range) => {
              val rune = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))
              rune
            }
            case RegionRunePT(range, None) => {
              vimpl() // isolates
            }
            case RegionRunePT(range, Some(NameP(_, name))) => {
              val isRuneFromLocalEnv = env.localDeclaredRunes().contains(CodeRuneS(name))
              if (isRuneFromLocalEnv) {
                val rune = rules.RuneUsage(evalRange(range), CodeRuneS(name))
                rune
              } else {
                // It's from a parent env
                val rune = addRuneParentEnvLookupRule(lidb.child(), ruleBuilder, evalRange(range), CodeRuneS(name))
                rune
              }
            }
            case NameOrRunePT(NameP(range, nameOrRune)) => {
              val isRuneFromEnv = env.allDeclaredRunes().contains(CodeRuneS(nameOrRune))
              if (isRuneFromEnv) {
                val isRuneFromLocalEnv = env.localDeclaredRunes().contains(CodeRuneS(nameOrRune))
                if (isRuneFromLocalEnv) {
                  val rune = rules.RuneUsage(evalRange(range), CodeRuneS(nameOrRune))
                  rune
                } else {
                  // It's from a parent env
                  val rune = addRuneParentEnvLookupRule(lidb.child(), ruleBuilder, evalRange(range), CodeRuneS(nameOrRune))
                  rune
                }
              } else {
                // e.g. "int"
                val name = interner.intern(CodeNameS(nameOrRune))
                val rune = addLookupRule(lidb.child(), ruleBuilder, evalRange(range), contextRegion, name)
                // For lookups like these, we bring them into the current region.
                rune
              }
            }
            case InterpretedPT(range, ownership, maybeRegion, innerP) => {
              val rangeS = evalRange(range)
              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))

              val maybeRegionRune =
                maybeRegion.map(runeName => {
                  val rune = CodeRuneS(vassertSome(runeName.name).str) // impl isolates
                  if (!env.allDeclaredRunes().contains(rune)) {
                    throw CompileErrorExceptionS(UnknownRegionError(rangeS, rune.name.str))
                  }
                  rules.RuneUsage(evalRange(range), rune)
                })

              // We need to use region as the new context region for everything under us, since
              // region annotations apply deeply.
              val newRegion =
                maybeRegionRune match {
                  case None => contextRegion
                  case Some(rune) => rune.rune
                }

              val innerRuneS =
                translateTemplex(env, lidb.child(), ruleBuilder, newRegion, innerP)

              ruleBuilder += rules.AugmentSR(evalRange(range), resultRuneS, ownership.map(_.ownership), innerRuneS)

              resultRuneS
            }
            case CallPT(rangeP, template, args) => {
              val rangeS = evalRange(rangeP)
              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder +=
                rules.MaybeCoercingCallSR(
                  rangeS,
                  resultRuneS,
                  translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, template),
                  args.map(translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, _)))
              resultRuneS
            }
            case FunctionPT(rangeP, mutability, paramsPack, returnType) => {
              val rangeS = evalRange(rangeP)
              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              val templateNameRuneS =
                addLookupRule(
                  lidb.child(), ruleBuilder, rangeS, contextRegion, interner.intern(CodeNameS(keywords.IFUNCTION)))
              val mutabilityRuneS =
                mutability match {
                  case None => addLiteralRule(lidb.child(), ruleBuilder, rangeS, rules.MutabilityLiteralSL(MutableP))
                  case Some(m) => translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, m)
                }
              ruleBuilder +=
                rules.MaybeCoercingCallSR(
                  rangeS,
                  resultRuneS,
                  templateNameRuneS,
                  Vector(
                    mutabilityRuneS,
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion,  paramsPack),
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, returnType)))
              resultRuneS
            }
            case FuncPT(range, NameP(nameRange, name), paramsRangeL, paramsP, returnTypeP) => {
              val rangeS = PostParser.evalRange(env.file, range)
              val paramsRangeS = PostParser.evalRange(env.file, paramsRangeL)
              val paramsS =
                paramsP.map(paramP => {
                  translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, paramP)
                })
              val paramListRuneS = rules.RuneUsage(paramsRangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder += PackSR(paramsRangeS, paramListRuneS, paramsS.toVector)

              val returnRuneS = translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, returnTypeP)

              val resultRuneS = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))

              // Only appears in call site; filtered out when solving definition
              ruleBuilder += CallSiteFuncSR(rangeS, resultRuneS, name, paramListRuneS, returnRuneS)
              // Only appears in definition; filtered out when solving call site
              ruleBuilder += DefinitionFuncSR(rangeS, resultRuneS, name, paramListRuneS, returnRuneS)
              // Only appears in call site; filtered out when solving definition
              ruleBuilder += ResolveSR(rangeS, resultRuneS, name, paramListRuneS, returnRuneS)

              resultRuneS
            }
            case PackPT(rangeP, members) => {
              val rangeS = PostParser.evalRange(env.file, rangeP)

              val templateRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder +=
                MaybeCoercingLookupSR(
                  rangeS,
                  templateRuneS,
                  CodeNameS(keywords.tupleHumanName(members.length)))

              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder += MaybeCoercingCallSR(
                rangeS,
                resultRuneS,
                templateRuneS,
                members.map(translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, _)))


              resultRuneS
//              val resultRuneS = rules.RuneUsage(evalRange(range), ImplicitRuneS(lidb.child().consume()))
//              ruleBuilder +=
//                rules.PackSR(
//                  evalRange(range),
//                  resultRuneS,
//                  members.map(translateTemplex(env, lidb.child(), ruleBuilder, _)).toVector)
//              resultRuneS
            }
            case StaticSizedArrayPT(rangeP, mutability, variability, size, element) => {
              val rangeS = evalRange(rangeP)
              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              val templateRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder +=
                rules.LookupSR(
                  rangeS,
                  templateRuneS,
                  interner.intern(CodeNameS(keywords.StaticArray)))
              ruleBuilder +=
                MaybeCoercingCallSR(
                  rangeS,
                  resultRuneS,
                  templateRuneS,
                  Vector(
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, size),
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, mutability),
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, variability),
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, element)))
              resultRuneS
            }
            case RuntimeSizedArrayPT(rangeP, mutability, element) => {
              val rangeS = evalRange(rangeP)
              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              val templateRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder +=
                rules.LookupSR(
                  rangeS,
                  templateRuneS,
                  interner.intern(CodeNameS(keywords.Array)))
              ruleBuilder +=
                MaybeCoercingCallSR(
                  rangeS,
                  resultRuneS,
                  templateRuneS,
                  Vector(
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, mutability),
                    translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, element)))
              resultRuneS
            }
            case TuplePT(rangeP, elements) => {
              val rangeS = evalRange(rangeP)
              val resultRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              val templateRuneS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
              ruleBuilder +=
                rules.MaybeCoercingLookupSR(
                  rangeS,
                  templateRuneS,
                  interner.intern(CodeNameS(keywords.tupleHumanName(elements.length))))
              ruleBuilder +=
                rules.MaybeCoercingCallSR(
                  rangeS,
                  resultRuneS,
                  templateRuneS,
                  elements.map(translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, _)))
//              ruleBuilder +=
//                rules.CallSR(
//                  evalRange(range),
//                  resultRuneS,
//                  templateRuneS,
//                  Vector(packRuneS))
//              ruleBuilder +=
//                rules.PackSR(
//                  evalRange(range),
//                  packRuneS,
//                  elements.map(translateTemplex(env, lidb.child(), ruleBuilder, _)).toVector)
              resultRuneS
            }
          }
        }
      }
    })
  }
*/
// Returns:
// - Rune for this type
fn translate_type_into_rune<'a, 'p>(
  interner: &Interner<'a>,
  keywords: &Keywords<'a>,
  env: IEnvironmentS<'a>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  // Nearest enclosing region marker, see RADTGCA.
  context_region: IRuneS<'a>,
  type_p: &ITemplexPT<'a, 'p>,
) -> RuneUsage<'a> {
  let file = env.file();
  match type_p {
    NameOrRune(NameOrRunePT(NameP(
      range,
      name_or_rune,
    )))
      if env.all_declared_runes().contains(&interner.intern_rune(CodeRune(CodeRuneS {
        name: *name_or_rune,
      }))) =>
    {
      let result_rune_s = RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune: interner.intern_rune(CodeRune(CodeRuneS {
          name: *name_or_rune,
        })),
      };
      result_rune_s
    }
    non_rune_templex_p => {
      let mut child_lidb = lidb.child();
      translate_templex(
        interner,
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
/*
  // Returns:
  // - Rune for this type
  def translateTypeIntoRune(
    env: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    contextRegion: IRuneS, // Nearest enclosing region marker, see RADTGCA.
    typeP: ITemplexPT):
  RuneUsage = {
    typeP match {
      case NameOrRunePT(NameP(range, nameOrRune)) if env.allDeclaredRunes().contains(CodeRuneS(nameOrRune)) => {
        val resultRuneS = rules.RuneUsage(PostParser.evalRange(env.file, range), CodeRuneS(nameOrRune))
        resultRuneS
      }
      case nonRuneTemplexP => {
        translateTemplex(env, lidb.child(), ruleBuilder, contextRegion, nonRuneTemplexP)
      }
    }
  }
*/
// Returns:
// - Rune for this type
pub fn translate_maybe_type_into_rune<'a, 'p>(
  interner: &Interner<'a>,
  keywords: &Keywords<'a>,
  env: IEnvironmentS<'a>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeS<'a>,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  context_region: IRuneS<'a>,
  maybe_type_p: Option<&ITemplexPT<'a, 'p>>,
) -> RuneUsage<'a> {
  match maybe_type_p {
    None => {
      let mut child_lidb = lidb.child();
      let result_rune_s = RuneUsage {
        range,
        rune: interner.intern_rune(ImplicitRune(ImplicitRuneS {
          lid: child_lidb.consume(),
        })),
      };
      result_rune_s
    }
    Some(type_p) => {
      translate_type_into_rune(interner, keywords, env, lidb, rule_builder, context_region, type_p)
    }
  }
}
/*
  // Returns:
  // - Rune for this type
  def translateMaybeTypeIntoRune(
    env: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    range: RangeS,
    ruleBuilder: ArrayBuffer[IRulexSR],
    contextRegion: IRuneS, // Nearest enclosing region marker, see RADTGCA.
    maybeTypeP: Option[ITemplexPT]):
  RuneUsage = {
    maybeTypeP match {
      case None => {
        val resultRuneS = rules.RuneUsage(range, ImplicitRuneS(lidb.child().consume()))
        resultRuneS
      }
      case Some(typeP) => {
        translateTypeIntoRune(env, lidb, ruleBuilder, contextRegion, typeP)
      }
  }
}
*/
fn translate_maybe_type_into_maybe_rune<'a, 'p>(
  _env: IEnvironmentS<'a>,
  _lidb: &mut LocationInDenizenBuilder,
  _range: RangeS<'a>,
  _rule_builder: &mut Vec<IRulexSR<'a>>,
  _rune_to_explicit_type: &mut HashMap<IRuneS<'a>, ITemplataType>,
  _context_region: IRuneS<'a>,
  _maybe_type_p: Option<&ITemplexPT<'a, 'p>>,
) -> Option<RuneUsage<'a>> {
  panic!("Unimplemented translate_maybe_type_into_maybe_rune");
}
/*
  def translateMaybeTypeIntoMaybeRune(
    env: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    range: RangeS,
    ruleBuilder: ArrayBuffer[IRulexSR],
    runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
    contextRegion: IRuneS, // Nearest enclosing region marker, see RADTGCA.
    maybeTypeP: Option[ITemplexPT]):
  Option[RuneUsage] = {
    if (maybeTypeP.isEmpty) {
      None
    } else {
      Some(
        translateMaybeTypeIntoRune(
          env, lidb.child(), range, ruleBuilder, contextRegion, maybeTypeP))
    }
  }
}
*/