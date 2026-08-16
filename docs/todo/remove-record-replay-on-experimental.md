# Removing record/replay on `experimental` — execution guide

**Purpose:** step 1 of redoing the FFI-drop arc on `experimental`. Removing
record/replay FIRST means the redone FFI flip never has to keep replay green
through the transition — which is where the original arc burned its hardest
days (the PSBCBO sign-convention saga, the interface-return replay SIGSEGV,
the read-side adjuster). Kill replay first and none of that work ever needs
to happen.

**Provenance:** written 2026-07 from the `pre-squash-ffi-drop` branch, where
the equivalent removal was executed and verified (suite 1260/1260 after).
All `experimental` line numbers below were verified against
`experimental@c160c2db8`. The worked example for the *full* retirement
(including Linear itself, which this guide deliberately does NOT do) is the
uncommitted diff on `pre-squash-ffi-drop` after checkpoint `a23561b2f`.

**Design rationale and successor:** `todo/metaprogrammed-record-replay.md`
(carry that doc over to experimental with this change — it is the plan of
record for what replaces this machinery).


## 0. Scope — the one thing this guide must not get wrong

On `pre-squash-ffi-drop`, Linear had already been demoted to replay-only (the
FFI-drop arc moved FFI to opaque handles), so the retirement deleted Linear
wholesale. **On `experimental` the situation is INVERTED: Linear is still the
live FFI mechanism.** Imm values crossing to C are linearized into buffers by
Linear's serialize path; imm values crossing back are reconstructed by
RCImm's unserialize path. Those are FFI-live, not replay-only.

**DELETE (replay-only):**
- `Backend/src/determinism/` (determinism.cpp + determinism.h) — entirely
- the recording/replaying branches of the extern-call wrapper in externs.cpp
- the deterministic-mode start/stop in mainFunction.cpp
- the `enableReplaying` / replay-whitelist flag plumbing, all five layers
- `--vale_record` / `--vale_replay` runtime args (they die with determinism.cpp,
  which emits their parsing)
- replay.rs, `assert_replay_test`, the `replayprint` fixture (if present)
- (optional, phase 2) Linear's `useOffsets`/adjuster machinery — dead once
  replay is gone, since FFI always uses `useOffsets=0`

**KEEP (FFI-live on experimental — do not touch):**
- `Backend/src/region/linear/` — all of it
- RCImm's unserialize family (`getUnserializePrototype`,
  `defineConcreteUnserializeFunction`, edge/interface unserialize thunks,
  `topLevelUnserialize`, `callUnserialize`) — this is the FFI receive path
- `receiveUnencryptedAlienReference` on IRegion/RCImm/Linear — FFI receive
- `boundary.cpp` — both helpers serve FFI
- Linear's serialize family, `serializeName`/`unserializeName` etc. in
  globalstate, the per-kind Linear registrations in vale.cpp
- `getExternalType`'s delegation to Linear — that IS the old FFI's C-ABI

(All of the KEEP list gets deleted later, as the *final* slice of the redone
FFI arc — at which point the pre-squash-ffi-drop retirement diff is the
template. Not now.)


## 1. Pre-flight facts (verified at experimental@c160c2db8)

