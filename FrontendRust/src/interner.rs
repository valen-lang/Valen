use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::names::{CodeNameS, CodeRuneS, LetImplicitRuneS};
use bumpalo::Bump;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

const MIN_INTERNING_ID: u64 = (1u64 << (7 * 8)) + 1;

#[derive(Copy, Clone)]
pub struct InternedStr {
  ptr: *const u8,
  len: usize,
}

impl InternedStr {
  pub fn new(s: &str) -> Self {
    InternedStr {
      ptr: s.as_ptr(),
      len: s.len(),
    }
  }

  pub fn as_str(&self) -> &str {
    // SAFETY: All InternedStr instances are created from valid UTF-8 slices
    // allocated in the compilation arena and outlive all uses.
    unsafe {
      let bytes = std::slice::from_raw_parts(self.ptr, self.len);
      std::str::from_utf8_unchecked(bytes)
    }
  }
}

impl Deref for InternedStr {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.as_str()
  }
}

impl Debug for InternedStr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for InternedStr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Display::fmt(self.as_str(), f)
  }
}

impl PartialEq for InternedStr {
  fn eq(&self, other: &Self) -> bool {
    self.as_str() == other.as_str()
  }
}

impl Eq for InternedStr {}

impl Hash for InternedStr {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_str().hash(state);
  }
}

impl PartialEq<&str> for InternedStr {
  fn eq(&self, other: &&str) -> bool {
    self.as_str() == *other
  }
}

#[derive(Copy, Clone)]
pub struct InternedSlice<T: Copy> {
  ptr: *const T,
  len: usize,
  _marker: PhantomData<T>,
}

impl<T: Copy> InternedSlice<T> {
  pub fn new(slice: &[T]) -> Self {
    InternedSlice {
      ptr: slice.as_ptr(),
      len: slice.len(),
      _marker: PhantomData,
    }
  }

  pub fn as_slice(&self) -> &[T] {
    // SAFETY: The backing slice is arena-allocated and outlives all uses.
    unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
  }

  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.as_slice().iter()
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  pub fn len(&self) -> usize {
    self.len
  }
}

impl<'b, T: Copy> IntoIterator for &'b InternedSlice<T> {
  type Item = &'b T;
  type IntoIter = std::slice::Iter<'b, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.as_slice().iter()
  }
}

impl<T: Copy + Debug> Debug for InternedSlice<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self.as_slice(), f)
  }
}

impl<T: Copy + PartialEq> PartialEq for InternedSlice<T> {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

impl<T: Copy + Eq> Eq for InternedSlice<T> {}

impl<T: Copy + Hash> Hash for InternedSlice<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_slice().hash(state);
  }
}

/// Interned string - immutable, cheap to copy
/// Matches Scala's StrI
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StrI {
  id: u64,
  pub str: InternedStr,
}

impl StrI {
  pub fn new(id: u64, s: InternedStr) -> Self {
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
  arena: &'a Bump,
  inner: RefCell<InternerInner<'a>>,
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
    pub fn $method_name(&self, $arg_name: $arg_ty) -> &'a $ty {
      let mut inner = self.inner.borrow_mut();
      let key: $ty = $ctor;
      if let Some(existing) = inner.$field.get(&key) {
        return *existing;
      }
      let new_ref = self.arena.alloc(key.clone());
      inner.$field.insert(key, new_ref);
      new_ref
    }
  };
}

impl<'a> Interner<'a> {
  pub fn with_arena(arena: &'a Bump) -> Self {
    Interner {
      arena,
      inner: RefCell::new(InternerInner {
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
  pub fn intern(&self, s: &str) -> &'a StrI {
    let mut inner = self.inner.borrow_mut();
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

    let arena_str = self.arena.alloc_str(s);
    let value_ref: &'a StrI = self.arena.alloc(StrI::new(id, InternedStr::new(arena_str)));
    inner.string_to_stri.insert(s.to_string(), value_ref);
    inner.id_to_stri.insert(id, value_ref);
    value_ref
  }

  /// Get StrI by id.
  pub fn get_by_id(&self, id: u64) -> Option<&'a StrI> {
    self.inner.borrow().id_to_stri.get(&id).cloned()
  }

  /// Intern a PackageCoordinate.
  pub fn intern_package_coordinate(
    &self,
    module: &'a StrI,
    packages: &[&'a StrI],
  ) -> &'a PackageCoordinate<'a> {
    let mut inner = self.inner.borrow_mut();
    let lookup_coord = PackageCoordinate {
      module,
      packages: InternedSlice::new(packages),
    };
    if let Some(existing) = inner.package_coord_to_ref.get(&lookup_coord) {
      return *existing;
    }
    let arena_packages = self.arena.alloc_slice_copy(packages);
    let coord = PackageCoordinate {
      module,
      packages: InternedSlice::new(arena_packages),
    };
    let new_ref: &'a PackageCoordinate<'a> = self.arena.alloc(coord.clone());
    inner.package_coord_to_ref.insert(coord, new_ref);
    new_ref
  }

  /// Intern a FileCoordinate
  pub fn intern_file_coordinate(
    &self,
    package_coord: &'a PackageCoordinate<'a>,
    filepath: &str,
  ) -> &'a FileCoordinate<'a> {
    let mut inner = self.inner.borrow_mut();
    let lookup_coord = FileCoordinate {
      package_coord,
      filepath: InternedStr::new(filepath),
    };
    if let Some(existing) = inner.file_coord_to_ref.get(&lookup_coord) {
      return *existing;
    }
    let arena_filepath = self.arena.alloc_str(filepath);
    let coord = FileCoordinate {
      package_coord,
      filepath: InternedStr::new(arena_filepath),
    };
    let new_ref: &'a FileCoordinate<'a> = self.arena.alloc(coord.clone());
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_short_string_encoding() {
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);
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
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);
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
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);
    let s1 = interner.intern("foo");
    let s2 = interner.intern("bar");

    // Different strings should have different IDs
    assert_ne!(s1.id(), s2.id());
  }

  #[test]
  fn test_coordinate_interning_canonicalizes() {
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let module = interner.intern("my_module");
    let pkg1 = interner.intern_package_coordinate(module, &[]);
    let pkg2 = interner.intern_package_coordinate(module, &[]);
    assert!(std::ptr::eq(pkg1, pkg2));

    let file1 = interner.intern_file_coordinate(pkg1, "main.vale");
    let file2 = interner.intern_file_coordinate(pkg1, "main.vale");
    assert!(std::ptr::eq(file1, file2));
    assert_eq!(file1.filepath, "main.vale");
  }
}
