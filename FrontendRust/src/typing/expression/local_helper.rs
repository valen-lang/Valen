use crate::postparsing::ast::LocationInDenizen;
use crate::typing::compiler::Compiler;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::expressions::*;
use crate::postparsing::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::compiler_outputs::*;
use crate::parsing::ast::*;
use crate::interner::Interner;
use crate::typing::names::names::TypingPassTemporaryVarNameT;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::function_environment_t::LocalVariable;


impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_temporary_local(&self, nenv: &mut NodeEnvironmentBox<'s, 't>, life: LocationInFunctionEnvironmentT<'t>, coord: KindT<'s, 't>) -> LocalVariable<'s, 't> {
        let var_id = self.typing_interner.intern_typing_pass_temporary_var_name(
            TypingPassTemporaryVarNameT { life });
        let rlv = LocalVariable { name: var_id.into(), tyype: coord };
        nenv.add_variable(IVariableT::Local(rlv));
        rlv
    }

    pub fn make_temporary_local_defer(&self, coutputs: &mut CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], call_location: LocationInDenizen<'s>, life: LocationInFunctionEnvironmentT<'t>, context_region: RegionT, r: ExpressionTE<'s, 't>) -> Result<&'t DeferTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let rlv = self.make_temporary_local(nenv, life, r.result());
        let let_expr_2 = ExpressionTE::LetAndLend(self.typing_interner.alloc(LetAndLendTE::new(
            self.typing_interner,
            rlv,
            r,
        )));
        let unlet = self.unlet_local_without_dropping(nenv, &rlv);
        let unlet_te: ExpressionTE<'s, 't> = ExpressionTE::Unlet(self.typing_interner.alloc(unlet));
        let snapshot: &'t NodeEnvironmentT<'s, 't> = nenv.snapshot(self.typing_interner);
        let env_in_denizen: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Node(snapshot);
        let destruct_expr_2 = self.drop(env_in_denizen, coutputs, range, call_location, context_region, unlet_te)?;
        assert_eq!(destruct_expr_2.result(), KindT::Void(VoidT));
        Ok(self.typing_interner.alloc(DeferTE::new(let_expr_2, destruct_expr_2)))
    }

    pub fn unlet_local_without_dropping(&self, nenv: &mut NodeEnvironmentBox<'s, 't>, local_var: &LocalVariable<'s, 't>) -> UnletTE<'s, 't> {
        nenv.mark_local_unstackified(local_var.name);
        UnletTE::new(*local_var)
    }

    pub fn unlet_and_drop_all(&self, coutputs: &mut CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], call_location: LocationInDenizen<'s>, context_region: RegionT, variables: &[&LocalVariable<'s, 't>]) -> Result<Vec<ExpressionTE<'s, 't>>, ICompileErrorT<'s, 't>> {
        variables.iter().map(|variable| {
            let unlet = self.unlet_local_without_dropping(nenv, variable);
            let unlet_ref = ExpressionTE::Unlet(self.typing_interner.alloc(unlet));
            let snapshot = nenv.snapshot(self.typing_interner);
            let snapshot_env = IInDenizenEnvironmentT::Node(snapshot);
            self.drop(snapshot_env, coutputs, range, call_location, context_region, unlet_ref)
        }).collect()
    }

    pub fn unlet_all_without_dropping(&self, _coutputs: &CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, _range: &[RangeS<'s>], variables: &[&LocalVariable<'s, 't>]) -> Vec<ExpressionTE<'s, 't>> {
        variables.iter().map(|variable| {
            ExpressionTE::Unlet(self.typing_interner.alloc(self.unlet_local_without_dropping(nenv, variable)))
        }).collect()
    }

    pub fn make_user_local_variable(&self, coutputs: &CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, range: &[RangeS<'s>], local_variable_a: &'s LocalS<'s>, reference_type2: KindT<'s, 't>) -> LocalVariable<'s, 't> {
        let var_id = self.translate_var_name_step(local_variable_a.var_name);

        if nenv.get_variable(var_id, self.typing_interner).is_some() {
            panic!("There's already a variable named {:?}", var_id);
        }

        let mutable = self.get_sharedness(coutputs, reference_type2);
        let local_var =
            LocalVariable {
                name: var_id,
                tyype: reference_type2,
            };
        nenv.add_variable(IVariableT::from(local_var));
        local_var
    }

    // pub fn maybe_borrow_soft_load(&self, coutputs: &CompilerOutputs<'s, 't>, expr2: &ExpressionTE<'s, 't>) -> ExpressionTE<'s, 't> {
    //     match expr2 {
    //         ExpressionTE::Reference(e) => *e,
    //         ExpressionTE::Address(e) => self.borrow_soft_load(coutputs, *e),
    //     }
    // }

    // pub fn soft_load(&self, nenv: &mut NodeEnvironmentBox<'s, 't>, load_range: &[RangeS<'s>], a: ExpressionTE<'s, 't>, load_as_p: LoadAsP, region: RegionT) -> ExpressionTE<'s, 't> {
    //     match a.result().ownership {
    //         OwnershipT::Share => {
    //             match load_as_p {
    //                 // VCOORD: revisit
    //                 // Bare-use of a Share local produces `Borrow + share-kind`. The
    //                 // (Borrow, Share) auto-alias in convert() reflavors via AliasTE at
    //                 // target boundaries that want Share.
    //                 LoadAsP::Use => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow })),
    //                 LoadAsP::Move => {
    //                     match a {
    //                         ExpressionTE::LocalLookup(ref lv_lookup) => {
    //                             nenv.mark_local_unstackified(lv_lookup.local_variable.name());
    //                             ExpressionTE::Unlet(self.typing_interner.alloc(UnletTE { variable: lv_lookup.local_variable }))
    //                         }
    //                         ExpressionTE::ReferenceMemberLookup(ref r) => {
    //                             panic!("unimplemented: {:?}", r.member_name);
    //                         }
    //                         ExpressionTE::AddressMemberLookup(ref r) => {
    //                             panic!("unimplemented: {:?}", r.member_name);
    //                         }
    //                         _ => {
    //                             unreachable!("OwnT+MoveP arm only matches LocalLookupTE/ReferenceMemberLookupTE/AddressMemberLookupTE");
    //                         }
    //                     }
    //                 }
    //                 LoadAsP::LoadAsBorrow => {
    //                     ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
    //                 }
    //                 LoadAsP::LoadAsWeak => {
    //                     ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Weak }))
    //                 }
    //             }
    //         }
    //         OwnershipT::Own => {
    //             match load_as_p {
    //                 LoadAsP::Use => {
    //                     match a {
    //                         ExpressionTE::LocalLookup(ref lv_lookup) => {
    //                             nenv.mark_local_unstackified(lv_lookup.local_variable.name());
    //                             ExpressionTE::Unlet(self.typing_interner.alloc(UnletTE { variable: lv_lookup.local_variable }))
    //                         }
    //                         ExpressionTE::RuntimeSizedArrayLookup(_) => {
    //                             ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
    //                         }
    //                         ExpressionTE::StaticSizedArrayLookup(_) => {
    //                             ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
    //                         }
    //                         ExpressionTE::ReferenceMemberLookup(_) => {
    //                             ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
    //                         }
    //                         ExpressionTE::AddressMemberLookup(_) => {
    //                             ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
    //                         }
    //                     }
    //                 }
    //                 LoadAsP::Move => {
    //                     match a {
    //                         ExpressionTE::LocalLookup(ref lv_lookup) => {
    //                             nenv.mark_local_unstackified(lv_lookup.local_variable.name());
    //                             ExpressionTE::Unlet(self.typing_interner.alloc(UnletTE { variable: lv_lookup.local_variable }))
    //                         }
    //                         ExpressionTE::ReferenceMemberLookup(ref r) => {
    //                             panic!("CantMoveOutOfMemberT: {:?}", r.member_name);
    //                         }
    //                         ExpressionTE::AddressMemberLookup(ref r) => {
    //                             panic!("CantMoveOutOfMemberT: {:?}", r.member_name);
    //                         }
    //                         _ => {
    //                             unreachable!("OwnT+MoveP arm only matches LocalLookupTE/ReferenceMemberLookupTE/AddressMemberLookupTE");
    //                         }
    //                     }
    //                 }
    //                 LoadAsP::LoadAsBorrow => {
    //                     ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow }))
    //                 }
    //                 LoadAsP::LoadAsWeak => {
    //                     ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Weak }))
    //                 }
    //             }
    //         }
    //         OwnershipT::Borrow => {
    //             match load_as_p {
    //                 LoadAsP::Move => panic!("vfail: soft_load BorrowT + MoveP"),
    //                 LoadAsP::Use => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: a.result().ownership })),
    //                 LoadAsP::LoadAsBorrow => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Borrow })),
    //                 LoadAsP::LoadAsWeak => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Weak })),
    //             }
    //         }
    //         OwnershipT::Weak => {
    //             match load_as_p {
    //                 LoadAsP::Use => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Weak })),
    //                 LoadAsP::Move => panic!("vfail: soft_load WeakT + MoveP"),
    //                 LoadAsP::LoadAsBorrow => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Weak })),
    //                 LoadAsP::LoadAsWeak => ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: a, target_ownership: OwnershipT::Weak })),
    //             }
    //         }
    //     }
    // }
    //
    // pub fn borrow_soft_load(&self, coutputs: &CompilerOutputs<'s, 't>, expr2: ExpressionTE<'s, 't>) -> ExpressionTE<'s, 't> {
    //     let ownership = self.get_borrow_ownership(coutputs, expr2.result());
    //     ExpressionTE::SoftLoad(self.typing_interner.alloc(SoftLoadTE { expr: expr2, target_ownership: ownership }))
    // }
    //
    // pub fn get_borrow_ownership(&self, coutputs: &CompilerOutputs<'s, 't>, kind: KindT<'s, 't>) -> OwnershipT {
    //     match kind {
    //         // VCOORD: doublecheck this: post-cut Int/Bool/Float/Void are Own (not Share); returning Share here is what forces the instantiator's (Share, Own)/(Share, MutableBorrow) paper-over arms.
    //         KindT::Int(_) => OwnershipT::Share,
    //         KindT::Bool(_) => OwnershipT::Share,
    //         KindT::Float(_) => OwnershipT::Share,
    //         KindT::Str(_) => OwnershipT::Share,
    //         KindT::Void(_) => OwnershipT::Share,
    //         KindT::StaticSizedArray(_) | KindT::RuntimeSizedArray(_) | KindT::KindPlaceholder(_) | KindT::Struct(_) | KindT::Interface(_) => {
    //             match self.get_sharedness(coutputs, kind) {
    //                 SharednessT::Single => OwnershipT::Borrow,
    //                 SharednessT::Shared => OwnershipT::Share,
    //             }
    //         }
    //         KindT::OverloadSet(_) => OwnershipT::Own,
    //         KindT::Never(_) => panic!("implement: get_borrow_ownership Never"),
    //     }
    // }

    // // See ClosureTests for requirements here
    // pub fn determine_if_local_is_addressible(
    //     sharedness: SharednessT,
    //     local_a: &'s LocalS<'s>,
    // ) -> bool {
    //     match sharedness {
    //         SharednessT::Single => {
    //             local_a.child_mutated != IVariableUseCertainty::NotUsed || local_a.child_moved != IVariableUseCertainty::NotUsed
    //         }
    //         SharednessT::Shared => {
    //             local_a.child_mutated != IVariableUseCertainty::NotUsed
    //         }
    //     }
    // }

    
}
