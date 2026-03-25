# NECX: Remaining Clones of FileCoordinateMap / PackageCoordinateMap

## Background

In Scala, `FileCoordinateMap` and `PackageCoordinateMap` are mutable `HashMap`-backed classes. Because the JVM uses reference semantics, returning one from a cache (`codeMapCache.get`, `parsedsCache.get`) just returns a reference — no deep copy. The Rust versions derive `Clone`, and several call sites clone entire maps (including their `HashMap` contents).

## Struct Definitions

```rust
// code_hierarchy.rs
pub struct FileCoordinateMap<'a, Contents> {
  pub package_coord_to_file_coords: HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>,
  pub file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
}

pub struct PackageCoordinateMap<'a, Contents> {
  pub package_coord_to_contents: HashMap<&'a PackageCoordinate<'a>, Contents>,
}
```

Note: `impl<'a, Contents: Clone> FileCoordinateMap` — the entire impl block requires `Contents: Clone`, which is a stronger constraint than needed for most methods.

---

## Clone Call Sites

### 1. `parser.rs:1951` — `get_code_map()` returns owned clone from cache

```rust
pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
    self.get_parseds()?;
    Ok(self.code_map_cache.clone().unwrap())
}
```

**Scala equivalent** (`Parser.scala`): `Ok(codeMapCache.get)` — returns reference to cached value, no copy.

### 2. `parser.rs:1969` — `expect_code_map()` returns owned clone from cache

```rust
pub fn expect_code_map(&self) -> FileCoordinateMap<'a, String> {
    self.code_map_cache.clone().expect("code_map_cache should be populated")
}
```

**Scala equivalent**: `vassertSome(codeMapCache)` — returns reference.

### 3. `parser.rs:1976` — `get_parseds()` early return clones cache

```rust
if let Some(ref parseds) = self.parseds_cache {
    return Ok(parseds.clone());
}
```

**Scala equivalent**: `case Some(parseds) => Ok(parseds)` — returns reference.

### 4. `parser.rs:1985` — `get_parseds()` final return clones cache

```rust
self.parseds_cache = Some(program_p_map);
Ok(self.parseds_cache.clone().unwrap())
```

**Scala equivalent**: `Ok(parsedsCache.get)` — returns reference.

### 5. `post_parser.rs:2752` — copies `package_coord_to_file_coords` into new map

```rust
scoutput.package_coord_to_file_coords = parseds.package_coord_to_file_coords.clone();
```

This clones only the `HashMap<&PackageCoordinate, Vec<&FileCoordinate>>` (pointers, not deep data), copying it from the parser's output into the scout's output. Scala does the same implicitly since both stages share the mutable map.

### 6. `code_hierarchy.rs:416` — `map()` copies `package_coord_to_file_coords`

```rust
pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<'a, T>
where F: Fn(&'a FileCoordinate<'a>, &Contents) -> T, T: Clone,
{
    // ...
    FileCoordinateMap {
        package_coord_to_file_coords: self.package_coord_to_file_coords.clone(),
        file_coord_to_contents: result_file_coord_to_contents,
    }
}
```

Creates a new `FileCoordinateMap` with transformed contents but the same package structure. The clone copies `HashMap<&ptr, Vec<&ptr>>` — shallow.

---

## Assessment

| Site | What's cloned | Cost | Avoidable? |
|------|--------------|------|------------|
| parser.rs:1951 | `Option<FileCoordinateMap<String>>` | **Expensive** — clones HashMap of Strings | Yes |
| parser.rs:1969 | `Option<FileCoordinateMap<String>>` | **Expensive** — same | Yes |
| parser.rs:1976 | `FileCoordinateMap<(FileP, Vec<RangeL>)>` | **Expensive** — clones HashMap of AST nodes | Yes |
| parser.rs:1985 | Same as above | **Expensive** — same | Yes |
| post_parser.rs:2752 | `HashMap<&ptr, Vec<&ptr>>` | **Cheap** — pointers only | Harder |
| code_hierarchy.rs:416 | `HashMap<&ptr, Vec<&ptr>>` | **Cheap** — pointers only | Harder |

