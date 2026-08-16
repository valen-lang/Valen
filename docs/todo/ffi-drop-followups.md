# FFI-drop arc — deferred work and known gaps

> **Note (2026-07, post-arc):** the Linear region and all record/replay
> machinery were subsequently RETIRED from the backend (see
> `todo/metaprogrammed-record-replay.md`). Sections below that
> reference linear.cpp, determinism.cpp, replay tests, or ICodec describe
> code that no longer exists; they remain accurate as history of the arc.

This doc captures every gap, deferral, and cosmetic cleanup that surfaced
during the FFI-drop arc (branch `pre-squash-ffi-drop`, based off `dcfbc055a`,
covering slices 4a-4g, 5, 5b, 6, 7).

At the end of the arc: **1259/1259 tests pass**, 22 skipped. All 22 skips
are pre-existing (WASI, experimental-2 baseline). Every capability
regression introduced by the arc has been closed in-arc:
- §1a interface-return replay — fixed via a design-consistent restoration
  of PSBCBO in linear.cpp + determinism.cpp
- §1b getMainArg — restored via a new `__vbi_getMainArg` intrinsic
- §1d OWN-flavored extern args — restored by making the extern-param ABI
  uniformly OWN (matches the other 3 FFI positions). Alias-before-send in
  `Backend/src/function/expressions/externs.cpp` bumps the object's RC so
  C receives a real +1; C discharges it by explicit `_dealias` before
  return or by ownership transfer (e.g. passing to a Vale export call).

Coverage added at end-of-arc (before commit) to leave the tree strictly ahead
of the `dcfbc055a` baseline:

- 8 direct `__vbi_` string-intrinsic tests
  (`floattostr`, `strcmp`, `substring`, `strindexof`, `strtoascii`,
  `strfromascii`, `stradd_empty`, `stradd_chained`) in
  `src/end_to_end_tests/tests/strings.rs`. These lock in the
  slice-5b intrinsic bodies at `Backend/src/function/expressions/externs.cpp:
  510-750`, five of which had zero coverage before.
- 3 replay fixtures exercising `__vbi_addStr` + `__vbi_castI32Str`,
  `__vbi_substring`, and `__vbi_castI32Str` through the record/replay path
  (`stradd_fromextern_replay`, `substring_fromextern_replay`,
  `casti32str_fromextern_replay`). None of the new intrinsics had any
  record/replay coverage before.

Everything in this doc is a candidate to pick up in a follow-up session.

---

## 1. Correctness gaps that survived the arc

### 1a. Interface-return replay — FIXED in-arc (design-consistent restoration of PSBCBO)

Both tests now pass:
- `end_to_end_tests::tests::replay::interfaceimmreturnextern_replay`
- `end_to_end_tests::tests::replay::interfaceimmreturnexport_replay`

**What was actually broken.** Two separate cross-process bugs in the linear
region's serialization pipeline, both violating the PSBCBO design (see the
PSBCBO/PRCBO appendix in `todo/metaprogrammed-record-replay.md`):

1. **`defineEdgeSerializeFunction` (linear.cpp:371) never wrote the interface
   fat struct into the buffer.** It built `{obj_ptr, edge_num}` as an
   in-register LLVM value and returned it. On replay, the reader loaded the
   concrete struct's bytes as if they were the fat struct → SIGSEGV.

2. **The whole write pipeline had the adjuster sign flipped from the design
   intent, and the read side never set an adjuster.** Per PSBCBO the stored
   form on disk should be `real - adjuster = destOffset + fileOffset` (the
   absolute file position of the target — process-independent). The code was
   storing `real + adjuster` and relying on same-process round-tripping. No
   existing test hit this because every recorded value is a struct-of-
   primitives (no pointer field ever crosses the disk boundary).
   Interface-return is the first test with an on-disk pointer to marshal.

**The fix (three sites, design-consistent):**

1. `linear.cpp defineEdgeSerializeFunction` (line 371) — reserve the fat
   struct's slot up-front (mirrors the RSA/str "reserve then fill" pattern),
   call the concrete struct serialize (appends its bytes after our slot),
   then LLVMBuildStore the assembled `{obj_ptr, edge_num}` into the reserved
   slot. Under the corrected PSBCBO convention `upcast()`'s `obj_ptr` is
   already stored form. Also added an `InterfaceKind` case to
   `predictShallowSize`.

