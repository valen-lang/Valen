use std::cell::Cell;
use crate::utils::fx::HashMap;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::instantiating::ast::types::{KindIT, VoidIT, IntIT, BoolIT, FloatIT, StrIT};
use crate::instantiating::ast::ast::{PrototypeI, LocalVariableI};
use crate::instantiating::ast::names::IVarNameI;
use crate::instantiating::ast::citizens::StructDefinitionI;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::scout_arena::ScoutArena;
use crate::testvm::vivem::ConstraintViolatedExceptionV;
use crate::testvm::vivem::PrintStream;
use crate::testvm::vivem::VmRuntimeErrorV;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Write;



/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct RRReferenceV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub hamut: KindIT<'s, 'i>,
  pub _phantom: PhantomData<&'v ()>,
}


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct RRKindV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub hamut: KindIT<'s, 'i>,
  pub _phantom: PhantomData<&'v ()>,
}


impl<'v, 'i, 's> RRKindV<'v, 'i, 's> where 's: 'i, 'i: 'v {
  /// See the free `strip_outer_references`.
  pub fn strip_outer_references(self) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: strip_outer_references(self.hamut), _phantom: PhantomData }
  }

  /// See the free `outer_ownership`.
  pub fn outer_ownership(self) -> OwnershipV {
    outer_ownership(self.hamut)
  }
}


/// Peel off any `BorrowRef`/`OwnRef`/`ShareRef`/`WeakRef` wraps down to the bare underlying kind.
/// A borrow and its owner must strip to the same bare kind so they share one allocation identity
/// (@identity-trap).
pub fn strip_outer_references<'s, 'i>(mut kind: KindIT<'s, 'i>) -> KindIT<'s, 'i> where 's: 'i {
  loop {
    match kind {
      KindIT::BorrowRefIT(w) => kind = w.inner,
      KindIT::OwnRefIT(w) => kind = w.inner,
      KindIT::ShareRefIT(w) => kind = w.inner,
      KindIT::WeakRefIT(w) => kind = w.inner,
      _ => break,
    }
  }
  kind
}


/// The ownership denoted by a kind's outermost wrap: a `BorrowRef`/`ShareRef`/`WeakRef` wrap is
/// Borrow/Share/Weak; a bare kind (or an `OwnRef`) is owned. Callers derive this from the wrapped
/// kind and hand it to `ReferenceV::new`, since `ReferenceV` stores kinds stripped.
pub fn outer_ownership<'s, 'i>(kind: KindIT<'s, 'i>) -> OwnershipV where 's: 'i {
  match kind {
    KindIT::BorrowRefIT(_) => OwnershipV::Borrow,
    KindIT::ShareRefIT(_) => OwnershipV::Share,
    KindIT::WeakRefIT(_) => OwnershipV::Weak,
    KindIT::OwnRefIT(_) => OwnershipV::Own,
    _ => OwnershipV::Own,
  }
}


/// The VM's derived ownership tag. Onion typing expresses ownership structurally (as ref-wraps
/// around a bare kind), but `ReferenceV` stores its kinds stripped, so it carries this tag —
/// derived from the wrap at construction — to answer borrow-vs-weak-vs-owned later (drives
/// weak-vs-strong referrer registration and the dealloc/cleanup rules).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum OwnershipV {
  Own,
  Borrow,
  Share,
  Weak,
}


/// Temporary state
pub struct AllocationV<'v, 'i, 's> {
  pub reference: ReferenceV<'v, 'i, 's>,
  pub kind: KindV<'v, 'i, 's>,
  pub strong_referrers: HashMap<IObjectReferrerV<'v, 'i, 's>, i32>,
  pub weak_referrers: HashMap<IObjectReferrerV<'v, 'i, 's>, i32>,
}


