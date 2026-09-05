use crate::interner::Interner;
use crate::parsing::ast::*;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::expressions::*;
use crate::postparsing::names::*;
use crate::postparsing::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::TypingPassTemporaryVarNameT;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;
use crate::utils::range::RangeS;
use std::mem;
use std::thread;
use crate::utils::drop_bomb::DropBomb;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn make_temporary_local(
    &self,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    loct: LocT<'t>,
    coord: KindT<'s, 't>,
  ) -> &'t LocalVariable<'s, 't> {
    let var_id = self
      .typing_interner
      .intern_typing_pass_temporary_var_name(TypingPassTemporaryVarNameT { loct: loct });
    let rlv: &'t LocalVariable<'s, 't> =
      self.typing_interner.alloc(LocalVariable { name: var_id.into(), tyype: coord });
    nenv.add_variable(IVariableT::Local(rlv));
    rlv
  }

  pub fn make_temporary_local_borrow(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    loct: LocT<'t>,
    context_region: RegionT,
    r: ExpressionTE<'s, 't>,
  ) -> Result<
    (&'t LetAndLendTE<'s, 't>, PendingTempDrops<'s, 't>),
    ICompileErrorT<'s, 't>
  > {
    let rlv = self.make_temporary_local(nenv, loct, r.result());
    let let_expr_2 = self.typing_interner.alloc(LetAndLendTE::new(
      self.typing_interner,
      rlv,
      r,
    ));
    Ok((let_expr_2, PendingTempDrops::of_one(rlv)))
  }

  pub fn unlet_local_without_dropping(
    &self,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    local_var: &'t LocalVariable<'s, 't>,
  ) -> UnletTE<'s, 't> {
    nenv.mark_local_unstackified(local_var.name);
    UnletTE::new(local_var)
  }

  pub fn unlet_and_drop_all(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    context_region: RegionT,
    variables: &[&'t LocalVariable<'s, 't>],
  ) -> Result<Vec<ExpressionTE<'s, 't>>, ICompileErrorT<'s, 't>> {
    variables
      .iter()
      .map(|variable| {
        let unlet = self.unlet_local_without_dropping(nenv, *variable);
        let unlet_ref = ExpressionTE::Unlet(self.typing_interner.alloc(unlet));
        let snapshot = nenv.snapshot(self.typing_interner);
        let snapshot_env = IInDenizenEnvironmentT::Node(snapshot);
        self.drop(snapshot_env, coutputs, range, call_location, context_region, unlet_ref)
      })
      .collect()
  }

  pub fn unlet_all_without_dropping(
    &self,
    _coutputs: &CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    _range: &[RangeS<'s>],
    variables: &[&'t LocalVariable<'s, 't>],
  ) -> Vec<ExpressionTE<'s, 't>> {
    variables
      .iter()
      .map(|variable| {
        ExpressionTE::Unlet(
          self.typing_interner.alloc(self.unlet_local_without_dropping(nenv, *variable)),
        )
      })
      .collect()
  }

  pub fn make_user_local_variable(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    range: &[RangeS<'s>],
    var_name: IVarDeclarationNameS<'s>,
    reference_type2: KindT<'s, 't>,
  ) -> &'t LocalVariable<'s, 't> {
    let var_id = self.translate_var_name_step(var_name);

    let imprecise = var_name.imprecise_name(self.scout_arena);
    if nenv.get_variable(imprecise, self.typing_interner).is_some() {
      panic!("There's already a variable named {:?}", var_id);
    }

    let local_var: &'t LocalVariable<'s, 't> =
      self.typing_interner.alloc(LocalVariable { name: var_id, tyype: reference_type2 });
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
  //                         ExpressionTE::MemberLookup(ref r) => {
  //                             panic!("unimplemented: {:?}", r.member_name);
  //                         }
  //                         ExpressionTE::AddressMemberLookup(ref r) => {
  //                             panic!("unimplemented: {:?}", r.member_name);
  //                         }
  //                         _ => {
  //                             unreachable!("OwnT+MoveP arm only matches LocalLookupTE/MemberLookupTE/AddressMemberLookupTE");
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
  //                         ExpressionTE::MemberLookup(_) => {
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
  //                         ExpressionTE::MemberLookup(ref r) => {
  //                             panic!("CantMoveOutOfMemberT: {:?}", r.member_name);
  //                         }
  //                         ExpressionTE::AddressMemberLookup(ref r) => {
  //                             panic!("CantMoveOutOfMemberT: {:?}", r.member_name);
  //                         }
  //                         _ => {
  //                             unreachable!("OwnT+MoveP arm only matches LocalLookupTE/MemberLookupTE/AddressMemberLookupTE");
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

/// A **linear obligation** carrying the temporary locals created to hold borrowed rvalues (see
/// `make_temporary_local_borrow`). Each such temp is stackified into the environment and MUST later be
/// either drained — `drain_pending_temp_drops` builds its `unlet + drop` — or, on a diverging path,
/// declined via `discard_pending_temp_drops_past_never` (unlet without dropping). Forgetting it leaks
/// the temp: `drop_since` would try to drop it again at block end, and the stackifier would flag it.
///
/// Enforcement is as close to linear as affine Rust allows: the type is move-only (no `Copy`/`Clone`)
/// and `#[must_use]`, so it cannot be silently ignored or duplicated, and the embedded `DropBomb`
/// (armed whenever a temp is outstanding) fires if a non-empty one falls out of scope. Sub-expression
/// obligations are combined with `absorb`, which consumes the child.
///
/// CAVEAT: the bomb fires on any armed drop that is not already unwinding a panic — a `?` early-return
/// on a compile error included. Keep a live non-empty token from spanning a fallible call: `absorb` it
/// into the accumulator you return (or drain it) before any `?`. The airtight complement is to seal
/// the function body so it can only be built from an empty token.
#[must_use = "pending temp-drops must be drained, discarded past a Never, or propagated up"]
pub struct PendingTempDrops<'s, 't>
where
  's: 't,
{
  vars: Vec<&'t LocalVariable<'s, 't>>,
  bomb: DropBomb,
}

const PENDING_TEMP_DROPS_MESSAGE: &str =
  "PendingTempDrops dropped with an outstanding temp-drop — drain or discard-past-Never \
   before it leaves scope";

impl<'s, 't> PendingTempDrops<'s, 't>
where
  's: 't,
{
  pub fn none() -> PendingTempDrops<'s, 't> {
    let bomb = DropBomb::armed(PENDING_TEMP_DROPS_MESSAGE);
    PendingTempDrops { vars: Vec::new(), bomb }
  }

  /// One outstanding temp-drop — what `make_temporary_local_borrow` produces for a single borrow.
  pub fn of_one(temp: &'t LocalVariable<'s, 't>) -> PendingTempDrops<'s, 't> {
    PendingTempDrops { vars: vec![temp], bomb: DropBomb::armed(PENDING_TEMP_DROPS_MESSAGE) }
  }

  /// Add one more temp to the obligation.
  pub fn push(&mut self, temp: &'t LocalVariable<'s, 't>) {
    self.vars.push(temp);
    self.bomb.arm();
  }

  /// Deliberately abandon this obligation without draining it, because this path is aborting the
  /// compile (returning an `ICompileErrorT`): no tree is produced, so there is nothing to drain into.
  /// This is the *only* sanctioned way to drop a token without discharging it — an accidental drop
  /// still trips the bomb. Use it exactly at error returns, never to sidestep a real drain.
  pub fn defuse_on_error(mut self) {
    self.bomb.defuse();
  }

  /// Merge a sub-expression's obligations into this one, consuming the child (whose bomb is defused
  /// as it is taken over). Preserves order: `child`'s temps land after ours, so a later drain reversal
  /// yields overall LIFO.
  pub fn absorb(&mut self, mut child: PendingTempDrops<'s, 't>) {
    if !child.vars.is_empty() {
      self.bomb.arm();
    }
    child.bomb.defuse();
    self.vars.append(&mut child.vars);
  }

  /// Consume the obligation, yielding its temps in LIFO (reverse-of-creation) order. Private so the
  /// only ways to discharge are the two `Compiler` methods above — no public raw-`Vec` escape hatch.
  pub fn take_vars(mut self) -> Vec<&'t LocalVariable<'s, 't>> {
    self.bomb.defuse();
    let vars = mem::take(&mut self.vars);
    vars
  }
}
