use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::names::{CodeNameS, CodeRuneS, LetImplicitRuneS};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

const MIN_INTERNING_ID: u64 = (1u64 << (7 * 8)) + 1;

/// Interned string - immutable, cheap to copy
/// Matches Scala's StrI
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StrI {
  id: u64,
  pub str: String,
}

impl StrI {
  pub fn new(id: u64, s: String) -> Self {
    StrI { id, str: s }
  }

  pub fn id(&self) -> u64 {
    self.id
  }

  /// Try to encode a short string (≤8 ASCII chars) into a u64 ID
  /// Matches Scala's StrI.shortcutNewInterningId
  fn try_encode_short_string(s: &str) -> Option<u64> {
    if s.len() > 8 {
      return None;
    }

    let bytes = s.as_bytes();

    // Check all bytes are ASCII (< 128)
    if bytes.iter().any(|&b| b >= 128) {
      return None;
    }

    // Encode up to 8 bytes into a u64
    // We do +1 so we don't collide with 0 (which means not interned)
    let mut buffer = [0u8; 8];
    buffer[..bytes.len()].copy_from_slice(bytes);

    let char0 = buffer[0] as u64;
    let char1 = buffer[1] as u64;
    let char2 = buffer[2] as u64;
    let char3 = buffer[3] as u64;
    let char4 = buffer[4] as u64;
    let char5 = buffer[5] as u64;
    let char6 = buffer[6] as u64;
    let char7 = buffer[7] as u64;

    let total = 1u64
      + ((char0 << (7 * 7))
        | (char1 << (7 * 6))
        | (char2 << (7 * 5))
        | (char3 << (7 * 4))
        | (char4 << (7 * 3))
        | (char5 << (7 * 2))
        | (char6 << 7)
        | (char7 << (7 * 0)));

    assert!(total < MIN_INTERNING_ID);
    Some(total)
  }
}

/// Generic interning system with interior mutability
/// Matches Scala's Interner
pub struct Interner<'a> {
  inner: Mutex<InternerInner<'a>>,
  _marker: PhantomData<&'a StrI>,
}

struct InternerInner<'a> {
  string_to_stri: HashMap<String, &'a StrI>,
  id_to_stri: HashMap<u64, &'a StrI>,
  package_coord_to_ref: HashMap<PackageCoordinate<'a>, &'a PackageCoordinate<'a>>,
  file_coord_to_ref: HashMap<FileCoordinate<'a>, &'a FileCoordinate<'a>>,
  code_name_to_ref: HashMap<CodeNameS<'a>, &'a CodeNameS<'a>>,
  code_rune_to_ref: HashMap<CodeRuneS<'a>, &'a CodeRuneS<'a>>,
  let_implicit_rune_to_ref: HashMap<LetImplicitRuneS, &'a LetImplicitRuneS>,
  next_id: u64,
}