impl<'v, 'i, 's> AllocationV<'v, 'i, 's> {
  pub fn id(&self) -> AllocationIdV<'v, 'i, 's> {
    panic!("Unimplemented: id");
  }

  pub fn increment_ref_count(&mut self, referrer: IObjectReferrerV<'v, 'i, 's>, is_weak: bool) {
    if matches!(self.kind, KindV::Void(_)) {
      return;
    }
    let referrers = if is_weak { &mut self.weak_referrers } else { &mut self.strong_referrers };
    match referrer {
      IObjectReferrerV::RegisterToObjectReferrer(_) => {
        // We can have multiple of these, thats fine
      }
      _ => {
        if referrers.contains_key(&referrer) {
          panic!("nooo");
        }
      }
    }
    let current = *referrers.get(&referrer).unwrap_or(&0);
    referrers.insert(referrer, current + 1);
  }

  pub fn decrement_ref_count(&mut self, referrer: IObjectReferrerV<'v, 'i, 's>, is_weak: bool) {
    if matches!(self.kind, KindV::Void(_)) {
      return;
    }
    let referrers = if is_weak { &mut self.weak_referrers } else { &mut self.strong_referrers };
    if !referrers.contains_key(&referrer) {
      panic!("nooooo");
    }
    let new_count = *referrers.get(&referrer).unwrap() - 1;
    referrers.insert(referrer, new_count);
    if new_count == 0 {
      referrers.remove(&referrer);
      assert!(!referrers.contains_key(&referrer));
    }
  }


  pub fn get_ref_count(&self) -> i32 {
    panic!("Unimplemented: get_ref_count");
  }

  /// `is_weak_filter`: None counts all referrers; Some(true) counts weak only; Some(false) counts strong only.
  pub fn ensure_ref_count(&self, scout_arena: &ScoutArena<'s>, is_weak_filter: Option<bool>, expected_num: i32) -> Result<(), VmRuntimeErrorV<'s>> {
    if matches!(self.kind, KindV::Void(_)) {
      // Void has no RC
      return Ok(());
    }
    let referrers: Vec<(&IObjectReferrerV<'v, 'i, 's>, &i32)> = match is_weak_filter {
      None => self.strong_referrers.iter().chain(self.weak_referrers.iter()).collect(),
      Some(true) => self.weak_referrers.iter().collect(),
      Some(false) => self.strong_referrers.iter().collect(),
    };
    let matching_referrers: Vec<i32> = referrers.iter().map(|(_, v)| **v).collect();
    if matching_referrers.len() as i32 != expected_num {
      let msg = format!("Expected {} of {}but was {}:\n{:?}",
        expected_num,
        is_weak_filter.map(|w| format!("{} ", if w { "weak" } else { "strong" })).unwrap_or_default(),
        matching_referrers.len(),
        matching_referrers);
      return Err(VmRuntimeErrorV::ConstraintViolatedException(ConstraintViolatedExceptionV { msg: scout_arena.intern_str(&msg) }));
    }
    Ok(())
  }


  pub fn print_refs(&self, vivem_dout: &mut PrintStream) {
    if self.get_total_ref_count(None) > 0 {
      let referrers_str = self.strong_referrers.iter().chain(self.weak_referrers.iter()).map(|(_k, _v)| -> String { panic!("vimpl: referrers.mkString entry toString") }).collect::<Vec<_>>().join(" ");
      writeln!(vivem_dout, "o{}: {}", self.reference.alloc_id().num, referrers_str).unwrap();
    }
  }

  /// `is_weak_filter`: None counts all referrers; Some(true) counts weak only; Some(false) counts strong only.
  pub fn get_total_ref_count(&self, is_weak_filter: Option<bool>) -> i32 {
    if matches!(self.kind, KindV::Void(_)) {
      return 1;
    }
    match is_weak_filter {
      None => (self.strong_referrers.len() + self.weak_referrers.len()) as i32,
      Some(true) => self.weak_referrers.len() as i32,
      Some(false) => self.strong_referrers.len() as i32,
    }
  }


  pub fn finalize(&self) {
    panic!("Unimplemented: finalize");
  }
}