- replay.rs has **20 tests, 19 already `#[ignore]`d** ("deferred at
  experimental-2 squash baseline"). The ONE still-running test is
  `strreturnexport_replay` (`strreturnexport`, 6, 12).
- experimental's externs.rs has **zero** imm-fixture tests — replay.rs run-0
  was the only imm-FFI coverage, and 19/20 of those are ignored. So imm-FFI
  is already effectively uncovered on experimental; this removal makes that
  no worse, but the redone FFI arc must rebuild that coverage (the imm-FFI
  section of pre-squash-ffi-drop's externs.rs is the template).
- Flag plumbing is POD-struct-based (no getopt): valec clap args →
  `midas.rs` → `BackendCompileOptions` → C POD (`backend_options_ffi.h`) →
  `valeopts.cpp` apply → `ValeOptions`.
- determinism.cpp and linear.cpp carry 2 + 9 `// VCOORD:` annotations;
  deleting/keeping per this guide retires the determinism ones with the file.


## 2. Backend removal, step by step

### 2a. Delete the determinism module

```
git rm Backend/src/determinism/determinism.cpp Backend/src/determinism/determinism.h
```

Remove both entries from `Backend/CMakeLists.txt` (they sit near
`src/fileio.cpp`). Do NOT remove the linear entries.

### 2b. externs.cpp — collapse the extern-call wrapper

At experimental line numbers:

- Delete `replayExportCalls` (**:35–:60**).
- Delete `replayReturnOrCallAndOrRecord` (**:168–~:323**, the whole
  function including its "Three options:" header comment). Everything in it
  except the `!enableReplaying` early-out is replay machinery. Note its
  `RecordingMode::` references die with determinism.h.
- Rewrite its one caller (**:568**) from

  ```cpp
  return replayReturnOrCallAndOrRecord(
      globalState, functionState, builder, prototype, args,
      [ ... ](LLVMBuilderRef builderWhenNotReplaying) {
        return buildCallOrSideCall(...);
      });
  ```

  to the simple path (this is what the deleted function's
  `!enableReplaying` branch did):

  ```cpp
  auto valeReturnRef = buildCallOrSideCall(globalState, functionState, builder, prototype, args);
  return buildResultOrEarlyReturnOfNever(globalState, functionState, builder, prototype, valeReturnRef);
  ```

- Remove `#include "determinism/determinism.h"`.
- `buildResultOrEarlyReturnOfNever` stays — the simple path uses it.

### 2c. mainFunction.cpp

- Delete the `if (globalState->opt->enableReplaying) { ...
  buildMaybeStartDeterministicMode ... }` block (**:192–:206**), including
  the argv-shifting arithmetic inside it (that existed only to strip
  `--vale_record`/`--vale_replay` from argv).
- Delete the `buildMaybeStopDeterministicMode` block (**:216–:219**).
- Remove the determinism include.
- **Trap (hit on pre-squash-ffi-drop):** determinism.h transitively provided
  `utils/definefunction.h` (for `addRawFunction`). Add
  `#include <utils/definefunction.h>` explicitly or the build breaks here.
- The `int8PtrPtrLT` local likely becomes unused → remove (warning
  discipline).

### 2d. globalstate.h / vale.cpp

- globalstate.h: delete `Determinism* determinism = nullptr;` and the
  `class Determinism;` forward declaration.
- vale.cpp: delete the `Determinism determinism(globalState);` +
  `globalState->determinism = &determinism;` construction, the two
  `determinism.registerFunction(prototype)` calls (one in the
  extern-declaration loop, one in the export loop), and
  `determinism.finalizeFunctionsMap();`. Remove the include.
- Do NOT touch `serializeName`/`unserializeName`/`freeName` interning or any
  `linearRegion->declare*/define*` registration — FFI-live here.

### 2e. Flag plumbing — five layers, backend side first

- `Backend/src/backend_options_ffi.h` **:40–:46**: delete
  `enable_replaying`, `replay_whitelist_count`,
  `replay_whitelist_modules`, `replay_whitelist_functions` from the POD.
- `Backend/src/valeopts.cpp` **:38–:45**: delete the lines applying those
  POD fields to `ValeOptions`.
- `Backend/src/valeopts.h` **:40**: delete `bool enableReplaying` and the
  `projectNameToReplayWhitelistedExterns` map (check whether the
  `unordered_set` include becomes unused).


## 3. FrontendRust removal

### 3a. POD mirror + marshaling — `backend_ffi/mod.rs`

Delete, keeping the struct layouts in sync with 2e (the repr(C) mirror MUST
match the C POD field-for-field):
- repr(C) fields **:34–:37**
- `BackendCompileOptions::{enable_replaying, replay_whitelist}` (**:68–:70**)
  and their defaults (**:90–:91**)
- the CString marshaling block (**:107–:134** region: `module_cs`,
  `function_cs`, pointer arrays, and the four struct-init lines)

### 3b. valec CLI

- `bin/valec/build.rs`: the `replay_whitelist_extern` arg (**:47–:49**), the
  `enable_replaying` flag (**:80**), and both from the
  `build_backend_argv`/options call (**:241**).
- `bin/valec/midas.rs`: the two params (**:30–:31**), the
  `opts.enable_replaying = ...` line (**:45**), and the whitelist
  `split_once('.')` block (**:69–:72**).

### 3c. Test harness

- `end_to_end_tests/mod.rs`: delete `assert_replay_test` (**:323+**,
  including its `|opts| { opts.enable_replaying = true; }` closure) and the
  doc-comment mention of `enable_replaying` (**:118**).
- **Convert, don't just delete, the one live test:** `strreturnexport_replay`
  is the only non-ignored replay test and thus the only live coverage of its
  fixture. Add to externs.rs:

  ```rust
  #[test] fn strreturnexport() { run("programs/externs/strreturnexport", 6); }
  ```

  (6 = the old run-0 expected value. Keep the wasi-skip status quo if
  externs.rs's `run` doesn't already handle it — check whether the fixture
  passes under wasi before deciding.)
- Delete `end_to_end_tests/tests/replay.rs` and its `pub mod replay;` line
  in `tests/mod.rs`. The other 19 tests are already ignored with a
  deferred-at-baseline marker; the metaprogrammed design doc's §Test-port
  map is the record of what they'll become.
- Delete the `replayprint` fixture if present (it requires the now-gone
  whitelist flag; grep `src/tests/programs` for `replay`).

### 3d. Expected suite delta

Baseline 1084 passed / 0 failed / 119 ignored. After: −1 passing (the replay
test) +1 (its conversion) = **1084 passed**, ignored **119 → 100**. Verify
the exact numbers locally; treat any *other* delta as a regression.


## 4. Optional phase 2: Linear's dead adjuster machinery

After replay removal, Linear's PSBCBO/offsets mode is dead code: the FFI
path always creates region instances with `useOffsets = 0`, so the
serialized-address-adjuster is always zero. (This is the mirror image of
pre-squash-ffi-drop, where the FFI mode was the dead one.) If taking this
now:

- Delete the `useOffsets` and `bufferBeginOffset` region-struct fields +
  accessors, the `createRegionInstanceLocal` params, and the
  `buildIfElse(useOffsetsLE, ...)` in `topLevelSerialize` — keep the
  **zero-adjuster** branch (opposite of the pre-squash cleanup, which kept
  the nonzero one).
- `setRegionInstanceSerializedAddressAdjuster` / PSBCBO comments / the
  adjust-on-deref sites in `translateBetweenBufferAddressAndPointer` can then
  collapse (adjuster ≡ 0 ⇒ translate is identity); that touches many
  linear.cpp sites, so it's fine to defer — the whole file dies at the end
  of the redone arc anyway. Doing just the field/param removal is cheap and
  keeps the file honest; doing the full collapse is probably wasted motion.
- If deferring entirely: fine. Note it in the followups doc so it isn't
  mistaken for live machinery.

Renumbering trap from the pre-squash execution: the region-instance struct
fields are accessed by GEP **index**; removing a field renumbers every
accessor below it. Update them together, then run the extern tests before
anything else.


## 5. Verification

1. `cargo build --manifest-path Cargo.toml --lib` — clean, zero
   warnings (backend rebuilds via build.rs).
2. Full suite: `cargo nextest run --manifest-path Cargo.toml`
   — expect the §3d numbers exactly.
3. Sweep greps — all must come back empty:

   ```
   grep -rn 'enableReplaying\|enable_replaying\|replay_whitelist\|RecordingMode\|vale_record\|vale_replay\|determinism' Backend/src src
   grep -rn 'assert_replay_test\|replayReturnOrCallAndOrRecord\|replayExportCalls\|DeterministicMode' Backend/src src
   ```

   (One benign hit class: comments about interner/iteration "determinism" in
   typing/ — unrelated, leave them.)
4. Run one extern test explicitly end-to-end (e.g. the converted
   `strreturnexport`) to confirm the FFI path is genuinely unaffected.


## 6. Traps learned from the pre-squash execution

1. **The coverage trap.** Replay tests' run 0 is a plain FFI run; wherever a
   replay test is the only referent of a fixture, deletion silently removes
   FFI coverage. On experimental only `strreturnexport_replay` is live, but
   re-check with an orphan scan before deleting fixtures:
   for each fixture dir, grep the test tree for `externs/<name>"`.
2. **Include transitivity.** determinism.h pulled in utils headers for
   bystander files (mainFunction.cpp / `addRawFunction` was the instance
   here). Expect one or two such breaks; fix with direct includes.
3. **Unused locals after block deletions** (`int8PtrPtrLT`, name fields,
   `hostRegionInstanceRef` threading). Chase warnings to zero — they're the
   tell that a deletion was left half-done.
4. **Don't touch the unserialize family here.** On pre-squash it was
   replay-only and died; on experimental it's the FFI receive path. If a
   grep for "unserialize" tempts you, re-read §0.
5. **argv semantics.** `buildMaybeStartDeterministicMode` consumed
   `--vale_record`/`--vale_replay` and shifted argv. After removal, programs
   see raw argv. Nothing on experimental depends on the shifting (it only
   fired under the flags), but if a getMainArg-style feature lands later,
   argv indexing assumptions change here.
6. **Guardian/CI text references.** `guardian.toml`, CI docs, and skill docs
   may mention replay tests or flags; grep docs/ and .github/ for
   `enable_replaying` and `replay` after the code sweep.


## 7. What comes after (redo sequencing)

1. **This guide** — replay gone; suite green at §3d numbers; Linear now has
   exactly one consumer: the old FFI.
2. **Redo the FFI-drop arc** (opaque handles, `__vbi_` string intrinsics,
   getMainArg, always-OWN) — now WITHOUT any replay burden: no PSBCBO
   restoration, no interface-return replay fix, no adjuster work, no
   dual-mode boundary discriminator. Reference: `pre-squash-ffi-drop`
   checkpoints `7e08e98bb` (the arc) — subtracting everything the followups
   doc marks as replay-related.

   **Handle ABI decision (architect, 2026-07) — do NOT reuse the 32-byte
   universal ref.** _Implemented on `pre-squash-ffi-drop`: the right-sizing
   below is live (8B concrete / 16B interface, plain i64 pointer-bit fields)
   in `Backend/src/region/handlestructlt.{h,cpp}`. Scrambling is still
   DEFERRED — today the fields carry plain pointer bits, so read "scrambled
   obj ptr" below as "obj ptr" until scrambling lands. For why all concretes share one LLVM type while each class gets its own C typedef, see @HTSLVBDTCZ (`Backend/docs/arcana/HandleTypesIsSameLLVMValueButDifferentTypedefsInC-HTSLVBDTCZ.md`)._ Handles stay *structs*
   (per-kind C typedefs, so C gets type distinctness and can't mix a
   `vtest_Firefly` with a `vtest_IShip` or a raw pointer) but are **sized to
   exactly what the ref layer needs**:

   - concrete share (struct/str/RSA/SSA): 8-byte struct
     `{ scrambled obj ptr }` — one register on every 64-bit ABI
   - interface share: 16-byte struct
     `{ scrambled obj ptr, scrambled typeinfo ptr }` — two registers on
     SysV; wasm32 still needs sret for these but not for concretes
   - future layers size themselves the same way, keeping the principle
     aligned with onion: handle shape is a function of the ref layer. The
     C type surface is (exported kind) x (crossing layer), each pair its
     own typedef: bare name for the canonical layer, suffixes for the rest
     (e.g. `vtest_Firefly_weak` = `{scrambled obj, gen}` 16B,
     `vtest_IShip_weak` = `{scrambled obj, scrambled typeinfo, gen}` 24B).
     Layer transitions are generated functions with honest signatures
     (`vtest_Firefly_downgrade` -> weak; `_weak_lock` -> strong, with the
     reserved-zero handle as the dead result). Emit only the (kind, layer)
     typedefs the exported API actually uses. Nested onion layers (&&T) are
     not expressible at the FFI — compile error at the export declaration.

   Rejected: type-info-in-object-header (fattens every share object — the
   architect wants objects small); always-32-byte universal ref (for
   concrete share kinds only the object pointer field was ever live —
   `explodeForRegularConcrete` discards the rest; generations/region
   ptr/tether bits are dead weight in this ABI). Consequences: the
   i52/i56 `buildCompressStruct` bit-packing dies; `typeNeedsPointerParameter`
   machinery shrinks to the interface-on-wasm32 case; `_ref_eq` compares the
   scrambled obj field directly (XOR with the build-time constant key
   preserves equality); scramble both fields of the interface handle for a
   uniform tripwire.
3. **Retire Linear** as the arc's final slice — at that point it is
   replay-free AND FFI-free, so the deletion is nearly mechanical; the
   uncommitted retirement diff after `a23561b2f` on `pre-squash-ffi-drop` is
   the template (including the RCImm::getExportName primitive-naming
   relocation and the ICodec-fold-back it subsumes).
4. Somewhere in 2–3, port the imm-FFI test suite (the externs.rs imm section
   on `pre-squash-ffi-drop`) with the `imm`→`share` keyword and no-imm-arrays
   adjustments experimental requires.
