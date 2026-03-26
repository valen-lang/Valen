use std::collections::HashMap;

use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::{INameDeclarationP, PatternPP};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::itemplatatype::{CoordTemplataType, ITemplataType};
use crate::postparsing::names::{IRuneS, IVarNameS};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::postparsing::post_parser::{IEnvironmentS, PostParser, StackFrame};
use crate::postparsing::rules::rules::IRulexSR;
use crate::postparsing::rules::templex_scout::translate_maybe_type_into_rune;
use crate::postparsing::variable_uses::VariableDeclarationS;
/*
package dev.vale.postparsing.patterns

import dev.vale._
import dev.vale.parsing.ast._
import dev.vale.postparsing._
import dev.vale.postparsing.rules.{IRulexSR, TemplexScout}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.rules._
import dev.vale.postparsing._

import scala.collection.immutable.List
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
*/

pub(crate) fn get_parameter_captures<'a>(
  pattern: &AtomSP<'a>,
) -> Vec<VariableDeclarationS<'a>> {
  let mut captures = Vec::new();
  if let Some(capture) = &pattern.name {
    captures.extend(get_capture_captures(capture));
  }
  if let Some(destructure) = &pattern.destructure {
    for inner_pattern in destructure {
      captures.extend(get_parameter_captures(inner_pattern));
    }
  }
  captures
}
/*
class PatternScout(
    interner: Interner,
    templexScout: TemplexScout) {
  def getParameterCaptures(pattern: AtomSP): Vector[VariableDeclaration] = {
    val AtomSP(_, maybeCapture, _, maybeDestructure) = pattern
  Vector.empty ++
      maybeCapture.toVector.flatMap(getCaptureCaptures) ++
        maybeDestructure.toVector.flatten.flatMap(getParameterCaptures)
  }
*/
fn get_capture_captures<'a>(
  capture: &CaptureS<'a>,
) -> Vec<VariableDeclarationS<'a>> {
  if capture.mutate {
    Vec::new()
  } else {
    vec![VariableDeclarationS {
      name: capture.name.clone(),
    }]
  }
}
/*
  private def getCaptureCaptures(capture: CaptureS): Vector[VariableDeclaration] = {
    if (capture.mutate) {
      Vector()
    } else {
      Vector(VariableDeclaration(capture.name))
    }
  }
*/
pub(crate) fn translate_pattern<'a, 's>(
  scout_arena: &'s bumpalo::Bump,
  interner: &Interner<'a>,
  keywords: &Keywords<'a>,
  stack_frame: StackFrame<'a>,
  lidb: &mut LocationInDenizenBuilder,
  rule_builder: &mut Vec<IRulexSR<'a, 's>>,
  rune_to_explicit_type: &mut HashMap<IRuneS<'a>, ITemplataType>,
  pattern_pp: &PatternPP<'a, '_>,
) -> AtomSP<'a> {
  let maybe_coord_rune = match &pattern_pp.templex {
    None => None,
    Some(type_p) => {
      let mut child_lidb = lidb.child();
      let coord_rune = translate_maybe_type_into_rune(
        scout_arena, interner,
        keywords,
        IEnvironmentS::FunctionEnvironment(stack_frame.parent_env.clone()),
        &mut child_lidb,
        PostParser::eval_range(stack_frame.file, pattern_pp.range),
        rule_builder,
        stack_frame.context_region.clone(),
        Some(type_p),
      );
      rune_to_explicit_type.insert(
        coord_rune.rune.clone(),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      );
      Some(coord_rune)
    }
  };

  let maybe_patterns_s = match &pattern_pp.destructure {
    None => None,
    Some(destructure_p) => {
      let mut patterns = Vec::new();
      for inner_pattern_p in destructure_p.patterns {
        let mut child_lidb = lidb.child();
        patterns.push(translate_pattern(
          scout_arena, interner,
          keywords,
          stack_frame.clone(),
          &mut child_lidb,
          rule_builder,
          rune_to_explicit_type,
          inner_pattern_p,
        ));
      }
      Some(patterns)
    }
  };

  let capture_s = match &pattern_pp.destination {
    None => None,
    Some(destination) => {
      let mutate = destination.mutate.is_some();
      match &destination.decl {
        INameDeclarationP::IgnoredLocalNameDeclaration(_) => None,
        INameDeclarationP::LocalNameDeclaration(name_p) => Some(CaptureS {
          name: IVarNameS::CodeVarName(name_p.str()),
          mutate,
        }),
        INameDeclarationP::ConstructingMemberNameDeclaration(name_p) => Some(CaptureS {
          name: IVarNameS::ConstructingMemberName(name_p.str()),
          mutate,
        }),
        INameDeclarationP::IterableNameDeclaration(range) => Some(CaptureS {
          name: IVarNameS::IterableName(PostParser::eval_range(stack_frame.file, *range)),
          mutate,
        }),
        INameDeclarationP::IteratorNameDeclaration(range) => Some(CaptureS {
          name: IVarNameS::IteratorName(PostParser::eval_range(stack_frame.file, *range)),
          mutate,
        }),
        INameDeclarationP::IterationOptionNameDeclaration(range) => Some(CaptureS {
          name: IVarNameS::IterationOptionName(PostParser::eval_range(stack_frame.file, *range)),
          mutate,
        }),
      }
    }
  };

  AtomSP {
    range: PostParser::eval_range(stack_frame.file, pattern_pp.range),
    name: capture_s,
    coord_rune: maybe_coord_rune,
    destructure: maybe_patterns_s,
  }
}
/*
  // Returns:
  // - Region rune, or None if it's an ignore pattern
  // - The translated patterns
  private[postparsing] def translatePattern(
    stackFrame: StackFrame,
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    runeToExplicitType: mutable.ArrayBuffer[(IRuneS, ITemplataType)],
    patternPP: PatternPP):
  AtomSP = {
    val PatternPP(range,maybeCaptureP, maybeTypeP, maybeDestructureP) = patternPP

    val maybeCoordRuneS =
      maybeTypeP match {
        case Some(typeP) => {
          val runeS =
            templexScout.translateTypeIntoRune(
              stackFrame.parentEnv,
              lidb.child(),
              ruleBuilder,
              stackFrame.contextRegion,
              typeP)
          runeToExplicitType += ((runeS.rune, CoordTemplataType()))
          Some(runeS)
        }
        case None => {
          // This happens in patterns in lets, and in lambdas' parameters that have no types.
          None
        }
      }

    val maybePatternsS =
      maybeDestructureP match {
        case None => None
        case Some(DestructureP(_, destructureP)) => {
          Some(
            destructureP.map(
              translatePattern(
                stackFrame, lidb.child(), ruleBuilder, runeToExplicitType, _)))
        }
      }

    val captureS =
      maybeCaptureP match {
        case None => {
//          val codeLocation = Scout.evalPos(stackFrame.file, patternPP.range.begin)
          None
        }
        case Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), _)) => {
          None
        }
        case Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, name)), maybeMutate)) => {
          // if (name.str == "set" || name.str == "mut") {
          //   throw CompileErrorExceptionS(CantUseThatLocalName(PostParser.evalRange(stackFrame.file, range), name.str))
          // }
          Some(CaptureS(interner.intern(CodeVarNameS(name)), maybeMutate.nonEmpty))
        }
        case Some(DestinationLocalP(ConstructingMemberNameDeclarationP(NameP(_, name)), maybeMutate)) => {
          Some(CaptureS(interner.intern(ConstructingMemberNameS(name)), maybeMutate.nonEmpty))
        }
        case Some(DestinationLocalP(IterableNameDeclarationP(range), maybeMutate)) => {
          Some(CaptureS(interner.intern(IterableNameS(PostParser.evalRange(stackFrame.file, range))), maybeMutate.nonEmpty))
        }
        case Some(DestinationLocalP(IteratorNameDeclarationP(range), maybeMutate)) => {
          Some(CaptureS(interner.intern(IteratorNameS(PostParser.evalRange(stackFrame.file, range))), maybeMutate.nonEmpty))
        }
        case Some(DestinationLocalP(IterationOptionNameDeclarationP(range), maybeMutate)) => {
          Some(CaptureS(interner.intern(IterationOptionNameS(PostParser.evalRange(stackFrame.file, range))), maybeMutate.nonEmpty))
        }
      }

    val patternS =
      AtomSP(PostParser.evalRange(stackFrame.file, range), captureS, maybeCoordRuneS, maybePatternsS)
    patternS
  }

}
*/