/// Temporary state
#[derive(Copy, Clone, Debug)]
pub enum KindV<'v, 'i, 's> {
  Void(VoidV),
  Int(IntV<'v, 'i, 's>),
  Bool(BoolV<'v, 'i, 's>),
  Float(FloatV<'v, 'i, 's>),
  Str(StrV<'v, 'i, 's>),
  Opaque(OpaqueV<'v, 'i, 's>),
  StructInstance(&'v StructInstanceV<'v, 'i, 's>),
  ArrayInstance(&'v ArrayInstanceV<'v, 'i, 's>),
}


impl<'v, 'i, 's> KindV<'v, 'i, 's> where 's: 'i, 'i: 'v {
  pub fn tyype(&self, interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    match self {
      KindV::Void(v) => v.tyype(interner),
      KindV::Int(v) => v.tyype(interner),
      KindV::Bool(v) => v.tyype(interner),
      KindV::Float(v) => v.tyype(interner),
      KindV::Str(v) => v.tyype(interner),
      KindV::Opaque(v) => v.tyype(interner),
      KindV::StructInstance(v) => v.tyype(interner),
      KindV::ArrayInstance(v) => v.tyype(interner),
    }
  }
}


/// Temporary state
#[derive(Copy, Clone)]
pub enum PrimitiveKindV<'v, 'i, 's> {
  Void(VoidV),
  Int(IntV<'v, 'i, 's>),
  Bool(BoolV<'v, 'i, 's>),
  Float(FloatV<'v, 'i, 's>),
  Str(StrV<'v, 'i, 's>),
  Opaque(OpaqueV<'v, 'i, 's>),
}

impl<'v, 'i, 's> From<PrimitiveKindV<'v, 'i, 's>> for KindV<'v, 'i, 's> {
  fn from(p: PrimitiveKindV<'v, 'i, 's>) -> Self {
    match p {
      PrimitiveKindV::Void(v) => KindV::Void(v),
      PrimitiveKindV::Int(v) => KindV::Int(v),
      PrimitiveKindV::Bool(v) => KindV::Bool(v),
      PrimitiveKindV::Float(v) => KindV::Float(v),
      PrimitiveKindV::Str(v) => KindV::Str(v),
      PrimitiveKindV::Opaque(v) => KindV::Opaque(v),
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidV;

impl VoidV {
  pub fn tyype<'v, 'i, 's>(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> where 's: 'i, 'i: 'v, {
    RRKindV { hamut: KindIT::VoidIT(VoidIT {}), _phantom: PhantomData }
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub value: i64,
  pub bits: i32,
  pub _phantom: PhantomData<(&'v (), &'i (), &'s ())>,
}


impl<'v, 'i, 's> IntV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: KindIT::IntIT(IntIT { bits: self.bits }), _phantom: PhantomData }
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub value: bool,
  pub _phantom: PhantomData<(&'v (), &'i (), &'s ())>,
}


impl<'v, 'i, 's> BoolV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: KindIT::BoolIT(BoolIT {}), _phantom: PhantomData }
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub value: f64,
  pub _phantom: PhantomData<(&'v (), &'i (), &'s ())>,
}


impl<'v, 'i, 's> FloatV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: KindIT::FloatIT(FloatIT {}), _phantom: PhantomData }
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub value: StrI<'s>,
  pub _phantom: PhantomData<(&'v (), &'i ())>,
}


impl<'v, 'i, 's> StrV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: KindIT::StrIT(StrIT {}), _phantom: PhantomData }
  }
}


/// Temporary state
///
/// The VM's stand-in for an opaque extern citizen (e.g. the test-only `Vec`). Onion typing has no
/// dedicated opaque kind, so we carry the citizen's onion `KindIT` directly. Step 3 (externs)
/// decides how the Vec externs build this.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OpaqueV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub opaque_ht: KindIT<'s, 'i>,
  pub _phantom: PhantomData<&'v ()>,
}


impl<'v, 'i, 's> OpaqueV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: self.opaque_ht, _phantom: PhantomData }
  }
}


/// Temporary state
pub struct StructInstanceV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub struct_h: &'i StructDefinitionI<'s, 'i>,
  pub members: Cell<Option<&'v [ReferenceV<'v, 'i, 's>]>>,
}

// `StructDefinitionI` has no `Debug` (its `ArenaIndexMap` bound maps don't), so print the struct by
// its instantiated type rather than recursing into the whole definition.
impl<'v, 'i, 's> Debug for StructInstanceV<'v, 'i, 's> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("StructInstanceV")
      .field("struct_h", self.struct_h.instantiated_citizen)
      .field("members", &self.members)
      .finish()
  }
}


impl<'v, 'i, 's> StructInstanceV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: KindIT::StructIT(self.struct_h.instantiated_citizen), _phantom: PhantomData }
  }


  pub fn get_reference_member(&self, index: i32) -> ReferenceV<'v, 'i, 's> {
    let members = self.members.get().expect("StructInstance has no members");
    let (_tyype, r#ref) = (self.struct_h.members[index as usize].tyype, members[index as usize]);
    r#ref
  }


  pub fn set_reference_member(&self, vivem_bump: &'v bumpalo::Bump, index: i32, reference: ReferenceV<'v, 'i, 's>) {
    let mut new_members: Vec<ReferenceV<'v, 'i, 's>> = self.members.get().expect("StructInstance has no members").to_vec();
    new_members[index as usize] = reference;
    self.members.set(Some(vivem_bump.alloc_slice_copy(&new_members)));
  }


  pub fn zero(&self) {
    self.members.set(None);
  }
}


