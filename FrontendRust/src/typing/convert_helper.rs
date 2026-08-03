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
      source_exprs: &[ExpressionTE<'s, 't>],
      target_pointer_types: &[KindT<'s, 't>],
    ) -> Result<Vec<ExpressionTE<'s, 't>>, ICompileErrorT<'s, 't>> {
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

    /// Coerces `source_expr` until its type is `target_pointer_type`, inserting whatever call
    /// or temporary that takes.
    ///
    /// For `foo(&2)` where `foo` takes an `&int`, the argument evaluates to a bare `int` but
    /// the parameter wants an `&int`. We give the 2 a hidden local to live in and lend that:
    ///   Defer(LetAndLend(ConstantInt(2)), drop)
    ///
    /// Two types can differ in their citizen, in their wraps, or in both:
    /// - A citizen difference is an upcast, e.g. `&Dog` to `&Animal`.
    /// - A wrap difference is a coercion. Each arm below is one row of the coercion table.
    pub fn convert(
      &self,
      nenv: &mut NodeEnvironmentBox<'s, 't>,
      life: LocationInFunctionEnvironmentT<'t>,
      coutputs: &mut CompilerOutputs<'s, 't>,
      range: &[RangeS<'s>],
      call_location: LocationInDenizen<'s>,
      context_region: RegionT,
      source_expr: ExpressionTE<'s, 't>,
      target_pointer_type: KindT<'s, 't>,
    ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let source_kind = source_expr.result();

        // The shapes already match, nothing to do.
        if source_kind == target_pointer_type {
            return Ok(source_expr);
        }
        // `Never` converts to anything.
        if matches!(source_kind, KindT::Never(_)) {
            return Ok(source_expr);
        }
        if matches!(target_pointer_type, KindT::Never(_)) {
            panic!("vcurious: convert targeting Never");
        }

        match (source_kind, target_pointer_type) {
            (KindT::BorrowRef(source_borrow), KindT::BorrowRef(target_borrow)) => {
                if source_borrow.inner == target_borrow.inner {
                    // Same pointee, so only the regions differ, e.g. `&r1'Ship` to `&r2'Ship`.
                    // Unreachable while every borrow carries RegionT::Default, since a matching
                    // pointee and region would have compared equal above.
                    // VCOORD: when regions get real, decide whether handing back the source is
                    // right. Its type still names the source's region, not the target's.
                    Ok(source_expr)
                } else {
                    // The borrow stays and the citizen widens, e.g. `&Dog` to `&Animal`.
                    self.convert_via_upcast(
                        nenv, coutputs, range, call_location, source_expr,
                        source_borrow.inner, target_borrow.inner)
                }
            }
            (source, KindT::BorrowRef(target_borrow)) if source == target_borrow.inner => {
                // A bare value reaching a borrow parameter, e.g. `&2`. Give it a temporary local to
                // live in, and lend that.

                let defer = self.make_temporary_local_defer(
                    coutputs, nenv, range, call_location, life, context_region, source_expr)?;
                Ok(ExpressionTE::Defer(defer))
            }
            (KindT::BorrowRef(source_borrow), target) if source_borrow.inner == target => {
                // Reading a borrow out into an owned value, e.g. `&int` to `int`. A builtin
                // covers the primitives. Any other type needs the user to define its own
                // `implicit_clone`.

                self.convert_via_implicit_clone(
                    nenv, coutputs, range, call_location, context_region,
                    source_expr, source_kind, target_pointer_type)
            }
            (source, target) if !matches!(source, KindT::BorrowRef(_)) && !matches!(target, KindT::BorrowRef(_)) => {
              // Upcast with no borrow in play: `Dog` -> `Animal`.

                self.convert_via_upcast(
                    nenv, coutputs, range, call_location, source_expr, source, target)
            }

            // VCOORD: the remaining coercion-table rows need their blankets to exist first —
            // nothing defines these blankets yet:
            // - `&@T` to `@T`, which bumps the refcount.
            // - `&weak T` to `weak T`, which bumps the weak count.
            // - `&@T` to `&T`, which peels the share ref.
            // - `&heap T` to `&T`, which peels the heap-own ref. Descoped for now.
            // Peeling a borrow, `&&T` to `&T`, stays an error on purpose. The borrow blanket
            // exists so bounds can resolve, not so callsites can silently drop a layer.
            _ => panic!("vfail: cannot convert {:?} to {:?}", source_kind, target_pointer_type),
        }
    }

    /// Upcasts `source_expr` to `target_kind` by resolving the impl that makes the source a
    /// subtype. Any wraps around the source are preserved; only the citizen changes.
    fn convert_via_upcast(
      &self,
      nenv: &mut NodeEnvironmentBox<'s, 't>,
      coutputs: &mut CompilerOutputs<'s, 't>,
      range: &[RangeS<'s>],
      call_location: LocationInDenizen<'s>,
      source_expr: ExpressionTE<'s, 't>,
      source_kind: KindT<'s, 't>,
      target_kind: KindT<'s, 't>,
    ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let range_alloc = self.typing_interner.alloc_slice_copy(range);
        let (source_sub_kind, target_super_kind) =
            match (ISubKindTT::try_from(source_kind), ISuperKindTT::try_from(target_kind)) {
                (Ok(source_sub_kind), Ok(target_super_kind)) => (source_sub_kind, target_super_kind),
                _ => {
                  // One of them isn't a citizen, e.g. converting an `int` to a `bool`. No impl could
                  // ever relate them, so there's nothing to look up.
                  return Err(ICompileErrorT::CouldntConvertT {
                    range: range_alloc,
                    source_type: source_kind,
                    target_type: target_kind,
                  })
                },
            };
        let calling_env = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
        match self.is_parent(coutputs, calling_env, range, call_location, source_sub_kind, target_super_kind) {
            IsParentResult::IsntParent(isnt_parent) => {
                // Both are citizens, but no impl relates them, e.g. a `Dog` where a `Cat` is
                // wanted. Hand back what the impl search rejected so the message can explain.
                Err(ICompileErrorT::CouldntUpcastT {
                    range: range_alloc,
                    source_type: source_kind,
                    target_type: target_kind,
                    isnt_parent,
                })
            }
          IsParentResult::IsParent(is_parent) => {
            assert!(coutputs.get_instantiation_bounds(self.typing_interner, is_parent.impl_id).is_some());
            Ok(ExpressionTE::Upcast(self.typing_interner.alloc(UpcastTE::new(
              self.typing_interner,
              source_expr,
              target_super_kind,
              is_parent.impl_id,
            ))))
          }
        }
    }

    /// Probes for an `implicit_clone(source) target` in the reachable namespaces and calls it.
    /// Absence is a user error, split three ways so the message can name the real cause.
    fn convert_via_implicit_clone(
      &self,
      nenv: &mut NodeEnvironmentBox<'s, 't>,
      coutputs: &mut CompilerOutputs<'s, 't>,
      range: &[RangeS<'s>],
      call_location: LocationInDenizen<'s>,
      context_region: RegionT,
      source_expr: ExpressionTE<'s, 't>,
      source_kind: KindT<'s, 't>,
      target_pointer_type: KindT<'s, 't>,
    ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let calling_env = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
        // resolve_function's outer Err fires on internal-lookup failures;
        // the "name not in scope" case comes back as Ok(Err(fff with rejected=[]))
        // through the inner path (Err(fff) arm below), not the outer. No Vale
        // program has been found that triggers the outer path here; if this
        // panics, that's a real bug to surface.
        let stamp_outcome = self.resolve_function(
            calling_env, coutputs, range, call_location,
            self.keywords.implicit_clone,
            &[source_kind],
            context_region, true,
        ).expect("resolve_function(implicit_clone) outer Err is unreachable from Vale source");
        match stamp_outcome {
            Ok(stamp) => {
                assert!(coutputs.get_instantiation_bounds(self.typing_interner, stamp.prototype.id).is_some());
                let args_te = self.typing_interner.alloc_slice_from_vec(vec![source_expr]);
                Ok(ExpressionTE::FunctionCall(self.typing_interner.alloc(
                    FunctionCallTE::new(stamp.prototype, args_te, stamp.prototype.return_type))))
            }
            Err(fff) => {
                let range_alloc = self.typing_interner.alloc_slice_copy(range);
                // Tell apart the two reasons the probe found nothing, so the error can name
                // the real one. If some candidate took the source's citizen but the wrong
                // shape, the user wrote an `implicit_clone` and got it wrong. Otherwise every
                // rejection is a builtin for some unrelated kind, and the user never wrote one.
                // VCOORD: this wants the innermost citizen of each side, not the whole
                // wrapped kind. A candidate taking `&Engine` should count as a try for Engine.
                let user_tried_for_this_kind = fff.rejected_callee_to_reason.iter().any(|(_candidate, reason)| {
                    match reason {
                        IFindFunctionFailureReason::SpecificParamDoesntMatchExactly { parameter, .. } =>
                            *parameter == source_kind,
                        IFindFunctionFailureReason::SpecificParamDoesntSend { parameter, .. } =>
                            *parameter == source_kind,
                        _ => false,
                    }
                });
                if user_tried_for_this_kind {
                    Err(ICompileErrorT::ImplicitCloneRejectedT {
                        range: range_alloc,
                        source_type: source_kind,
                        target_type: target_pointer_type,
                        fff,
                    })
                } else {
                    Err(ICompileErrorT::NoImplicitCloneDefinedT {
                        range: range_alloc,
                        source_type: source_kind,
                        target_type: target_pointer_type,
                    })
                }
            }
        }
    }

}
