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
fn get_parameter_captures<'a>(
  _pattern: &crate::postparsing::patterns::AtomSP<'a>,
) -> Vec<crate::postparsing::variable_uses::VariableDeclarationS<'a>> {
  panic!("Unimplemented get_parameter_captures");
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
  _capture: &crate::postparsing::patterns::CaptureS<'a>,
) -> Vec<crate::postparsing::variable_uses::VariableDeclarationS<'a>> {
  panic!("Unimplemented get_capture_captures");
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
fn translate_pattern<'a>(
  _stack_frame: crate::postparsing::post_parser::StackFrame<'a>,
  _lidb: &mut crate::postparsing::ast::LocationInDenizenBuilder,
  _rule_builder: &mut Vec<crate::postparsing::rules::rules::IRulexSR<'a>>,
  _rune_to_explicit_type: &mut std::collections::HashMap<crate::postparsing::names::IRuneS<'a>, crate::postparsing::itemplatatype::ITemplataType>,
  _pattern_pp: &crate::parsing::ast::PatternPP<'a, '_>,
) -> crate::postparsing::patterns::AtomSP<'a> {
  panic!("Unimplemented translate_pattern");
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