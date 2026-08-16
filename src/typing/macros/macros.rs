use crate::interner::StrI;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::ast::ImplS;
use crate::postparsing::ast::InterfaceS;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::ast::StructS;
use crate::typing::ast::ast::FunctionHeaderT;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::ast::ast::ParameterT;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::function_environment_t::FunctionEnvironmentT;
use crate::typing::env::i_env_entry::{FunctionEnvEntry, IEnvEntryT, ImplEnvEntry, StructEnvEntry};
use crate::typing::names::names::IdT;
use crate::typing::types::types::KindT;
use crate::utils::range::RangeS;

/// An AHT (abstract high-level tree) denizen a sibling-entry macro synthesized and wants
/// registered in the postparsed cache. Macros return these instead of registering directly.
pub enum GeneratedAhtDenizen<'s, 't>
where
  's: 't,
{
  Function(&'t IdT<'s, 't>, &'s FunctionS<'s>),
  Struct(&'t IdT<'s, 't>, &'s StructS<'s>),
  Impl(&'t IdT<'s, 't>, &'s ImplS<'s>),
  // We could one day have interfaces here too
}

impl<'s, 't> GeneratedAhtDenizen<'s, 't>
where
  's: 't,
{
  pub fn template_id(&self) -> &'t IdT<'s, 't> {
    match self {
      GeneratedAhtDenizen::Function(id, _) => id,
      GeneratedAhtDenizen::Struct(id, _) => id,
      GeneratedAhtDenizen::Impl(id, _) => id,
    }
  }

  // The env entry is fully derivable from the denizen: the variant fixes the entry kind, the id
  // is the key, and a struct's tyype rides on its StructS.
  pub fn env_entry(&self) -> IEnvEntryT<'s, 't> {
    match self {
      GeneratedAhtDenizen::Function(id, _) => {
        IEnvEntryT::Function(FunctionEnvEntry { template_id: id })
      }
      GeneratedAhtDenizen::Struct(id, struct_a) => {
        IEnvEntryT::Struct(StructEnvEntry { template_id: id, tyype: struct_a.tyype })
      }
      GeneratedAhtDenizen::Impl(id, _) => IEnvEntryT::Impl(ImplEnvEntry { template_id: id }),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FunctionBodyMacro {
  LockWeak,
  AsSubtype,
  StructDrop,
  StructConstructor,
  AbstractBody,
  SameInstance,
  RsaLen,
  RsaNew,
  RsaDropInto,
  RsaCapacity,
  RsaPop,
  RsaPush,
  SsaLen,
  SsaDropInto,
}

impl FunctionBodyMacro {
  pub fn generate_function_body<'s, 'ctx, 't>(
    &self,
    compiler: &Compiler<'s, 'ctx, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    life: LocationInFunctionEnvironmentT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&'s FunctionS<'s>>,
    param_coords: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> Result<(FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>), ICompileErrorT<'s, 't>>
  where
    's: 't,
  {
    match self {
      FunctionBodyMacro::LockWeak => compiler.generate_function_body_lock_weak(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      ),
      FunctionBodyMacro::AsSubtype => compiler.generate_function_body_as_subtype(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      ),
      FunctionBodyMacro::StructDrop => compiler.generate_function_body_struct_drop(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      ),
      FunctionBodyMacro::StructConstructor => {
        Ok(compiler.generate_function_body_struct_constructor(
          coutputs,
          env,
          generator_id,
          life,
          call_range,
          call_location,
          origin_function,
          param_coords,
          maybe_ret_coord,
        ))
      }
      FunctionBodyMacro::AbstractBody => compiler.generate_function_body_abstract_body(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      ),
      FunctionBodyMacro::SameInstance => Ok(compiler.generate_function_body_same_instance(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::RsaLen => Ok(compiler.generate_function_body_rsa_len(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::RsaNew => Ok(compiler.generate_function_body_rsa_new(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::RsaDropInto => compiler.generate_function_body_rsa_drop_into(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      ),
      FunctionBodyMacro::RsaCapacity => Ok(compiler.generate_function_body_rsa_capacity(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::RsaPop => Ok(compiler.generate_function_body_rsa_pop(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::RsaPush => Ok(compiler.generate_function_body_rsa_push(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::SsaLen => Ok(compiler.generate_function_body_ssa_len(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      )),
      FunctionBodyMacro::SsaDropInto => compiler.generate_function_body_ssa_drop_into(
        coutputs,
        env,
        generator_id,
        life,
        call_range,
        call_location,
        origin_function,
        param_coords,
        maybe_ret_coord,
      ),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnStructDefinedMacro {
  StructConstructor,
  StructDrop,
}

impl OnStructDefinedMacro {
  pub fn get_struct_sibling_entries<'s, 'ctx, 't>(
    &self,
    compiler: &Compiler<'s, 'ctx, 't>,
    struct_name: IdT<'s, 't>,
    struct_a: &'s StructS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>>
  where
    's: 't,
  {
    match self {
      OnStructDefinedMacro::StructConstructor => {
        compiler.get_struct_sibling_entries_struct_constructor(struct_name, struct_a)
      }
      OnStructDefinedMacro::StructDrop => {
        compiler.get_struct_sibling_entries_struct_drop(struct_name, struct_a)
      }
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnInterfaceDefinedMacro {
  AnonymousInterface,
  InterfaceDrop,
}

impl OnInterfaceDefinedMacro {
  pub fn get_interface_sibling_entries<'s, 'ctx, 't>(
    &self,
    compiler: &Compiler<'s, 'ctx, 't>,
    interface_name: IdT<'s, 't>,
    interface_a: &'s InterfaceS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>>
  where
    's: 't,
  {
    match self {
      // VCOORD: re-enable anonymous interface macro after we do the ITypeST migration
      OnInterfaceDefinedMacro::AnonymousInterface => vec![],
      OnInterfaceDefinedMacro::InterfaceDrop => {
        compiler.get_interface_sibling_entries_interface_drop(interface_name, interface_a)
      }
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnImplDefinedMacro {}
