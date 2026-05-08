package dev.vale.typing.expression

import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules.{IRulexSR, RuneUsage}
import dev.vale.postparsing.GlobalFunctionFamilyNameS
import dev.vale.solver.{FailedSolve, RuleError}
import dev.vale.typing.OverloadResolver.{FindFunctionFailure, FindFunctionResolveFailure, InferFailure}
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.infer.IsaFailed
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.function._
import dev.vale.typing.types._
import dev.vale.typing.{ast, _}
import dev.vale.typing.ast._
import dev.vale.typing.names._

import scala.collection.immutable.List

class CallCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    templataCompiler: TemplataCompiler,
    convertHelper: ConvertHelper,
    localHelper: LocalHelper,
    overloadCompiler: OverloadResolver) {

  private def evaluateCall(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    callableExpr: ReferenceExpressionTE,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    positionalExplicitTemplateArgRunesS: Vector[IRuneS],
    receivingRuneToExplicitTemplateArgRune: Vector[(RuneUsage, RuneUsage)],
    givenArgsExprs2: Vector[ReferenceExpressionTE]):
  (ReferenceExpressionTE) = {
    callableExpr.result.coord.kind match {
      case NeverT(true) => vwat()
      case NeverT(false) | BoolT() => {
        throw CompileErrorExceptionT(RangedInternalErrorT(
          range,
          "wot " + callableExpr.result.coord.kind))
      }
      case OverloadSetT(overloadSetEnv, functionName) => {
        // Here we have a special case for overload sets.
        // It makes cases like these work:
        //     myOverloadSet = print;
        //     myOverloadSet("hello");
        // However, this code here only works when the user is specifically doing that form:
        // an overload set, then some parens. We then do a lookup of the overload set's
        // stored name ("print") in the overload set's stored env.
        //
        // This might not be the best long-term approach because it doesn't work in other cases:
        //     fn myFunc<F>(f &F) where func(&F)void { f() }
        //     myFunc(print);
        // The below code doesn't match this example because it's not literally overloadset
        // then parens; this example is instead trying to feed an overloadset argument into a
        // parameter that has a __call method available. OverloadSet is not that; OverloadSet is
        // a very surface level judo trick.
        //
        // See (failing) test "Pass overload set into placeholder parameter" (see @POSIPP).
        //
        // I think the right solution long term is to give OverloadSet some sort of __call
        // function that under the hood calls some sort of builtin that knows how to do the
        // below machinery.

        val unconvertedArgsPointerTypes2 =
          givenArgsExprs2.map(_.result.expectReference().coord)

        // We want to get the prototype here, not the entire header, because
        // we might be in the middle of a recursive call like:
        // func main():Int(main())

        val prototype =
          overloadCompiler.findFunction(
            overloadSetEnv,
            coutputs,
            range,
            callLocation,
            functionName,
            explicitTemplateArgRulesS,
            positionalExplicitTemplateArgRunesS,
            receivingRuneToExplicitTemplateArgRune,
            contextRegion,
            unconvertedArgsPointerTypes2,
            Vector.empty,
            false) match {
            case Err(e @ FindFunctionFailure(CodeNameS(asName), _, _)) if asName == keywords.as => {
              val isaFailures = e.rejectedCalleeToReason.flatMap { case (_, reason) =>
                reason match {
                  case InferFailure(fs @ FailedSolve(_, _, _, _, RuleError(IsaFailed(sub, suuper)))) =>
                    Some((sub, suuper, fs))
                  case FindFunctionResolveFailure(ResolvingSolveFailedOrIncomplete(fs @ FailedSolve(_, _, _, _, RuleError(IsaFailed(sub, suuper))))) =>
                    Some((sub, suuper, fs))
                  case _ => None
                }
              }
              if (isaFailures.nonEmpty) {
                val (sub, suuper, _) = isaFailures.head
                val failedSolves = isaFailures.map(_._3).toVector
                throw CompileErrorExceptionT(CantDowncastUnrelatedTypes(range, suuper, sub, failedSolves))
              } else {
                throw CompileErrorExceptionT(CouldntFindFunctionToCallT(range, e))
              }
            }
            case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(range, e))
            case Ok(x) => x
          }
        val argsExprs2 =
          convertHelper.convertExprs(
            nenv.snapshot, coutputs, range, callLocation, givenArgsExprs2, prototype.prototype.paramTypes)

        checkTypes(
          coutputs,
          nenv.snapshot,
          range,
          callLocation,
          prototype.prototype.paramTypes,
          argsExprs2.map(a => a.result.coord),
          exact = true)

        vassert(coutputs.getInstantiationBounds(prototype.prototype.id).nonEmpty)
        val resultTE =
          prototype.prototype.returnType
        ast.FunctionCallTE(prototype.prototype, argsExprs2, resultTE)
      }
      case other => {
        evaluateCustomCall(
          nenv,
          coutputs,
          life,
          range,
          callLocation,
          contextRegion,
          callableExpr.result.coord.kind,
          explicitTemplateArgRulesS,
          positionalExplicitTemplateArgRunesS,
          receivingRuneToExplicitTemplateArgRune,
          callableExpr,
          givenArgsExprs2)
      }
    }
  }

  // given args means, the args that the user gave, like in
  // a = 6;
  // f = {[a](x) print(6, x) };
  // f(4);
  // in the f(4), the given args is just 4.
  //
  // however, since f is actually a struct, it's secretly this:
  // a = 6;
  // f = {[a](x) print(6, x) };
  // f.__function(f.__closure, 4);
  // in that f.__function(f.__closure, 4), the given args is just 4, but the actual args is f.__closure and 4.
  // also, the given callable is f, but the actual callable is f.__function.

  // By "custom call" we mean calling __call.
  private def evaluateCustomCall(
    nenv: NodeEnvironmentBox,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    kind: KindT,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    positionalExplicitTemplateArgRunesS: Vector[IRuneS],
    receivingRuneToExplicitTemplateArgRune: Vector[(RuneUsage, RuneUsage)],
    givenCallableUnborrowedExpr2: ReferenceExpressionTE,
    givenArgsExprs2: Vector[ReferenceExpressionTE]):
    (FunctionCallTE) = {
    // Whether we're given a borrow or an own, the call itself will be given a borrow.
    val givenCallableBorrowExpr2 =
      givenCallableUnborrowedExpr2.result.coord match {
        case CoordT(BorrowT | ShareT, _, _) => (givenCallableUnborrowedExpr2)
        case CoordT(OwnT, _, _) => {
          localHelper.makeTemporaryLocal(
            coutputs,
            nenv,
            range,
            callLocation,
            life,
            contextRegion,
            givenCallableUnborrowedExpr2,
            BorrowT)
        }
      }

    val env = nenv.snapshot
//    val env = coutputs.getEnvForKind(kind)
//      citizenRef match {
//        case sr @ StructTT(_) => coutputs.getEnvForKind(sr) // coutputs.envByStructRef(sr)
//        case ir @ InterfaceTT(_) => coutputs.getEnvForKind(ir) // coutputs.envByInterfaceRef(ir)
//      }

    val argsTypes2 = givenArgsExprs2.map(_.result.coord)
    val closureParamType = CoordT(givenCallableBorrowExpr2.result.coord.ownership, RegionT(DefaultRegionT), kind)
    val paramFilters = Vector(closureParamType) ++ argsTypes2
    val resolved =
      overloadCompiler.findFunction(
        env,
        coutputs,
        range,
        callLocation,
        interner.intern(CodeNameS(keywords.underscoresCall)),
        explicitTemplateArgRulesS,
        positionalExplicitTemplateArgRunesS,
        receivingRuneToExplicitTemplateArgRune,
        contextRegion,
        paramFilters,
        Vector.empty,
        false) match {
        case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(range, e))
        case Ok(x) => x
      }

    val mutability = Compiler.getMutability(coutputs, kind)
    val ownership =
      mutability match {
        case MutabilityTemplataT(MutableT) => BorrowT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(idT, MutabilityTemplataType()) => BorrowT
      }
    vassert(givenCallableBorrowExpr2.result.coord.ownership == ownership)
    val actualCallableExpr2 = givenCallableBorrowExpr2

    val actualArgsExprs2 = Vector(actualCallableExpr2) ++ givenArgsExprs2

    val StampFunctionSuccess(prototype, inferences) = resolved
    val argTypes = actualArgsExprs2.map(_.result.coord)
    if (argTypes != resolved.prototype.paramTypes) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "arg param type mismatch. params: " + resolved.prototype.paramTypes + " args: " + argTypes))
    }

    checkTypes(coutputs, env, range, callLocation, resolved.prototype.paramTypes, argTypes, exact = true)

    vassert(coutputs.getInstantiationBounds(resolved.prototype.id).nonEmpty)
    val resultTE =
      prototype.returnType
    val resultingExpr2 = FunctionCallTE(resolved.prototype, actualArgsExprs2, resultTE);

    (resultingExpr2)
  }


  def checkTypes(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    params: Vector[CoordT],
    args: Vector[CoordT],
    exact: Boolean):
  Unit = {
    vassert(params.size == args.size)
    params.zip(args).foreach({ case (paramsHead, argsHead) =>
      if (paramsHead == argsHead) {

      } else {
        if (!exact) {
          templataCompiler.isTypeConvertible(coutputs, callingEnv, parentRanges, callLocation, argsHead, paramsHead) match {
            case (true) => {

            }
            case (false) => {
              // do stuff here.
              // also there is one special case here, which is when we try to hand in
              // an owning when they just want a borrow, gotta account for that here
              vfail("do stuff " + argsHead + " and " + paramsHead)
            }
          }
        } else {
          argsHead.kind match {
            case NeverT(_) => {
              // This is fine, no conversion will ever actually happen.
              // This can be seen in this call: +(5, panic())
            }
            case _ => {
              // do stuff here.
              // also there is one special case here, which is when we try to hand in
              // an owning when they just want a borrow, gotta account for that here
              vfail("do stuff " + argsHead + " and " + paramsHead)
            }
          }
        }
      }
    })
//    checkTypes(params.tail, args.tail)
//    vassert(argTypes == callableType.paramTypes, "arg param type mismatch. params: " + callableType.paramTypes + " args: " + argTypes)
  }

  def evaluatePrefixCall(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    callableReferenceExpr2: ReferenceExpressionTE,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    positionalExplicitTemplateArgRunesS: Vector[IRuneS],
    receivingRuneToExplicitTemplateArgRune: Vector[(RuneUsage, RuneUsage)],
    argsExprs2: Vector[ReferenceExpressionTE]):
  (ReferenceExpressionTE) = {
    val callExpr =
      evaluateCall(
        coutputs,
        nenv,
        life,
        range,
        callLocation,
        region,
        callableReferenceExpr2,
        explicitTemplateArgRulesS,
        positionalExplicitTemplateArgRunesS,
        receivingRuneToExplicitTemplateArgRune,
        argsExprs2)
    (callExpr)
  }
}