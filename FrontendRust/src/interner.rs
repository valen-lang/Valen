use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::utils::code_hierarchy::{PackageCoordinate, FileCoordinate};

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
pub struct Interner {
    inner: Mutex<InternerInner>,
}

struct InternerInner {
    string_to_id: HashMap<String, u64>,
    package_coord_to_id: HashMap<PackageCoordinate, u64>,
    file_coord_to_id: HashMap<FileCoordinate, u64>,
    next_id: u64,
}

impl Interner {
    pub fn new() -> Self {
        Interner {
            inner: Mutex::new(InternerInner {
                string_to_id: HashMap::new(),
                package_coord_to_id: HashMap::new(),
                file_coord_to_id: HashMap::new(),
                next_id: MIN_INTERNING_ID,
            }),
        }
    }

    /// Intern a string, returning an Arc<StrI>
    /// Matches Scala's interner.intern(StrI(s))
    pub fn intern(&self, s: &str) -> Arc<StrI> {
        let mut inner = self.inner.lock().unwrap();
        // Try shortcut for short strings
        if let Some(id) = StrI::try_encode_short_string(s) {
            return Arc::new(StrI::new(id, s.to_string()));
        }

        // Look up existing
        if let Some(&existing_id) = inner.string_to_id.get(s) {
            return Arc::new(StrI::new(existing_id, s.to_string()));
        }

        // Create new
        let new_id = inner.next_id;
        inner.next_id += 1;
        inner.string_to_id.insert(s.to_string(), new_id);
        Arc::new(StrI::new(new_id, s.to_string()))
    }

    /// Intern a PackageCoordinate
    /// Matches Scala's interner.intern(PackageCoordinate(...))
    pub fn intern_package_coordinate(&self, coord: PackageCoordinate) -> Arc<PackageCoordinate> {
        let mut inner = self.inner.lock().unwrap();
        
        // Look up existing
        if let Some(&_existing_id) = inner.package_coord_to_id.get(&coord) {
            return Arc::new(coord);
        }

        // Create new
        let new_id = inner.next_id;
        inner.next_id += 1;
        inner.package_coord_to_id.insert(coord.clone(), new_id);
        Arc::new(coord)
    }

    /// Intern a FileCoordinate
    /// Matches Scala's interner.intern(FileCoordinate(...))
    pub fn intern_file_coordinate(&self, coord: FileCoordinate) -> Arc<FileCoordinate> {
        let mut inner = self.inner.lock().unwrap();
        
        // Look up existing
        if let Some(&_existing_id) = inner.file_coord_to_id.get(&coord) {
            return Arc::new(coord);
        }

        // Create new
        let new_id = inner.next_id;
        inner.next_id += 1;
        inner.file_coord_to_id.insert(coord.clone(), new_id);
        Arc::new(coord)
    }
}

impl Default for Interner {
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
        
        // Same string should have same ID
        assert_eq!(s1.id(), s2.id());
        assert_eq!(s1.str, "hello");
        
        // Short strings should use encoded ID
        assert!(s1.id() < MIN_INTERNING_ID);
    }

    #[test]
    fn test_long_string_interning() {
        let interner = Interner::new();
        let s1 = interner.intern("this is a long string");
        let s2 = interner.intern("this is a long string");
        
        // Same string should have same ID
        assert_eq!(s1.id(), s2.id());
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

