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
use crate::typing::types::types::{KindT, RegionT, ISubKindTT, ISuperKindTT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::typing::names::names::IFunctionNameT;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::templata_compiler::{peel_all_references, replace_value_type_in_ref};
use crate::typing::infer_compiler::IResolvingError;


impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_as_subtype(
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

        let local_name: IFunctionNameT<'s, 't> = env.id.local_name.try_into().expect("vassertSome: local_name as IFunctionNameT");
        let target_kind = match local_name.template_args().first().expect("vassertSome: templateArgs.headOption") {
            ITemplataT::Kind(c) => c.kind,
            _ => panic!("vwat"),
        };
        // let incoming_ownership = local_name.parameters().first().expect("vassertSome: parameters.headOption").ownership;

        let incoming_coord = param_coords[0].tyype;
        // The declared param is `&SuperType` or `SuperType`, so the citizen sits under whatever wraps
        // the signature wrote. is_parent and ISuperKindTT below want the citizen, not the reference.
        let incoming_kind = peel_all_references(incoming_coord);

        let success_coord = replace_value_type_in_ref(self.typing_interner, incoming_coord, target_kind);
        let fail_coord = incoming_coord;
        let (result_coord, ok_constructor, ok_result_impl, err_constructor, err_result_impl) =
            self.get_result(coutputs, env, call_range, call_location, RegionT::Default, success_coord, fail_coord)?;
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
            IsParentResult::IsntParent(isnt) => {
                return Err(ICompileErrorT::CantDowncastUnrelatedTypes {
                    range: self.typing_interner.alloc_slice_copy(call_range),
                    source_kind: incoming_kind,
                    target_kind,
                    candidates: self.typing_interner.alloc_slice_from_vec(isnt.candidates),
                });
            }
        };

        let as_subtype_expr = ExpressionTE::AsSubtype(self.typing_interner.alloc(AsSubtypeTE::new(
            ExpressionTE::ArgLookup(self.typing_interner.alloc(
                ArgLookupTE::new(0, incoming_coord))),
            success_coord,
            result_coord,
            self.typing_interner.alloc(ok_constructor),
            self.typing_interner.alloc(err_constructor),
            impl_id,
            ok_result_impl,
            err_result_impl,
        )));

        let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(
            ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(
                as_subtype_expr,
            ))),
        )));
        Ok((header, body))
    }

}
