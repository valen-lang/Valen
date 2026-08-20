use crate::utils::fx::HashMap;
use crate::instantiating::ast::types::KindIT;
use crate::testvm::values::{
    CallIdV, ReferenceV, VariableAddressV, VariableV,
};



/// Temporary state
pub struct CallV<'v, 'i, 's> {
  pub call_id: CallIdV<'v, 'i, 's>,
  pub in_args: &'v [ReferenceV<'v, 'i, 's>],
  pub args: HashMap<i32, Option<ReferenceV<'v, 'i, 's>>>,
  pub locals: HashMap<VariableAddressV<'v, 'i, 's>, VariableV<'v, 'i, 's>>,
}


impl<'v, 'i, 's> CallV<'v, 'i, 's> {
  pub fn add_local(&mut self, var_addr: VariableAddressV<'v, 'i, 's>, reference: ReferenceV<'v, 'i, 's>, tyype: KindIT<'s, 'i>) {
    assert_eq!(var_addr.call_id, self.call_id);
    let locals = &mut self.locals;
    assert!(!locals.contains_key(&var_addr));
    // A local's identity is its (per-function-unique) name, so this catches re-adding the same
    // underlying local under any address.
    assert!(!locals.iter().any(|(addr, _)| addr.name == var_addr.name));
    locals.insert(var_addr, VariableV {
      id: var_addr,
      reference,
      expected_type: tyype,
    });
  }


  pub fn remove_local(&mut self, var_addr: VariableAddressV<'v, 'i, 's>) {
    assert_eq!(var_addr.call_id, self.call_id);
    let locals = &mut self.locals;
    assert!(locals.contains_key(&var_addr));
    locals.remove(&var_addr);
  }


  pub fn get_local(&self, addr: VariableAddressV<'v, 'i, 's>) -> VariableV<'v, 'i, 's> {
    let locals = &self.locals;
    let result = locals.get(&addr).expect("get_local: not found").clone();
    result
  }


  pub fn mutate_local(&mut self, var_addr: VariableAddressV<'v, 'i, 's>, reference: ReferenceV<'v, 'i, 's>, _expected_type: KindIT<'s, 'i>) {
    self.locals.get_mut(&var_addr).expect("mutate_local: not found").reference = reference;
  }


  pub fn take_argument(&mut self, index: i32) -> ReferenceV<'v, 'i, 's> {
    assert!((index as usize) < self.args.len());
    match self.args.get(&index).copied() {
      Some(Some(r#ref)) => {
        self.args.insert(index, None);
        r#ref
      }
      Some(None) => panic!("Already took from argument {}", index),
      None => panic!("take_argument: missing argument key {} (assert should have caught this)", index),
    }
  }


  pub fn prepare_to_die(&mut self) {
    let locals = &self.locals;
    assert!(locals.is_empty());
    let args = &self.args;
    let undead_args: Vec<_> = args.iter().filter_map(|(i, v)| v.map(|val| (*i, val))).collect();
    if !undead_args.is_empty() {
        panic!("Undead arguments:\n{:?}", undead_args);
    }
  }
}