Sites 1–4 are the expensive ones. Sites 5–6 clone only pointer-sized data.

---

## Options for Removing Clone

### Option A: Return references from parser cache methods

Change `get_code_map`, `expect_code_map`, `get_parseds`, `expect_parseds` to return `&FileCoordinateMap` instead of owned values. This is what Scala does — the cache owns the data, callers borrow it.

```rust
pub fn get_code_map(&mut self) -> Result<&FileCoordinateMap<'a, String>, FailedParse<'a>> {
    self.get_parseds()?;
    Ok(self.code_map_cache.as_ref().unwrap())
}

pub fn get_parseds(&mut self) -> Result<&FileCoordinateMap<'a, (FileP<'a, 'p>, Vec<RangeL>)>, FailedParse<'a>> {
    if self.parseds_cache.is_some() {
        return Ok(self.parseds_cache.as_ref().unwrap());
    }
    // ... populate cache ...
    Ok(self.parseds_cache.as_ref().unwrap())
}
```

**Pros**: Direct Scala parity. Eliminates all 4 expensive clones. No new types.
**Cons**: Callers hold a `&` borrow on the parser, which may conflict with `&mut self` calls elsewhere. May require some refactoring of caller code to work with references instead of owned values.

### Option B: Wrap in `Rc` / `Arc`

Wrap the cached maps in `Rc<FileCoordinateMap>`. Cloning an `Rc` is O(1).

```rust
pub fn get_parseds(&mut self) -> Result<Rc<FileCoordinateMap<'a, (FileP<'a, 'p>, Vec<RangeL>)>>, FailedParse<'a>> {
    if let Some(ref parseds) = self.parseds_cache {
        return Ok(Rc::clone(parseds));
    }
    // ...
}
```

**Pros**: Cheap sharing without lifetime complications. Callers can hold the Rc as long as needed.
**Cons**: Adds refcounting overhead. Diverges from Scala's model. Rc is not Send/Sync (Arc would be, but heavier).

### Option C: Split the `Contents: Clone` bound off `map()`

Even if Clone stays on the struct, remove it from the main impl block so most methods don't require it:

```rust
impl<'a, Contents> FileCoordinateMap<'a, Contents> {
    pub fn new() -> Self { ... }
    pub fn put(...) { ... }
    // all methods that don't need Clone
}

impl<'a, Contents: Clone> FileCoordinateMap<'a, Contents> {
    pub fn map<T, F>(...) -> FileCoordinateMap<'a, T> { ... }
    // only methods that actually need Clone
}
```

**Pros**: Makes it clear which operations require Clone. Doesn't block other work.
**Cons**: Doesn't actually eliminate any clones — just a cleanliness improvement.

### Option D: Make `map()` take ownership instead of cloning

Change `map()` to consume `self`, avoiding the need to clone `package_coord_to_file_coords`:

```rust
pub fn map<T, F>(self, func: F) -> FileCoordinateMap<'a, T>
where F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
{
    FileCoordinateMap {
        package_coord_to_file_coords: self.package_coord_to_file_coords, // moved, not cloned
        file_coord_to_contents: self.file_coord_to_contents.into_iter()
            .map(|(k, v)| (k, func(k, &v)))
            .collect(),
    }
}
```

**Pros**: Zero-copy for `map()`. Removes `T: Clone` bound on map.
**Cons**: Caller must own the map. Only fixes site 6, not the parser cache clones.

---

## Recommendation

**Option A** is the closest to Scala parity and fixes the 4 expensive clones. Start there, and combine with **Option C** (split the Clone bound) for cleanliness. Sites 5 and 6 are cheap pointer clones and can stay as-is.

If Option A causes borrow-checker difficulties (e.g., callers need the map while also mutating the parser), fall back to **Option B** with `Rc`.