/// Temporary state
#[derive(Debug)]
pub struct ArrayInstanceV<'v, 'i, 's> {
  pub type_h: KindIT<'s, 'i>,
  pub element_type_h: KindIT<'s, 'i>,
  pub capacity: i32,
  pub elements: Cell<&'v [ReferenceV<'v, 'i, 's>]>,
}


impl<'v, 'i, 's> ArrayInstanceV<'v, 'i, 's> {
  pub fn tyype(&self, _interner: &InstantiatingInterner<'s, 'i>) -> RRKindV<'v, 'i, 's> {
    RRKindV { hamut: self.type_h, _phantom: PhantomData }
  }


  pub fn get_element(&self, index: i64) -> ReferenceV<'v, 'i, 's> {
    let elements = self.elements.get();
    if index < 0 || index as usize >= elements.len() {
      panic!("PanicException");
    }
    elements[index as usize]
  }


  pub fn set_element(&self, vivem_bump: &'v bumpalo::Bump, index: i64, ref_: ReferenceV<'v, 'i, 's>) {
    let elements = self.elements.get();
    if index < 0 || index as usize >= elements.len() {
      panic!("PanicException");
    }
    let mut new_vec = bumpalo::collections::Vec::with_capacity_in(elements.len(), vivem_bump);
    new_vec.extend_from_slice(elements);
    new_vec[index as usize] = ref_;
    self.elements.set(new_vec.into_bump_slice());
  }


  pub fn initialize_element(&self, vivem_bump: &'v bumpalo::Bump, ref_: ReferenceV<'v, 'i, 's>) {
    let elements = self.elements.get();
    assert!(elements.len() < self.capacity as usize);
    let mut new_vec = bumpalo::collections::Vec::with_capacity_in(elements.len() + 1, vivem_bump);
    new_vec.extend_from_slice(elements);
    new_vec.push(ref_);
    self.elements.set(new_vec.into_bump_slice());
  }


  pub fn deinitialize_element(&self) -> ReferenceV<'v, 'i, 's> {
    let elements = self.elements.get();
    assert!(!elements.is_empty());
    let r#ref = elements[elements.len() - 1];
    self.elements.set(&elements[0..elements.len() - 1]);
    r#ref
  }


  pub fn get_size(&self) -> i64 {
    self.elements.get().len() as i64
  }
}


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AllocationIdV<'v, 'i, 's> {
  pub tyype: RRKindV<'v, 'i, 's>,
  pub num: i32,
}


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ReferenceV<'v, 'i, 's> {
  pub actual_kind: RRKindV<'v, 'i, 's>,
  pub seen_as_kind: RRKindV<'v, 'i, 's>,
  /// Derived from the reference's outer wrap at construction (the kinds are stored stripped). Reads
  /// borrow-vs-weak-vs-owned for referrer-strength and dealloc/cleanup decisions.
  pub ownership: OwnershipV,
  pub num: i32,
  /// Module-private; forces construction via `ReferenceV::new(...)` so the wrap-strip runs.
  /// External code can destructure with `..`.
  _sealed: (),
}


