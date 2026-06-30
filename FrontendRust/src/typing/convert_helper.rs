use crate::utils::range::RangeS;

use crate::typing::types::types::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::compiler_outputs::*;
use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::compiler::Compiler;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::typing::ast::expressions::UpcastTE;
use crate::typing::env::function_environment_t::NodeEnvironmentBox;

// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)



impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn convert_exprs(
        &self,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        source_exprs: &[ReferenceExpressionTE<'s, 't>],
        target_pointer_types: &[CoordT<'s, 't>],
    ) -> Vec<ReferenceExpressionTE<'s, 't>> {
        if source_exprs.len() != target_pointer_types.len() {
            panic!(r"num exprs mismatch, source:
{:?}
target:
{:?}", source_exprs, target_pointer_types);
        }

        let mut previous_ref_exprs = Vec::new();
        for (source_expr, target_pointer_type) in source_exprs.iter().zip(target_pointer_types.iter()) {
            let ref_expr =
                self.convert(nenv, life, coutputs, range, call_location, context_region, *source_expr, *target_pointer_type);
            previous_ref_exprs.push(ref_expr);
        }
        previous_ref_exprs
    }

    pub fn convert(
        &self,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        source_expr: ReferenceExpressionTE<'s, 't>,
        target_pointer_type: CoordT<'s, 't>,
    ) -> ReferenceExpressionTE<'s, 't> {
        if source_expr.result().coord == target_pointer_type {
            return source_expr;
        }

        match source_expr.result().coord.kind {
            KindT::Never(_) => return source_expr,
            _ => {}
        }

        let target_ownership = target_pointer_type.ownership;
        let target_kind = target_pointer_type.kind;
        let source_ownership = source_expr.result().coord.ownership;
        let source_kind = source_expr.result().coord.kind;

        match target_kind {
            KindT::Never(_) => panic!("vcurious: convert targeting Never"),
            _ => {}
        }

        let converted_kind_expr =
            if source_kind == target_kind {
                source_expr
            } else {
                match (ISubKindTT::try_from(source_kind), ISuperKindTT::try_from(target_kind)) {
                    (Ok(source_sub_kind), Ok(target_super_kind)) => {
                        self.convert_with_subkind(
                            IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner)),
                            coutputs, range, call_location, source_expr, source_sub_kind, target_super_kind)
                    }
                    _ => panic!("vfail: cannot convert {:?} to {:?}", source_kind, target_kind),
                }
            };

        let converted_expr =
            match (source_ownership, target_ownership) {
                (OwnershipT::Own, OwnershipT::Own) => converted_kind_expr,
                (OwnershipT::Borrow, OwnershipT::Own) => panic!("Supplied a borrow but target wants to own the argument"),
                (OwnershipT::Own, OwnershipT::Borrow) => panic!("Supplied an owning but target wants to only borrow"),
                (OwnershipT::Borrow, OwnershipT::Borrow) => converted_kind_expr,
                (OwnershipT::Share, OwnershipT::Share) => converted_kind_expr,
                (OwnershipT::Weak, OwnershipT::Weak) => converted_kind_expr,
                _ => panic!("Supplied a {:?} but target wants {:?}", source_ownership, target_ownership),
            };

        converted_expr
    }

    pub fn convert_with_subkind(
        &self,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_expr: ReferenceExpressionTE<'s, 't>,
        source_sub_kind: ISubKindTT<'s, 't>,
        target_super_kind: ISuperKindTT<'s, 't>,
    ) -> ReferenceExpressionTE<'s, 't> {
        match self.is_parent(coutputs, calling_env, range, call_location, source_sub_kind, target_super_kind) {
            IsParentResult::IsParent(is_parent) => {
                assert!(coutputs.get_instantiation_bounds(self.typing_interner, is_parent.impl_id).is_some());
                ReferenceExpressionTE::Upcast(self.typing_interner.alloc(UpcastTE {
                    inner_expr: source_expr,
                    target_super_kind,
                    impl_name: is_parent.impl_id,
                }))
            }
            IsParentResult::IsntParent(_candidates) => {
                panic!("Can't upcast a {:?} to a {:?}", source_sub_kind, target_super_kind)
            }
        }
    }

}