2. `linear.cpp` — flipped the `translate(...)` sign at 7 sites so the write
   pipeline matches PSBCBO: `getDestinationRef` now applies
   `translate(true)`=SUB (produces `real - adjuster` = stored form), and
   the paired stores in `initializeMember`, `innerConstructStaticSizedArray`,
   `innerConstructRuntimeSizedArray`, `innerMallocStr`,
   `pushRuntimeSizedArrayNoBoundsCheck`, `initializeElementInSSA` all now
   apply `translate(false)`=ADD to recover the real pointer for
   `LLVMBuildStore`. FFI encrypt/decrypt path is unaffected (it uses
   `useOffsets=0` where the adjuster is always 0, so the flag flip is a
   no-op).

3. `determinism.cpp buildReadValueFromFile` — after `createRegionInstance-
   Local`, compute `readAdjusterLE = tempBufferPtrLE - fileOffset` and call
   `setRegionInstanceSerializedAddressAdjuster` (moved to public in
   linear.h). For the non-interface top-level case, pass a stored-form
   pointer (`fileOffset` int-to-ptr'd) instead of `tempBufferPtrLE` to
   `receiveHostObjectIntoVale`, so `loadMember2`'s translate applies against
   the read-side adjuster and recovers the correct real pointer.

**Why existing struct-of-primitives replay tests still pass:** those records
contain no pointer fields on disk, so the sign flip changes no stored bytes.
Reader-side changes leave read-back of primitive fields correct because the
new adjuster fed into `translate(false)` at load time exactly cancels the
new "stored form" of the top-level pointer we now pass in.

**Follow-up to lock this in:** add a replay fixture with a nested reference
(e.g. an RSA of interfaces, or a struct with a str field returned from C)
so the marshaling stays exercised. Today only the interface fat struct
pointer is on-disk-marshaled; nested-refs-within-concrete-structs is
theoretically supported by the design but has no test coverage.

--------------------------------

**Historical context (kept for future spelunking):** originally
marked `#[ignore]` in `src/end_to_end_tests/tests/replay.rs`.
These tests did not exist at `dcfbc055a` baseline — they were added during
slice 3b. They exercise a scenario that never had coverage before: **a Vale
program in replay mode receiving a fresh interface value that C constructed
during recording**.

Repro (manual):

```
cargo nextest run interfaceimmreturnextern_replay
cd tmp/vale-test-runs/end_to_end_tests-tests-replay-interfaceimmreturnextern_replay/out
# Run 0 (normal) exits 42 — OK
./a.out
# Run 1 (record) exits 42, writes recording.bin — OK
./a.out --vale_record recording.bin
# Run 2 (replay) exits 139 (SIGSEGV) — the bug
./a.out --vale_replay recording.bin
# Crash location via lldb:
lldb -o "run --vale_replay recording.bin" -o "bt 25" -o "quit" ./a.out
# EXC_BAD_ACCESS at address 0x2a in __main_argc_argv + 660
```

**Root cause (as far as investigated):** The linear-region serializer under-
records interface values. The fixture's `cMakeShip()` builds a `Firefly(42)`,
upcasts to `IShip`, returns it. On record, `determinism.cpp:buildWriteValueToFile`
walks the interface value through `Linear::receiveUnencryptedAlienReference`
→ `topLevelSerialize`. What lands in `recording.bin` after the header is
**16 bytes** — exactly one interface fat struct `{void* obj, uint64_t type}`
containing `{obj=0x2a, type=0}`.

The `0x2a` is the Firefly's `fuel` field value (42) being written into the
interface's `obj` slot. The Firefly's actual struct bytes are never written
after the fat struct — so on replay, the reconstruction reads `obj=42` as
though it were a linear-buffer offset and dereferences it.

Flare trace during replay (excerpted, with `VALE_FLARES=1`):
```
buildReadValueFromFile: Read I64: 16       ← size from file
Malloc'ing size 16
linear.cpp:2208 edge num: 0                ← type tag: correct
linear.cpp:2209 ptr: 42                    ← obj pointer: WRONG (fuel value)
Suspending function __vale_unserialize_thunk
Calling function __vale_unserialize
[SIGSEGV in main() at ldr w22, [x22] where x22 = 0x2a]
```

**Where to start:** The bug is in the linear-region interface serialize path,
not in slice 5 changes. Look at:

- `Backend/src/region/linear/linear.cpp:1304 topLevelSerialize` — the entry
  point for serializing a Vale-side share value into a linear buffer.
- `Backend/src/region/linear/linear.cpp:1549 callSerialize` — dispatches
  interface case via virtual dispatch to `defineEdgeSerializeFunction`.
- `Backend/src/region/linear/linear.cpp:371 defineEdgeSerializeFunction` —
  emits the Firefly→IShip serialize thunk. It calls the concrete struct's
  serialize (which walks fields into the buffer), then upcasts to interface.
- `Backend/src/region/linear/linear.cpp:1473 predictShallowSize` — **does not
  have an InterfaceKind case**; falls through to `assert(false); throw 1337;`.
  Suggests interface serialize takes a different sizing path — worth
  understanding.

Hypothesis to verify: `topLevelSerialize` for interface only writes the fat
struct and doesn't recursively serialize the concrete struct's fields into
the buffer. Or it does, but the fat struct's `obj` field is being overwritten
by the concrete struct's data (aliasing bug).

**Note on scope:** dcfbc055 didn't test this specific direction (C returning
a freshly-constructed interface via extern → Vale). The two ignored tests are
the first coverage for this path. Investigation should confirm whether the
bug is truly pre-existing or an interaction with slice 5's changes.