impl<'v, 'i, 's> ReferenceV<'v, 'i, 's> {
  /// Construct a ReferenceV. Both `actual_kind` and `seen_as_kind` must already be stripped of their
  /// outer ref-wraps: `actual_kind` is the bare underlying kind that keys `alloc_id` (so a borrow and
  /// its owner share one allocation identity), and `seen_as_kind` is the bare viewed kind (`IShip`
  /// for an upcast, else the bare concrete kind). `ownership` is the borrow/own/share/weak the caller
  /// derived from the wrap (via `RRKindV::outer_ownership`) before stripping (@identity-trap).
  pub fn new(
    actual_kind: RRKindV<'v, 'i, 's>,
    seen_as_kind: RRKindV<'v, 'i, 's>,
    ownership: OwnershipV,
    num: i32,
  ) -> Self {
    assert!(
      !matches!(actual_kind.hamut,
        KindIT::BorrowRefIT(_) | KindIT::OwnRefIT(_) | KindIT::ShareRefIT(_) | KindIT::WeakRefIT(_)),
      "ReferenceV::new: actual_kind still carries an outer reference wrap");
    assert!(
      !matches!(seen_as_kind.hamut,
        KindIT::BorrowRefIT(_) | KindIT::OwnRefIT(_) | KindIT::ShareRefIT(_) | KindIT::WeakRefIT(_)),
      "ReferenceV::new: seen_as_kind still carries an outer reference wrap");
    ReferenceV { actual_kind, seen_as_kind, ownership, num, _sealed: () }
  }

