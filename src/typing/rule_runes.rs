use crate::postparsing::rules::rules::{IRulexSR, RuneUsage};

/// The runes a rule mentions, so a solve knows which runes it must conclude.
///
/// The typing pass ignores regions and groups (that is the borrow checker's job), so a `BorrowRef`
/// contributes only its result and inner runes. Its region rune, when it has one, is deliberately
/// left out, so the solver never treats a region as a rune it must conclude.
pub fn rune_usages<'s>(rule: &IRulexSR<'s>) -> Vec<RuneUsage<'s>> {
  match rule {
    IRulexSR::Equals(x) => vec![x.left.clone(), x.right.clone()],
    IRulexSR::Literal(x) => vec![x.rune.clone()],
    IRulexSR::Lookup(x) => vec![x.rune.clone()],
    IRulexSR::Call(x) => {
      let mut usages = vec![x.result_rune.clone(), x.template_rune.clone()];
      usages.extend(x.args.iter().cloned());
      usages
    }
    IRulexSR::RuneParentEnvLookup(x) => vec![x.rune.clone()],
    IRulexSR::KindList(x) => {
      let mut usages = vec![x.result_rune.clone()];
      usages.extend(x.members.iter().cloned());
      usages
    }
    IRulexSR::CallSiteFunc(x) => {
      vec![x.prototype_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()]
    }
    IRulexSR::DefinitionFunc(x) => {
      vec![x.result_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()]
    }
    IRulexSR::Resolve(x) => {
      vec![x.result_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()]
    }
    IRulexSR::BorrowRef(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
    IRulexSR::WeakRef(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
    IRulexSR::OwnRef(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
  }
}
