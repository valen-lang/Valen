use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::expressions::{DiscardTE, FunctionCallTE, ExpressionTE};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::function::function_compiler::StampFunctionSuccess;
use crate::typing::types::types::{KindT, RegionT};
use crate::utils::range::RangeS;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::CodeNameS;
use crate::utils::fx::IndexMap;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_drop_function(
      &self,
      env: IInDenizenEnvironmentT<'s, 't>,
      coutputs: &mut CompilerOutputs<'s, 't>,
      call_range: &[RangeS<'s>],
      call_location: LocationInDenizen<'s>,
      context_region: RegionT,
      type_2: KindT<'s, 't>,
    ) -> Result<StampFunctionSuccess<'s, 't>, ICompileErrorT<'s, 't>> {
        let name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::CodeName(
                CodeNameS { name: self.keywords.drop }));
        let args = &[type_2];
        // ZLOOK: those three empty slices are the explicit template args, so dropping a generic
        // citizen needs T deduced from the argument — and that deduction is dead everywhere, not
        // just here. assemble_initial_sends_from_args builds exactly the argument-to-parameter
        // sends that would carry it, and all four callers bind the result and never read it. So
        // drop is one victim of a general gap rather than a special case: see
        // opt_with_undroppable_contents, and Vale4's synthesized drop<T>(Holder<T>) failing
        // SolveIncomplete with T unsolved.
        //
        // Two fixes, not exclusive. Wire the sends back up, which serves every call that omits its
        // type arguments. Or have the synthesizer write the argument here — it stands at the
        // binding holding the local's resolved type, so it is the one caller that never has to
        // infer, and Vale4's arch prescribes that shape as __vale_drop<T>(&local).
        let explicit_template_arg_rules_s = &[];
        let positional_explicit_template_arg_runes_s = &[];
        let receiving_rune_to_explicit_template_arg_rune = &[];
        let extra_envs_to_look_in = &[];
        let potential_banner = self.find_function(
            env,
            coutputs,
            call_range,
            call_location,
            name,
            explicit_template_arg_rules_s,
            positional_explicit_template_arg_runes_s,
            receiving_rune_to_explicit_template_arg_rune,
            context_region,
            args,
            extra_envs_to_look_in,
            true,
            false)?;
            // VCOORD: simplify
        match (match potential_banner {
            Err(e) => Ok(Err(e)),
            Ok(potential_banner) => {
                Ok(Ok(StampFunctionSuccess {
                    prototype: potential_banner.prototype,
                    inferences: IndexMap::default(),
                }))
            }
        })?
        {
            Err(e) => Err(ICompileErrorT::CouldntFindFunctionToCallT {
                range: self.typing_interner.alloc_slice_copy(call_range),
                fff: e,
            }),
            Ok(x) => Ok(x),
        }
    }

    pub fn drop(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        undestructed_expr_2: ExpressionTE<'s, 't>,
    ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let result_coord = undestructed_expr_2.result();
        let result_expr_2 = match result_coord {
            KindT::Never(_) => {
                // Return the original Never-typed expr, so the current block still knows that it's unreachable.
                undestructed_expr_2
            }
            KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Float(_) | KindT::USize(_) | KindT::OverloadSet(_) | KindT::BorrowRef(_) | KindT::OwnRef(_) | KindT::ShareRef(_) | KindT::WeakRef(_) => {
                // Just discard
                ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE::new(undestructed_expr_2)))
            }
            KindT::Str(_) => {
                // Discard here will drop the reference count.
                // VCOORD: at some point we'll want to have more precise instructions for the backend for this probably
                ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE::new(undestructed_expr_2)))
            }
            // Every one of these resolves `drop` by name against the value's own kind, so they share
            // one body: an interface dispatches to its abstract drop, an array to arrays.vale's
            // `drop<E>([]E)` or its StaticArray twin, and a placeholder to whatever the denizen's
            // `where func drop(T)void` bound conjured.
            KindT::Struct(_) | KindT::Interface(_) | KindT::StaticSizedArray(_)
            | KindT::RuntimeSizedArray(_) | KindT::KindPlaceholder(_) => {
                let StampFunctionSuccess { prototype: destructor_prototype, .. } =
                    self.get_drop_function(env, coutputs, call_range, call_location, RegionT::Default, result_coord)?;
                assert!(coutputs.get_instantiation_bounds(self.typing_interner, destructor_prototype.id).is_some());
                let result_tt = destructor_prototype.return_type;
                ExpressionTE::FunctionCall(self.typing_interner.alloc(FunctionCallTE::new(
                    destructor_prototype,
                    self.typing_interner.alloc_slice_from_vec(vec![undestructed_expr_2]),
                    result_tt,
                )))
            }
        };
        // let result_expr_2 = match (result_coord.ownership, result_coord.kind) {
        //     // VCOORD: doublecheck this: post-cut Share+Never is rejected by CoordT::new, so this arm should be unreachable.
        //     (OwnershipT::Share, KindT::Never(_)) => undestructed_expr_2,
        //     (OwnershipT::Share, _) => {
        //         ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE { expr: undestructed_expr_2 }))
        //     }
        //     (OwnershipT::Own, KindT::Never(_)) => undestructed_expr_2,
        //     (OwnershipT::Own, KindT::OverloadSet(_)) => {
        //         ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE { expr: undestructed_expr_2 }))
        //     }
        //     (OwnershipT::Own, kind) if self.is_primitive(kind) => {
        //         ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE { expr: undestructed_expr_2 }))
        //     }
        //     (OwnershipT::Own, _) => {
        //         let StampFunctionSuccess { prototype: destructor_prototype, .. } =
        //             self.get_drop_function(env, coutputs, call_range, call_location, RegionT::Default, result_coord)?;
        //         assert!(coutputs.get_instantiation_bounds(self.typing_interner, destructor_prototype.id).is_some());
        //         let result_tt = destructor_prototype.return_type;
        //         ExpressionTE::FunctionCall(self.typing_interner.alloc(FunctionCallTE {
        //             callable: destructor_prototype,
        //             args: self.typing_interner.alloc_slice_from_vec(vec![undestructed_expr_2]),
        //             return_type: result_tt,
        //         }))
        //     }
        //     (OwnershipT::Borrow, _) => {
        //         ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE { expr: undestructed_expr_2 }))
        //     }
        //     (OwnershipT::Weak, _) => {
        //         ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE { expr: undestructed_expr_2 }))
        //     }
        // };
        match result_expr_2.result() {
            KindT::Void(_) | KindT::Never(_) => {}
            _ => {
                panic!("Unexpected return type for drop autocall.\nReturn: {:?}\nParam: {:?}", result_expr_2.result(), undestructed_expr_2.result());
            }
        }
        Ok(result_expr_2)
    }

}