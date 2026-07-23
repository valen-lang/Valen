use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::ast::LocationInDenizen;
use crate::typing::types::types::RegionT;


impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_lock_weak(
      &self,
      coutputs: &mut CompilerOutputs<'s, 't>,
      env: &'t FunctionEnvironmentT<'s, 't>,
      generator_id: StrI<'s>,
      life: LocationInFunctionEnvironmentT<'t>,
      call_range: &[RangeS<'s>],
      call_location: LocationInDenizen<'s>,
      origin_function: Option<&FunctionS<'s>>,
      param_coords: &[ParameterT<'s, 't>],
      maybe_ret_coord: Option<KindT<'s, 't>>,
    ) -> Result<(FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {
        let header = FunctionHeaderT {
            id: env.id,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            params: self.typing_interner.alloc_slice_from_vec(param_coords.to_vec()),
            return_type: maybe_ret_coord.expect("vassertSome: maybeRetCoord"),
            maybe_origin_function_templata: Some(env.templata()),
        };
        let borrow_coord = unimplemented!();//KindT::new(OwnershipT::Borrow, param_coords[0].tyype.region, param_coords[0].tyype.kind);
        let (opt_coord, some_constructor, none_constructor, some_impl_id, none_impl_id) =
            self.get_option(coutputs, env, call_range, call_location, RegionT::Default, borrow_coord)?;
        let lock_expr = ExpressionTE::LockWeak(self.typing_interner.alloc(LockWeakTE::new(
            ExpressionTE::ArgLookup(self.typing_interner.alloc(
                ArgLookupTE::new(0, param_coords[0].tyype))),
            opt_coord,
            self.typing_interner.alloc(some_constructor),
            self.typing_interner.alloc(none_constructor),
            some_impl_id,
            none_impl_id,
        )));
        let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(
            ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(
                lock_expr,
            ))),
        )));
        Ok((header, body))
    }

}
