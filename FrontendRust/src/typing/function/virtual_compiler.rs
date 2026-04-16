/*
package dev.vale.typing.function

import dev.vale.Interner
import dev.vale.typing.citizen.StructCompiler
import dev.vale.postparsing.GlobalFunctionFamilyNameS
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.{OverloadResolver, TypingPassOptions}
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env.TemplatasStore
import dev.vale.Err

import scala.collection.immutable.List
*/
use std::collections::{HashMap, HashSet};

use crate::interner::StrI;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compilation::*;
use crate::interner::Interner;
use crate::typing::overload_resolver::*;
use crate::postparsing::itemplatatype::ITemplataType;

// mig: struct VirtualCompiler
pub struct VirtualCompiler<'s, 'ctx, 't> {
    opts: TypingPassOptions<'s>,
    interner: &'ctx Interner<'s>,
    overload_compiler: OverloadResolver<'s, 't>,
}
// mig: impl VirtualCompiler
impl<'s, 'ctx, 't> VirtualCompiler<'s, 'ctx, 't> {}
/*
class VirtualCompiler(opts: TypingPassOptions, interner: Interner, overloadCompiler: OverloadResolver) {
//  // See Virtuals doc for this function's purpose.
//  // For the "Templated parent case"
//  def evaluateParent(
//    env: IEnvironment, coutputs: CompilerOutputs, callRange: List[RangeS], sparkHeader: FunctionHeaderT):
//  Unit = {
//    vassert(sparkHeader.params.count(_.virtuality.nonEmpty) <= 1)
//    val maybeSuperInterfaceAndIndex =
//      sparkHeader.params.zipWithIndex.collectFirst({
//        case (ParameterT(_, Some(OverrideT(ir)), CoordT(_, _, StructTT(_))), index) => (ir, index)
//      })
//
//    maybeSuperInterfaceAndIndex match {
//      case None => {
//        // It's not an override, so nothing to do here.
//
//      }
//      case Some((superInterfaceRef2, virtualIndex)) => {
//        val overrideFunctionParamTypes = sparkHeader.params.map(_.tyype)
//        val needleSuperFunctionParamTypes =
//          overrideFunctionParamTypes.zipWithIndex.map({ case (paramType, index) =>
//            if (index != virtualIndex) {
//              paramType
//            } else {
//              paramType.copy(kind = superInterfaceRef2)
//            }
//          })
//
//        val needleSuperFunctionParamFilters =
//          needleSuperFunctionParamTypes.zipWithIndex.map({
//            case (needleSuperFunctionParamType, index) => {
//              ParamFilter(needleSuperFunctionParamType, if (index == virtualIndex) Some(AbstractT()) else None)
//            }
//          })
//
//        val nameToScoutFor =
//          vassertSome(TemplatasStore.getImpreciseName(interner, sparkHeader.fullName.last))
//
//        // See MLIOET
//        val superInterfaceEnv = coutputs.getEnvForKind(superInterfaceRef2)
//        val extraEnvsToLookIn = Vector(superInterfaceEnv)
//
//        // Throw away the result prototype, we just want it to be in the coutputs.
//
//        overloadCompiler.findFunction(
//          env,
//          coutputs,
//          callRange,
//          nameToScoutFor,
//          Vector.empty,
//          Vector.empty, needleSuperFunctionParamFilters, extraEnvsToLookIn, true) match {
//          case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, e))
//          case Ok(x) => x
//        }
//      }
//    }
//  }
}
*/
