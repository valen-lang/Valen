/*
package dev.vale.typing.macros

import dev.vale.{RangeS, StrI}
import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast.{FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT}
import dev.vale.typing.env.{FunctionEnvironmentT, IEnvEntry}
import dev.vale.typing.names.{INameT, IdT}
import dev.vale.typing.types._
import dev.vale.RangeS
import dev.vale.highertyping.{FunctionA, ImplA, InterfaceA, StructA}
import dev.vale.postparsing.{LocationInDenizen, MutabilityTemplataType}
import dev.vale.typing.ast._
import dev.vale.typing.env.IEnvEntry
import dev.vale.typing.names.CitizenTemplateNameT
import dev.vale.typing.templata.ITemplataT
import dev.vale.typing.types.InterfaceTT
*/

// Dispatch-tag enum replacing Scala's IFunctionBodyMacro trait; bodies live as
// Compiler::generate_function_body_<suffix> methods.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FunctionBodyMacro {
    LockWeak,
    AsSubtype,
    StructDrop,
    StructConstructor,
    AbstractBody,
    SameInstance,
    RsaLen,
    RsaMutableNew,
    RsaImmutableNew,
    RsaDropInto,
    RsaMutableCapacity,
    RsaMutablePop,
    RsaMutablePush,
    SsaLen,
    SsaDropInto,
}
/*
trait IFunctionBodyMacro {
//  def generatorId: String
*/
impl FunctionBodyMacro {
    pub fn generate_function_body<'s, 'ctx, 't>(
        &self,
        compiler: &crate::typing::compiler::Compiler<'s, 'ctx, 't>,
        coutputs: &mut crate::typing::compiler_outputs::CompilerOutputs<'s, 't>,
        env: &'t crate::typing::env::function_environment_t::FunctionEnvironmentT<'s, 't>,
        generator_id: crate::interner::StrI<'s>,
        life: crate::typing::ast::ast::LocationInFunctionEnvironmentT<'s, 't>,
        call_range: &[crate::utils::range::RangeS<'s>],
        call_location: crate::postparsing::ast::LocationInDenizen<'s>,
        origin_function: Option<&'s crate::higher_typing::ast::FunctionA<'s>>,
        param_coords: &[crate::typing::ast::ast::ParameterT<'s, 't>],
        maybe_ret_coord: Option<crate::typing::types::types::CoordT<'s, 't>>,
    ) -> (crate::typing::ast::ast::FunctionHeaderT<'s, 't>, crate::typing::ast::expressions::ReferenceExpressionTE<'s, 't>)
    where 's: 't,
    {
        match self {
            FunctionBodyMacro::LockWeak => compiler.generate_function_body_lock_weak(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::AsSubtype => compiler.generate_function_body_as_subtype(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::StructDrop => compiler.generate_function_body_struct_drop(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::StructConstructor => compiler.generate_function_body_struct_constructor(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::AbstractBody => compiler.generate_function_body_abstract_body(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::SameInstance => compiler.generate_function_body_same_instance(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaLen => compiler.generate_function_body_rsa_len(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaMutableNew => compiler.generate_function_body_rsa_mutable_new(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaImmutableNew => compiler.generate_function_body_rsa_immutable_new(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaDropInto => compiler.generate_function_body_rsa_drop_into(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaMutableCapacity => compiler.generate_function_body_rsa_mutable_capacity(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaMutablePop => compiler.generate_function_body_rsa_mutable_pop(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::RsaMutablePush => compiler.generate_function_body_rsa_mutable_push(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::SsaLen => compiler.generate_function_body_ssa_len(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
            FunctionBodyMacro::SsaDropInto => compiler.generate_function_body_ssa_drop_into(coutputs, env, generator_id, life, call_range, call_location, origin_function, param_coords, maybe_ret_coord),
        }
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
  (FunctionHeaderT, ReferenceExpressionTE)
*/
}
/*
}
*/
// Dispatch-tag enum replacing Scala's IOnStructDefinedMacro trait; bodies live on impl Compiler.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnStructDefinedMacro {
    StructConstructor,
    StructDrop,
}
/*
trait IOnStructDefinedMacro {
  def getStructSiblingEntries(
    structName: IdT[INameT], structA: StructA):
  Vector[(IdT[INameT], IEnvEntry)]
}
*/
// Dispatch-tag enum replacing Scala's IOnInterfaceDefinedMacro trait; bodies live on impl Compiler.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnInterfaceDefinedMacro {
    AnonymousInterface,
    InterfaceDrop,
}
/*
trait IOnInterfaceDefinedMacro {
  def getInterfaceSiblingEntries(
    interfaceName: IdT[INameT], interfaceA: InterfaceA):
  Vector[(IdT[INameT], IEnvEntry)]
}
*/
// Dispatch-tag enum replacing Scala's IOnImplDefinedMacro trait; bodies live on impl Compiler.
// (No concrete implementors in the current codebase — Scala initializes this map empty.)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnImplDefinedMacro {}
/*
trait IOnImplDefinedMacro {
  def getImplSiblingEntries(implName: IdT[INameT], implA: ImplA):
  Vector[(IdT[INameT], IEnvEntry)]
}
*/