macro_rules! define_payload_interner_1arg {
  ($method_name:ident, $ty:ty, $field:ident, $arg_name:ident : $arg_ty:ty, $ctor:expr) => {
    pub fn $method_name(&'a self, $arg_name: $arg_ty) -> &'a $ty {
      let mut inner = self.inner.lock().unwrap();
      let key: $ty = $ctor;
      if let Some(existing) = inner.$field.get(&key) {
        return *existing;
      }
      let new_ref = Box::leak(Box::new(key.clone()));
      inner.$field.insert(key, new_ref);
      new_ref
    }
  };
}

impl<'a> Interner<'a> {
  pub fn new() -> Self {
    Interner {
      inner: Mutex::new(InternerInner {
        string_to_stri: HashMap::new(),
        id_to_stri: HashMap::new(),
        package_coord_to_ref: HashMap::new(),
        file_coord_to_ref: HashMap::new(),
        code_name_to_ref: HashMap::new(),
        code_rune_to_ref: HashMap::new(),
        let_implicit_rune_to_ref: HashMap::new(),
        next_id: MIN_INTERNING_ID,
      }),
      _marker: PhantomData,
    }
  }

  /// Intern a string, returning a canonical shared StrI value.
  pub fn intern(&'a self, s: &str) -> &'a StrI {
    let mut inner = self.inner.lock().unwrap();
    if let Some(existing) = inner.string_to_stri.get(s) {
      return *existing;
    }

    let id = if let Some(encoded_id) = StrI::try_encode_short_string(s) {
      encoded_id
    } else {
      let new_id = inner.next_id;
      inner.next_id += 1;
      new_id
    };

    let value_ref: &'a StrI = Box::leak(Box::new(StrI::new(id, s.to_string())));
    inner.string_to_stri.insert(s.to_string(), value_ref);
    inner.id_to_stri.insert(id, value_ref);
    value_ref
  }

  /// Get StrI by id.
  pub fn get_by_id(&'a self, id: u64) -> Option<&'a StrI> {
    self.inner.lock().unwrap().id_to_stri.get(&id).cloned()
  }

  /// Intern a PackageCoordinate.
  pub fn intern_package_coordinate(&'a self, coord: PackageCoordinate<'a>) -> &'a PackageCoordinate<'a> {
    let mut inner = self.inner.lock().unwrap();
    if let Some(existing) = inner.package_coord_to_ref.get(&coord) {
      return *existing;
    }
    let new_ref: &'a PackageCoordinate<'a> = Box::leak(Box::new(coord.clone()));
    inner.package_coord_to_ref.insert(coord, new_ref);
    new_ref
  }

  /// Intern a FileCoordinate
  pub fn intern_file_coordinate(&'a self, coord: FileCoordinate<'a>) -> &'a FileCoordinate<'a> {
    let mut inner = self.inner.lock().unwrap();
    if let Some(existing) = inner.file_coord_to_ref.get(&coord) {
      return *existing;
    }
    let new_ref: &'a FileCoordinate<'a> = Box::leak(Box::new(coord.clone()));
    inner.file_coord_to_ref.insert(coord, new_ref);
    new_ref
  }

  define_payload_interner_1arg!(
    intern_code_name,
    CodeNameS<'a>,
    code_name_to_ref,
    name: &'a StrI,
    CodeNameS { name }
  );

  define_payload_interner_1arg!(
    intern_code_rune,
    CodeRuneS<'a>,
    code_rune_to_ref,
    name: &'a StrI,
    CodeRuneS { name }
  );

  define_payload_interner_1arg!(
    intern_let_implicit_rune,
    LetImplicitRuneS,
    let_implicit_rune_to_ref,
    lid: LocationInDenizen,
    LetImplicitRuneS { lid }
  );
}

impl<'a> Default for Interner<'a> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_short_string_encoding() {
    let interner = Interner::new();
    let s1 = interner.intern("hello");
    let s2 = interner.intern("hello");

    // Same string should have same ID and same canonical instance.
    assert_eq!(s1.id(), s2.id());
    assert_eq!(s1, s2);
    assert_eq!(s1.str, "hello");

    // Short strings should use encoded ID
    assert!(s1.id() < MIN_INTERNING_ID);
  }

  #[test]
  fn test_long_string_interning() {
    let interner = Interner::new();
    let s1 = interner.intern("this is a long string");
    let s2 = interner.intern("this is a long string");

    // Same string should have same ID and same canonical instance.
    assert_eq!(s1.id(), s2.id());
    assert_eq!(s1, s2);
    assert_eq!(s1.str, "this is a long string");

    // Long strings should use regular interning
    assert!(s1.id() >= MIN_INTERNING_ID);
  }

  #[test]
  fn test_different_strings() {
    let interner = Interner::new();
    let s1 = interner.intern("foo");
    let s2 = interner.intern("bar");

    // Different strings should have different IDs
    assert_ne!(s1.id(), s2.id());
  }
}
