# Postparser Interning: Dual-Enum Pattern For Lookups (IDEPFL)

Scala's `Interner` used GC-backed `HashMap[T, T]` to canonicalize case classes. Rust replaces this with arena-backed interning using two parallel enums per type hierarchy: a **reference enum** (canonical, holds `&'a` pointers) and a **value enum** (owned, used as HashMap lookup keys).

---

## The Five Dual-Enum Pairs

| Reference Enum (canonical) | Value Enum (lookup key) | Interner Method |
|---|---|---|
| `IRuneS<'a>` | `IRuneValS<'a>` | `intern_rune()` |
| `IImpreciseNameS<'a>` | `IImpreciseNameValS<'a>` | `intern_imprecise_name()` |
| `INameS<'a>` | `INameValS<'a>` | `intern_name()` |
| `IFunctionDeclarationNameS<'a>` | `IFunctionDeclarationNameValS<'a>` | via `INameS` |
| `IVarNameS<'a>` | `IVarNameValS<'a>` | via `INameS` |

---

## How They Differ

The **reference enum** holds `&'a` references to arena-allocated payloads:

```rust
pub enum IRuneS<'a> {
  CodeRune(&'a CodeRuneS<'a>),
  ImplicitRune(&'a ImplicitRuneS),
  ImplicitRegionRune(&'a ImplicitRegionRuneS<'a>),
  // ...
}
```

The **value enum** holds the same payload structs *by value* (owned):

```rust
pub enum IRuneValS<'a> {
  CodeRune(CodeRuneS<'a>),
  ImplicitRune(ImplicitRuneS),
  ImplicitRegionRune(ImplicitRegionRuneValS<'a>),
  // ...
}
```

---

## Why Both Exist

You can't skip the Val enum because:

1. The reference enum contains `&'a` pointers — you can't construct one without first allocating into the arena.
2. You need an owned, hashable key to check whether a value was already interned.
3. If you allocated first and then checked, you'd waste arena space on duplicates.

---

## Interning Flow

1. **Build a Val** — construct an owned `IRuneValS` with all data inline.
2. **Look it up** — the interner checks `HashMap<IRuneValS<'a>, IRuneS<'a>>`. If found, return the existing canonical `IRuneS`.
3. **Allocate if new** — allocate the payload into the `'a` arena via `self.arena.alloc(payload)`, wrap the `&'a` ref in the corresponding `IRuneS` variant, store the mapping, return it.

```rust
// Caller builds a Val, interner returns canonical ref:
let rune = self.interner.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneS {
    lid: lidb.child().consume(),
}));
// rune is IRuneS::ImplicitRune(&'a ImplicitRuneS)
```

---

## Simple vs Shallow Val Variants

### Simple: same struct in both enums

When a payload struct contains only simple/Copy fields (like `StrI<'a>`), the Val enum holds the same struct type by value. No separate Val struct is needed:

- `IRuneS::CodeRune(&'a CodeRuneS<'a>)` — reference
- `IRuneValS::CodeRune(CodeRuneS<'a>)` — owned

### Shallow: separate Val struct for nested interned types

When a payload struct contains references to *other* interned types, a separate Val struct exists. The Val struct holds the child as an already-canonical owned `IRuneS<'a>` (it's "shallow" — children must be interned first):

```rust
// Canonical payload (lives in arena):
pub struct ImplicitRegionRuneS<'a> {
  pub original_rune: IRuneS<'a>,
}

// Lookup key (owned, for HashMap):
pub struct ImplicitRegionRuneValS<'a> {
  pub original_rune: IRuneS<'a>,
}
```

The fields look identical in this case, but they're separate types so the type system enforces going through the interner. You can't accidentally use a Val where a canonical ref is expected.

Note: `IRuneS<'a>` is already just a tagged pointer (discriminant + `&'a` to arena payload), so holding it owned vs `&'a IRuneS<'a>` is storing the tagged pointer directly vs a pointer-to-a-pointer. Owned is simpler and equally cheap. Identity is checked via `IRuneS::ptr_eq`/`canonical_ptr` which look at the inner payload pointer.

Other shallow Val structs follow the same pattern:
- `ImplicitCoercionOwnershipRuneValS` — holds `IRuneS<'a>` for its child rune
- `AnonymousSubstructImplDeclarationNameValS` — holds `&'a TopLevelInterfaceDeclarationNameS<'a>`
- `ImplImpreciseNameValS` — holds two `IImpreciseNameS<'a>` children
- `ForwarderFunctionDeclarationNameValS` — holds `IFunctionDeclarationNameS<'a>`

**Rule**: intern children first, then build the parent Val with canonical child runes.

---

## Identity via `ptr_eq`

The canonical enums provide `ptr_eq()` and `canonical_ptr()` methods. Since the interner guarantees structurally equal values get the same arena allocation, pointer equality is identity equality:

```rust
impl<'a> IRuneS<'a> {
  pub fn canonical_ptr(&self) -> *const () { /* extracts inner &'a pointer */ }
  pub fn ptr_eq(&self, other: &IRuneS<'a>) -> bool {
    std::ptr::eq(self.canonical_ptr(), other.canonical_ptr())
  }
}
```

This mirrors Scala's `eq` (reference equality) after interning.

---

## Conversion: Ref → Val

`IFunctionDeclarationNameS` provides `to_val()` for converting a canonical reference back to a value key. This is used when you have an existing canonical name and need to build a parent Val that wraps it.

---

## What Scala Had

In Scala, all of these were just `case class`es extending `sealed trait`s with `IInterning`. The `Interner` used `HashMap[T, T]` where the JVM's GC managed memory and `eq` gave reference identity. Rust splits each sealed trait into two enums to separate "owned value for lookup" from "arena-backed reference for storage."
