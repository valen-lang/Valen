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
use crate::typing::env::environment::get_imprecise_name;
use crate::typing::types::types::RegionT;
use crate::typing::templata::templata::FunctionTemplataT;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::environment::IInDenizenEnvironmentT;

/*
package dev.vale.typing.macros

import dev.vale.{Err, Interner, Keywords, Ok, RangeS, StrI, vassert, vassertSome, vimpl, vpass}
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, CouldntFindFunctionToCallT, OverloadResolver, TemplataCompiler, ast}
import dev.vale.typing.ast.{AbstractT, ArgLookupTE, BlockTE, FunctionDefinitionT, FunctionHeaderT, InterfaceFunctionCallTE, LocationInFunctionEnvironmentT, ParameterT, ReturnTE}
import dev.vale.typing.env.{FunctionEnvironmentT, TemplatasStore}
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.function._
import dev.vale.typing.templata._
*/
// (Scala `class AbstractBodyMacro(interner, keywords, overloadResolver)` absorbed onto
//  `Compiler`; the method body lives at
//  `Compiler::generate_function_body_abstract_body` below.)
/*
class AbstractBodyMacro(interner: Interner, keywords: Keywords, overloadResolver: OverloadResolver) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.abstractBody
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_abstract_body(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&'s FunctionA<'s>>,
        params2: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> Result<(FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {

        let return_reference_type2 = maybe_ret_coord.expect("vassertSome: maybeRetCoord");
        assert!(params2.iter().any(|p| p.virtuality == Some(AbstractT)));
        let header = FunctionHeaderT {
            id: env.id,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            params: self.typing_interner.alloc_slice_from_vec(params2.to_vec()),
            return_type: return_reference_type2,
            maybe_origin_function_templata: origin_function.map(|f| FunctionTemplataT {
                outer_env: env.parent_env,
                function: f,
            }),
        };

        // Find self, but instead of calling it like a regular function call, call it like an interface.
        // We do this instead of grabbing the prototype out of the environment because we want to get its
        // instantiation bounds too (well, we want them to be added to the coutputs).
        // Per @DRSINI, this triggers overload resolution with 0 explicit template args and
        // placeholder-typed self arg. Defaults must not be in the initial rules or they'd
        // conflict with arg-inferred placeholders.
        let imprecise_name = get_imprecise_name(self.scout_arena, env.id.local_name)
            .expect("vassertSome: TemplatasStore.getImpreciseName env.id.localName");
        let param_types: Vec<CoordT<'s, 't>> = params2.iter().map(|p| p.tyype).collect();
        let env_as_iindenizen = self.typing_interner.alloc(IInDenizenEnvironmentT::Function(env));
        let prototype = match self.find_function(
            *env_as_iindenizen,
            coutputs,
            call_range,
            call_location,
            imprecise_name,
            &[],
            &[],
            &[],
            RegionT { region: IRegionT::Default },
            &param_types,
            &[],
            true,
        )? {
            Ok(stamp) => stamp.prototype,
            Err(_fff) => panic!("CouldntFindFunctionToCallT"),
        };

        let virtual_index = header.get_virtual_index()
            .expect("vassertSome: header.getVirtualIndex") as i32;
        let args: Vec<ReferenceExpressionTE<'s, 't>> = prototype.param_types().iter().enumerate()
            .map(|(index, param_type)| {
                ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
                    param_index: index as i32,
                    coord: *param_type,
                }))
            }).collect();
        let args_slice = self.typing_interner.alloc_slice_from_vec(args);
        let ifc = InterfaceFunctionCallTE {
            super_function_prototype: self.typing_interner.alloc(prototype),
            virtual_param_index: virtual_index,
            result_reference: prototype.return_type,
            args: args_slice,
        };
        let body = ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE {
            inner: ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE {
                source_expr: ReferenceExpressionTE::InterfaceFunctionCall(self.typing_interner.alloc(ifc)),
            })),
        }));

        Ok((header, body))
    }
/*
Guardian: temp-disable: SPDMX — Cross-section sandwich: find_function gained receivingRuneToExplicitTemplateArgRune; Rust call and audit-trail need twin update. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-796-1779427516899/hook-796/generate_function_body_abstract_body--44.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  override def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
      callLocation: LocationInDenizen,
    originFunction: Option[FunctionA],
    params2: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE) = {
    val returnReferenceType2 = vassertSome(maybeRetCoord)
    if (!params2.exists(_.virtuality == Some(AbstractT()))) {
      vpass()
    }
    vassert(params2.exists(_.virtuality == Some(AbstractT())))
    val header =
      FunctionHeaderT(
        env.id,
        Vector.empty,
        params2,
        returnReferenceType2,
        originFunction.map(FunctionTemplataT(env.parentEnv, _)))

    // Find self, but instead of calling it like a regular function call, call it like an interface.
    // We do this instead of grabbing the prototype out of the environment because we want to get its
    // instantiation bounds too (well, we want them to be added to the coutputs).
    // Per @DRSINI, this triggers overload resolution with 0 explicit template args and
    // placeholder-typed self arg. Defaults must not be in the initial rules or they'd
    // conflict with arg-inferred placeholders.
    val prototype =
      overloadResolver.findFunction(
        env,
        coutputs,
        callRange,
        callLocation,
        vassertSome(TemplatasStore.getImpreciseName(interner, env.id.localName)),
        Vector(),
        Vector(),
        Vector(),
        RegionT(DefaultRegionT),
        params2.map(_.tyype),
        Vector(),
        true) match {
        case Ok(StampFunctionSuccess(prototype, _)) => prototype
        case Err(fff @ FindFunctionFailure(_, _, _)) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, fff))
      }

    val body =
      BlockTE(
        ReturnTE(
          InterfaceFunctionCallTE(
            prototype,
            vassertSome(header.getVirtualIndex),
            prototype.returnType,
            prototype.paramTypes.zipWithIndex.map({ case (paramType, index) => ArgLookupTE(index, paramType) }))))

    (header, body)
  }
}
*/
}
