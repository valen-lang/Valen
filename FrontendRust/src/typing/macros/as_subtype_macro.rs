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
use crate::typing::types::types::{CoordT, RegionT, OwnershipT, ISubKindTT, ISuperKindTT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::typing::names::names::IFunctionNameT;
use crate::typing::env::environment::IInDenizenEnvironmentT;

/*
package dev.vale.typing.macros

import dev.vale.{Keywords, RangeS, StrI, vassertSome, vfail, vimpl, vwat}

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.{CantDowncastToInterface, CantDowncastUnrelatedTypes, CompileErrorExceptionT, CompilerOutputs, RangedInternalErrorT}

import dev.vale.typing.ast.{ArgLookupTE, AsSubtypeTE, BlockTE, FunctionCallTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, ReferenceExpressionTE, ReturnTE}

import dev.vale.typing.citizen.{ImplCompiler, IsParent, IsntParent}

import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.expression.ExpressionCompiler
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.env.FunctionEnvironmentBoxT
import dev.vale.typing.types.InterfaceTT
import dev.vale.typing.ast
import dev.vale.typing.function.DestructorCompiler
*/
// (Scala `class AsSubtypeMacro(keywords, implCompiler, expressionCompiler, destructorCompiler)`
//  absorbed onto `Compiler`; the method body lives at
//  `Compiler::generate_function_body_as_subtype` below.)
/*
class AsSubtypeMacro(
  keywords: Keywords,
  implCompiler: ImplCompiler,
  expressionCompiler: ExpressionCompiler,
  destructorCompiler: DestructorCompiler) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_as_subtype
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_as_subtype(
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

        let local_name: IFunctionNameT<'s, 't> = env.id.local_name.try_into().expect("vassertSome: local_name as IFunctionNameT");
        let target_kind = match local_name.template_args().first().expect("vassertSome: templateArgs.headOption") {
            ITemplataT::Coord(c) => c.coord.kind,
            _ => panic!("vwat"),
        };
        let incoming_ownership = local_name.parameters().first().expect("vassertSome: parameters.headOption").ownership;

        let incoming_coord = param_coords[0].tyype;
        let incoming_kind = incoming_coord.kind;

        // Because we dont yet put borrows in structs
        let result_ownership = incoming_ownership;
        let success_coord = CoordT { ownership: result_ownership, region: RegionT { region: IRegionT::Default }, kind: target_kind };
        let fail_coord = CoordT { ownership: result_ownership, region: RegionT { region: IRegionT::Default }, kind: incoming_kind };
        let (result_coord, ok_constructor, ok_result_impl, err_constructor, err_result_impl) =
            self.get_result(coutputs, env, call_range, call_location, RegionT { region: IRegionT::Default }, success_coord, fail_coord);
        if result_coord != maybe_ret_coord.expect("vassertSome: maybeRetCoord") {
            panic!("CompileErrorExceptionT: RangedInternalErrorT: Bad result coord");
        }

        let sub_kind = match ISubKindTT::try_from(target_kind) {
            Ok(x) => x,
            Err(_) => panic!("vwat"),
        };
        let super_kind = match ISuperKindTT::try_from(incoming_kind) {
            Ok(x) => x,
            Err(_) => panic!("vwat"),
        };

        let impl_id = match self.is_parent(
            coutputs,
            IInDenizenEnvironmentT::from(env),
            call_range,
            call_location,
            sub_kind,
            super_kind,
        ) {
            IsParentResult::IsParent(p) => p.impl_id,
            IsParentResult::IsntParent(_) => panic!("vwat"),
        };

        let as_subtype_expr = ReferenceExpressionTE::AsSubtype(self.typing_interner.alloc(AsSubtypeTE {
            source_expr: ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
                param_index: 0,
                coord: incoming_coord,
            })),
            target_type: success_coord,
            result_result_type: result_coord,
            ok_constructor: self.typing_interner.alloc(ok_constructor),
            err_constructor: self.typing_interner.alloc(err_constructor),
            impl_name: impl_id,
            ok_impl_name: ok_result_impl,
            err_impl_name: err_result_impl,
        }));

        let body = ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE {
            inner: ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE {
                source_expr: as_subtype_expr,
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

    val CoordTemplataT(CoordT(_, _, targetKind)) = vassertSome(env.id.localName.templateArgs.headOption)
    val CoordT(incomingOwnership, _, _) = vassertSome(env.id.localName.parameters.headOption)

    val incomingCoord = paramCoords(0).tyype
    val incomingKind = incomingCoord.kind

    // Because we dont yet put borrows in structs
//    val resultOwnership = incomingCoord.ownership
    val resultOwnership = incomingOwnership
    val successCoord = CoordT(resultOwnership, RegionT(DefaultRegionT), targetKind)
    val failCoord = CoordT(resultOwnership, RegionT(DefaultRegionT), incomingKind)
    val (resultCoord, okConstructor, okResultImpl, errConstructor, errResultImpl) =
      expressionCompiler.getResult(coutputs, env, callRange, callLocation, RegionT(DefaultRegionT), successCoord, failCoord)
    if (resultCoord != vassertSome(maybeRetCoord)) {
      throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "Bad result coord:\n" + resultCoord + "\nand\n" + vassertSome(maybeRetCoord)))
    }


    val subKind = targetKind match { case x : ISubKindTT => x case other => vwat(other) }

    val superKind = incomingKind match { case x : ISuperKindTT => x case other => vwat(other) }


    val implId =
      implCompiler.isParent(coutputs, env, callRange, callLocation, subKind, superKind) match {
        case IsParent(_, _, implId) => implId
      }


    val asSubtypeExpr =
      AsSubtypeTE(
        ArgLookupTE(0, incomingCoord),
        successCoord,
        resultCoord,
        okConstructor,
        errConstructor,
        implId,
        okResultImpl,
        errResultImpl)

    (header, BlockTE(ReturnTE(asSubtypeExpr)))
  }

}
*/
}
