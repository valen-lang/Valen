use crate::utils::range::RangeS;

use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::templata::templata::*;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn get_functor_for_prototype(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &FunctionEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    drop_function: PrototypeTemplataT<'s, 't>,
  ) -> ReinterpretTE<'s, 't> {
    panic!("Unimplemented: get_functor_for_prototype");
    // vfail()
  }
}
