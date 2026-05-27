package dev.vale.postparsing

import dev.vale.postparsing.patterns.PatternScout
import dev.vale.postparsing.rules.{IRulexSR, IntLiteralSL, LiteralSR, MaybeCoercingCallSR, MutabilityLiteralSL, RuleScout, RuneUsage, TemplexScout, VariabilityLiteralSL}
import dev.vale.parsing.ast._
import dev.vale.parsing.{ast, _}
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, postparsing, vassert, vassertSome, vcurious, vfail, vimpl, vwat}
import PostParser.{evalRange, noDeclarations, noVariableUses}
import dev.vale
import dev.vale.lexing.RangeL
import dev.vale.parsing.ast
import dev.vale.parsing.ast.{AndPE, AugmentPE, BinaryCallPE, BlockPE, BorrowP, BraceCallPE, BreakPE, CallPT, ConsecutorPE, ConstantBoolPE, ConstantFloatPE, ConstantIntPE, ConstantStrPE, ConstructArrayPE, DestructPE, DotPE, EachPE, FinalP, FunctionCallPE, FunctionP, IExpressionPE, ITemplexPT, IfPE, IndexPE, LambdaPE, LetPE, LoadAsBorrowP, LoadAsP, LoadAsWeakP, LookupNameP, LookupPE, MagicParamLookupPE, MethodCallPE, MoveP, MutableP, MutatePE, NameOrRunePT, NameP, NotPE, OrPE, PackPE, RangePE, ReturnPE, RuntimeSizedP, ShortcallPE, StaticSizedP, StrInterpolatePE, SubExpressionPE, TemplateArgsP, TuplePE, UnletPE, UseP, VoidPE, WeakP, WhilePE}
import dev.vale.postparsing.patterns.PatternScout
//import dev.vale.postparsing.predictor.{Conclusions, PredictorEvaluator}
import dev.vale.postparsing.rules.TemplexScout

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
//import dev.vale.postparsing.predictor.Conclusions
import dev.vale.postparsing.rules.RuleScout
//import dev.vale.postparsing.templatepredictor.PredictorEvaluator

trait IExpressionScoutDelegate {
  def scoutLambda(
    parentStackFrame: StackFrame,
    lambdaFunction0: FunctionP):
  (FunctionS, VariableUses)
}

sealed trait IScoutResult[+T <: IExpressionSE]
// Will contain the address of a local.
case class LocalLookupResult(range: RangeS, name: IVarNameS) extends IScoutResult[IExpressionSE] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
// Looks up something that's not a local.
// Should be just a function, but its also super likely that the user just forgot
// to declare a variable, and we interpreted it as an outside lookup.
case class OutsideLookupResult(
  range: RangeS,
  name: StrI,
  maybeTemplateArgs: Option[Vector[ITemplexPT]]
) extends IScoutResult[IExpressionSE] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
// Anything else, such as:
// - Result of a function call
// - Address inside a struct
case class NormalResult[+T <: IExpressionSE](expr: T) extends IScoutResult[T] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  def range: RangeS = expr.range
}

