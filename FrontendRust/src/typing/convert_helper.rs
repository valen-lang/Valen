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
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::overload_resolver::IFindFunctionFailureReason;

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
    ) -> Result<Vec<ReferenceExpressionTE<'s, 't>>, ICompileErrorT<'s, 't>> {
        if source_exprs.len() != target_pointer_types.len() {
            panic!(r"num exprs mismatch, source:
{:?}
target:
{:?}", source_exprs, target_pointer_types);
        }

        let mut previous_ref_exprs = Vec::new();
        for (source_expr, target_pointer_type) in source_exprs.iter().zip(target_pointer_types.iter()) {
            let ref_expr =
                self.convert(nenv, life, coutputs, range, call_location, context_region, *source_expr, *target_pointer_type)?;
            previous_ref_exprs.push(ref_expr);
        }
        Ok(previous_ref_exprs)
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
    ) -> Result<ReferenceExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        if source_expr.result().coord == target_pointer_type {
            return Ok(source_expr);
        }

        match source_expr.result().coord.kind {
            KindT::Never(_) => return Ok(source_expr),
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
                        let calling_env = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
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
                    _ => panic!("vfail: cannot convert {:?} to {:?}", source_kind, target_kind),
                }
            };

        let converted_expr =
            match (source_ownership, target_ownership) {
                (OwnershipT::Own, OwnershipT::Own) => converted_kind_expr,
                (OwnershipT::Borrow, OwnershipT::Own) => {
                    let source_coord = source_expr.result().coord;
                    let calling_env = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
                    // resolve_function's outer Err fires on internal-lookup failures;
                    // the "name not in scope" case comes back as Ok(Err(fff with rejected=[]))
                    // through the inner path (Err(fff) arm below), not the outer. No Vale
                    // program has been found that triggers the outer path here; if this
                    // panics, that's a real bug to surface.
                    let stamp_outcome = self.resolve_function(
                        calling_env, coutputs, range, call_location,
                        self.keywords.implicit_clone,
                        &[source_coord],
                        context_region, true,
                    ).expect("resolve_function(implicit_clone) outer Err is unreachable from Vale source");
                    match stamp_outcome {
                        Ok(stamp) => {
                            assert!(coutputs.get_instantiation_bounds(self.typing_interner, stamp.prototype.id).is_some());
                            let args_te = self.typing_interner.alloc_slice_from_vec(vec![converted_kind_expr]);
                            ReferenceExpressionTE::FunctionCall(self.typing_interner.alloc(FunctionCallTE {
                                callable: stamp.prototype,
                                args: args_te,
                                return_type: stamp.prototype.return_type,
                            }))
                        }
                        Err(fff) => {
                            let range_alloc = self.typing_interner.alloc_slice_copy(range);
                            // Distinguish "no user-defined implicit_clone for this kind"
                            // (all rejections are builtins for other kinds — noise) from
                            // "user defined one but got the shape wrong" (a rejection whose
                            // parameter kind matches the source kind we're probing).
                            let user_tried_for_this_kind = fff.rejected_callee_to_reason.iter().any(|(_candidate, reason)| {
                                match reason {
                                    IFindFunctionFailureReason::SpecificParamDoesntMatchExactly { parameter, .. } =>
                                        parameter.kind == source_coord.kind,
                                    IFindFunctionFailureReason::SpecificParamDoesntSend { parameter, .. } =>
                                        parameter.kind == source_coord.kind,
                                    _ => false,
                                }
                            });
                            if user_tried_for_this_kind {
                                return Err(ICompileErrorT::ImplicitCloneRejectedT {
                                    range: range_alloc,
                                    source_type: source_coord,
                                    target_type: target_pointer_type,
                                    fff,
                                });
                            } else {
                                return Err(ICompileErrorT::NoImplicitCloneDefinedT {
                                    range: range_alloc,
                                    source_type: source_coord,
                                    target_type: target_pointer_type,
                                });
                            }
                        }
                    }
                }
                // Own → Borrow materializes the temporary into a hidden local +
                // LetAndLend + deferred drop, uniformly (no is_primitive check).
                (OwnershipT::Own, OwnershipT::Borrow) => {
                    let defer = self.make_temporary_local_defer(
                        coutputs, nenv, range, call_location, life, context_region,
                        converted_kind_expr, OwnershipT::Borrow,
                    )?;
                    ReferenceExpressionTE::Defer(defer)
                }
                (OwnershipT::Borrow, OwnershipT::Borrow) => converted_kind_expr,
                (OwnershipT::Share, OwnershipT::Share) => converted_kind_expr,
                (OwnershipT::Weak, OwnershipT::Weak) => converted_kind_expr,
                // `Borrow + share-kind` → `Share` auto-alias. Both flavors point at
                // the same refcounted object; represented as an AliasTE IR node.
                (OwnershipT::Borrow, OwnershipT::Share) => {
                    ReferenceExpressionTE::Alias(self.typing_interner.alloc(AliasTE {
                        source_expr: converted_kind_expr,
                        target_ownership: OwnershipT::Share,
                    }))
                }
                _ => panic!("Supplied a {:?} but target wants {:?}", source_ownership, target_ownership),
            };

        Ok(converted_expr)
    }

}
