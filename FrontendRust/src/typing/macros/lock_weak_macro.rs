use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::higher_typing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;
use crate::postparsing::ast::LocationInDenizen;
use crate::typing::types::types::RegionT;

/*
package dev.vale.typing.macros

import dev.vale.{Keywords, RangeS, StrI, vimpl}

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast.{ArgLookupTE, BlockTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, LockWeakTE, ParameterT, ReturnTE}

import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.expression.ExpressionCompiler
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.types._
import dev.vale.typing.ast

*/
// (Scala `class LockWeakMacro(keywords, expressionCompiler)` absorbed onto `Compiler`;
//  the method body lives at `Compiler::generate_function_body_lock_weak` below.)
/*
class LockWeakMacro(
  keywords: Keywords,
  expressionCompiler: ExpressionCompiler
) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_lock_weak

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_lock_weak(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&FunctionA<'s>>,
        param_coords: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        let header = FunctionHeaderT {
            id: env.id,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            params: self.typing_interner.alloc_slice_from_vec(param_coords.to_vec()),
            return_type: maybe_ret_coord.expect("vassertSome: maybeRetCoord"),
            maybe_origin_function_templata: Some(env.templata()),
        };
        let borrow_coord = CoordT { ownership: OwnershipT::Borrow, ..param_coords[0].tyype };
        let (opt_coord, some_constructor, none_constructor, some_impl_id, none_impl_id) =
            self.get_option(coutputs, env, call_range, call_location, RegionT { region: IRegionT::Default }, borrow_coord);
        let lock_expr = ReferenceExpressionTE::LockWeak(self.typing_interner.alloc(LockWeakTE {
            inner_expr: ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
                param_index: 0,
                coord: param_coords[0].tyype,
            })),
            result_opt_borrow_type: opt_coord,
            some_constructor: self.typing_interner.alloc(some_constructor),
            none_constructor: self.typing_interner.alloc(none_constructor),
            some_impl_name: some_impl_id,
            none_impl_name: none_impl_id,
        }));
        let body = ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE {
            inner: ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE {
                source_expr: lock_expr,
            })),
        }));
        (header, body)
    }
/*
  def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE) = {
    val header =
      FunctionHeaderT(env.id, Vector.empty, paramCoords, maybeRetCoord.get, Some(env.templata))

    val borrowCoord = paramCoords.head.tyype.copy(ownership = BorrowT)
    val (optCoord, someConstructor, noneConstructor, someImplId, noneImplId) =
      expressionCompiler.getOption(coutputs, env, callRange, callLocation, RegionT(DefaultRegionT), borrowCoord)
    val lockExpr =
      LockWeakTE(
        ArgLookupTE(0, paramCoords.head.tyype),
        optCoord,
        someConstructor,
        noneConstructor,
        someImplId,
        noneImplId)

    val body = BlockTE(ReturnTE(lockExpr))

    (header, body)
  }

}
*/
}