class ExpressionScout(
    delegate: IExpressionScoutDelegate,
    templexScout: TemplexScout,
    ruleScout: RuleScout,
    patternScout: PatternScout,
    interner: Interner,
    keywords: Keywords) {
  val loopPostParser = new LoopPostParser(interner, keywords)

  def endsWithReturn(exprSE: IExpressionSE): Boolean = {
    exprSE match {
      case ReturnSE(_, _) => true
      case ConsecutorSE(exprs) => endsWithReturn(exprs.last)
      case _ => false
    }
  }

  def scoutBlock(
    parentStackFrame: StackFrame,
    lidb: LocationInDenizenBuilder,
    // When we scout a function, it might hand in things here because it wants them to be considered part of
    // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
    initialLocals: VariableDeclarations,
    blockPE: BlockPE):
  (IExpressionSE, VariableUses, VariableUses) = {
    val BlockPE(rangeP, pure, maybeNewDefaultRegion, inner) = blockPE
    val rangeS = PostParser.evalRange(parentStackFrame.file, rangeP)
    vassert(maybeNewDefaultRegion.isEmpty)
    val (blockSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove) =
      newBlock(
        parentStackFrame.parentEnv,
        Some(parentStackFrame),
        lidb.child(),
        rangeS,
        maybeNewDefaultRegion match {
          case None => parentStackFrame.contextRegion
          case Some(RegionRunePT(range, name)) => {
            val regionRuneS = CodeRuneS(vassertSome(name).str) // impl isolates
            if (!parentStackFrame.parentEnv.allDeclaredRunes().contains(regionRuneS)) {
              throw CompileErrorExceptionS(CouldntFindRuneS(rangeS, vassertSome(name).str.str)) // impl isolates
            }
            regionRuneS
          }
        },
        initialLocals,
        (stackFrame1, lidb) => {
          val (stackFrame2, innerExprSE, selfUses, childUses) =
            scoutExpressionAndCoerce(
              stackFrame1, lidb, inner, UseP)
          (stackFrame2, innerExprSE, selfUses, childUses)
        })
    val resultingExprSE =
      if (blockPE.maybePure.nonEmpty) {
        PureSE(evalRange(parentStackFrame.file, blockPE.range), lidb.child().consume(), blockSE)
      } else {
        blockSE
      }
    (resultingExprSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove)
  }

  def scoutImpureBlock(
    parentStackFrame: StackFrame,
    lidb: LocationInDenizenBuilder,
    // When we scout a function, it might hand in things here because it wants them to be considered part of
    // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
    initialLocals: VariableDeclarations,
    blockPE: BlockPE):
  (BlockSE, VariableUses, VariableUses) = {
    val (exprSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove) =
      scoutBlock(parentStackFrame, lidb, initialLocals, blockPE)
    exprSE match {
      case b @ BlockSE(_, _, _) => (b, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove)
      case other => vfail("Expected impure block!")
    }
  }

  def newBlock(
    functionBodyEnv: FunctionEnvironmentS,
    parentStackFrame: Option[StackFrame],
    lidb: LocationInDenizenBuilder,
    rangeS: RangeS,
    contextRegion: IRuneS,
    // When we scout a function, it might hand in things here because it wants them to be considered part of
    // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
    initialLocals: VariableDeclarations,
    // If there's anything else we'd like to put at the end of the block, we can pass it in here
    scoutContents: (StackFrame, LocationInDenizenBuilder) => (StackFrame, IExpressionSE, VariableUses, VariableUses)):
  (BlockSE, VariableUses, VariableUses) = {
    val initialStackFrame =
      StackFrame(
        functionBodyEnv.file,
        functionBodyEnv.name,
        functionBodyEnv,
        parentStackFrame,
        contextRegion,
        parentStackFrame.map(_.pureHeight + 1).getOrElse(0),
        initialLocals)
//    val rangeS = evalRange(functionBodyEnv.file, blockPE.range)
//
//    val (stackFrameBeforeExtrasAndConstructing, exprsWithoutExtrasWithoutConstructingWithoutVoidS, selfUsesBeforeExtrasAndConstructing, childUsesBeforeExtrasAndConstructing) =
//      scoutElementsAsExpressions(initialStackFrame, lidb.child(), blockPE.elements)

    val (stackFrameBeforeConstructing, exprWithoutConstructingWithoutVoidS, selfUsesBeforeConstructing, childUsesBeforeConstructing) =
      scoutContents(initialStackFrame, lidb.child())

    // If we had for example:
    //   func MyStruct() {
    //     this.a = 5;
    //     println("flamscrankle");
    //     this.b = true;
    //   }
    // then here's where we insert the final
    //     MyStruct(`this.a`, `this.b`);
    val constructedMembersNames =
      stackFrameBeforeConstructing.locals.vars.map(_.name).collect({ case ConstructingMemberNameS(n) => n })

    val (stackFrame, exprWithConstructingIfNecessary, selfUses, childUses) =
      if (constructedMembersNames.isEmpty) {
        // Add a void to the end, unless:
        // - we're constructing
        // - we end with a return
        // - a result was requested.
        val exprsS = exprWithoutConstructingWithoutVoidS
        (stackFrameBeforeConstructing, exprsS, selfUsesBeforeConstructing, childUsesBeforeConstructing)
      } else {
        val rangeAtEnd = RangeL(rangeS.end.offset, rangeS.end.offset)
        val constructorCallP =
          FunctionCallPE(
            rangeAtEnd,
            RangeL.zero,
            LookupPE(
              stackFrameBeforeConstructing.parentEnv.name match {
                case FunctionNameS(n, _) => LookupNameP(NameP(rangeAtEnd, n))
                case _ => vwat()
              }, None),
            constructedMembersNames.map(n => DotPE(rangeAtEnd, LookupPE(LookupNameP(NameP(rangeAtEnd, keywords.self)), None), RangeL.zero, NameP(rangeAtEnd, n))))

        val (stackFrameAfterConstructing, NormalResult(constructExpression), selfUsesAfterConstructing, childUsesAfterConstructing) =
          scoutExpression(stackFrameBeforeConstructing, lidb.child(), constructorCallP)
        val exprAfterConstructing =
          PostParser.consecutive(
            Vector(
              exprWithoutConstructingWithoutVoidS,
              constructExpression))
        (stackFrameAfterConstructing, exprAfterConstructing, selfUsesBeforeConstructing.thenMerge(selfUsesAfterConstructing), childUsesBeforeConstructing.thenMerge(childUsesAfterConstructing))
      }

    val locals =
        stackFrame.locals.vars.map({ declared =>
        LocalS(
          declared.name,
          selfUses.isBorrowed(declared.name),
          selfUses.isMoved(declared.name),
          selfUses.isMutated(declared.name),
          childUses.isBorrowed(declared.name),
          childUses.isMoved(declared.name),
          childUses.isMutated(declared.name))
      })

    val selfUsesOfThingsFromAbove =
      VariableUses(selfUses.uses.filter(selfUseName => !locals.map(_.varName).contains(selfUseName.name)))
    val childUsesOfThingsFromAbove =
      VariableUses(childUses.uses.filter(selfUseName => !locals.map(_.varName).contains(selfUseName.name)))

    val blockSE = BlockSE(rangeS, locals, exprWithConstructingIfNecessary)
    (blockSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove)
  }

  def findLocal(stackFrame0: StackFrame, rangeS: RangeS, impreciseNameS: IImpreciseNameS):
  Option[LocalLookupResult] = {
    stackFrame0.findVariable(impreciseNameS) match {
      case Some(fullName) => Some(LocalLookupResult(rangeS, fullName))
      case None => None
    }
  }

  // Returns:
  // - new seq num
  // - declared variables
  // - new expression
  // - variable uses by self
  // - variable uses by child blocks
  private def translateMaybeTemplateArgs(
    parentEnv: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    contextRegion: IRuneS,
    maybeTemplateArgs: Option[Vector[ITemplexPT]]):
  Vector[RuneUsage] = {
    maybeTemplateArgs match {
      case None => Vector()
      case Some(templateArgs) => {
        templateArgs.map(templateArg => {
          templexScout.translateTemplex(parentEnv, lidb.child(), ruleBuilder, contextRegion, templateArg)
        })
      }
    }
  }

  private def scoutExpression(
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    expr: IExpressionPE):
  (StackFrame, IScoutResult[IExpressionSE], VariableUses, VariableUses) = {
    Profiler.frame(() => {
      val evalRange = (range: RangeL) => PostParser.evalRange(stackFrame0.file, range)

      expr match {
        case VoidPE(range) => (stackFrame0, NormalResult(VoidSE(evalRange(range))), noVariableUses, noVariableUses)
        case lam @ LambdaPE(captures,_) => {
          val (function1, childUses) =
            delegate.scoutLambda(stackFrame0, lam.function)

          (stackFrame0, NormalResult(FunctionSE(function1)), noVariableUses, childUses)
        }
        case StrInterpolatePE(range, partsPE) => {
          val (stackFrame1, partsSE, partsSelfUses, partsChildUses) =
            scoutElementsAsExpressions(stackFrame0, lidb.child(), partsPE)

          val rangeS = evalRange(range)
          val startingExpr: IExpressionSE = ConstantStrSE(RangeS(rangeS.begin, rangeS.begin), "")
          val addedExpr =
            partsSE.foldLeft(startingExpr)({
              case (prevExpr, partSE) => {
                val addCallRange = RangeS(prevExpr.range.end, partSE.range.begin)
                val overloadSetSE =
                  OverloadSetSE(
                    OutsideLoadSE(
                      addCallRange,
                      Vector(),
                      Vector(LoadPartSE(interner.intern(CodeNameS(keywords.plus)), Vector()))))
                FunctionCallSE(addCallRange, lidb.child().consume(), overloadSetSE, Vector(prevExpr, partSE))
              }
            })
          (stackFrame1, NormalResult(addedExpr), partsSelfUses, partsChildUses)
        }
        case AugmentPE(range, targetOwnership, innerPE) => {
          val loadAs =
            targetOwnership match {
              case BorrowP => LoadAsBorrowP
              case WeakP => LoadAsWeakP
            }
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, loadAs)
          inner1 match {
            case OwnershippedSE(_, _, innerLoadAs) => vassert(loadAs == innerLoadAs)
            case LocalLoadSE(_, _, innerLoadAs) => vassert(loadAs == innerLoadAs)
//            case OutsideLoadSE(_, _, _, innerLoadAs) => vassert(loadAs == innerLoadAs) DO NOT SUBMIT
            case _ => vwat()
          }
          (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
        }
        case ReturnPE(range, innerPE) => {
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)
          (stackFrame1, NormalResult(ReturnSE(evalRange(range), inner1)), innerSelfUses, innerChildUses)
        }
        case BreakPE(range) => {
          (stackFrame0, NormalResult(BreakSE(evalRange(range))), noVariableUses, noVariableUses)
        }
        case NotPE(range, innerPE) => {
          val overloadSetSE =
            OverloadSetSE(
              OutsideLoadSE(
                evalRange(range),
                Vector(),
                Vector(LoadPartSE(interner.intern(CodeNameS(keywords.not)), Vector()))))

          val (stackFrame1, innerSE, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)

          val result =
            NormalResult(
              FunctionCallSE(evalRange(range), lidb.child().consume(), overloadSetSE, Vector(innerSE)))

          (stackFrame1, result, innerSelfUses, innerChildUses)
        }
        case RangePE(range, beginPE, endPE) => {
          val callableSE =
            OverloadSetSE(
              OutsideLoadSE(
              evalRange(range),
              Vector(),
              Vector(LoadPartSE(interner.intern(CodeNameS(keywords.range)), Vector()))))

          val loadBeginAs =
            beginPE match {
              // For subexpressions, just use what they give.
              case SubExpressionPE(_, _) => UseP
              // For anything else, default to borrowing.
              case _ => LoadAsBorrowP
            }
          val (stackFrame1, beginSE, beginSelfUses, beginChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), beginPE, loadBeginAs)

          val loadEndAs =
            endPE match {
              // For subexpressions, just use what they give.
              case SubExpressionPE(_, _) => UseP
              // For anything else, default to borrowing.
              case _ => LoadAsBorrowP
            }
          val (stackFrame2, endSE, endSelfUses, endChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), endPE, loadEndAs)

          val resultSE =
              FunctionCallSE(evalRange(range), lidb.child().consume(), callableSE, Vector(beginSE, endSE))

          (stackFrame2, NormalResult(resultSE), beginSelfUses.thenMerge(endSelfUses), beginChildUses.thenMerge(endChildUses))
        }
        case SubExpressionPE(range, innerPE) => {
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)
          (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
        }
        case ConstantIntPE(range, value, bitsP) => {
          val bits = bitsP.getOrElse(32L).toInt
          (stackFrame0, NormalResult(ConstantIntSE(evalRange(range), value, bits)), noVariableUses, noVariableUses)
        }
        case ConstantBoolPE(range,value) => (stackFrame0, NormalResult(ConstantBoolSE(evalRange(range), value)), noVariableUses, noVariableUses)
        case ConstantStrPE(range, value) => {
          (stackFrame0, NormalResult(ConstantStrSE(evalRange(range), value)), noVariableUses, noVariableUses)
        }
        case ConstantFloatPE(range,value) => (stackFrame0, NormalResult(ConstantFloatSE(evalRange(range), value)), noVariableUses, noVariableUses)

        case MagicParamLookupPE(range) => {
          val name = interner.intern(MagicParamNameS(PostParser.evalPos(stackFrame0.file, range.begin)))
          val lookup = LocalLookupResult(evalRange(range), name)
          // We dont declare it here, because then scoutBlock will think its a local and
          // hide it from those above.
          //   val declarations = VariableDeclarations(Vector(VariableDeclaration(lookup.name, FinalP)))
          // Leave it to scoutLambda to declare it.
          (stackFrame0, lookup, noVariableUses.markMoved(name), noVariableUses)
        }
        case LookupPE(lookupName, None) => {
          val rangeS = evalRange(lookupName.range)
          val impreciseNameS = PostParser.translateImpreciseName(interner, stackFrame0.file, lookupName)
          val lookup =
            findLocal(stackFrame0, rangeS, impreciseNameS) match {
              case Some(result) => result
              case None => {
                impreciseNameS match {
                  case CodeNameS(name) => {
                    if (stackFrame0.parentEnv.allDeclaredRunes().contains(CodeRuneS(name))) {
                      NormalResult(RuneLookupSE(rangeS, CodeRuneS(name)))
                    } else {
                      OutsideLookupResult(rangeS, name, None)
                    }
                  }
                  case _ => vwat(impreciseNameS)
                }
              }
            }
          (stackFrame0, lookup, noVariableUses, noVariableUses)
        }
        case LookupPE(impreciseName, Some(TemplateArgsP(_, templateArgs))) => {
          val (range, templateName) =
            impreciseName match {
              case LookupNameP(NameP(range, templateName)) => (range, templateName)
              case _ => vwat()
            }
          val result =
            OutsideLookupResult(
              evalRange(range),
              templateName,
              Some(templateArgs.toVector))
          (stackFrame0, result, noVariableUses, noVariableUses)
        }
        case DestructPE(range, innerPE) => {
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)
          (stackFrame1, NormalResult(DestructSE(evalRange(range), inner1)), innerSelfUses, innerChildUses)
        }
        case UnletPE(range, localNameP) => {
          val rangeS = evalRange(range)
          val impreciseNameS = PostParser.translateImpreciseName(interner, stackFrame0.file, localNameP)
          val varNameS =
            findLocal(stackFrame0, rangeS, impreciseNameS) match {
              case Some(LocalLookupResult(_, name)) => name
              case None => {
                throw CompileErrorExceptionS(RangedInternalErrorS(rangeS, "Can't unlet local: " + localNameP))
              }
            }
          val result = NormalResult(UnletSE(rangeS, varNameS))
          (stackFrame0, result, noVariableUses.markMoved(varNameS), noVariableUses)
        }
  //      case ResultPE(range, innerPE) => {
  //        scoutExpression(stackFrame0, lidb.child(), innerPE, true)
  //      }
        case PackPE(range, innersPE) => {
          vassert(innersPE.size == 1)
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innersPE.head, UseP)
          (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
        }
        case FunctionCallPE(range, _, callablePE, argsPE) => {
          val (stackFrame0a, subjectUncoercedScoutResult, uncoercedCallableSelfUses, uncoercedCallableChildUses) =
            scoutExpression(stackFrame0, lidb.child(), callablePE)
          subjectUncoercedScoutResult match {
            case OutsideLookupResult(range, containerName, containerMaybeTemplateArgs) => {
              val ruleBuilder = ArrayBuffer[IRulexSR]()
              val containerTemplateArgRuneUsages =
                translateMaybeTemplateArgs(stackFrame0.parentEnv, lidb, ruleBuilder, stackFrame0.contextRegion, containerMaybeTemplateArgs)
              val overloadSetSE =
                OverloadSetSE(
                  OutsideLoadSE(
                    range,
                    ruleBuilder.toVector,
                    Vector(LoadPartSE(interner.intern(CodeNameS(containerName)), containerTemplateArgRuneUsages))))
              val (stackFrame3, argsSE, selfUses, childUses) =
                scoutElementsAsExpressions(stackFrame0a, lidb.child(), argsPE)
              val result = NormalResult(FunctionCallSE(range, lidb.child().consume(), overloadSetSE, argsSE))
              (stackFrame3, result, selfUses, childUses)
            }
            case _ => {
              val callableChildUses = uncoercedCallableChildUses
              val (stackFrame2, callable1, callableSelfUses) =
                coerce(stackFrame0a, subjectUncoercedScoutResult, lidb.child(), uncoercedCallableSelfUses, LoadAsBorrowP)
              val (stackFrame3, args1, argsSelfUses, argsChildUses) =
                scoutElementsAsExpressions(stackFrame2, lidb.child(), argsPE)
              val result = NormalResult(FunctionCallSE(evalRange(range), lidb.child().consume(), callable1, args1.toVector))
              (stackFrame3, result, callableSelfUses.thenMerge(argsSelfUses), callableChildUses.thenMerge(argsChildUses))
            }
          }
        }
        case BinaryCallPE(range, namePE, leftPE, rightPE) => {
          val overloadSetSE =
            OverloadSetSE(
              OutsideLoadSE(
              evalRange(range),
              Vector(),
              Vector(LoadPartSE(interner.intern(CodeNameS(namePE.str)), Vector()))))

          val (stackFrame1, leftSE, leftSelfUses, leftChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), leftPE, LoadAsBorrowP)
          val (stackFrame2, rightSE, rightSelfUses, rightChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), rightPE, LoadAsBorrowP)

          val result =
            NormalResult(
              FunctionCallSE(evalRange(range), lidb.child().consume(), overloadSetSE, Vector(leftSE, rightSE)))
          (stackFrame2, result, leftSelfUses.thenMerge(rightSelfUses), leftChildUses.thenMerge(rightChildUses))
        }
        case BraceCallPE(range, operatorRange, subjectPE, args, callableReadwrite) => {
          val loadSubjectAs =
            subjectPE match {
              // For subexpressions, just use what they give.
              case SubExpressionPE(_, _) => UseP
              // For anything else, default to borrowing.
              case _ => {
                LoadAsBorrowP
              }
            }

          vassert(args.size == 1)
          val argPE = args.head

          val (stackFrame1, callableSE, callableSelfUses, callableChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), subjectPE, loadSubjectAs)
          val (stackFrame2, argSE, argSelfUses, argChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), argPE, UseP)

          val resultSE = IndexSE(evalRange(range), callableSE, argSE)
          (stackFrame2, NormalResult(resultSE), callableSelfUses.thenMerge(argSelfUses), callableChildUses.thenMerge(argChildUses))
        }
        case MethodCallPE(methodCallRangeP, subjectExpr, _, LookupPE(methodName, maybeMethodTemplateArgs), methodArgs) => {
          val methodCallRangeS = evalRange(methodCallRangeP)
          val (stackFrame0a, subjectUncoercedScoutResult, uncoercedSubjectSelfUses, uncoercedSubjectChildUses) =
            scoutExpression(stackFrame0, lidb.child(), subjectExpr)
          val methodImpreciseNameS = PostParser.translateImpreciseName(interner, stackFrame0.file, methodName)
          val ruleBuilder = ArrayBuffer[IRulexSR]()
          val methodTemplateArgRuneUsages =
            translateMaybeTemplateArgs(stackFrame0.parentEnv, lidb, ruleBuilder, stackFrame0.contextRegion, maybeMethodTemplateArgs.map(_.args))
          val (stackFrame3, args, selfUses, childUses, containerParts) =
            subjectUncoercedScoutResult match {
              case OutsideLookupResult(subjectRangeS, containerName, containerMaybeTemplateArgs) => {
                vassert(uncoercedSubjectSelfUses.isEmpty)
                vassert(uncoercedSubjectChildUses.isEmpty)
                val containerTemplateArgRuneUsages =
                  translateMaybeTemplateArgs(stackFrame0.parentEnv, lidb, ruleBuilder, stackFrame0.contextRegion, containerMaybeTemplateArgs)
                val (stackFrame3, args, selfUses, childUses) =
                  scoutElementsAsExpressions(stackFrame0a, lidb.child(), methodArgs)
                // We don't support more than 2 parts yet
                val containerParts = Vector(
                  LoadPartSE(interner.intern(CodeNameS(containerName)), containerTemplateArgRuneUsages))
                (stackFrame3, args, selfUses, childUses, containerParts)
              }
              case _ => {
                val loadSubjectAs =
                  subjectExpr match {
                    // For locals, just borrow.
                    case LookupPE(_, _) => LoadAsBorrowP
                    // For anything else, default to moving.
                    case _ => UseP
                  }
                val subjectChildUses = uncoercedSubjectChildUses
                val (stackFrame1, subject1, subjectSelfUses) =
                  coerce(stackFrame0a, subjectUncoercedScoutResult, lidb.child(), uncoercedSubjectSelfUses, loadSubjectAs)
                val (stackFrame3, tailArgs1, tailArgsSelfUses, tailArgsChildUses) =
                  scoutElementsAsExpressions(stackFrame1, lidb.child(), methodArgs)
                val containerParts = Vector()
                (stackFrame3, Vector(subject1) ++ tailArgs1, subjectSelfUses.thenMerge(tailArgsSelfUses), subjectChildUses.thenMerge(tailArgsChildUses), containerParts)
              }
            }
          val parts = containerParts :+ LoadPartSE(methodImpreciseNameS, methodTemplateArgRuneUsages)
          val overloadSetSE =
            OverloadSetSE(OutsideLoadSE(methodCallRangeS, ruleBuilder.toVector, parts))
          val result = NormalResult(FunctionCallSE(methodCallRangeS, lidb.child().consume(), overloadSetSE, args))
          (stackFrame3, result, selfUses, childUses)
        }
        case TuplePE(range, elementsPE) => {
          val (stackFrame1, elements1, selfUses, childUses) =
            scoutElementsAsExpressions(stackFrame0, lidb.child(), elementsPE)
          (stackFrame1, NormalResult(TupleSE(evalRange(range), elements1.toVector)), selfUses, childUses)
        }
        case ConstructArrayPE(rangeP, maybeTypePT, maybeMutabilityPT, maybeVariabilityPT, size, initializingIndividualElements, argsPE) => {
          val rangeS = evalRange(rangeP)
          val ruleBuilder = mutable.ArrayBuffer[IRulexSR]()
          val maybeTypeRuneS =
            maybeTypePT.map(typePT => {
              templexScout.translateTemplex(
                stackFrame0.parentEnv, lidb.child(), ruleBuilder, stackFrame0.contextRegion, typePT)
            })
          val mutabilityRuneS =
            maybeMutabilityPT match {
              case None => {
                val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                ruleBuilder += LiteralSR(rangeS, rune, MutabilityLiteralSL(MutableP))
                rune
              }
              case Some(mutabilityPT) => {
                templexScout.translateTemplex(
                  stackFrame0.parentEnv, lidb.child(), ruleBuilder, stackFrame0.contextRegion, mutabilityPT)
              }
            }
          val variabilityRuneS =
            maybeVariabilityPT match {
              case None => {
                val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                ruleBuilder += rules.LiteralSR(rangeS, rune, VariabilityLiteralSL(FinalP))
                rune
              }
              case Some(variabilityPT) => {
                templexScout.translateTemplex(
                  stackFrame0.parentEnv, lidb.child(), ruleBuilder, stackFrame0.contextRegion, variabilityPT)
              }
            }

          val (stackFrame1, argsSE, selfUses, childUses) =
            scoutElementsAsExpressions(stackFrame0, lidb.child(), argsPE)

          val result =
            size match {
              case RuntimeSizedP => {
                vassert(!initializingIndividualElements)
                if (argsSE.isEmpty || argsSE.size > 2) {
                  throw CompileErrorExceptionS(InitializingRuntimeSizedArrayRequiresSizeAndCallable(rangeS))
                }
                val sizeSE = argsSE.head
                val callableSE = argsSE.lift(1)

                NewRuntimeSizedArraySE(rangeS, ruleBuilder.toVector, maybeTypeRuneS, mutabilityRuneS, sizeSE, callableSE)
              }
              case StaticSizedP(maybeSizePT) => {
                val maybeSizeRuneS =
                  maybeSizePT match {
                    case None => None
                    case Some(sizePT) => {
                      Some(
                        templexScout.translateTemplex(
                          stackFrame0.parentEnv,
                          lidb.child(),
                          ruleBuilder,
                          stackFrame0.contextRegion,
                          sizePT))
                    }
                  }

                if (initializingIndividualElements) {
                  val sizeRuneS =
                    maybeSizeRuneS match {
                      case Some(s) => s
                      case None => {
                        val runeS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                        ruleBuilder += rules.LiteralSR(rangeS, runeS, IntLiteralSL(argsSE.size))
                        runeS
                      }
                    }

                  StaticArrayFromValuesSE(
                    rangeS, ruleBuilder.toVector, maybeTypeRuneS, mutabilityRuneS, variabilityRuneS, sizeRuneS, argsSE.toVector)
                } else {
                  if (argsSE.size != 1) {
                    throw CompileErrorExceptionS(InitializingStaticSizedArrayRequiresSizeAndCallable(rangeS))
                  }
                  val sizeRuneS = vassertSome(maybeSizeRuneS)
                  val Vector(callableSE) = argsSE
                  StaticArrayFromCallableSE(
                    rangeS, ruleBuilder.toVector, maybeTypeRuneS, mutabilityRuneS, variabilityRuneS, sizeRuneS, callableSE)
                }
              }
            }

          (stackFrame1, NormalResult(result), selfUses, childUses)
        }
        case b @ BlockPE(_, _, maybeNewDefaultRegion, _) => {
          vassert(maybeNewDefaultRegion.isEmpty)
          val (resultSE, selfUses, childUses) =
            scoutBlock(stackFrame0, lidb.child(), noDeclarations, b)
          (stackFrame0, NormalResult(resultSE), selfUses, childUses)
        }
        case ConsecutorPE(inners) => {
          val (stackFrame1, unfilteredResultsSE, selfUses, childUses) =
            scoutElementsAsExpressions(stackFrame0, lidb.child(), inners)

          // Strip trailing voids after return
          // The boolean is whether we've seen a return yet
          val (_, filteredResultsSE) =
            unfilteredResultsSE.foldLeft((false, Vector[IExpressionSE]()))({
              case ((false, previous), r @ ReturnSE(_, _)) => (true, previous :+ r)
              case ((false, previous), next) => (false, previous :+ next)
              case ((true, previous), VoidSE(_)) => (true, previous)
              case ((true, previous), next) => {
                throw CompileErrorExceptionS(StatementAfterReturnS(next.range))
              }
            })

          (stackFrame1, NormalResult(PostParser.consecutive(filteredResultsSE)), selfUses, childUses)
        }
        case AndPE(range, leftPE, rightPE) => {
          val rightRange = evalRange(rightPE.range)
          val endRange = RangeS(rightRange.end, rightRange.end)

          val (stackFrameZ, ifSE, selfUses, childUses) =
            newIf(
              stackFrame0, lidb, range,
              (stackFrame1, lidb) => {
                scoutExpressionAndCoerce(stackFrame1, lidb, leftPE, UseP)
              },
              (stackFrame2, lidb) => {
                val (thenSE, thenUses, thenChildUses) =
                  scoutImpureBlock(stackFrame2, lidb.child(), noDeclarations, rightPE)
                (stackFrame2, thenSE, thenUses, thenChildUses)
              },
              (stackFrame3, lidb) => {
                val elseSE =
                  BlockSE(endRange, Vector.empty, ConstantBoolSE(endRange, false))
                (stackFrame3, elseSE, noVariableUses, noVariableUses)
              })

          (stackFrameZ, NormalResult(ifSE), selfUses, childUses)
        }
        case OrPE(range, leftPE, rightPE) => {
          val rightRange = evalRange(rightPE.range)
          val endRange = RangeS(rightRange.end, rightRange.end)

          val (stackFrameZ, ifSE, selfUses, childUses) =
            newIf(
              stackFrame0, lidb, range,
              (stackFrame1, lidb) => {
                scoutExpressionAndCoerce(stackFrame1, lidb, leftPE, UseP)
              },
              (stackFrame2, lidb) => {
                val elseSE =
                  BlockSE(endRange, Vector.empty, ConstantBoolSE(endRange, true))
                (stackFrame2, elseSE, noVariableUses, noVariableUses)
              },
              (stackFrame3, lidb) => {
                val (thenSE, thenUses, thenChildUses) =
                  scoutImpureBlock(stackFrame3, lidb.child(), noDeclarations, rightPE)
                (stackFrame3, thenSE, thenUses, thenChildUses)
              })

          (stackFrameZ, NormalResult(ifSE), selfUses, childUses)
        }
        case IfPE(range, condition, thenBody, elseBody) => {
          val (resultSE, selfUses, childUses) =
            newBlock(
              stackFrame0.parentEnv,
              Some(stackFrame0),
              lidb.child(),
              evalRange(range),
              stackFrame0.contextRegion,
              noDeclarations,
              (stackFrame1, lidb) => {
                val (stackFrame2, condSE, condUses, condChildUses) =
                  scoutExpressionAndCoerce(stackFrame1, lidb.child(), condition, UseP)
                val (thenSE, thenUses, thenChildUses) =
                  scoutImpureBlock(stackFrame2, lidb.child(), noDeclarations, thenBody)
                val (elseSE, elseUses, elseChildUses) =
                  scoutImpureBlock(stackFrame2, lidb.child(), noDeclarations, elseBody)

                val selfCaseUses = thenUses.branchMerge(elseUses)
                val selfUses = condUses.thenMerge(selfCaseUses);
                val childCaseUses = thenChildUses.branchMerge(elseChildUses)
                val childUses = condChildUses.thenMerge(childCaseUses);

                val ifSE = IfSE(evalRange(range), condSE, thenSE, elseSE)
                (stackFrame2, ifSE, selfUses, childUses)
              })
          (stackFrame0, NormalResult(resultSE), selfUses, childUses)
        }
        case WhilePE(range, conditionPE, uncombinedBodyPE) => {
          val (loopSE, loopSelfUses, loopChildUses) =
            loopPostParser.scoutWhile(
              this, stackFrame0, lidb, range, conditionPE, uncombinedBodyPE)

          (stackFrame0, NormalResult(loopSE), loopSelfUses, loopChildUses)
        }
        case EachPE(range, maybePure, entryPatternPP, inKeywordRange, iterableExpr, body) => {
          val (loopSE, selfUses, childUses) =
            loopPostParser.scoutEach(this, stackFrame0, lidb, range, maybePure.nonEmpty, entryPatternPP, inKeywordRange, iterableExpr, body)
          (stackFrame0, NormalResult(loopSE), selfUses, childUses)
        }
        case LetPE(range, patternP, exprPE) => {
          val codeLocation = PostParser.evalPos(stackFrame0.file, range.begin)
          val (stackFrame1, expr1, selfUses, childUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), exprPE, UseP);

          val ruleBuilder = ArrayBuffer[IRulexSR]()
          val runeToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

          ruleScout.translateRulexes(
            stackFrame0.parentEnv, lidb.child(), ruleBuilder, runeToExplicitType, stackFrame1.contextRegion, Vector())

          val patternS =
            patternScout.translatePattern(
              stackFrame1, lidb.child(), ruleBuilder, runeToExplicitType, patternP)

          val declarationsFromPattern = VariableDeclarations(patternScout.getParameterCaptures(patternS))

          val nameConflictVarNames =
            stackFrame1.locals.vars.map(_.name).intersect(declarationsFromPattern.vars.map(_.name))
          nameConflictVarNames.headOption match {
            case None =>
            case Some(nameConflictVarName) => {
              throw CompileErrorExceptionS(VariableNameAlreadyExists(evalRange(range), nameConflictVarName))
            }
          }

          val letSE = LetSE(evalRange(range), ruleBuilder.toVector, patternS, expr1)
          (stackFrame1 ++ declarationsFromPattern, NormalResult(letSE), selfUses, childUses)
        }
        case MutatePE(mutateRange, destinationExprPE, sourceExprPE) => {
          val (stackFrame1, sourceExpr1, sourceInnerSelfUses, sourceChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), sourceExprPE, UseP);
          val (stackFrame2, destinationResult1, destinationSelfUses, destinationChildUses) =
            scoutExpression(stackFrame1, lidb.child(), destinationExprPE);
          val (mutateExpr1, sourceSelfUses) =
            destinationResult1 match {
              case LocalLookupResult(range, name) => {
                (LocalMutateSE(range, name, sourceExpr1), sourceInnerSelfUses.markMutated(name))
              }
              case OutsideLookupResult(range, name, maybeTemplateArgs) => {
                throw CompileErrorExceptionS(CouldntFindVarToMutateS(range, name.str))
              }
              case NormalResult(destinationExpr1) => {
                (ExprMutateSE(destinationExpr1.range, destinationExpr1, sourceExpr1), sourceInnerSelfUses)
              }
            }
          (stackFrame2, NormalResult(mutateExpr1), sourceSelfUses.thenMerge(destinationSelfUses), sourceChildUses.thenMerge(destinationChildUses))
        }
        case DotPE(rangeP, containerExprPE, _, NameP(_, memberName)) => {
          containerExprPE match {
            // Here, we're special casing lookups of this.x when we're in a constructor.
            // We know we're in a constructor if there's no `this` variable yet. After all,
            // in a constructor, `this` is just an imaginary concept until we actually
            // fill all the variables.
            case LookupPE(LookupNameP(NameP(range, s)), _) if s == keywords.self && (stackFrame0.findVariable(interner.intern(CodeNameS(interner.intern(StrI("self"))))).isEmpty) => {
              val result = LocalLookupResult(evalRange(range), interner.intern(ConstructingMemberNameS(memberName)))
              (stackFrame0, result, noVariableUses, noVariableUses)
            }
            case _ => {
              val (stackFrame1, containerExpr, selfUses, childUses) =
                scoutExpressionAndCoerce(stackFrame0, lidb.child(), containerExprPE, LoadAsBorrowP)
              (stackFrame1, NormalResult(DotSE(evalRange(rangeP), containerExpr, memberName, true)), selfUses, childUses)
            }
          }
        }
        case IndexPE(range, containerExprPE, Vector(indexExprPE)) => {
          val (stackFrame1, containerExpr1, containerSelfUses, containerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), containerExprPE, LoadAsBorrowP)
          val (stackFrame2, indexExpr1, indexSelfUses, indexChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), indexExprPE, UseP);
          val dot1 = IndexSE(evalRange(range), containerExpr1, indexExpr1)
          (stackFrame2, NormalResult(dot1), containerSelfUses.thenMerge(indexSelfUses), containerChildUses.thenMerge(indexChildUses))
        }
        case ShortcallPE(range, argExprs) => {
          throw CompileErrorExceptionS(UnimplementedExpression(evalRange(range), "shortcalling"));
        }
      }
    })
  }

  def newIf(
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    range: RangeL,
    makeCondition: (StackFrame, LocationInDenizenBuilder) => (StackFrame, IExpressionSE, VariableUses, VariableUses),
    makeThen: (StackFrame, LocationInDenizenBuilder) => (StackFrame, BlockSE, VariableUses, VariableUses),
    makeElse: (StackFrame, LocationInDenizenBuilder) => (StackFrame, BlockSE, VariableUses, VariableUses)):
  (StackFrame, IfSE, VariableUses, VariableUses) = {
    val (stackFrame1, condSE, condUses, condChildUses) =
      makeCondition(stackFrame0, lidb.child())

    val (stackFrame2, thenSE, thenUses, thenChildUses) =
      makeThen(stackFrame1, lidb.child())
    val (stackFrame3, elseSE, elseUses, elseChildUses) =
      makeElse(stackFrame2, lidb.child())

    val selfCaseUses = thenUses.branchMerge(elseUses)
    val selfUses = condUses.thenMerge(selfCaseUses);
    val childCaseUses = thenChildUses.branchMerge(elseChildUses)
    val childUses = condChildUses.thenMerge(childCaseUses);

    val ifSE =
      IfSE(PostParser.evalRange(stackFrame0.file, range), condSE, thenSE, elseSE)
    (stackFrame3, ifSE, selfUses, childUses)
  }

  // If we load an immutable with targetOwnershipIfLookupResult = Own or Borrow, it will just be Share.
  def scoutExpressionAndCoerce(
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    exprPE: IExpressionPE,
    loadAsP: LoadAsP):
  (StackFrame, IExpressionSE, VariableUses, VariableUses) = {
    val (stackFrame1, uncoercedScoutResult, exprSelfUses, exprChildUses) =
      scoutExpression(stackFrame0, lidb.child(), exprPE)
    val (stackFrame2, coercedExpr, coercedSelfUses) =
      coerce(stackFrame1, uncoercedScoutResult, lidb.child(), exprSelfUses, loadAsP)
    (stackFrame2, coercedExpr, coercedSelfUses, exprChildUses)
  }

  def coerce(
      stackFrame0: StackFrame,
      firstResult1: IScoutResult[IExpressionSE],
      lidb: LocationInDenizenBuilder,
      selfUsesBefore: VariableUses,
      loadAsP: LoadAsP
  ): (StackFrame, IExpressionSE, VariableUses) = {
    firstResult1 match {
      case LocalLookupResult(range, name) => {
        val selfUsesAfter =
          loadAsP match {
            case LoadAsBorrowP => selfUsesBefore.markBorrowed(name)
            case LoadAsWeakP => selfUsesBefore.markBorrowed(name)
            case UseP | MoveP => selfUsesBefore.markMoved(name)
          }
        (stackFrame0, LocalLoadSE(range, name, loadAsP), selfUsesAfter)
      }
      case OutsideLookupResult(range, name, maybeTemplateArgs) => {
        // When we have globals, here's where we'd check that the user declared they're accessing a global.
        // But since we don't have globals yet, we just throw an error...
        // V: open question... why don't we coerce to overload set here? Is that done somewhere else?
        throw CompileErrorExceptionS(CouldntFindVarToMutateS(range, "a"))
      }
      case NormalResult(innerExpr1) => {
        loadAsP match {
          case UseP => (stackFrame0, innerExpr1, selfUsesBefore)
          case _ => (stackFrame0, OwnershippedSE(innerExpr1.range, innerExpr1, loadAsP), selfUsesBefore)
        }
      }
    }
  }

  // Need a better name for this...
  // It's more like, scout elements as non-lookups, in other words,
  // if we get lookups then coerce them into moves.
  def scoutElementsAsExpressions(
    initialStackFramePE: StackFrame,
    lidb: LocationInDenizenBuilder,
    exprsPE: Vector[IExpressionPE]):
  (StackFrame, Vector[IExpressionSE], VariableUses, VariableUses) = {
    var selfUses = noVariableUses
    var childUses = noVariableUses
    val (finalStackFrame, reversedExprsSE) =
      exprsPE.foldLeft((initialStackFramePE, List[IExpressionSE]()))({
        case ((prevStackFrame, reversedPrevExprsSE), exprPE) => {
          val (nextStackFrame, exprSE, firstSelfUses, firstChildUses) =
            scoutExpressionAndCoerce(prevStackFrame, lidb.child(), exprPE, UseP)
          selfUses = selfUses.thenMerge(firstSelfUses)
          childUses = childUses.thenMerge(firstChildUses)
          (nextStackFrame, exprSE :: reversedPrevExprsSE)
        }
      })
    (finalStackFrame, reversedExprsSE.toVector.reverse, selfUses, childUses)
  }
}

object ExpressionScout {
  def flattenExpressions(expr: IExpressionSE): Vector[IExpressionSE] = {
    expr match {
      case ConsecutorSE(exprs) => exprs.flatMap(flattenExpressions)
      case other => Vector(other)
    }
  }
}
