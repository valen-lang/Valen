use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::*;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn generate_function_body_lock_weak(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    loct: LocT<'t>,
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
    let borrow_coord = match param_coords[0].tyype {
      KindT::WeakRef(w) => KindT::BorrowRef(
        self.typing_interner.alloc(BorrowRefT { inner: w.inner}),
      ),
      other => panic!("lock's parameter must be a weak: {:?}", other),
    };
    let (opt_coord, some_constructor, none_constructor, some_impl_id, none_impl_id) =
      self.get_option(coutputs, env, call_range, call_location, RegionT::Default, borrow_coord)?;
    // This is a compiler-generated builtin body, so its nodes have no user source; the honest range is a synthesized internal one.
    let synth_range = RangeS::internal(self.scout_arena, -70020);
    let lock_expr = ExpressionTE::LockWeak(self.typing_interner.alloc(LockWeakTE::new(
      synth_range,
      ExpressionTE::ArgLookup(
        self.typing_interner.alloc(ArgLookupTE::new(synth_range, 0, param_coords[0].tyype)),
      ),
      opt_coord,
      self.typing_interner.alloc(some_constructor),
      self.typing_interner.alloc(none_constructor),
      some_impl_id,
      none_impl_id,
    )));
    let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(synth_range, ExpressionTE::Return(
      self.typing_interner.alloc(ReturnTE::new(synth_range, lock_expr)),
    ))));
    Ok((header, body))
  }
}