  pub fn alloc_id(&self) -> AllocationIdV<'v, 'i, 's> {
    // Strip ref-wraps so a borrow and its owner share one key (see `RRKindV::strip_outer_references`, @identity-trap).
    AllocationIdV { tyype: self.actual_kind.strip_outer_references(), num: self.num }
  }

  pub fn actual_coord(&self) -> RRReferenceV<'v, 'i, 's> {
    RRReferenceV {
      hamut: self.actual_kind.hamut,
      _phantom: PhantomData,
    }
  }

  pub fn seen_as_coord(&self) -> RRReferenceV<'v, 'i, 's> {
    RRReferenceV {
      hamut: self.seen_as_kind.hamut,
      _phantom: PhantomData,
    }
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum IObjectReferrerV<'v, 'i, 's> {
  VariableToObjectReferrer(VariableToObjectReferrerV<'v, 'i, 's>),
  MemberToObjectReferrer(MemberToObjectReferrerV<'v, 'i, 's>),
  ElementToObjectReferrer(ElementToObjectReferrerV<'v, 'i, 's>),
  RegisterToObjectReferrer(RegisterToObjectReferrerV<'v, 'i, 's>),
  RegisterHoldToObjectReferrer(RegisterHoldToObjectReferrerV<'v, 'i, 's>),
  ArgumentToObjectReferrer(ArgumentToObjectReferrerV<'v, 'i, 's>),
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VariableToObjectReferrerV<'v, 'i, 's> {
  pub var_addr: VariableAddressV<'v, 'i, 's>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemberToObjectReferrerV<'v, 'i, 's> {
  pub member_addr: MemberAddressV<'v, 'i, 's>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ElementToObjectReferrerV<'v, 'i, 's> {
  pub element_addr: ElementAddressV<'v, 'i, 's>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegisterToObjectReferrerV<'v, 'i, 's> {
  pub call_id: CallIdV<'v, 'i, 's>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegisterHoldToObjectReferrerV<'v, 'i, 's> {
  pub expression_id: ExpressionIdV<'v, 'i, 's>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentToObjectReferrerV<'v, 'i, 's> {
  pub argument_id: ArgumentIdV<'v, 'i, 's>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VariableAddressV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub call_id: CallIdV<'v, 'i, 's>,
  // A local's identity is its name, which the typing pass makes unique per function (@VCOORD). The
  // instantiator reallocates a fresh `LocalVariableI` per mention, so the pointer is NOT a stable
  // key — the name is.
  pub name: IVarNameI<'s, 'i>,
}


impl<'v, 'i, 's> Display for VariableAddressV<'v, 'i, 's> where 's: 'i, 'i: 'v {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "*v:{}#v{:?}", self.call_id, self.name)
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemberAddressV<'v, 'i, 's> {
  pub struct_id: AllocationIdV<'v, 'i, 's>,
  pub field_index: i32,
}


impl<'v, 'i, 's> MemberAddressV<'v, 'i, 's> {
  pub fn to_string(&self) -> String {
    format!("*o:{}.{}", self.struct_id.num, self.field_index)
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ElementAddressV<'v, 'i, 's> {
  pub array_id: AllocationIdV<'v, 'i, 's>,
  pub element_index: i64,
}


impl<'v, 'i, 's> ElementAddressV<'v, 'i, 's> {
  pub fn to_string(&self) -> String {
    format!("*o:{}.{}", self.array_id.num, self.element_index)
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CallIdV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub call_depth: i32,
  pub function: &'i PrototypeI<'s, 'i>,
  pub _phantom: PhantomData<&'v ()>,
}


impl<'v, 'i, 's> CallIdV<'v, 'i, 's> {
  pub fn to_string(&self) -> StrI<'s> {
    panic!("Unimplemented: to_string_call_id");
  }
}

impl<'v, 'i, 's> Display for CallIdV<'v, 'i, 's> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "ƒ{}/{:?}", self.call_depth, self.function.id.local_name)
  }
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentIdV<'v, 'i, 's> {
  pub call_id: CallIdV<'v, 'i, 's>,
  pub index: i32,
}


/// Temporary state
#[derive(Clone)]
pub struct VariableV<'v, 'i, 's> {
  pub id: VariableAddressV<'v, 'i, 's>,
  pub reference: ReferenceV<'v, 'i, 's>,
  pub expected_type: KindIT<'s, 'i>,
}


/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionIdV<'v, 'i, 's> {
  pub call_id: CallIdV<'v, 'i, 's>,
  pub path: &'v [i32],
}


impl<'v, 'i, 's> ExpressionIdV<'v, 'i, 's> {
  pub fn add_step(&self, bump: &'v bumpalo::Bump, i: i32) -> ExpressionIdV<'v, 'i, 's> {
    let old_len = self.path.len();
    let new_path: &'v mut [i32] = bump.alloc_slice_fill_with(old_len + 1, |idx| {
        if idx < old_len { self.path[idx] } else { i }
    });
    ExpressionIdV { call_id: self.call_id, path: new_path }
  }
}


/// Temporary state
pub enum RegisterV<'v, 'i, 's> {
  ReferenceRegister(&'v ReferenceRegisterV<'v, 'i, 's>),
}


impl<'v, 'i, 's> RegisterV<'v, 'i, 's> {
  pub fn expect_reference_register(&self) -> ReferenceRegisterV<'v, 'i, 's> {
    panic!("Unimplemented: expect_reference_register");
  }
}


/// Temporary state
pub struct ReferenceRegisterV<'v, 'i, 's> {
  pub reference: ReferenceV<'v, 'i, 's>,
}


/// Temporary state
pub struct VivemPanicV<'v, 'i, 's>
where 's: 'i, 'i: 'v,
{
  pub message: StrI<'s>,
  pub _phantom: PhantomData<(&'v (), &'i ())>,
}
