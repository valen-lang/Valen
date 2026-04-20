use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::expressions::ReferenceExpressionTE;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::function::function_compiler::StampFunctionSuccess;
use crate::typing::types::types::{CoordT, RegionT};
use crate::utils::range::RangeS;

/*
package dev.vale.typing.function


import dev.vale.postparsing._
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.expression.CallCompiler
import dev.vale.{Err, Interner, Keywords, Ok, PackageCoordinate, RangeS, vassert, vfail, vimpl}
import dev.vale.highertyping._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules.OwnershipLiteralSL
import dev.vale.postparsing.GlobalFunctionFamilyNameS
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, CouldntFindFunctionToCallT, OverloadResolver, RangedInternalErrorT, TypingPassOptions}
import dev.vale.typing.ast.{DiscardTE, FunctionCallTE, PrototypeT, ReferenceExpressionTE}
import dev.vale.typing.env.{GlobalEnvironment, IInDenizenEnvironmentT, PackageEnvironmentT}
import dev.vale.typing.names.{IdT, PackageTopLevelNameT}
import dev.vale.typing.types._
import dev.vale.typing.{ast, _}
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.typing.names.PackageTopLevelNameT

import scala.collection.immutable.List

*/
/*
class DestructorCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    structCompiler: StructCompiler,
    overloadCompiler: OverloadResolver) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_drop_function(
        &self,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        type_2: CoordT<'s, 't>,
    ) -> StampFunctionSuccess<'s, 't> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def getDropFunction(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    type2: CoordT):
  StampFunctionSuccess = {
    val name = interner.intern(CodeNameS(keywords.drop))
    val args = Vector(type2)
    overloadCompiler.findFunction(
      env, coutputs, callRange, callLocation, name, Vector.empty, Vector.empty, contextRegion, args, Vector(), true) match {
      case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, e))
      case Ok(x) => x
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn drop(
        &self,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        undestructed_expr_2: &'t ReferenceExpressionTE<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def drop(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    undestructedExpr2: ReferenceExpressionTE):
  (ReferenceExpressionTE) = {
    val resultExpr2 =
      undestructedExpr2.result.coord match {
        case CoordT(ShareT, _, NeverT(_)) => undestructedExpr2
        case CoordT(ShareT, _, _) => DiscardTE(undestructedExpr2)
        case r@CoordT(OwnT, _, _) => {
          val StampFunctionSuccess(destructorPrototype, _) =
            getDropFunction(env, coutputs, callRange, callLocation, RegionT(), r)
          vassert(coutputs.getInstantiationBounds(destructorPrototype.id).nonEmpty)
          val resultTT =
            destructorPrototype.returnType
          FunctionCallTE(destructorPrototype, Vector(undestructedExpr2), resultTT)
        }
        case CoordT(BorrowT, _, _) => (DiscardTE(undestructedExpr2))
        case CoordT(WeakT, _, _) => (DiscardTE(undestructedExpr2))
      }
    resultExpr2.result.coord.kind match {
      case VoidT() | NeverT(_) =>
      case _ => {
        throw CompileErrorExceptionT(
          RangedInternalErrorT(
            callRange,
            "Unexpected return type for drop autocall.\nReturn: " + resultExpr2.result.coord.kind + "\nParam: " + undestructedExpr2.result.coord))
      }
    }
    resultExpr2
  }
}
*/
}