### 1b. `getMainArg` intrinsic — FIXED in-arc

Slice 5b deleted the old C implementation because it depended on `ValeStrNew`
(part of the removed `ValeStr*` allocator block). Restored via the
`__vbi_getMainArg` intrinsic in `Backend/src/function/expressions/externs.cpp`
(mirrors `__vbi_castI64Str`'s shape): calls two C helpers in
`Backend/builtins/mainargs.c` — `__vale_rt_get_main_arg_len(i)` returns
argv[i]'s length and `__vale_rt_get_main_arg_ptr(i)` returns the argv[i]
byte pointer — then `mallocStr(len, ptr)` copies into a fresh Vale share
str. The Vale-visible `func getMainArg(i int) str` wrapper is back in
`src/builtins/resources/mainargs.vale` and its Scala mirror.
Locked in by `end_to_end_tests::tests::externs::getmainarg_basic` which
runs the compiled program with `hello` as argv[1] and asserts return 5.

--------------------------------

**Historical context (for future spelunking):** the deleted C code looked like:

```c
// Backend/builtins/mainargs.c — deleted:
ValeStr* __vale_getMainArg(int64_t i) {
  char* argCStr = __main_args[i];
  int64_t len = strlen(argCStr);
  ValeStr* vstr = ValeStrNew(len);
  strncpy(vstr->chars, argCStr, len);
  return vstr;
}
```

The Vale-side `extern func getMainArg(i int) str` declaration was removed too
(from both `src/builtins/resources/mainargs.vale` and the Scala
mirror). Only `numMainArgs()` (primitive-returning) still works.

An `#[ignore]`d `getmainarg_basic` marker test now lives in
### 1c. C-side construction of Vale strings from raw bytes

Under the old ABI, `ValeStrFrom("hello")` gave C a valid `ValeStr*` that Vale
could accept. Under the opaque-handle FFI there is no equivalent primitive —
C can't allocate a Vale-managed string.

Current mitigation: user-authored Vale factories (e.g. `makeRepeatingHello(n
int) str { … }`) that C calls via extern to get a str. Works but limits
what dynamic content C can produce.

**No test regressed** on this — the one test that used `ValeStrFrom` (in
`structmutreturnexport`) was rewritten to use a Vale-side factory during
slice 3a. Documented as a **capability gap**: C-side dynamic-string
construction from raw bytes is not currently possible.

**To restore:** add a `__vbi_str_from_bytes(ptr *u8, len i32) str` intrinsic.
Blocker: Vale doesn't currently have a raw-pointer type in the surface
language. Options:

- Add `*u8` as a Vale primitive
- Alternative: `__vale_rt_alloc_str_bytes(len i32) *u8` returning a raw
  buffer that C fills, then a `__vbi_finalize_str(buf, len) str` that wraps
  it. Requires the raw pointer type either way.
- Or: make it a C helper (`__vale_rt_construct_str_from_bytes`) that reaches
  into RCImm's control-block layout — ugly, couples C to internal Vale
  layout.

### 1d. OWN-flavored extern arg semantics — FIXED in-arc, later SUPERSEDED

> **SUPERSEDED:** the "always-OWN" scheme below (a backend `alias()` injected at
> the extern-arg site) was later removed in favor of the move/consume model — the
> boundary does no RC, the arg simply moves into C, and C manages ownership
> explicitly. See the arcanum @FRMACZ
> (`Backend/docs/arcana/FFIRefsMoveAccessorsConsume-FRMACZ.md`). The rest of this
> section, and the always-OWN references later in this doc, are kept as arc history.

Restored by making the extern-param ABI **uniformly OWN** — no new syntax,
no `^` marker. This matches how the other three FFI positions were already
behaving (extern-return, export-param, export-return all pass +1's across
the boundary). Only extern-param was BORROW; making it OWN closes the
asymmetry.

**What changed:** in `Backend/src/function/expressions/externs.cpp` the
arg-pack loop in `buildCallOrSideCall` now calls
`getRegion(argRefMT)->alias(...)` on each non-primitive arg before packing
it. That bumps the shared object's RC; Vale's local end-of-scope dealias
balances Vale's own copy; C is now conceptually holding a +1 it must
discharge — either via explicit `Foo_dealias(arg)` before returning, or by
transferring ownership onward (e.g. passing to a Vale export call, which
absorbs the +1 in its own body).

*Why the alias goes at the extern-arg site and not inside
`sendValeObjectIntoHost`:* that helper is shared with the export-
return path in `Backend/src/function/function.cpp`, where an unconditional
alias would leak (Vale-callee's returned +1 is already correct for send).
Localizing the alias to the extern-arg loop keeps the export path silent.

**Fixture proving it works:** `interfaceimmparamextern_owned` was
`#[ignore]`d as a capability marker; now un-ignored and its C impl
explicitly `IShip_dealias(s)`es the received arg before returning — the
exact "C-is-terminal-owner" pattern that the OLD linear-ABI's `free(s.obj)`
supported. Test at `end_to_end_tests::tests::externs::
interfaceimmparamextern_owned` returns 42.

**Aftermath for the pre-existing fixtures:** the 26 existing `param*`
fixture files still have `// s is BORROW (extern arg)` comments. Under the
new ABI those comments are stale — the fixtures still pass because:
- For `imm` types the object is IMMUTABLE_SHARE (RC-free), so the missing
  C-side `_dealias` doesn't leak.
- Passthrough fixtures (C forwards the arg to a Vale export) already
  discharge via the export call's implicit +1 absorption.

The comments could be swept later; not blocking correctness.

---

## 2. Cosmetic cleanups that would tidy the arc

None of these cause bugs; they're all follow-up hygiene.

### 2a. `Linear::generate*DefsC` are dead — FIXED

Post-arc, Linear was decoupled from `IRegion` inheritance entirely (see the
"Linear no longer inherits IRegion" section below). The four dead
`generate*DefsC` methods were deleted outright along with 22 other stubs.
Suite still at 1259/0/22 after the change.

### Linear no longer inherits IRegion — FIXED

Follow-up cleanup after the arc landed. Linear used to be an `IRegion`
subclass with 22 pure-virtual overrides that were `{ assert(false); throw
1337; }` stubs — dead code the class contract forced Linear to carry.

Introduced a small `ICodec` base interface (`Backend/src/region/icodec.h`)
with the 5 methods generic code-gen helpers actually call
(`translateType`, `checkValidReference`, `explodeInterfaceRef`,
`getInterfaceMethodVirtualParamAnyType`, `wrapToLiveRef`). Every method
is a wrap or unwrap operation across the boundary between Vale's
ref/type model and LLVM's value/type model — the interface is
literally a region's LLVM codec.

Both `IRegion` (which adds ~80 more semantic-operation methods) and
Linear inherit from `ICodec`. Full-fat regions (RCImm, Unsafe, mut) are
`(codec + semantics)`; Linear is `codec` only — a serialization format
with no ownership discipline.

`GlobalState::getCodec(Reference*)` is the ONE place where the
Linear-vs-IRegion routing lives. Every generic helper (`buildCallV`,
`buildInterfaceCall`, `loadElement`, `loadInnerInnerStructMember`, both
`toRef` overloads that take a `Reference*`, `toLiveRef`,
`translateInterfaceMethodToFunctionType`, `translateTypes`) reaches
regions through it. `isLinearKind` stays exposed for the FFI-vs-replay
discriminator at `boundary.cpp:40` — the single legitimate consumer.

The 4 `toRef(Region*, ...)` overloads now just take `ICodec*` instead
of the interim template — the concrete region classes upcast
implicitly.

Net: ~500 lines removed from Linear, plus ~50 lines of `if isLinearKind`
branches in shared helpers collapsed into single-line `getCodec(refM)`
dispatches. Linear stands as a self-contained serialization codec rather
than an abstract-region shell, and the region abstraction on the shared-
helper side is a proper base interface rather than a runtime type-tag
check.

### 2b. `sendValeObjectIntoHostAndDealias` renamed — DONE in-arc

Now `sendValeObjectIntoHost` (`Backend/src/function/boundary.h:21` +
`.cpp:74`, callers at `externs.cpp:102` and `function.cpp:180`). The
"AndDealias" was a lie under the silent boundary — RC touching is done at
the extern-arg call site (`externs.cpp` arg-pack loop, via `alias()` for
non-primitive args) rather than inside the send helper. Header now carries
a one-line comment explaining that split.

### 2c. `receiveHostObjectIntoVale` has stale doc comments

The 15-line comment block at `Backend/src/function/boundary.cpp:15-30`
describes the old semantics (`we do need to encrypt it`, `moving/copying
between regions`, `receiveUnencryptedAlienReference`). Under the new ABI the
two branches are `FFI (encrypt/decrypt)` vs `Replay (linear buffer)`, which
is captured accurately in the newer comment block right below at lines 31-40.

The old comment block can be deleted (or the whole comment reduced to a
one-liner referencing lines 31-40). Cosmetic.

### 2d. `Backend/src/vale.cpp` still has stale slots for `_vasp`

Slice 6 deleted `includeSizeParam`, but the `for` loop over params that used
to emit trailing i32 size args at `vale.cpp:321-340` was collapsed to just
`s << ")"; // (Trailing paramNsize size params for the retired _vasp/SASP
ABI have been removed.)`. The comment can be dropped once the git history
is squashed away.

### 2e. `Backend/src/function/expressions/externs.cpp` `(void)argSizeLE`

Already resolved during slice 7. `sendValeObjectIntoHost` (renamed in §2b)
returns `LLVMValueRef` directly; both callers use the value, no `(void)`
discard remains. Entry retained so future spelunkers see the resolution.

### 2f. Test-tmpdir hygiene

`src/end_to_end_tests/mod.rs` contains a `KeepDir` enum with
a `Kept(PathBuf)` variant that's only reached when the `NEXTEST_TEST_NAME`
env var is set. If future test infrastructure work adopts nextest as the
canonical runner, the `KeepDir::Temp(tempfile::TempDir)` fallback could be
removed and every test would auto-preserve its outputs at a known path.

Cosmetic. Kept both branches for compatibility with `cargo test`.

---

## 2g. ASan/LSan opt-in for RC-balance auditing

Wired in during the always-OWN arc for `strlenextern`'s undetected leak.
Turn on with `VALE_TEST_ASAN=1 cargo nextest run …`. Effects:

- `end_to_end_tests/mod.rs` sets `asan: true` on the ClangConfig only when
  the env var is set AND the backend is Native (WASI's wasmtime executor
  doesn't run the sanitizer runtime).
- `clang.rs` adds `-fsanitize=address` (all platforms) and `-fsanitize=
  leak` (non-Apple only — arm64-apple-darwin refuses standalone LSan; on
  Apple, LSan integrates into ASan and is toggled at runtime).
- The `CompiledProgram::run` path sets `ASAN_OPTIONS=detect_leaks=1:
  abort_on_error=1:halt_on_error=1` and `LSAN_OPTIONS=suppressions=<path>`
  when ASan is on. Suppressions file at `lsan-suppressions.txt`
  filters false positives from Apple's ObjC runtime (dyld-init class-list
  allocations that persist for the process lifetime by design).

**Known finding:** `strlenextern`'s C impl was silently leaking one strong
RC per call (str is MUTABLE_SHARE; my always-OWN alias-before-send made the
missing dealias observable). Fixed by adding `vtest_str_dealias(
haystackContainerStr)` before return. All 112 end-to-end tests now clean
under ASan+LSan.

**Not audited under ASan — intentionally left alone.** The 22 imm-type
extern-param fixtures still have BORROW-era comments and no C-side
`_dealias`. Under the always-OWN ABI they *should* dealias, but
`IMMUTABLE_SHARE` ref-counting is asymmetric (Vale-side alias/dealias are
no-ops; only C-side auto-gen'd Foo_dealias does actual RC), and LSan
didn't flag them.

**Why we're not fixing this:** the whole `IMMUTABLE_SHARE` / flat-
`OwnershipT` model is being retired by the **onion-typing arc**
(experimental-2's `Vale2/vcoord-handoff.md`). Under onion typing, share-
flavored-ness becomes intrinsic to the citizen (`share_flavored: bool` on
`StructTT`/`InterfaceTT`), share refs become an explicit `Kind::ShareRef`
layer, and the ShareRef semantic is "RC'd handle" — no more asymmetric
IMMUTABLE_SHARE vs MUTABLE_SHARE split. The Backend representation
rewrites when the Backend arc lands after the frontend arc. Any comment
sweep or fake-dealias insertion done now against the current model would
be re-swept when share unifies, so it's throwaway work.

The always-OWN extern-param ABI we landed (alias before send) *does*
match the future direction — under onion, "Vale passes a share ref to
C" is ownership transfer of a +1 handle, and C is responsible for the
ref. So the Backend edit stays right; only the imm-type fixture comments
are stale-but-irrelevant.

## 3. Cross-cutting: how to debug a failing Vale test

Documenting this here because it took me a lot of thrashing to work out and
the pattern is useful for future arcs.

### 3a. Standard workflow

```
cargo nextest run <test_name>
```

If the test fails, its compiled binary and generated Vale/C output live at:

```
tmp/vale-test-runs/<test_name_underscored>/out/
```

The directory is wiped at the start of every test run for that same test, so
you always see just the most recent state. To debug:

```
cd tmp/vale-test-runs/end_to_end_tests-tests-replay-<name>/out
./a.out                                  # run
./a.out --vale_record recording.bin      # record
./a.out --vale_replay recording.bin      # replay
lldb -o "run --vale_replay recording.bin" -o "bt 25" -o "quit" ./a.out
```

Wired up via `NEXTEST_TEST_NAME` env var in
`src/end_to_end_tests/mod.rs`. Under plain `cargo test` (without
nextest) the compiled binary still goes into a `tempfile::TempDir` that
disappears on drop — nextest is the recommended runner.

### 3b. Debug env vars in `mod.rs`

```
VALE_FLARES=1  cargo nextest run <test>   # emit --flares to backend
                                          # (LLVM IR gets flare printouts
                                          # at every expression eval)
VALE_LLVM_IR=1 cargo nextest run <test>   # emit --llvmir to backend
                                          # (dumps opt.ll / raw.ll to
                                          # <out_dir>/build/)
```

Wired at `src/end_to_end_tests/mod.rs:213-218`.

### 3c. Pre-opt LLVM verify

When a test fails with SIGSEGV inside "Running release optimizations..."
(vale.cpp:1434), the Backend is emitting invalid IR that trips LLVM's
optimizer rather than the verifier. Temporary diagnostic:

```cpp
// In Backend/src/vale.cpp, right before the optimize() calls:
char *error = NULL;
LLVMVerifyModule(globalState->mod, LLVMReturnStatusAction, &error);
if (error && *error) {
  std::cerr << "PRE-OPT VERIFY FAILED:\n" << error << std::endl;
  LLVMDisposeMessage(error);
  exit(2);
}
```

This surfaces the actual IR error (e.g. "AddrSpaceCast result must be a
pointer" for a bad `LLVMBuildPointerCast` to a struct type) before the
optimizer crashes on it. Remove after debugging.

### 3d. Cargo build tracks Backend sources

`build.rs` watches the entire `Backend/src/` tree recursively
via `watch_dir_recursive`. Any C++ edit will re-run cmake next time
`cargo build` / `cargo nextest run` runs — no manual `touch build.rs`
needed. Wired in slice-5 debugging.

---

## 4. Nice-to-have future work

### 4a. Auto-gen'd upcast doesn't cover super-interfaces

Slice 4d's upcast emitter (`Backend/src/region/rcimm/rcimm.cpp:
declareConcreteUpcastFunction`) generates one `Bar_asIFoo` per direct impl
edge. If `IFoo` has a super-interface `IBar` and struct `Bar` transitively
implements `IBar`, we don't emit `Bar_asIBar` — because there's no direct
edge in the AST.

No test currently exercises this; may or may not be an issue in practice.
Worth tracking if super-interfaces become common.

### 4b. `Frontend/` Scala mirror maintenance

Slice 5b renamed string operators in both `src/builtins/
resources/*.vale` AND `Frontend/Builtins/src/dev/vale/resources/*.vale`. The
Scala tree is still built somewhere (or was) but no active tests use it
directly. Long-term this duplication is a footgun — every FFI-facing change
has to happen in both trees. Worth deciding whether Scala is fully
deprecated (then delete `Frontend/Builtins/`) or actively maintained (then
formalize which files must stay in sync).

### 4c. Test framework `LEAK` classification

Nextest reported one leaky test in the final run
(`end_to_end_tests::tests::arrays::ssamutfromcallable`). Test passed but
nextest thinks a child process didn't clean up. Pre-existing (not caused by
this arc). Worth investigating as general test hygiene.

### 4d. Universal ref bit layout is fragile — RESOLVED

Historically `Backend/src/region/urefstructlt.cpp` bit-packed a 32-byte
universal ref into a compressed 256-bit integer (56 bits for obj ptr, 52 for
type-info ptr, etc.), only safe as long as no real pointer exceeded those
widths.

Resolved on `pre-squash-ffi-drop`: FFI handles are now **right-sized
structs** with plain i64 pointer-bit fields — no compression, no bit-width
invariant. See `Backend/src/region/ffihandlestructs.{h,cpp}`:
- concrete kinds (struct/str/RSA/SSA) cross as `{ i64 obj }` — 8 bytes
- interfaces cross as `{ i64 obj, i64 typeinfo }` — 16 bytes

All concretes share one LLVM handle type and all interfaces share one; per-class
distinctness lives only in the generated C typedefs — see @HTSLVBDTCZ
(`Backend/docs/arcana/HandleTypesIsSameLLVMValueButDifferentTypedefsInC-HTSLVBDTCZ.md`).

The `compressI64PtrToI5x` / `buildCompressStruct` helpers and the
`int256`-hasher/`simplehash` fossils they fed were deleted with this change.
Scrambling of the pointer bits remains deferred (plain pointer bits today).

### 4e. Attach implementing edges to `InterfaceDefinition` in the AST

The auto-export machinery needs, per interface, its implementing edges in a
stable order — the typeTag function body and `generateInterfaceDefsC`'s
`TAG_*` constants must agree on which substruct gets which tag. The AST only
provides edges the *forward* way (`StructDefinition.edges`, and each `Edge`
knows both endpoints); `InterfaceDefinition` carries no implementing-edge
list. So `RCImm` builds the inverse index itself (`edgesByInterface`,
populated in declaration order during `declareEdge`).

If the frontend attached the implementing edges to `InterfaceDefinition` (in a
deterministic order both emitters can rely on), the backend could read
`interfaceDef->edges` directly and drop `edgesByInterface` entirely. This is a
FrontendRust + metal-cache-FFI change, not a backend one. Low priority — the
current index is correct and small; this is purely about removing a
backend-side inversion the AST could provide.

---

## 5. Where to start when picking up this work

If you had 4 hours to pick one thing:
1. **1a (interface-return replay)** — most concrete, most impactful, has
   a clear repro and a specific hypothesis to test. Getting these two tests
   green would close a known gap in determinism coverage.

If you had 8 hours:
1. 1a as above
2. **1b (getMainArg intrinsic)** — small, mechanical, unblocks the
   `cellularautomata.vale` demo program

If you had a week:
1. 1a
2. 1d (OWN-arg semantics) — real design work, but small implementation
3. 1c (str_from_bytes) — real design decision about raw pointers in Vale

Cosmetic cleanups (§2) are best rolled into whichever slice happens to
touch the affected files, not done as a standalone pass.
