# Claude Conversation Log

Session ID: d9c281b0-0029-46b7-84b4-abc6afaa43f8
Date: 2026-07-25 22:53:55

---

## 👤 User

<command-message>full-read</command-message>
<command-name>/full-read</command-name>
<command-args>/tmp/claude-conversation-2026-07-25-43fdb10f.md
then please also read /Volumes/V/Vale4/docs/convos/rust_interop/synthesized-declarations-plan.md.
then, please look in ~/.claude to find the agent output for that one remaining agent that finished after that session ended</command-args>

---

## 🤖 Claude

I'll start with the conversation log.

---

## 🤖 Claude

Conversation log complete (4,642 lines). Now the plan doc.

---

## 🤖 Claude

Both files read. Now finding the agent output.

---

## 🤖 Claude

All three done.

## 1. The conversation log — 4,642 lines

Session 7 of the Rust-interop arc. The through-line: it opened with ~12 unfinished threads from session 6, narrowed to "what blocks calling one Rust function that adds two ints," made seven gated decisions, built Milestone 2 for real, and then **discovered the whole design was wrong** and pivoted.

**Decisions made** (each one asked, answered, recorded): skip the C++ backend link until the LLVM 16→~21 port (temporary scaffold, not the backend becoming optional); stay single-crate, split when `per_instance_mir`/`layout_of` land; driver is a bin carrying its own assertions with a thin test spawning it; panic (naming the gap) on unrepresentable Rust types; explicit allowlist now, import-driven later; replace `--cfg rust_interop` with a cargo feature; fixture crosses a crate boundary via `--extern` from the start.

**What got built and committed** (`699241ffb`, 41 files): cargo-feature migration, `build.rs` backend gate (verified firing — 1 line of script output vs 199–276), `rustc-dev` installed but deliberately *not* pinned, the driver host under `rust_interop/driver/`, `TyCtxtOracle`, the logging oracle, the `Oracles` struct that de-cfg'd `compiler.rs`/`compilation.rs`, the type importer, and `(make_counter()).get()` typechecking against a live `TyCtxt`.

**The three moments that mattered most:**
- The `Vec::new()` guard question → traced to "absence and elsewhere are the same value" → produced the import-materialization design → showed the guard was speculative dead code → deleted.
- The oracle log caught a **vacuous pass on its first run** (`packages_to_build: vec![]` compiles nothing and returns `Ok`).
- Your skepticism about registering `ITemplataT::Prototype` at env-build time was correct. Two read-only surveys confirmed it's a known dead end — `ExternFunctionTemplataT` exists in a sibling repo with zero producers and `vfail()`, and `RustInteropReiImpl` is **a branch of this repo** whose machinery (`ExternBody`, `make_extern_function`, `ExternFunctionCallTE`, opaque lowering) is already in our tree, unused.

## 2. The plan doc — 254 lines

Clean write-up of the pivot: why the shipped design violates `@ECSIIOSZ`/`@BDPFWDZ`, why both siblings converge on `extern`-as-body-kind, why *not* to generate `.vale` text (their reason was IPC with a separate process; ours is in-process), the verified shapes, seven hard constraints, the `@SMLRZ` trap ("they had to climb toward the mistake; we'd fall into it"), and what the pivot deletes.

## 3. The orphaned agent report — found, and it revises the plan doc

`~/.claude/projects/-Volumes-V-Vale4/43fdb10f-….jsonl:3219`. Task `a2fd89df0cb7d60a2`, "Study RustInteropReiImpl design," completed 22:52:26. The session hit **"Prompt is too long"** on the very next turn, so it was never read. Its content is not in the plan doc.

**It corrects a claim the plan rests on.** `FunctionTemplataT` was *not* migrated to `#[derive]` with the other three definition templatas — it still hand-rolls `(range, name)` equality (`templata.rs:352-371`). Overload candidates dedup through a `HashSet` on exactly that (`overload_resolver.rs:1156-1158`), and `FunctionNameS` is `(human_name, code_location)`. So two synthesized extern functions sharing a sentinel location **collapse to one candidate, silently** — and `Vec::new` / `String::new` / `Box::new` all have human name `new`. Plan §5 constraint 2 becomes the single highest-risk item, and the fix is a monotonically-decreasing counter, not a shared constant. (`struct_drop_macro`'s `-1672147` constants are safe only because they land on *rules*, which are never identity keys; that file uses the real `struct_a.range.begin` for identity.)

**It also revises §4's level.** `higher_typing_pass.rs:74-78` caches by `range.begin` and panics on cycles — two synthesized `StructS` sharing a `begin` silently yields the first's `StructA`. That applies only if you synthesize `StructS`, so the recommendation moves from `FunctionS`/`StructS` to **`FunctionA`/`StructA`**, which skips the cache entirely.

Plus: `is_internal()` is a real escape hatch (the solver humanizer's `range.begin.offset - loc.offset` underflows to ~4 billion without it) but `CodeLocationS::internal` hardcodes package coord `""`, conflicting with the `templata.rs:472`/`:390` invariant — mint your own `FileCoordinate` instead; impls dedup by range; nothing has `Ord` on locations, so determinism rides on your oracle emitting in a stable order; and the standing caveat that **"synthesized `FunctionA` + `IBodyS::ExternBody`" has no in-tree precedent** (all four macros use `GeneratedBody`) — the first thing worth a test.

One operational note: **Bash is blocked in this session.** The Guardian PreToolUse hook gets an empty `${GUARDIAN_PORT}` and exits 2, so every Bash call fails before running. I found the transcript with the Read tool instead. That'll block any build/test work until the port is set — `/guardian-ordain` won't help since it needs Bash too.

Want me to fold the agent's corrections into `synthesized-declarations-plan.md`?

---

## 👤 User

just did export GUARDIAN_PORT.

so, where does that leave us? would we synthesize FunctionS, or what are we doing?

---

## 🤖 Claude

Good. Let me verify Bash works and check the crux — because the plan doc and the late agent report disagree about the level, and the answer turns on facts I should read rather than infer.

---

## 🤖 Claude

Decisive finding. Let me confirm the parts that *do* apply to us.

---

## 🤖 Claude

Bash works. I checked the crux, and the answer is cleaner than the plan doc left it.

## We synthesize `FunctionS` / `StructS`. There is no other level here.

The agent's headline — "go up to `FunctionA`/`StructA`" — **doesn't apply to our tree.** It was reading `/Volumes/V/RustInteropReiImpl`, and reported that tree's line numbers as if they were ours:

| | ours (`experimental-4`) | ReiImpl branch |
|---|---|---|
| `higher_typing/` | **retired outright** (`lib.rs:26`) | present |
| `FunctionA` | doesn't exist | exists |
| `FunctionTemplataT.function` | `&'s FunctionS` (`templata.rs:158-160`) | `&'s FunctionA` (their `:352-371`) |
| `Astrouts` / `code_location_to_struct` cache | doesn't exist | `higher_typing_pass.rs:74-78` |

`IEnvEntryT::Function(&'s FunctionS<'s>)` (`i_env_entry.rs:15`) is what env entries hold, and `entry_to_templata` feeds it straight into `FunctionTemplataT` (`environment.rs:408-413`). `FunctionS` is the level. Plan §4 stands as written.

And the agent's *strongest* argument for the A level — the `range.begin`-keyed struct cache that silently returns the wrong `StructA` — **is a hazard we don't have**, because the pass that owned it is gone.

## But its correction survives translation, and it's worse for us than it reported

Verified in our tree, all four links in the chain:

- `FunctionTemplataT` eq/hash on `(function.range, function.name)` — `templata.rs:162-176`
- `StructDefinitionTemplataT` eq/hash on `(origin_struct.range, origin_struct.name)` — `templata.rs:202-214`. **The agent said this one was migrated to `#[derive]`/pointer identity. In our tree it wasn't** — so the hazard covers synthesized structs too, not just functions.
- `FunctionNameS { name, code_location }` — `postparsing/names.rs:497`
- candidate dedup `undeduped_candidates.into_iter().filter(|c| seen.insert(*c))` — `overload_resolver.rs:576`, and again at `:191`

So two synthesized denizens sharing a sentinel range **collapse to one candidate, silently, no error** — and `Vec::new` / `String::new` / `Box::new` all have human name `new`. Plan §5 constraint 2 goes from seventh in a list to **the top risk**, and it's now a struct-and-function rule.

## What that makes the recipe

Copy `struct_drop_macro.rs:60-107` exactly — it already draws the line in the right place. It uses the **real** `struct_a.range.begin` for the identity-bearing `FunctionNameS.code_location`, and sentinels (`-64002`, `-1340`) only on rules and params, which are never identity keys.

Our version, per synthesized item:
- `CodeLocationS { file, offset }` with a **distinct negative offset derived deterministically from the rustc `DefId`** (not a counter, so it's stable across runs per @IIIOZ) — this is the whole fix
- `generic_params` from the oracle; a method reuses the struct's via `let function_generic_parameters = struct_a.generic_params;`
- two rules per generic mention (`LookupSR` + `CallSR`), not three — we have no `CoerceToCoordSR`
- `maybe_ret_kind_rune: Some(..)` — externs can't infer returns
- `IBodyS::ExternBody(ExternBodyS {})` instead of `GeneratedBody`

Note `StructTemplateNameT { human_name }` carries **no** code_location (`names.rs:1408`) while `FunctionTemplateNameT` does (`:1304`) — the deliberate "structs don't overload" asymmetry. So unique offsets disambiguate function *templatas*, but two same-named Rust structs still collide at the name level. That's the deferred collision question, and it stays deferred as long as types arrive from signatures.

## The one genuinely untested thing

**`FunctionS` + `IBodyS::ExternBody` has no in-tree precedent** — all four macros use `GeneratedBody`, and every `ExternBody` today comes from parsed source. That's the first thing to test, before building anything on top of it.

Still open and unchanged: the package-coordinate call (plan §5 constraint 6). `CodeLocationS::internal` hardcodes package coord `""` and file `"internal"` (`range.rs:29-37`), which buys graceful humanizer degradation but conflicts with the `declaring_env.package_coord == origin.range.file.package_coordinate` invariant. The alternative is minting our own `FileCoordinate` with the real `rust.mycrate` coord, a synthetic filename, and negative offsets.

Want me to fold these corrections into `synthesized-declarations-plan.md` — flagging §4/§5 as revised and marking which agent findings were tree-mismatched — or go straight at the `ExternBody` smoke test?

---

## 🤖 Claude

Watcher resident as **Vale4-opal**. One message was queued — a reply from Vale2-rowan to a planning message an earlier Vale4 session sent. Four items bear directly on what we just decided.

**`.` is not pure sugar for `foo(x)`.** They've refuted that: the dot performs a **receiver adjustment** (autoref and deref). The namespace half stands — dot doesn't change which functions are findable — but if our method story assumed pure sugar, it's wrong. That's exactly the seam our `(make_counter()).get()` workaround sits on.

**And the cluster blocking the borrow form is owned but not started.** `convert()` / `is_type_convertible` unification belongs to their "overload resolution & dispatch model redesign" mission, no date. So `substitute_templatas_in_kind`'s ref-wrap arms and the `BorrowRef(T) -> T` read-out stay broken for a while — our by-value-`self` fixture is a longer-lived workaround than "next slice." They also report **share upcasts don't work at all** today (`convert_via_upcast` does `ISubKindTT::try_from` on a wrapped kind, so `@Dog → @Animal` reports `CouldntConvertT`), which lands on the same argument-crossing surface.

**`ITemplataT::Region` is theirs, and our likely mapping of it is semantically wrong.** It's rung 0 of the region borrow checker, independently roadmapped — so the fork resolves to "wait for it, build on it." But the caveat matters more: a group is an **algebra** (paths, unions, territory closures, laws), and its semantics are **identity, not extent** — groups name pointee *sets*, deliberately not validity durations. A Rust lifetime is the other thing. So mapping Rust lifetime slots onto `Region` is not a one-variant change and may be the wrong shape entirely. Worth pinning before any representation commitment.

**Our own interop arch doc has a vocabulary collision that hands the wrong answer.** It uses Vale-era "interface" meaning what the spec now calls `trait`/`open trait`; the *current* class-tier `interface` is a different, RC-erased kind that gets **no Rust projection at all** — Rust holds an opaque handle and calls through it, never instantiating one. That's stated as the specification, not a gap. If any of our mapping assumes an interface becomes a Rust `dyn`, that's first to revisit — and it touches the importer's sealedness declaration for imported Rust traits.

Smaller, but useful: `is_primitive` is worse than we filed — **six** of sixteen variants disagree, and the fix is *renaming, not reconciling*. `compiler.rs:1683`'s `Str => true` is load-bearing (it means "needs no user export declaration"; flipping it breaks `stdlib/src/path/path.vale:4`), and filling the four ref-wrap arms with `false` isn't sufficient without `peel_all_references` before both the `is_primitive` call and the exports-map lookup. Our `ensure_deep_exports` finding is still unclaimed by anyone. And `reachability.rs` is optional for them — they'll take our spec or let us write it.

Their calibration note is itself one commit stale: they have us at `af3a3c17a`/`f9b2ab1de`; we're at `699241ffb`. Suite numbers unchanged either way.

None of this changes the answer I gave you — we synthesize `FunctionS`, the unique-`CodeLocationS`-per-denizen rule is the top risk, and `ExternBody` is the untested combination. It does confirm the borrow-form method gap isn't clearing soon, and it flags that the `Region` question is a bigger design commitment than the plan doc implies.

---

## 👤 User

would we do FunctionS + IBodyS::ExternBody, or would we have a generatedbody that uses an externcall node? IIRC the normal C externs do the latter

---

## 🤖 Claude

Good question — let me check what the C extern path actually does rather than trust the notes.

---

## 🤖 Claude

Checked it end to end. **It's not a fork — `ExternBody` *is* the "wrapper with an externcall node" path.** Your recollection of the shape is right; the dispatch you're remembering it through isn't.

`IBodyS::ExternBody` dispatches to `make_extern_function` (`function_compiler_core.rs:149-160`), and that function builds exactly what you described:

- a `FunctionHeaderT` for the Vale-facing wrapper
- an `ExternFunctionNameT`-named `extern_prototype`
- **empty instantiation bounds, registered right there** (`:352-362`) — at instantiation time, which is the timing the plan doc argues for
- one `ArgLookupTE` per param, body = `Return(ExternFunctionCall(extern_prototype, arg_lookups))` (`:369-380`)
- `add_function` + `add_function_extern`

And there is exactly one producer of each, with no generator alternative:

| | producers |
|---|---|
| `ExternFunctionCallTE::new` | **1** — `function_compiler_core.rs:376` |
| `IBodyS::ExternBody` | **1** — `postparsing/function_scout.rs:657` (postparser, on a source `extern func`) |

No body macro in `name_to_function_body_macro` emits an extern call. So "GeneratedBody + externcall node" isn't a thing that exists in this tree — picking it would mean writing a new generator that duplicates `make_extern_function`. **`FunctionS` + `IBodyS::ExternBody`.**

That also shrinks the risk I flagged last turn. What lacks precedent is *synthesizing* a `FunctionS` that carries `ExternBody` (macros synthesize with `GeneratedBody`; the parser produces `ExternBody`). But `make_extern_function` reads nothing parser-specific — it works off the solved `env.id` (`human_name`, `template_args`, `parameters`), `params2`, `ret_coord`, and `full_env.function.{range, attributes}`. Nothing requires the declaration to have come from source text.

Three things fell out that sharpen the plan:

**`ExternTemplateNameT { code_loc }` — code_loc is its only field** (`names.rs:1236-1238`), interned, `#[derive(Eq, Hash)]`, built from `range.begin` at `:387`. So the synthetic range's `begin` is a **third** identity keyed on it, alongside `FunctionNameS.code_location` and `FunctionTemplataT`'s eq/hash. Unique-offset-per-denizen isn't just the top risk now, it's the linchpin of three separate identities.

**The @PRIIROZ ordering disagreement is resolved — by our own tree.** Plan §5 constraint 4 flagged that the two surveys contradicted each other (prepend vs. inherited-last). The comment at `:398-402` settles it: internal-method externs inherit the container's generic params **at the end** of their `template_args`. Better, `GenericParametersInheritance { num_inherited_generic_parameters }` is computed from citizen arity, handed to `add_function_extern`, and used by Hammer to reshape the wire-format SimpleId so inherited args land on the citizen step — *"`Vec<i32>::capacity` rather than `Vec::capacity<i32>` … which is what Backend's `rustifySimpleId` expects per @SMLRZ."* **The boundary projector the plan doc treats as future work already exists here.**

**And a concrete first blocker on the method path.** `maybe_inheritance` is derived from `ICitizenTemplateNameT::try_from(...)` on the prototype's init step, and when that matches it calls `lookup_citizen_by_template_name` → `lookup_struct_template` → `.expect("Struct template not found")` (`compiler_outputs.rs:564-565`). Option A's premise is that we never build a `StructDefinitionT` for a Rust struct — so **`Counter::get` will panic there**, while `add_two_numbers` is fine (a top-level function's init step yields `Err`, so nothing is looked up). That's a real core question to settle rather than discover: does a Rust-backed citizen get a `StructDefinitionT` after all, or does `make_extern_function` need to tolerate its absence?

Which suggests the smoke test order: **synthesize `add_two_numbers` first** — free function, no citizen step, exercises `FunctionS` + `ExternBody` + the solver + `make_extern_function` end to end with nothing to look up. Then `Counter::get`, where that panic is the expected next finding.

Want me to fold all of this into the plan doc — §4 confirmed, §5 constraint 4 resolved, the `ExternTemplateNameT` identity added, the ReiImpl tree-mismatch marked, and the `lookup_struct_template` blocker recorded — before writing any code?

---

## 👤 User

ive been kind of out of it. tell me what the open issues/risks are with the current direction w.r.t. synthesizing FunctionS? lots of CodeLocationS/RangeS stuff i gather? what else?

---

## 🤖 Claude

Let me verify the one I'd rank highest before summarizing, because I've been repeating it from the last session rather than checking it.

---

## 🤖 Claude

Here's the honest picture. The direction itself is confirmed sound — synthesize a `FunctionS` with `IBodyS::ExternBody`, inject it as an ordinary `IEnvEntryT::Function`, let the normal solver and `make_extern_function` do the rest. The risks are all in the details of *how*.

## 1. Identity and location — yes, and it's four consumers, not one

You're right that this is the bulk of it. Four separate things key on the synthetic range/location:

- `FunctionTemplataT` eq/hash on `(range, name)` → feeds candidate dedup at `overload_resolver.rs:576` and `:191`
- `StructDefinitionTemplataT` eq/hash on `(range, name)` — `templata.rs:202-214`
- `FunctionNameS.code_location` → `FunctionTemplateNameT.code_location`
- `ExternTemplateNameT` — `code_loc` is its **only** field (`names.rs:1236`)

One mechanism covers all four: a distinct negative offset per synthesized denizen, derived deterministically from the rustc `DefId`. Mechanically simple. It ranks first anyway because **the first failure mode is silent** — two synthesized externs sharing a location collapse to one overload candidate with no error, and `Vec::new`/`String::new`/`Box::new` are exactly that shape.

Open sub-decision inside it: the package coordinate. `CodeLocationS::internal` hardcodes coord `""` and file `"internal"` (`range.rs:29-37`), which buys free humanizer short-circuiting but conflicts with the package-coord-agreement invariant. Not decided.

## 2. Name resolution — the biggest genuinely open one

A `FunctionS` carries **runes and rules**, not types. And of the twelve rule variants, `LookupSR { rune, name: IImpreciseNameS }` resolves **by name**, while `LiteralSR` carries only int/string/bool. There is no rule that carries a pre-resolved templata.

So the naive encoding of `fn make_counter() -> Counter` turns every Rust type mention into a Vale name lookup — which reopens `panic!("Too many with name")` and the ignored `_get_only_nearest`, the collision question the oracle approach let us defer indefinitely.

**But there's an escape hatch, and it's a real in-tree protocol.** `RuneParentEnvLookupSR` (@MKRFA) carries only a rune, and the strip resolves it via `lookup_nearest_with_imprecise_name(RuneName(rune), TemplataLookupContext)` — a *rune* name, synthetic and unique, never a user-visible type name (`overload_resolver.rs:361-370`). Seed the Rust type as a rune-bound templata in the env and human-name lookup never happens.

Two caveats I won't paper over: @MKRFA is documented as leaky — not type-enforced, violations produce silent "couldn't solve" errors rather than faulting (`infer_compiler.rs:195-204`, with queued fix work). And I've verified the strip on the **call-site** path and in `array_compiler`; I have **not** verified the function-*definition* rule path strips them. That one check decides the whole encoding.

## 3. The struct side is much less settled than the function side

Verified today: `make_extern_function` → `lookup_citizen_by_template_name` → `lookup_struct_template` → `.expect("Struct template not found")` (`compiler_outputs.rs:564`). Rust **methods** hit that; free functions don't. It forces a real decision — does a Rust citizen get a `StructDefinitionT` after all (contradicting Option A's premise), or does that path learn to tolerate absence?

Still inherited-and-unverified from the surveys: suppressing the struct-constructor macro (the two surveys disagreed about which default macro set applies), and whether an extern `drop` coexists with the macro-generated one or collides — stakes there are leak-vs-crash, and nobody has determined it.

And `StructTemplateNameT` has **no** code_location (`names.rs:1408`), so two same-named Rust structs collide at the name level *regardless* of unique ranges. The collision question resurfaces specifically on the struct side.

## 4. Injection point — a known unknown

Synthesized declarations have to exist before the indexing phase walks the top-level environments. Both current hooks run later than that. Not investigated at all.

## 5. Not ours to fix

Borrow-form methods stay blocked on `substitute_templatas_in_kind`'s ref-wrap arms and `is_type_convertible`'s `BorrowRef(T) -> T`. Vale2 confirmed yesterday that cluster is owned but **not started, no date** — so by-value `self` plus parenthesized call is a long-lived workaround, not a short one. They also flagged that `.` performs receiver adjustment (autoref/deref) rather than being pure sugar for `foo(x)`, which may bite the method story independently.

## What got better, so this isn't all downside

`ExternBody` already builds the wrapper, the externcall node, and registers empty bounds at instantiation time — none of that is ours to write. @PRIIROZ's ordering ambiguity is resolved in-tree (inherited args at the **end**), and the @SMLRZ re-split projector the plan doc treats as future work **already exists** at the Hammer boundary. `higher_typing` being retired means the `range.begin` cache hazard that dominated the late agent's report doesn't exist here. And six of seven `FunctionS::new` sites are already typing-pass synthesis, with `struct_drop_macro` as a near-exact template.

## What I'd do next

The MKRFA definition-path check (risk 2) is one grep and it determines the encoding, so it should come before anything is written. Then smoke-test **`add_two_numbers` only** — free function, no citizen init step, so it sidesteps risk 3 entirely and exercises `FunctionS` + `ExternBody` + solver + `make_extern_function` end to end.

Want me to run that check and update the plan doc with all of the above?

---

## 👤 User

1. "two synthesized externs sharing a location collapse to one overload candidate" how does that happen?
2. why+when would we need a pre-resolved templata?
3. one of those other trees had a type named OpaqueHT, and presumably an OpaqueTT. grep their trees to see which one, and then ask them more about this.
4. indeed... and presumably we'd want the TyCtx for reading what the rust shapes are, so we can recreate them in FunctionS form.
and yeah, do the MKRFA investigation.
dont update the plan doc yet.

---

## 🤖 Claude

Taking these in order. First the greps for 1 and 3, plus the MKRFA start.

---

## 🤖 Claude

Now the MKRFA investigation — where function *definition* rules get solved.

---

## 🤖 Claude

Dispatching the Opaque investigation while I finish MKRFA — both other trees have it, so it's one sweep.

---

## 🤖 Claude

Now finishing MKRFA — the definition/call-site rule split is the last piece.

---

## 🤖 Claude

Answers in order. The MKRFA result changed my read of #2, so those two connect.

## 1. How two synthesized externs collapse into one candidate

The chain is four links, and the last one is hand-rolled:

```
get_candidate_banners → undeduped_candidates: Vec<ICalleeCandidate>   (overload_resolver.rs:570-573)
seen.insert(*c)                                                        (:574-576)
ICalleeCandidate::Function(FunctionCalleeCandidate { ft })             (ast/ast.rs:194-204, #[derive(Eq, Hash)])
FunctionTemplataT eq/hash = (function.range, function.name)            (templata.rs:162-176)
```

So a candidate's identity is `(range, human_name, code_location)` — and note `FunctionTemplataT` **ignores `outer_env` entirely**, so being in different environments doesn't save you.

Worked example. Synthesize `Vec::new` and `String::new` as two `FunctionS`, both with human name `new`, and give both `RangeS::internal(-9001)` — a shared sentinel, which is exactly what `struct_drop_macro` does for *its* rules. Both get found by imprecise name `new`, both get pushed as candidates, then `seen.insert` returns false for the second and **it is silently discarded**. Overload resolution then proceeds with one candidate: either you get "couldn't find function to call" for a function that plainly exists, or — worse — the survivor happens to match the args and you call the wrong one. No error, no assert, in either case.

What makes `struct_drop_macro` safe is an accident of construction: it uses the **real** `struct_a.range` and `struct_a.range.begin` for the generated function's identity, and sentinels only for rules and params. Every struct has a distinct real range, so every generated drop is distinct for free. We have no real range to borrow, which is precisely why we have to manufacture distinctness deliberately.

## 2. Why and when we'd need a pre-resolved templata — and the MKRFA answer, together

**When:** immediately, and everywhere. A `FunctionS` doesn't carry types, it carries runes plus rules. Every Rust type mention in a Rust signature — the return of `make_counter() -> Counter`, the receiver of `get(self Counter) int`, every param — has to become a rule that names that type somehow. So it's not a later concern; it's the first fixture.

**Why it's a question at all:** the only rule that names a type is `LookupSR { rune, name: IImpreciseNameS }`, which resolves **by name**. `LiteralSR` carries int/string/bool only. So the naive encoding hands the Rust type back to Vale's name resolution — and we already *had* the answer from the oracle as a `DefId`. That's throwing away an exact identity and hoping to recover it by string, which (a) reopens `panic!("Too many with name")` at `environment.rs:164` with no shadowing rule to fall back on, and (b) gives the same fact two sources of truth.

**The MKRFA verdict: `RuneParentEnvLookupSR` is not the escape hatch.** Three independent reasons:

- It's stripped on exactly three paths — call-site overload attempt (`overload_resolver.rs:361`), array rules (`array_compiler` ×3), patterns (`pattern_compiler.rs:73`). The **function-definition** solve (`function_compiler_solving_layer.rs:710`, and `:556`) does not strip it, and the SROACSD filters don't remove it either — they only drop `DefinitionFunc` from call-site solves and `CallSiteFunc`/`Resolve` from definition solves (`infer_compiler.rs:908-926`).
- If one reaches the solver it now **panics loudly**: `panic!("vwat: RuneParentEnvLookupSR should have been MKRFA-preprocessed before reaching the solver")` (`compiler_solver.rs:1045-1049`). Good news incidentally — the queued fix from `mkrfa-protocol-leak.md` has landed, so the "silent couldn't-solve" hazard the doc comment warns about is gone.
- And even where it *is* stripped, it resolves against `calling_env` by **rune** name. A Vale program calling `make_counter()` has no `Counter` rune bound in its environment, so it wouldn't resolve anyway.

Definition solves also pass `&[]` for `initial_knowns` unconditionally (`:711`), so there's no seam there either.

**But there's a third option I didn't have before.** `IImpreciseNameValS` has fourteen-plus variants, and most are *synthetic names a user cannot type* — `LambdaStructImpreciseName`, `AnonymousSubstructTemplateImpreciseName`, `PlaceholderImpreciseName`, `RuneName` (`postparsing/names.rs:223-238`). So we can keep an ordinary `LookupSR`, going through ordinary name resolution, but in a namespace source code can't reach. That's how lambdas and anonymous substructs already avoid colliding with user names.

Cost is one new enum arm plus its interner and humanizer arms — a small core change, in exactly the shape the enum already has. Benefit is that the collision decision stays deferred and Rust type identity stays anchored to the `DefId`.

The tension worth naming: plan §6's self-check says a synthesized declaration should be *structurally indistinguishable* from what the postparser emits for a hand-written `extern struct Counter`. A synthetic imprecise name is a visible deviation from that. My read is that it's the right deviation — it's a *namespacing* choice, not a Rust-shape leak, and it's precedented — but it is a deviation and you should know I'm making it.

## 3. Opaque

Grepped all three. **All three have `OpaqueHT`; none has an `OpaqueTT` or any typing-level opaque concept.** Ours at `final_ast/types.rs:141`, ReiImpl's Rust side at its `final_ast/types.rs:242`, ValeRustInterop's in Scala `FinalAST/…/types.scala`. Opacity is a simplifying/H-level concept everywhere — the typing pass keeps an ordinary struct, and `hamuts.rs:48` keys the map on `StructIT`, i.e. the *instantiated* struct.

One detail worth chasing: ReiImpl carries a commented-out `override def isRustOpaqueType(): Boolean = id.packageCoordinate.module.str == "rust"` — package-coordinate-driven, the same trick as our `is_rust_backed`. I've dispatched a read-only agent across all three trees to answer what `OpaqueHT` holds, what decides a struct becomes one, what an extern struct's members and size/align look like, and — the crux — **whether a Rust-backed struct gets a real `StructDefinitionT` in their tree**, since that's what determines whether `lookup_struct_template`'s `.expect` is a real blocker or a non-issue. I'll report when it lands.

## 4. On the oracle's role

Agreed, and that's the shape the pivot leaves it in: the oracle stops being a query service consulted at every call site and becomes a **binding generator** — read `TyCtxt` once for the shapes, emit declarations, and let the ordinary machinery run. `fn_sig(item, args, ..)` survives and finally receives real args, from `make_extern_function`, per instantiation. Which is the one thing the old design structurally couldn't do.

The injection point is still the open piece there — declarations have to exist before the indexing phase walks the top-level environments, and both current hooks run later. I haven't investigated that yet.

Plan doc untouched, as you said.

---

## 👤 User

eli5 #2? im not sure i understand the question

---

## 🤖 Claude

Sure. The whole thing hinges on one surprising fact about how Vale stores a function declaration.

**A `FunctionS` doesn't contain types.** You'd expect `fn make_counter() -> Counter` to be stored as "returns: Counter." It isn't. It's stored as a set of **blanks** (Vale calls them runes) plus **instructions for filling in the blanks** (rules). So it's really:

> return type = blank #1
> instruction: blank #1 = go look up the name `Counter`

And the only instruction that can name a type is "go look it up **by name**." There's no instruction that says "blank #1 is *this exact thing*." ("Templata" is just Vale's word for a resolved compile-time thing — a type, a number, a function. So "pre-resolved templata" means "the actual type, already found, in hand.")

**Here's why that's awkward for us.** rustc already told us exactly which type this is — it handed us a precise ID. We're holding the right answer. But to write it into a `FunctionS`, we have to throw that away and write down the *string* `Counter`, so Vale can go find it again later.

Two things go wrong when it goes looking:

- If the Vale program also has something called `Counter`, the lookup finds two and hard-panics — `"Too many with name"`. There's no "prefer the imported one" rule; that's the collision question we've been deferring all along.
- Even when it works, the same fact now has two sources of truth: rustc's ID, and whatever Vale's namespace happens to resolve. They can disagree.

**So the question was:** is there a way to hand Vale the answer directly, instead of making it look up a name?

I checked. There's a rule that looks like it might be that — `RuneParentEnvLookup`, "get this blank's value from the surrounding scope." It doesn't work for us, for three separate reasons: the code path that compiles a function definition never processes it (and the solver now panics outright if one reaches it), and even where it *is* processed it searches the **caller's** scope — and a Vale program calling `make_counter()` has no `Counter` in scope to find.

**But there's a way out I hadn't seen.** Look-up-by-name is fine — the problem isn't the lookup, it's that `Counter` is a name a *user* could also use. And Vale already has a bunch of internal name kinds that user code physically cannot spell — that's how lambdas and anonymous structs get names that never collide with anything you write.

So: keep the ordinary "look it up by name" instruction, but register the Rust type under one of those unspellable name kinds. Lookup works exactly as normal, and it is structurally impossible for it to find a user's `Counter` instead.

Cost is one new variant on that name enum plus a couple of matching arms — small, and shaped like things already in there. What it buys is that the collision question stays deferred, and a Rust type's identity stays pinned to rustc's ID rather than to whatever Vale's namespace looks like that day.

The one honest wrinkle: our plan doc says a synthesized declaration should look *identical* to what you'd get from hand-writing `extern struct Counter` in Vale source. A secret name is a visible departure from that. I think it's the right departure — it's a namespacing decision, not us smuggling Rust's shape into the IR — but it is a departure, and it's your call as much as mine.

---

## 👤 User

so the problem is just that the name "Counter" might not be precise enough? is that right?

---

## 🤖 Claude

Essentially yes — but let me be precise about *which* kind of imprecision, and verify it, since I've been repeating this from the previous session.

---

## 🤖 Claude

Yes — and there's a sharpening worth having, because it names exactly what we're losing.

Vale has **two** kinds of name:

- **Precise** — an `INameT` inside an `IdT`, which carries the package coordinate: `rust.["mycrate"] :: Struct(Counter)`. Unambiguous.
- **Imprecise** — `IImpreciseNameS`, the source-level form. Essentially just the string `Counter`.

We *have* the precise one. rustc handed us the exact item and we intern a `rust`-packaged id for it. No ambiguity at all.

The problem is that **the rule language only speaks imprecise.** `LookupSR` takes an `IImpreciseNameS`. So writing the declaration forces us to downgrade a precise fact into an imprecise query.

And that downgrade is unrecoverable, because the imprecise lookup has no tiebreak. I verified both halves just now:

- `PackageEnvironmentT::lookup_with_name_inner` takes the parameter `_get_only_nearest` and **ignores it** (`environment.rs:880`) — it walks builtins plus *every* global namespace and concatenates everything it finds. So nearest-wins shadowing doesn't happen.
- `lookup_nearest_with_imprecise_name` then does `_ => panic!("Too many with name")` on more than one hit (`:164`).

So it isn't "ambiguity gets resolved badly" — there's no resolution step at all, and two hits is a compiler crash rather than an error message.

That's also, incidentally, why the *old* oracle design could defer this forever: the type arrived by identity from `fn_sig` and never went through a name lookup. Synthesizing declarations is precisely what puts us on the name path. That's the real, previously-unpriced cost of the pivot.

And it's why the secret-name fix works cleanly: it doesn't make the lookup precise, it makes the namespace **single-tenant**. Still an imprecise query, still ordinary machinery — but nothing else can ever put an entry there, so the missing tiebreak never gets exercised.

---

## 🤖 Claude

Agent's back. It got the tree wrong last time, so let me verify the load-bearing claims against our own tree before reporting them.

---

## 🤖 Claude

Both confirmed verbatim. Now the ignored tests and the other two blockers.

---

## 🤖 Claude

Verified every load-bearing claim against our own tree this time — all of them hold verbatim. Two things change the plan, and one of them is a gate.

## The correction: a Rust struct *does* get a real `StructDefinitionT`

Option A's premise — that we never create one — is wrong. What the working trees do is create an honest **empty** one, with the `extern` attribute carrying the opacity. Confirmed three ways in our own code:

- `instantiator.rs:1133` inserts into `monouts.structs` **before** the `ExternI` check at `:1134` — so an extern struct is in `structs` like any other
- `struct_hammer.rs:177` — `translate_opaque_i` does `hinputs.lookup_struct(&struct_it.id)`, i.e. opaque translation *reads* the definition
- `struct_hammer.rs:178` — `assert!(struct_def_i.members.is_empty())`

So `lookup_struct_template`'s `.expect("Struct template not found")` isn't a landmine to route around — it's an invariant to satisfy. That answers the question I flagged last turn: **synthesize the `StructS`, with `members: &[]`.**

Related, and it simplifies things: the pipeline gate is the **`extern` citizen attribute**, not a package-coordinate check. It threads parse → postparse → typing → instantiating unmodified (`parser.rs:585` → `post_parser.rs:1215` → `struct_compiler_core.rs:183` → `instantiator.rs:1207`), lands in `kind_externs`, and simplifying branches on membership (`struct_hammer.rs:113`, `type_hammer.rs:49`, `hammer.rs:323`). Our `is_rust_backed` package check is *not* what the machinery uses. No new mechanism needed anywhere.

## The gate: extern drop panics unconditionally, today

Both barricades verified verbatim:

- `struct_compiler_core.rs:86-92` — extern + `share` → `panic!("post-cut design forbids share-flavored extern structs")`
- `struct_drop_macro.rs:232-234` — `SharednessT::Single` + extern → `panic!("auto-generated drop for extern struct is unsupported; supply an explicit extern func drop(...)")`

And `derive_struct_drop` is in the default macro set (`struct_compiler_core.rs:94-102`). So **every extern struct in this tree panics one way or the other.** Synthesizing a `StructS` hits `struct_drop_macro.rs:234` the first time a Rust type is instantiated — not eventually, immediately.

This was already known here, incidentally: four tests carry the reason string *"blocked on opaque-extern-drop design — auto-derived drop for extern struct panics; see todo/opaque-extern-drop.md"* (`integration_tests_a.rs:1249/1284/1320/1357`, plus `hammer_tests.rs:453`). The agent rediscovered a blocker a previous session had already scoped.

**One qualifier it missed, and it matters:** those tests aren't merely `#[ignore]`d — `integration_tests` is commented out of `lib.rs:37` entirely, so they don't compile at all. Un-ignoring them is not a shortcut; relinking that module is onion-arc work.

The reason the sibling tree's equivalents are green is one keyword: they use `extern struct Vec<T> **imm**`, and `imm` → `Shared` → drop short-circuits to a bare `DiscardTE`, never touching layout. We banned `share` on extern structs deliberately, in the post-cut design. So this is a live architectural decision of ours colliding with the plan, not a bug. (Their design also leaks knowingly — with `imm`, their generated `extern func drop` never actually runs. Plumbing model, not correctness model.)

## Three open items I can now close

- **@PRIIROZ ordering** — inherited generic params go **last**. Four independent sources in ReiImpl, and our own `function_compiler_core.rs:398-402` comment agrees. Plan §5.4's ⚠ is resolved. (The arcana's expansion, "Parent Runes Inherited In Reverse Order," is what misled one of the earlier surveys — it means parent-last, not reversed-within.)
- **Constructor-macro suppression** — a non-issue. Our default macro set is only `derive_struct_drop`; `DeriveStructConstructor` isn't a keyword in this tree. Plan §5.5 can be struck.
- **Package coordinate (§5.6)** — now has a concrete tiebreak. `hammer.rs:332-333`, verified: `panic!("translate functionExterns: rust-package empty-name branch")` fires when `package_coord.module.0 == "rust"`. A real `rust.*` coordinate walks straight into it; an internal coordinate sidesteps.

Also worth knowing: the "give the opaque struct one synthetic blob member" idea was tried and abandoned — commented-out corpses across ValeRustInterop (`StructCompilerCore.scala:92-99` and friends). Both successors converged on empty members plus the attribute, and now *assert* emptiness. Don't revive it. And layout (size/align) never enters the frontend in their pipeline at all — a separate tool runs a generated Rust program to print `size_of`/`align_of` and the backend consumes it. Our `TyCtxt` oracle could supply that directly later, but nothing needs it now.

## What this does to sequencing

It reinforces the smoke-test order I suggested — **`add_two_numbers` first**, free function, no citizen step, no struct, so it dodges the drop gate entirely and still exercises `FunctionS` + `ExternBody` + solver + `make_extern_function` end to end.

But `Counter` is now gated on extern drop, and that's core work in `struct_drop_macro.rs` (and possibly `struct_compiler_core.rs`) — your side of the split, per `todo/opaque-extern-drop.md`. So the question I'd put to you before I write anything: **do you want to resolve extern drop first, or should I build and prove the free-function path while that's pending?** I'd rather ask than start on `Counter` and hand you a panic.

Plan doc still untouched.

---

## 👤 User

get Harmonious's opinion on all these things too, im curious

---

## 🤖 Claude

`Harmonious-coral` is live. Writing the message body to a file first (heredocs trip the mailbox shield).

---

## 🤖 Claude

Sent to `Harmonious-coral` (live, pid 63313). My watcher is armed, so their reply will wake me.

What I asked, weighted toward where their experience actually transfers rather than Vale internals:

**The pivot itself** — that the shipped design built a fully-resolved prototype from `fn_sig(item, &[])` at environment-build time, which is structurally incapable of representing `fn pick<A, B>(a: A, b: B) -> A`, and is why the `args` parameter they helped us think about testing had never once been passed a non-empty value. Plus that both sibling implementations had already tried and abandoned it, one still carrying the corpse.

**Four things I want them to react to:**

- **Identity.** Our candidate dedup is a hash set keyed on `(range, name, location)` that ignores the originating environment, so synthesized declarations sharing a sentinel location silently merge — `Vec::new` and `String::new` become one candidate, no error. I asked whether they generated declarations into their own IR, how they gave them identity, and whether two Rust items with the same short name ever collided in a map they didn't expect to be name-keyed. That's the one I most expect them to have a scar about, since "same name, different crate" is endemic in Rust and rare in hand-written code.
- **Namespacing.** That we're forced to downgrade a precise, package-qualified identity into a bare source-level name, and our imprecise lookup has no tiebreak — it concatenates every namespace and hard-panics on two hits. I pointed out they *sidestepped* this (bare `&str` resolver, stub crate's re-export table as the namespace) rather than solving it, and asked whether that's held up, and whether "put foreign items in a namespace the host can't collide with" was deliberate or fell out of the stub-crate boundary.
- **Empty definitions.** That foreign types get an ordinary definition with zero members plus an "extern" attribute, not no definition at all — and that the single-opaque-blob-member alternative was tried and abandoned. Asked how a Rust type is represented in Sky's own type system, and whether layout ever enters their frontend.
- **Drop** — which I flagged as the one I suspect is genuinely hard. Whether Sky owns Rust values whose `Drop` must run or keeps everything handle-shaped; how they reach rustc's drop glue if so; and whether a Rust panic has ever unwound back into Sky-generated code.

**Two smaller ones:** whether eager-resolve-and-park versus lazy-resolve-at-use-site is the same lesson as their `CARGO_PRIMARY_PACKAGE` → in-band marker redo (invocation-property vs artifact-property) and whether the genre has a name we can inherit the way we took "vacuous"; and how they handle dead-but-reachable types that encode an abandoned design.

I also closed the loop on what we owed them: all three of yesterday's fixes are in and earned their keep, and the assertion-presentation fix needed a second half neither of us called out — `install_ice_hook` sets a *process-global* panic hook that's never restored, so moving assertions after `run_compiler` returns wasn't sufficient without restoring plain panic reporting first. And that their warm-cache prediction is still unaddressed.

Nothing else started; plan doc still untouched, and your call on extern-drop sequencing is still open.

---

## 🤖 Claude

Reply's in, and the drop answer bears directly on your open sequencing decision. Let me check it against our own tree before relaying.

---

## 🤖 Claude

Substantive reply, and one part of it changes the decision that's pending. Leading with that.

## Drop — they answered both questions our own todo doc defers to "the Rust-interop TL"

**They never name the glue symbol, and not naming it is the entire design.** One generic wrapper in the stub crate:

```rust
#[inline(always)]
pub unsafe fn __toylang_drop<T>(x: *mut T) { core::ptr::drop_in_place(x) }
```

At scope end their compiler appends an ordinary call node — `__toylang_drop::<T>(&local)`. `drop_in_place::<T>` inside the wrapper body is where rustc's `DropGlue` resolution happens, while walking the wrapper's MIR, invisibly. They never construct drop glue, never name it, never special-case it. Yes, they do own Rust values whose `Drop` must run; both directions ship.

`todo/opaque-extern-drop.md` has two questions marked "defer to the Rust-interop TL" — that's us — and this answers both:

- *"Do `Vec<i32>` and `Vec<str>` call the same drop symbol, or does each monomorphization get a distinct mangled one?"* → **Neither.** One generic wrapper, monomorphized by rustc. There's no mangling decision to make.
- *"Someone must supply a `#[no_mangle] extern "C" fn drop(x: X) { std::mem::drop(x) }` per monomorphization… this is the user's obligation."* → **The obligation disappears.** One wrapper covers every `T`, and we generate the stub crate ourselves. Worth noting the todo's proposed shim is literally the by-value shape they tried and **reverted the same day** — by-value materializes a drop through a stack copy for every `let` including moved-out ones, and `Vec<Vec<Widget>>` double-frees. By-pointer `drop_in_place` preserves post-move state.

Two things I checked here. Vale **does** track moves statically (`unstackified_locals`, `function_environment_t.rs:201`), so that specific double-free mechanism is less likely to bite us — but the pointer shape is correct regardless, and it's what we need anyway since opaque layout means we can't `Destroy`. And they deleted their `needs_drop` predicate (~25 lines) because the wrapper gives "trivially-droppable costs nothing" for free, and the predicate could never answer correctly for a bare type parameter `let x: T` where the answer depends on the substitution. Their advice: skip writing it.

On unwinding they made it structurally impossible rather than handling it — global `panic = "abort"`, which cargo enforces across the whole build graph, so no landing pads exist anywhere. That's the abort-shim decision made **once, globally**, instead of per-boundary, and it dissolves our `Void`/`Never` destructor-return constraint instead of answering it. Cost is real: `catch_unwind` silently stops working, including inside Rust libraries that sandbox with it. They flagged it as a known documented constraint, and said if Vale must support unwinding they have nothing earned to offer.

**So the gating question I put to you may be smaller than I framed it.** Not "design extern drop from scratch with two open TL calls," but "synthesize a call to one generic wrapper" — assuming a global panic=abort commitment is available to us, which is a language-level call I'd want from you, not one I'd make.

## Identity — they have this exact scar, and it's their sharpest

Codified as `@ATAFLBZ`. They matched an impl's self type by short name — `tcx.item_name(adt_def.did()) == "Box"` — which is ambiguous three ways at once: `alloc::boxed::Box`, `std::ffi::os_str::Box<OsStr>`, and their own fixture crate's `Box`. `all_impls` returns impls from every loaded crate, so it matched whichever came first in iteration order.

The consequence is worse than a wrong candidate: the matched DefId determined the rustc-mangled symbol their backend emitted under, so matching std's `Box` produced mangling from std's DefId pointing at a symbol they never define. Silent at the mistake, link error much later, naming a plausible-looking symbol.

Fix was one line — filter the walk by provenance — **applied at six-plus sites**, because once they knew to look the same ambiguity was latent everywhere they walked a rustc-global collection. Their sharpened version of the lesson: don't let short name be load-bearing *anywhere*, including in linear scans you don't think of as maps. Their bug wasn't in a map at all.

## Namespacing — they got it free, and it did *not* save them

Blunt answer: the stub-crate boundary gave it to them as a side effect of two other decisions (the two-crate split, and @RTMEIZ's one-`pub use`-per-import). Nobody designed it as a namespace.

And it hasn't fully held. `@ATAFLBZ` happened *below* the layer their resolver operates on — in impl walks against rustc's global view rather than their curated table. A clean namespace at one layer bought nothing at the layer underneath. They also noted two imported items colliding on short name *within* the table would silently first-wins; they've never hit it because their fixtures are small, which they called luck rather than design.

They endorse our synthetic-name-variant fix with an amendment: **it only helps at layers that consult our namespace.** Anywhere we query rustc's global collections directly we still need provenance filtering. Do both.

## Foreign types — honest scoping, plus one hard warning

They scoped out of (a) properly: their direction is the mirror of ours (Sky types visible to rustc, not Rust types inside their type system), so they never had to decide how much of a Rust type to model.

On size/align: **get it from `tcx.layout_of`**, which they called the single clearest win in the message — three steps and a process boundary to obtain something the compiler already knows.

The warning is specific and good. They shipped "declared, defined, zero layout fields" on a struct with ≥1 source field, and it eventually ICE'd rustc's **debuginfo walker** — `build_struct_type_di_node` iterates source `FieldDef`s and calls `layout.fields.offset(i)` per field, assuming `source_fields.len() == layout.fields.count()`. It only surfaced when the type appeared *inside a generic* (`Vec<ToyShip>`), because outer-type debuginfo recursion descends into type params, so it lay dormant for a long time. Fix was making source shape and layout shape agree via a real wrapper field.

Transferable form: *"empty" is safe right up until some consumer iterates your members and cross-references them against something parallel* — and the siblings' `assert!(members.is_empty())` protects the invariant, not against that consumer.

I went looking. The closest thing here is five positional member lookups in `simplifying/` (`mutate_hammer.rs:144/190`, `load_hammer.rs:173/227/352`), all `members.iter().position(...).expect("member not found")`. But those are name-keyed lookups reached only via field access, which you can't do on an opaque type — so the parallel-walker shape doesn't obviously exist in our tree. Reporting that as "I looked and didn't find it," not as a clean bill; their point is that it's the consumer you don't think of.

## The two smaller ones

**Eager-vs-lazy** they call **pre-pass versus interleaving**, and it's the spine of their architecture. But they declined to unify it with the marker redo, which I think is the right call: artifact-vs-invocation is about *where a fact lives*; pre-pass-vs-interleaving is about *when a question is answered*. Shared root in one sentence — "don't commit to an answer before the context that determines it exists." No name for it; they'd adopt one if we coin it.

**Corpses: they delete, and keep the narrative in docs.** Empirically they deleted a ~600-LOC module plus its 22 tests, and later resurrected a different deleted thing from git history when the design came back around — which is the answer to "what if we need it." They'd go further than I was inclined to: delete it in the sibling too, with the reasoning written where the next person looks. And if deleting is politically expensive, make the constructor unreachable rather than leaving it public-and-dead. Their line: *"'Never ported' isn't a guard; it's a plan to remember."* They'd eliminated an equivalent hazard elsewhere by splitting an enum in two so the invalid state became unrepresentable.

---

Two things now sit with you: whether a **global `panic = "abort"`** commitment is available to Vale (it would dissolve the destructor-return constraint rather than us engineering around it), and — given drop may now be much cheaper than it looked — whether you still want extern-drop resolved before I prove the free-function path, or whether I should build `add_two_numbers` now and take drop with the wrapper design after.

Plan doc still untouched.

---

## 🤖 Claude

Watcher re-armed as **Vale4-opal**, resident with an empty queue.

Standing where we left it: nothing written, plan doc untouched, and two calls waiting on you —

1. **Global `panic = "abort"` for Vale?** It would dissolve the `Void`/`Never` destructor-return constraint instead of us building a per-boundary abort shim, at the cost of `catch_unwind` silently not working anywhere.
2. **Sequencing** — extern drop first, or build and prove the `add_two_numbers` free-function path now and take drop with the `drop_in_place` wrapper design after. The wrapper answer makes drop look considerably cheaper than when I first framed it as a gate.

---

## 👤 User

1. yes, global panic=abort.
2. your call which.

also, whats this about a hardcoded "Box" and linear scan? we arent doing that right?
whats provenance filtering?
and whats this synthetic thing youre talking about?
and yes, valen types shouldnt have to be visible to rustc unless the user explicitly exports them to rust. a lot of people think that we need to make all the valen-private functions and structs known to rustc and that's ABSOLUTELY not true. valen-private things (not explicitly declared by user as exported) should be *invisible* and unknown to rustc. please make sure thats somewhere near the top of our architecture doc.

with that in mind, what are all the differences between ours and their architecture on these and related/nearby aspects? i worked with them to make their architecture, with our current project in mind, so i want to know about any large deviations.

---

## 🤖 Claude

Several things. Let me check the factual one first — because I think the answer is "yes, we are."

---

## 🤖 Claude

Confirmed — we are. Now let me look at the arch doc's top so I can place the visibility principle correctly.

---

## 🤖 Claude

Two things already jump out — let me confirm before I edit.

---

## 🤖 Claude

The principle is already in the doc at §9.4 — but at line 998, not near the top. Let me see §1.7.

---

## 🤖 Claude

§1.7 is the right home, and two things there are worth noting immediately. Making the edit:

---

## 🤖 Claude

That flagged something. Let me read further — §10.3 and §10.1 may be inconsistent in exactly the way Harmonious warned about.

---

## 🤖 Claude

Both recorded. Taking your questions in order — and the first one has an uncomfortable answer.

## "We aren't doing that, right?" — we are, in two places

`tyctxt_oracle.rs:73-76`:
```rust
for &cnum in tcx.crates(()).iter() {
    for child in tcx.module_children(cnum.as_def_id()) {
        let name = child.ident.to_string();
        if !allowed.contains(&name.as_str()) { continue; }
```
That walks **every loaded crate** — std, core, alloc, every transitive dep — and matches on the bare identifier. Two crates exporting `Counter` both match.

And `resolve_method` at `:240-256` is a linear scan matching the owner type **by human-name string**:
```rust
i.name == method_name && self.items[owner].name == owner_name
```
`resolve_function` at `:259-264` is the same shape — `position(...)` by name, first wins.

That is structurally their `@ATAFLBZ` bug. Ours is currently masked by a three-element hand-written allowlist and a single fixture crate, which is luck, not design — the same thing Harmonious said about their own re-export table. The `DefKind` filter at `:81-85` guards against *kind* confusion (a module answering where a function was asked for), not against *identity* confusion.

## Provenance filtering

"Where did this item come from?" — filter a rustc-global query by whether the answer originates in a crate you control, instead of trusting a name match.

Their fix was one predicate — `is_from_lang_stubs(tcx, adt_def.did())` — applied to every walk over a rustc-global collection. For us the equivalent is: match on `DefId`, and where we must walk, filter by the crate number of our own fixture/stub crate rather than accepting anything named `Counter`. It's cheap and it's the difference between "the allowlist is small so we're fine" and "wrong answers are unrepresentable."

The reason it matters more than a normal name collision: the DefId we match eventually determines the mangled symbol our backend emits against. Match the wrong one and you get a link error much later, naming a symbol that looks entirely plausible.

## The synthetic name

Different problem, same instinct. A Vale declaration can only refer to a type by writing down a **source-level name** to be looked up later — and our lookup concatenates every namespace and hard-panics on two hits. So registering a Rust `Counter` under the ordinary name `Counter` puts it in the same pot as any Vale `Counter`.

The fix is to register it under a name variant that user source **physically cannot spell** — the way lambdas and anonymous structs already get names you can't type. The lookup stays completely ordinary; the namespace just becomes single-tenant, so the missing tiebreak never gets exercised.

Note these are complementary, not alternatives: the synthetic name protects lookups that consult *our* namespace, provenance filtering protects walks over *rustc's*. Harmonious's amendment was precisely that a clean namespace at one layer bought them nothing at the layer underneath. Do both.

## Visibility — added, and it was already there but buried

Added as a §1.7 bullet, stating that Vale-private items are invisible and unknown to rustc at every level, naming the misreading explicitly ("interop requires publishing Vale's whole universe so the mono collector can reach everything") and saying designing for that would be a serious error, with pointers to §9.4/§9.5.

Worth knowing: the principle was already in the doc — **§9.4, "Non-export items: invisible to rustc at every level," marked "architecturally imperative"** — but at line 998 of 3540, which is why you'd never see it. §1.7 now carries it.

## Differences from Sky — and the more useful finding

**Most of what Harmonious warned us about, we already inherited.** All four of yesterday's and today's big items are in our arch doc already:

- `__vale_drop<T>(&local)` scope-end wrapper, drop not architecturally special — §1.7, §1.2
- Global `panic = "abort"`, no landing pads, no `catch_unwind` — §1.7, §16. **Your answer ratifies existing architecture rather than changing it.**
- Wrapper-as-field opacity, *explicitly* to avoid the debuginfo-walker ICE they described — §10.1, §10.4.5, which even records that Sky retired its debuginfo-clamp fork patch once wrapper-as-field landed and that "Vale starts post-retirement"
- Explicit-import-only for Rust types (@RTMEIZ) — §1.7

**The four divergences the doc names**, in its own ranking:

1. **Two binaries vs Sky's single binary** — called "the principal architectural divergence" (§3). Downstream: it's why the dual-LLVM problem is ours alone, since Sky binds Inkwell to rustc's own LLVM.
2. **Send/Sync honest vs Sky's global lie** — "the most architecturally significant Vale-vs-Sky divergence in the boundary mechanism" (§12.4). Sky emits `unsafe impl Send` for every type and enforces sendability its own side; we emit only claims our analysis backs.
3. **Source-only distribution, no sidecars** — "Vale's second-largest divergence" (§21). Sky originally shipped sidecars and migrated toward our model in June.
4. **Vale owns every byte of emitted LLVM IR via the C++ backend** — §1.7, non-negotiable for backend pluralism.

Smaller: we adopt content-hash const args from day 1 to sidestep a dual-Instance/single-symbol conflict Sky hit, and we drop 2 of Sky's 9 cache axes.

**The real finding is that our *implementation* has drifted from our own architecture, not from theirs.** `todo/opaque-extern-drop.md` proposes per-monomorphization symbol naming plus a user-written `#[no_mangle] extern "C" fn drop(x: X) { std::mem::drop(x) }` shim per instantiation, and defers two questions "to the Rust-interop TL." Both questions are already answered by arch §1.7's `__vale_drop<T>` wrapper — one generic wrapper, rustc resolves DropGlue inside it, no symbol to name and no user obligation. And the todo's proposed shim is the by-value shape Harmonious reverted the same day it shipped. That todo predates or diverges from the arch doc, and I'd treat the arch doc as authoritative.

One small internal inconsistency I noticed while checking: §10.4 still literally shows `FieldsShape::Arbitrary { offsets: [], memory_index: [] }` while §10.4.5 says layout reports 1 field. Those can't both be right; §10.4.5 is clearly the intended one. Not fixing it unasked.

## Sequencing — my call

**`add_two_numbers` first.** It's the smallest thing that proves `FunctionS` + `ExternBody` + solver + `make_extern_function` end to end; it has no citizen step so it avoids `lookup_struct_template` and the drop path entirely; and it's where the encoding decision (synthetic imprecise name) gets validated before anything is built on top of it. Drop comes after, with the `__vale_drop<T>` design the arch doc already specifies — which also means reconciling that todo.

I'll also fix the two name-based matches in the oracle to key on `DefId` while I'm in there, since I'd otherwise be building on top of them.

Plan doc still untouched; the only edit I've made is the §1.7 bullet.

---

## 👤 User

that was a lot. can you walk me through all the things you just said, one by one. first one please

---

## 🤖 Claude

Sure. First one: **the name-matching in our oracle.**

## What the code does

When the driver starts up, `TyCtxtOracle::new` builds a table of every Rust item Vale is allowed to see. It does that like this (`tyctxt_oracle.rs:73-76`):

```rust
for &cnum in tcx.crates(()).iter() {
    for child in tcx.module_children(cnum.as_def_id()) {
        let name = child.ident.to_string();
        if !allowed.contains(&name.as_str()) { continue; }
```

`tcx.crates(())` is **every crate loaded into this compilation** — our fixture crate, but also `std`, `core`, `alloc`, and every transitive dependency. For each one we walk its public children, turn each child's identifier into a `String`, and keep it if that string is in our hand-written allowlist (`["add_two_numbers", "make_counter", "Counter"]`).

So the only thing deciding "is this the item we meant" is **string equality on a short name**.

Then two lookups read that table, both linear scans:

```rust
// resolve_function, :259-264
.position(|i| i.kind == ItemKind::Function && i.name == function_name)

// resolve_method, :240-256
i.name == method_name && self.items[owner].name == owner_name
```

Note the second one especially: it identifies *which type a method hangs off* by comparing the owner's **human name string**. Not its `DefId` — its name.

## Why that's a problem

Rust has no rule that short names are unique. `Vec`, `new`, `len`, `push`, `Error`, `Result`, `Box` — these appear in dozens of crates at once, and `tcx.crates(())` hands us all of them.

So if any other loaded crate happens to export something named `Counter`, both land in our table. `resolve_function` and `resolve_method` use `.position(...)`, which returns the **first** match. First-wins, silently.

Concretely: we'd attach `Counter`'s methods to a `Counter` from some other crate, or resolve `make_counter` to a different crate's function with a compatible-looking signature. Nothing errors. The compile succeeds.

## Why it's worse than an ordinary mix-up

Harmonious hit this exact bug and their consequence is the instructive part. The `DefId` you match eventually determines the **mangled symbol name** the backend emits code against. Match the wrong `Counter` and you emit against a symbol derived from *that* crate's DefId — a symbol nobody ever defines.

Result: no error at the point of the mistake, a **link error much later**, and the link error names a symbol that looks completely plausible. Theirs was `Box`, ambiguous three ways at once — `alloc::boxed::Box`, `std::ffi::os_str::Box<OsStr>`, and their own fixture crate's `Box` — and it took real time to find.

## How exposed are we today

Not very — and that's the trap. Our allowlist has three entries, and our fixture is one tiny crate. Nothing in std is named `add_two_numbers` or `make_counter`. So it works, and will keep working right up until we point this at a real crate.

Harmonious said the same thing about their own namespace: "we've never hit it because our fixtures are small, which is luck rather than design."

One thing we *do* get right already: there's a `DefKind` filter at `:81-85`, with a comment explaining that a crate's module children include its own `extern crate std`, so an unfiltered name match hands back a *module* where a function was asked for. That guards against confusing one **kind** of thing for another. It does nothing about confusing two things of the **same** kind.

## The fix, in one line

Key on `DefId` instead of on name, and where we genuinely have to walk a rustc-wide collection, filter by which crate the answer came from rather than trusting the string.

That second half is the thing you asked about next — provenance filtering — so I'll leave it there.

Say when you want the second one.

---

## 👤 User

wait, we walk every single visible item in rust? that sounds very expensive

---

## 🤖 Claude

Good instinct, but it's shallower than it reads — and the real problem turns out to be the opposite one.

**`module_children` on a crate root returns that crate's *direct* children, not its transitive contents.** It's one level deep. Our own driver logged exactly this earlier:

```
found crate: mycrate
  child: add_two_numbers -> Def(Fn, DefId(20:3 ~ mycrate[baf7]::add_two_numbers))
  child: std             -> Def(Mod, DefId(1:0 ~ std[27ce]))
```

Two children for `mycrate`: the function, and the `extern crate std` declaration. That's it. And there's no recursion anywhere in `new()` — it's a flat double loop.

So the cost is roughly *(number of loaded crates) × (items at each crate root)*. For a plain binary that's std plus its dependency tree — order tens of crates, each with maybe tens of root-level items. Order a thousand string comparisons, once, at oracle construction. Nothing.

**But here's the thing that actually matters: that walk can't find most items.** `std::vec::Vec` is a child of the `std::vec` module, not of the `std` crate root. So our current oracle **could never resolve `Vec`** — it would walk right past it. The fixture only works because `add_two_numbers` and `Counter` happen to sit at `mycrate`'s root.

So the design isn't expensive. It's *insufficient*. And the obvious repair — recurse into every module — is what would make it expensive, and would simultaneously make the collision problem far worse, because now you're comparing short names against the entire visible namespace of every loaded crate instead of just the roots.

Which is a real argument for the direction we're already heading. Under `import rust.std.vec.Vec`, we don't enumerate anything: we resolve *that one path*, segment by segment, to exactly one item. Cost is O(imports), not O(crate graph), and there's no ambiguity to resolve because the user wrote the full path. That's the Q5(c) end state, and it makes both problems disappear rather than trading one for the other.

The current walk is scaffolding that works for a flat one-crate fixture and doesn't generalize in either direction.

I can get you a real number — build the driver and have it print how many crates and children it actually visits — if you'd rather have the measurement than my estimate. Otherwise, ready for the second one whenever.

---

## 👤 User

awesome, that path sounds good. please note somewhere that we want to get rid of this linear walking and up-front iterating. second one please

---

## 🤖 Claude

Clean — 7 warnings, the known baseline. Noted in the code where whoever touches that walk will see it (`tyctxt_oracle.rs`, on `new`), with both halves: that the walk is *insufficient* rather than merely slow, and that short names aren't identity. I'll mirror it into the plan doc when we unfreeze that.

Let me verify one thing before the second item, since it bears on it.

---

## 🤖 Claude

Second one: **provenance filtering** — and checking it turned up something.

## What it is

"Provenance" here means: *which crate did this answer come from?* Provenance filtering means filtering the results of a rustc query by origin — keep only the ones from a crate you control — instead of deciding by what they're called.

The shape of the bug it fixes is always the same. You ask rustc a question whose answer ranges over **every loaded crate**:

- `tcx.crates(())` → all crates
- `tcx.all_impls(trait_def_id)` → every impl of that trait, anywhere
- `tcx.inherent_impls(type)` → every inherent impl block

Then you narrow the result with a name comparison. But short names aren't unique across crates, so you can keep a stranger. Filtering by origin instead makes the wrong answer structurally impossible rather than statistically unlikely.

## What it looks like concretely

Theirs was a single predicate — `is_from_lang_stubs(tcx, adt_def.did())` — dropped into every walk. Ours would be the same idea: after getting a `DefId`, check `def_id.krate` against the crate we generated, and discard anything else.

## The subtle part: how do you know which crate is yours?

This is where it gets interesting, and it connects to the other Harmonious lesson. You *cannot* identify your crate by name (names collide, and users rename crates) and you *cannot* identify it by something passed at invocation — an env var, a RUSTFLAG, a wrapper — because a legitimate caller can fail to supply it. That was their `CARGO_PRIMARY_PACKAGE` bug: a published library consumed as a transitive dep got compiled as an ordinary dep, the var was unset, the machinery stayed dormant, and stub bodies got codegenned for real.

Their fix was an **in-band marker**: put a constant in the generated crate, and detect it. That makes "is this mine" a property of the artifact rather than of how you were called.

## And here's the thing — this is already our architecture

I went to check whether we'd want to adopt it. We already did:

> **§26, ATAFLBZ** — *"Walks of `tcx.all_impls(...)` filter by `is_from_vale_stubs(self_type_did)`."*

> **§26.x** — *"Walks of `tcx.all_impls(trait_def_id)` return impls from every crate including std. Self-type-name check is ambiguous because std and Vale could both define a type named (e.g.) `Box`. Add `is_from_vale_stubs(tcx, adt_def.did())` filter inside impl walks. Under single-symbol architecture, wrong DefId produces wrong rustc-mangled name when Vale's bitcode emits a body."*

The `Box` example, the mangling consequence, the fix — all of it, already written down as a cross-cutting invariant. And §6.3 already specifies `__VALE_STUBS_MARKER` plus a **DefId-parentage check**, because Sky found that glob re-exports across dependencies can otherwise re-export the marker into a downstream crate and falsely flag it.

So `TyCtxtOracle` isn't missing a new idea — it's **violating an invariant our own architecture doc already records**. Slightly pointedly: the oracle's doc comment cites `@DPSFDOZ` (don't use `def_path_str` outside diagnostics), which is invariant **C1** in the same chapter. Whoever wrote it was reading that list and applied one item from it.

That's now three for three today — the drop wrapper, global panic=abort, and provenance filtering. Each time Harmonious "taught" us something, it turned out to already be in our arch doc, and the drift is in the implementation and the todo notes. Worth knowing as a pattern: our design capital is better than our code's use of it, and the gap is checkable.

Say when you want the third.

---

## 👤 User

well yeah, our architecture doc was inspired by Harmonious, they wrote half of it.

next

---

## 🤖 Claude

Fair — that deflates the point. It's not convergence, it's just implementation drifting from a doc they helped write.

Third one: **the synthetic name.**

## The setup

A synthesized `FunctionS` can't hold a type directly. It holds blanks and rules, and the only rule that names a type — `LookupSR` — carries an `IImpreciseNameS`, the *source-level* name form. So to say "`make_counter` returns `Counter`," we have to write the name `Counter` into the declaration and let Vale find it later.

## The problem

Vale's imprecise lookup has no tiebreak. `PackageEnvironmentT::lookup_with_name_inner` takes a `_get_only_nearest` parameter and ignores it — it walks builtins plus every global namespace and concatenates everything. Then `lookup_nearest_with_imprecise_name` does `_ => panic!("Too many with name")` on more than one hit.

So there's no shadowing rule, no "prefer the imported one," and no error — a compiler crash. Which means the moment a Vale program has anything named `Counter`, our Rust `Counter` collides with it fatally.

## The move

Don't make the lookup precise — make the *namespace* single-tenant.

Vale already has this pattern. `IImpreciseNameValS` has around a dozen variants and most of them are names a user physically cannot type: `LambdaImpreciseName`, `LambdaStructImpreciseName`, `ClosureParamImpreciseName`, `AnonymousSubstructTemplateImpreciseName`, `PlaceholderImpreciseName`, `RuneName`. Only `CodeName` holds something the programmer wrote.

That's how lambdas and anonymous structs already avoid colliding with user code: they live in the same lookup machinery, but in a compartment source syntax can't reach into.

So we'd add one more variant for imported Rust items. Lookup stays completely ordinary — same function, same walk, same concatenation — but nothing else can ever put an entry in that compartment, so "more than one hit" never happens and the missing tiebreak never gets exercised.

## Cost and benefit

Cost is one enum arm plus its interner and humanizer arms. Small, and shaped exactly like a dozen things already there.

Benefit is that the collision/precedence question — which is genuinely hard, needs Rust's three-tier model, and touches Vale's own name semantics — stays deferred instead of becoming a prerequisite. And a Rust type's identity stays anchored to its `DefId` rather than to whatever Vale's namespace happens to resolve that day.

## Two honest caveats

**It's a visible deviation from a rule we set ourselves.** The plan doc's §6 self-check says a synthesized declaration should be *structurally indistinguishable* from what the postparser emits for a hand-written `extern struct Counter`. A name variant the postparser would never produce fails that test literally. My read is it's the right deviation — it's a namespacing decision, not us smuggling Rust's shape into the IR, which is what §6 is actually guarding against — but I want it on the record as a deliberate exception rather than something I slid past.

**It only protects lookups that go through our namespace.** Anywhere we query rustc's global collections directly, it does nothing — that's what the previous item covers. Harmonious's amendment was exactly this: a clean namespace at one layer bought them nothing at the layer underneath. The two are complementary halves, not alternatives.

Say when you want the fourth.

---

## 👤 User

would it help if we had some sort of ImpreciseNameS or CodeNameS that was qualified with namespaces? or rather, change CodeNameS to contain a vec of strings, each for a part in eg rust.std.vec.Vec

---

## 🤖 Claude

Good question — and I think it's better than what I proposed, with one adjustment. Let me check the blast radius before answering.

---

## 🤖 Claude

Yes — and I think it's better than my compartment idea, for a reason I under-weighted: qualification is the *honest* fix. A compartment hides the ambiguity; a path removes it.

One adjustment though, on encoding.

## Make it a sibling variant, not a reshape of `CodeNameS`

`CodeNameS { name: StrI }` has 102 references across ~20 files, and it's the representation of *every source identifier in the language*. Widening it to a slice means every ordinary name — which is 99% of them — pays an indirection, and equality/hash go from interned-symbol comparison to slice comparison, on the hottest name type in the compiler.

Whereas adding `IImpreciseNameValS::QualifiedCodeName(&[StrI])` alongside `CodeName` costs one arm plus the exhaustive-match sites, touches nothing existing, and **works as a lookup key immediately** — the store is `IndexMap<IImpreciseNameS, Vec<IEnvEntryT>>`, keyed on the interned enum, so structural equality of a new variant just works with no special-casing.

And crucially it's *Vale's* qualified-name concept, not a Rust compartment. `import X.Y.Z` can populate it later; it's not something we'd throw away.

## The thing it changes, which is actually the point

Under qualification, a bare `Vec` **stops matching** a Rust item. The parser produces `CodeName("Vec")`; our store key would be `QualifiedCodeName(["rust","mycrate","Vec"])`; those aren't equal, so bare mentions never find Rust types.

That's either the feature or the bug, depending on your earlier answer — you told me `x = Vec<int>();` is legal, bare.

The resolution is to register under **both** keys: always the qualified one, and *additionally* the bare one when the program actually imported it. Then:

- `rust.mycrate.Vec` always resolves
- bare `Vec` resolves **iff you imported it** — which is exactly the import semantics we've been deferring, and it makes `import` mean something for the first time
- the collision panic can only fire when two *imported* things share a bare name — precisely where Rust itself raises `E0252`

That's not a hack: `add_entries` already registers prototypes under multiple imprecise keys today (`environment.rs:568-573` puts each one under a template key, a local key, *and* a `PrototypeName` key). Multi-key registration is an established pattern in the exact function we'd be extending.

## What it still doesn't fix

The general defect is untouched — `lookup_with_imprecise_name_inner` still ignores `_get_only_nearest`, still concatenates every namespace, still panics on two hits. Qualification means we mostly stop *triggering* it, not that it's fixed.

But the failure mode gets much better. Instead of "a Rust type crashes the compiler because a Vale type shares its name," it becomes "two things you explicitly imported have the same bare name" — which is a real user error, at a moment we can point at, and the panic is then just the wrong *diagnostic* rather than the wrong *behavior*. That's a fair thing to leave for whenever we do Rust's three-tier model properly.

So: your instinct, my encoding, and it collapses the import question into the same change. Want me to write that up as the naming design before I start on `add_two_numbers`, or keep it in flight and settle it when I get to the first `LookupSR`?

---

## 👤 User

hmmm. not sure about this. show me an example of the generated code and the offending CodeNameS?

---

## 🤖 Claude

Fair — let me ground it in real types rather than describing it.

---

## 🤖 Claude

Here's the concrete thing. Fixture:

```rust
pub struct Counter { pub value: i32 }
impl Counter { pub fn get(self) -> i32 { self.value } }
pub fn make_counter() -> Counter { Counter { value: 7 } }
```

**What a human would write in Vale** — the gold standard we're supposed to be structurally indistinguishable from:

```vale
extern struct Counter { }
extern func make_counter() Counter;
```

**What we'd synthesize for `make_counter`**, modeled on `struct_drop_macro.rs:60-107` (the closest in-tree precedent):

```rust
FunctionS::new(
  /* range              */ RangeS::internal(scout_arena, -8_000_001),  // DefId-derived, unique per item
  /* name               */ IFunctionDeclarationNameS::FunctionName(FunctionNameS {
                               name: scout_arena.intern_str("make_counter"),
                               code_location: CodeLocationS { file, offset: -8_000_001 },
                           }),
  /* attributes         */ &[],
  /* generic_params     */ &[],
  /* tyype              */ TemplateTemplataType { param_types: &[], return_type: FunctionTemplataType },
  /* params             */ &[],
  /* maybe_ret_kind_rune*/ Some(ret_kind_rune),
  /* rules              */ &[
      IRulexSR::Lookup(LookupSR {
          range,
          rune: ret_template_rune,
          name: CodeName(CodeNameS { name: "Counter" }),   // <-- this one
      }),
      IRulexSR::Call(CallSR {
          range,
          result_rune:   ret_kind_rune,
          template_rune: ret_template_rune,
          args:          &[],
      }),
  ],
  /* body               */ &IBodyS::ExternBody(ExternBodyS {}),
)
```

**The offending `CodeNameS` is exactly one field: `LookupSR.name`.** That's it. It's how the declaration says "my return type is whatever `Counter` resolves to."

Worth separating two things that both look like names here:

- `FunctionNameS { name: "make_counter", .. }` — the function's *own* name. **Not a problem.** Function names go through `lookup_all_with_imprecise_name` (plural) feeding overload resolution, which handles multiple hits by scoring them.
- `LookupSR.name = CodeName("Counter")` — a **type** reference. This one goes through `lookup_nearest_with_imprecise_name`, which does `_ => panic!("Too many with name")`.

So if the Vale program contains any `struct Counter`, resolving that single rule crashes the compiler. And for `Counter::get(self Counter) int` the identical `CodeName("Counter")` appears again, in the param's type rules — so it's one field, appearing once per Rust-type mention across all synthesized declarations.

**Under your proposal**, that field becomes:

```rust
name: QualifiedCodeName(&["rust", "mycrate", "Counter"]),
```

and the store registers `Counter` under that key instead of (or as well as) the bare one.

That's the whole delta — which is why I'd rather add a sibling variant than reshape `CodeNameS`. We're not changing how names work; we're changing what goes in one field of one rule kind, at sites we generate ourselves.

The honest cost, stated plainly: a hand-written `extern func make_counter() Counter;` postparses to `CodeName("Counter")`, so ours would visibly differ from the human-written equivalent. That's the §6 "structurally indistinguishable" rule we'd be breaking. My argument is that it's a namespacing difference rather than a Rust-shape leak — but you can see it now rather than take my word for it.

One thing I'd verify before writing it: whether the `CallSR` is actually required at zero arity, or whether `LookupSR` alone yields the kind for a non-generic citizen. The drop macro always emits both, but its citizen is generic.

---

## 👤 User

what about this instead: LookupSR has a vec of CodeNameS

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

what about this instead: LookupSR has a vec of CodeNameS

---

## 🤖 Claude

Let me check the decisive thing — what the solver actually does with `LookupSR.name`.

---

## 🤖 Claude

That settles it, I think. The solver's `Lookup` handler (`compiler_solver.rs:1028-1033`):

```rust
IRulexSR::Lookup(r) => {
    let result = match self.lookup_templata_imprecise(env, state, &ranges, r.name) {
```

`r.name` goes **straight into the imprecise lookup**, which reads `imprecise_to_entries: IndexMap<IImpreciseNameS, Vec<IEnvEntryT>>` — a map keyed on a single interned `IImpreciseNameS`.

So a vec on `LookupSR` doesn't reach the thing that matters. Two ways it could go:

**(a) Walk the segments** — resolve `rust` to a namespace, then `mycrate` within it, then `Counter`. That's a real multi-segment path resolver. It doesn't exist, nothing else needs it, and Harmonious explicitly sidestepped building one (their resolver takes a bare `&str` against a curated table).

**(b) Mash the vec into one key** — but then the *key* has to be able to represent a path, which means you need the `IImpreciseNameS` variant anyway, and the vec on `LookupSR` is redundant with it.

On blast radius your instinct is reasonable — `LookupSR {` has 37 construction sites vs `CodeNameS`'s 102 references, so it *is* the smaller surface. It's just not the deciding axis; the key type is.

And note the qualified-key version handles nesting fine without any walking. `rust.std.vec.Vec` is just the key `["rust","std","vec","Vec"]` — we register it, we look it up, one map get. We don't need a resolver because **we generate every name in that namespace**, so there's nothing to resolve that we didn't put there.

The path-walking version only becomes necessary when something *other than us* mints names in a namespace — i.e. Vale's own `import X.Y.Z` for Vale packages, where the user writes a path to something the compiler didn't synthesize. That's a real future need, and when it lands the `LookupSR` change might well be part of it. It just isn't load-bearing for Rust interop, and building it now would be paying for generality we can't use.

So I'd still put it on the name: one `IImpreciseNameValS::QualifiedCodeName(&[StrI])` arm, `LookupSR` untouched, no resolver.

Unless you're thinking of the path resolver as something Vale wants soon for its own package story — in which case doing it once, properly, and having interop be its first consumer is a defensible sequencing, just a considerably bigger piece of work than what's in front of us.

---

## 👤 User

what are the downsides of (a)? and would it be useful for actual vale programs, outside of rust interop?

---

## 🤖 Claude

Let me check two things that determine the answer.

---

## 🤖 Claude

Checked both, and the answer is clearer than I expected.

## Downsides of (a)

**1. There's nothing for an intermediate segment to resolve to.** This is the big one. To walk `rust` → `mycrate` → `Counter`, the first two segments each have to produce *some value*. `ITemplataT` has 15 variants — Kind, Placeholder, Integer, Boolean, String, Prototype, Isa, CoordList, the two array templates, Function, StructDefinition, InterfaceDefinition, ImplDefinition, ExternFunction — and **none of them is a module or namespace.**

So (a) isn't one enum arm. It's a new `ITemplataT` variant, a matching `ITemplataType`, rune-typing support for it, and solver handling. That's a new concept in the type system, not a new spelling of an existing one.

**2. Environments and templatas are different types.** The thing you can look up *in* is `IEnvironmentT` (`PackageEnvironmentT` and friends); the thing lookup *returns* is `ITemplataT`. Walking means "look up X inside the thing I just found," so you'd have to bridge those two, or make namespaces exist as both.

**3. The root is still ambiguous.** `rust.std.vec.Vec` is only unambiguous if `rust` itself resolves unambiguously — and root lookup is the same concatenate-everything-then-panic path. Qualification doesn't bootstrap itself; you'd still need the root anchored specially.

**4. Diagnostics become real work.** "Couldn't find `a.b.C`" is useless without knowing which segment failed and what was in scope at that point.

**5. It doesn't fix the underlying defect** — `_get_only_nearest` stays ignored, `lookup_nearest_*` still panics on multiplicity. Same as the cheap option.

## Useful for Vale outside interop? Yes — but not the walking part

Today Vale has **no way at all** to disambiguate two same-named items from different packages. No shadowing, no qualification, no escape hatch; the program is simply uncompilable with a panic. That's a genuine language gap, not just an interop one.

But I owe you a correction that changes the shape of it. I said earlier that `import`'s name was parsed and discarded. **It isn't.** `ImportS { range, module_name, package_names, importee_name }` carries the full path *and* the imported name intact into postparsing. The only thing that reads `program.imports` anywhere is a test traversal. The data is all there; nothing consumes it.

So Vale's name story is three pieces, in value order:

1. **Make `import X.Y.Z` bind `Z` in the importing scope.** Registration-time mapping. No resolver needed, and the data already exists.
2. **Turn `panic!("Too many with name")` into a real ambiguity error.**
3. **Qualified paths as an escape hatch.**

(1) and (2) are the high-value ones and neither needs path-walking. And even (3) doesn't, really — the compiler knows every item's full path at registration time for Vale packages just as much as for Rust ones. Source-level `mypackage.Foo` parses to the key `["mypackage","Foo"]`, the store already holds that key, one map get.

**Walking is only needed to resolve *through* something whose contents aren't known at registration time** — re-export chains, aliases, dynamically-assembled namespaces. Vale has none of those.

So (a) buys generality that Vale's own story doesn't appear to need either. My read is that the qualified key is the right representation for both, and the genuinely valuable Vale work is (1) and (2) — which are independent of everything we're doing and would make `import` mean something for the first time.

---

## 👤 User

isnt there a PackageTemplataT?

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

please send an agent into ~/rust to figure out what they do here

---

## 🤖 Claude

It's a full rustc checkout (1.95.0, with the fork sysroot). Dispatching.

---

## 🤖 Claude

Agent's off into `~/rust`, read-only.

I pointed it at `rustc_resolve` and asked it to lead with the two questions that actually decide our design:

- **Are modules first-class values in rustc, or a separate structure that resolution walks?** That's the exact thing Vale lacks — `ITemplataT` has no module variant — so whatever rustc does here tells us whether (b)'s missing piece is a small gap or a whole parallel universe.
- **What does path-*walking* buy that a pre-registered full-path key can't?** With my hypothesis stated so it can be refuted: that walking only earns its keep when you must resolve *through* something whose contents aren't known at registration — re-export chains, globs, macro-generated items, aliases. If that's right, (a) is sufficient for us and (b) is generality we can't use.

Plus the supporting mechanics: how `ast::Path`/`PathSegment` represent a bare name versus a path, what the accumulator is inside `resolve_path` and how the first segment is special-cased, the `TypeNS`/`ValueNS`/`MacroNS` axis, whether `use` copies a binding or installs an indirection, and the precedence story — how "explicit `use` silently shadows a glob" is implemented, and what structurally separates `E0252` (two explicit uses, error at the import) from `E0659` (two globs, error only at the use site). That last one matters because it's the three-tier model we said we want long-term and currently can't express at all.

I also asked for a size estimate with a specific split: how much of `rustc_resolve` is intrinsic to "resolve a qualified name" versus incidental to hygiene, macro-expansion interleaving, and edition differences — since a language without macros shouldn't be scared off by a line count that's mostly macro machinery.

I'll report when it lands. Meanwhile the state is unchanged: two edits made (the §1.7 visibility bullet, the `tyctxt_oracle.rs` note), interop build clean at 7 pre-existing warnings, plan doc untouched, and nothing started on `add_two_numbers` yet.

---

## 🤖 Claude

Let me verify the linchpin claim before I act on it, since it kills option (a).

---

## 🤖 Claude

Verified the three load-bearing claims. The report kills option (a), and dissolves my main objection to (b).

## (a) is dead for Rust interop — here's the proof

`library/std/src/lib.rs:575` is `pub use alloc_crate::vec;`, and `Vec` is defined at `library/alloc/src/vec/mod.rs:440`.

So the key `["rust","std","vec","Vec"]` **names no definition.** Its def path is `alloc::vec::Vec`. Populate the table from def paths and users can't write what they'd write; populate from use paths and you have to know every re-export in advance.

And there's no canonical answer to pick: rustc runs an entire query, `visible_parent_map`, doing a **BFS over the whole forest of crates**, purely to invert def-path → writable-path for diagnostics — and the result is explicitly many-to-one and lossy ("prefer the shortest"). `std` is re-exports all the way down.

The clincher is that **both real-world analogues of our oracle walk segments.** clippy's `lookup_with_base` sits outside the resolver against a finished `TyCtxt` — exactly our position — and loops segment by segment over `tcx.module_children`. rustdoc splits a literal string on `::` and feeds it to the same loop. Neither builds a key map, because neither can.

Two details from clippy worth stealing directly: its header says *"this function is expensive, use sparingly"*, and it returns `Vec<DefId>` rather than `Option`, because `memchr::memchr` can resolve to two major versions of the crate at once. Our `resolve_function` returning a single `Option` is already the wrong shape.

## But my objection to (b) was wrong

I said path-walking needs a namespace value type, and `ITemplataT` has none, so it's a new concept in the type system. **rustc doesn't have one either.** There's no `Res::Module`; a module is `Res::Def(DefKind::Mod, def_id)`, and `DefKind::Mod` is rejected by every position check in `PathSource::is_expected`. It never reaches `Ty`.

What rustc actually has is a **resolver result type that's strictly larger than the typechecker's value universe** — modules are legal *intermediates*, illegal *finals* — plus a side graph of module nodes keyed by `DefId`, never handed to the typechecker.

So we don't need `ITemplataT::Module`. We need a resolver-result enum that isn't `ITemplataT`. That's much cheaper than what I described, and it retracts downside #1 from my earlier answer.

## Which makes the synthesis: your representation, walking resolution

Your instinct about qualified names was right — that's the *source-level representation*, and it fixes Vale's actual bug (no way to disambiguate anything, ever). The walking is the *resolution strategy*. They're not competing; I had them as alternatives and they're layers.

Concretely: interned qualified name in `LookupSR`, and a per-step primitive `children_of(item) -> [(name, ns, item)]` — backed by our existing store for Vale modules, and by `tcx.module_children` for Rust, exactly as clippy does.

## The design to steal outright

`NameResolution` in `imports.rs:243-266` is five lines and it's the whole three-tier model we said we wanted:

```rust
pub(crate) struct NameResolution<'ra> {
    pub single_imports: FxIndexSet<Import<'ra>>,
    pub non_glob_decl: Option<Decl<'ra>>,
    pub glob_decl: Option<Decl<'ra>>,
}
pub(crate) fn best_decl(&self) -> Option<Decl<'ra>> {
    self.non_glob_decl.or(self.glob_decl)
}
```

**Precedence is a struct field, not a comparison.** "Explicit `use` silently shadows a glob" is literally `non_glob_decl.or(glob_decl)` — there's no ambiguity to detect because the two live in different slots. E0252 is a collision *in the data structure* (two things want `non_glob_decl`); E0659 is a loser stapled to the winner and reported lazily at the use site. That's exactly the split I described to you earlier as something Vale "can't express" — and it turns out to cost one struct with two Options.

Also worth adopting on day one, from `ident.rs:66`: user-defined names outrank built-in/stdlib names **so that adding to the stdlib isn't a breaking change**.

## Scale reality-check

`rustc_resolve` is 26,099 lines, but the irreducible kernel — walk a path against a module tree honoring imports and shadowing — is **~1,500–2,500 lines**. The rest is diagnostics (~40%), macros, hygiene, rustdoc, lints, and Rust's own features. Editions cost ~120 lines.

The one caveat that matters for us: **globs are what force the fixed-point iteration.** Without globs, imports form a DAG and a topological sort suffices. rustc's own fixed point still fails to converge in four open issues from 2024, all "explicit import shadows a glob whose resolution depends on that glob." Strong argument for Vale not having globs, or having them late.

And one thing that transfers straight to the oracle regardless of everything above: rustc populates a foreign module's children **on first touch**, not up front. That's the fix for the walk I flagged earlier — don't enumerate anything, decode a module when a path steps into it.

## What this changes right now — less than it sounds

For `add_two_numbers` none of this is needed: one free function at a crate root, one segment. The immediate decision is still just what goes in `LookupSR.name`, and a qualified interned name serves that fine whether resolution later becomes a walk or not.

What it *does* change is that I should stop treating the qualified key as the destination. It's the representation; the walk is the resolution; and the oracle's `resolve_function -> Option<RustItemId>` signature is already wrong in the way clippy warns about.

There's also a cheaper escape hatch worth knowing about: rustc's own answer for naming library items from compiler code isn't paths at all — it's `#[rustc_diagnostic_item = "Vec"]`, 397 of them, a flat `Symbol → DefId` map declared at the *definition* site. If Vale's prelude only needs to name a handful of Rust items, that's a registry rather than a resolver.

Want me to write this up properly — the naming/resolution design as its own doc — or park it and get `add_two_numbers` working first?

---

## 👤 User

tldr your recommendation?

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

tldr your recommendation?

---

## 🤖 Claude

**Build `add_two_numbers` first. Don't build a resolver yet.**

Short version of why: the walking-vs-key question only bites when a path has more than one segment or crosses a re-export. `add_two_numbers` is one item at a crate root. So it proves the thing actually in doubt — that a synthesized `FunctionS` with an `ExternBody` flows through the solver and `make_extern_function` — without paying for name machinery we can't yet validate.

Three things I'd carry into it now, because they're cheap and they'd be expensive to retrofit:

1. **Qualified interned name in `LookupSR`**, not a bare `CodeName`. Right representation under either resolution strategy.
2. **Key on `DefId`, not strings**, in the oracle — kills the `@ATAFLBZ` shape before we build on it.
3. **Unique DefId-derived `CodeLocationS` per synthesized item** — the silent-overload-drop hazard, and the one failure here that produces no error at all.

Deferred deliberately: the path walker, the `children_of` primitive, lazy module population, and the `non_glob_decl`/`glob_decl` precedence struct. All correct, none needed for one crate-root function, and all better designed once we've seen real signatures flow through.

The one thing I'd flag as a real bet: `resolve_function -> Option<RustItemId>` is already the wrong shape — clippy returns `Vec<DefId>` because `memchr::memchr` resolves to two crate versions at once. I'd leave it wrong for now and fix it when the walker lands, rather than widen a signature we're about to redesign.

---

## 👤 User

sounds good. make sure our markdown docs reflect this

---

## 🤖 Claude

Now rewriting the plan doc to integrate everything.

---

## 🤖 Claude

Now the todo doc, which actively conflicts with arch §1.7.

---

## 🤖 Claude

Two more things — one is a cross-reference I got wrong in my own rewrite.

---

## 🤖 Claude

Docs updated. Five files:

**`synthesized-declarations-plan.md` — rewritten** (254 → ~400 lines), integrating everything from today:

- **§2.1 new** — a tree-mismatch warning. The survey recommending `FunctionA`/`StructA` was reading RustInteropReiImpl; `lib.rs:26` says *"higher_typing was retired outright"* here, and `FunctionTemplataT.function` is `&'s FunctionS`. `FunctionS` is the only level in Vale4, and future surveys of that branch must say which tree each `file:line` belongs to.
- **§4.1 new** — `ExternBody` *is* the wrapper-with-externcall path. One producer of each end, `make_extern_function` already registers empty bounds at instantiation time. Not a fork, and the only untested combination is a *synthesized* `FunctionS` carrying `ExternBody`.
- **§5 new** — naming and resolution: why `LookupSR` forces a name, why MKRFA isn't an escape hatch, and the four rustc findings — no `Res::Module` (so no `ITemplataT::Module` needed, just a resolver-result enum), `std::vec::Vec` is a re-export so a full-path key can't work, clippy and rustdoc both walk, and precedence as a struct field rather than a comparison. Plus the ~1,500–2,500-line kernel estimate and the globs-force-the-fixed-point argument.
- **§6 new** — identity promoted to top risk, with all four consumers of the synthetic range and the silent-candidate-drop mechanism.
- **§7 new** — foreign types get a real definition with empty members; Option A's premise corrected, verified three ways.
- **§8 new** — extern drop as the gate, with §8.1 recording the conflict against arch §1.7 and §8.2 marking `panic = "abort"` ratified.
- **§9** — @PRIIROZ resolved (inherited last), constructor macro struck, package-coordinate tiebreak added from `hammer.rs:332-333`.
- **§12 new** — the near-term plan: `add_two_numbers` first, three things to carry, explicit defer list.
- **§13 new** — the oracle's name-matching defect as a violation of arch §26's @ATAFLBZ, which already states the rule.

**`todo/opaque-extern-drop.md`** — superseded banner on the `extern(rust)` rows only. Its two "defer to the Rust-interop TL" questions are answered by arch §1.7's `__vale_drop<T>`, and its proposed shim is flagged as the by-value shape Sky reverted.

**`rust-interop-frontend-plan.md`** and **`rust-interop-callout-map.md`** — banners distinguishing what still holds from what doesn't. For the callout map that's specifically "inventory current, prescribed fixes historical," plus the two corrections carried over from earlier sessions (§5.2 wrong about `IEnvEntryT::Templata`, §4.3 conflating legitimate-absence with invariant-violation panics).

**`README.md`** — the plan doc is now the "start here," with the other two marked.

Also still in the tree from earlier: the arch doc's §1.7 visibility bullet, and the `VCOORD` note on `TyCtxtOracle::new`. Interop build clean at 7 pre-existing warnings; nothing committed.

One thing I did *not* do: fix the §10.4/§10.4.5 inconsistency I spotted in the arch doc, where §10.4 literally shows `offsets: []` while §10.4.5 says layout reports 1 field. It's a real contradiction and §10.4.5 is clearly the intended reading, but it's outside what we discussed — say the word and I'll correct it.

---

## 👤 User

i forget, did we solve rust_package_stores(self, &mut coutputs) taking that &mut?

---

## 🤖 Claude

Yes — solved by deletion, not by fixing. Let me confirm against the current code.

---

## 🤖 Claude

Yes — but not by fixing it. It's solved by the pivot, and your objection is what caused the pivot.

The chain as it stands today:

- `rust_package_stores` puts `ITemplataT::Prototype` entries in the store (`importer.rs:204-206`)
- `overload_resolver.rs:218` asserts every prototype candidate already has registered bounds — `assert!(coutputs.get_instantiation_bounds(...).is_some())`, with three more at `:435/:457/:488`
- only `add_instantiation_bounds` creates one, and it takes `&mut CompilerOutputs`
- so `importer.rs:179` takes `&mut`, and `compiler.rs:725` had to move `CompilerOutputs::new()` **above** the `GlobalEnvironmentT` construction to make it available that early

That's exactly the thing you objected to — registering prototypes at environment-build time instead of templates. The `&mut` was never independently fixable; it's a symptom of the prototype being minted too early.

Under the pivot the store entry is a **template** — `IEnvEntryT::Function(&FunctionS)` — not a prototype. Templates carry no instantiation bounds, because bounds belong to instantiations. They get registered inside `make_extern_function` (`function_compiler_core.rs:352-362`), which is where the real extern path already does it, at the right moment and with real args.

So the `&mut` disappears. And the function disappears with it — plan §11 lists `rust_package_stores` and its hook among what the pivot deletes, since synthesized declarations are ordinary env entries and need no Rust-specific store construction at all.

Worth noting: that also means the `CompilerOutputs::new()` move can revert. It was made *only* to satisfy this. So the pivot un-does two core-compiler changes, not one — `compiler.rs` ends up closer to untouched than it is now.

None of that has happened yet; it's all still in the tree as uncommitted current-state code, and it goes when the synthesized-declaration path lands.

---

## 👤 User

what else needs to change in the core for that to happen?

---

## 👤 User

what else needs to change, in the core and in the non-core, for that to happen?

---

## 🤖 Claude

Let me verify the highest-uncertainty piece first — whether a declaration that exists only in an env store ever gets compiled.

---

## 🤖 Claude

That check paid off — and it answers the injection-point question I'd flagged as open.

## The injection point is already solved

The **function compile phase** (`compiler.rs:1010-1035`) iterates `global_env.name_to_top_level_environment` — the env stores themselves, *not* the program's denizen list:

```rust
for (_name, entry) in templatas.name_to_entry.iter() {
    match entry {
        IEnvEntryT::Function(function_a) => {
            let templata = FunctionTemplataT { outer_env: package_env_t, function: function_a };
            let _header = self.evaluate_generic_function_from_non_call(...)?;
```

So a synthesized `FunctionS` placed in a top-level store is **automatically indexed and eagerly evaluated**, down the identical path a Vale function takes. No new hook, and the existing `rust_package_stores` position already lands the store there before this loop runs. (The `if !package_id.init_steps.is_empty() { continue; }` guard is satisfied — our rust package id has empty init steps.)

## Core — two deletions, zero additions expected

| file | change |
|---|---|
| `compiler.rs:732` | drop the `&mut coutputs` argument; the hook itself **stays**, since it's how the store reaches `name_to_top_level_environment` |
| `compiler.rs:725` | revert the `CompilerOutputs::new()` move back below the `GlobalEnvironmentT` construction |

Everything else the path needs already exists: `get_imprecise_name`'s `FunctionTemplate` arm (`environment.rs:440`), the `ExternBody` dispatch, `make_extern_function`, `add_function_extern`.

One to check rather than assume: the `INameT::ExternFunction` arm we added to `get_imprecise_name` earlier was for the prototype-store design. Under the pivot the store holds `IEnvEntryT::Function`, so that arm may become dead — worth confirming before leaving it in.

## Non-core — inside `rust_interop/`

1. **A declaration synthesizer** (new file). Builds a `FunctionS` per importable item: `DefId`-derived unique `RangeS`/`CodeLocationS`, `FunctionNameS`, empty `generic_params` for now, `params`, `maybe_ret_kind_rune: Some(..)`, the `Lookup`/`Call` rules, and `body: &IBodyS::ExternBody(ExternBodyS {})`. Modeled on `struct_drop_macro.rs:60-107`.
2. **Rewrite `rust_package_stores`** to emit `IEnvEntryT::Function(&FunctionS)` instead of `Templata(Prototype)`, and drop the `coutputs` parameter.
3. **Delete the prototype-minting path** — `make_prototype_entry` and its `add_instantiation_bounds` call, for the function case.
4. **Oracle**: `importable_functions` stays; `fn_sig` stays but changes role (below). `resolve_function` / `resolve_method` become unused for this path and should go rather than linger.

Worth noting `add_two_numbers` needs **no naming work at all** — it's `i32 -> int`, so its `LookupSR` names `int`, a Vale builtin already in the builtins store. The qualified-name question doesn't arise until a Rust *type* appears in a signature. That's a stronger argument for this ordering than I gave earlier.

## A correction that falls out — and it affects the plan doc

Plan §11 says *"`fn_sig(item, args, ..)` survives and finally receives real args — from `make_extern_function`, per instantiation."* **That's wrong under the pivot.**

`make_extern_function` reads its types from `params2`/`ret_coord`, which the solver produced from the rules we synthesized. So the oracle is consulted **once per item, at synthesis time**, and its job is to lower a *generic* rustc signature into Vale runes and rules — with `ty::Param` becoming a rune reference, not a concrete type. Vale's own solver does the instantiating. That's exactly how the sibling trees work: their generated `extern func with_capacity(c i64) Vec<T>;` is generic in Vale terms, and no per-instantiation rustc query ever happens.

Consequence: `instantiate_identity()` — which I currently have panicking loudly for anything generic, on the grounds that it's only valid for structural inspection — becomes **correct**, because structural inspection is precisely what we're now doing. That guard needs revisiting rather than keeping.

## Genuine unknowns

- Whether `evaluate_generic_function_from_non_call` drives an extern-bodied declaration cleanly, or whether the definition solve trips on something. This is the untested `FunctionS` + `ExternBody` combination.
- Whether `CallSR` is needed at zero arity, or `LookupSR` alone yields the kind.
- What eager evaluation does for a *generic* extern — `make_extern_function` would read placeholder `template_args` off the env id. Presumably fine (it's what generic Vale functions do), but unverified.

Want me to start on it? I'd do the synthesizer and the store rewrite first, leave the two `compiler.rs` deletions until it compiles, and bring you the core diff at the end — same shape as last time.

---

## 👤 User

do only the core changes right now for me

---

## 🤖 Claude

Checked, and the two core edits are coupled to the non-core body — `rust_package_stores` calls `make_prototype_entry(compiler, coutputs, ...)` per importable function (`importer.rs:204-206`), so `coutputs` is load-bearing for it as written.

That means neither edit can land alone:

- dropping `&mut coutputs` from the call → signature mismatch, interop build fails
- reverting the `CompilerOutputs::new()` move → `coutputs` doesn't exist at the call site yet

And the only way to make it compile today is to have `rust_package_stores` stop minting prototypes — which is exactly the synthesizer work. Until that exists the store comes back empty, which regresses `calls_a_rust_free_function` (574/175/8 → 573/176/8) and the driver's assertions along with it.

So the options are:

**(a) Synthesizer first, then both core edits landed verified in one go** — same shape as last time, everything stays green throughout. My recommendation.

**(b) Land the core edits now and accept a red interop config** until the synthesizer lands — one test goes from pass to fail, the driver stops resolving. Defensible if you want the core diff reviewed and owned before I build on top of it, which I suspect is why you asked.

**(c) Core edits plus a deliberately-stubbed `rust_package_stores` returning empty**, with a TODO — compiles, but same test regression as (b), just with a tidier intermediate state.

I'd take (a), but I flagged rather than picked because (b) knowingly turns a passing test red, and that's your call not mine. Say which and I'll do it immediately — if it's (a) I'll bring you the core diff at the end exactly as before, so you still get to review it as a standalone hunk.

---

## 👤 User

just do the edits and dont build or test, just do the edits please

---

## 🤖 Claude

Understood — your call, proceeding. Let me read the region so I put `CompilerOutputs::new()` back in the right place.

---

## 🤖 Claude

Both edits:

---

## 🤖 Claude

Both core edits are in, no build or test run.

**`compiler.rs`, two hunks:**

1. `CompilerOutputs::new()` moved back down — it now sits immediately after the `GlobalEnvironmentT` construction, where it was before the package store forced it up.
2. `rust_package_stores(self)` — the `&mut coutputs` argument dropped, with a comment recording *why* it's no longer needed: the entries are declarations rather than resolved prototypes, so instantiation bounds belong to instantiations and `make_extern_function` registers them when one is actually minted.

The rest of `evaluate` is untouched — `import_rust_types(self, global_env, &mut coutputs)` still follows the new `coutputs` binding, and `compile_static_sized_array` / `compile_runtime_sized_array` are unchanged below it.

State to be aware of, since you asked me not to verify: the tree won't compile under `--features rust_interop` until `rust_package_stores` drops its `coutputs` parameter and stops calling `make_prototype_entry` — that's the synthesizer work on my side. The default (non-interop) build is unaffected by both hunks, since they're inside the `#[cfg]` and a plain relocation.

Ready to start on the synthesizer whenever you want it.

---

## 👤 User

lgtm, proceed

---

## 🤖 Claude

Starting on the synthesizer. Reading the current importer in full first.

---

## 🤖 Claude

Good — `int` is in builtins as a `Kind` templata directly, so no `CallSR` is needed for primitives. Let me confirm what the postparser attaches for a source-level `extern func`.

---

## 🤖 Claude

Now the oracle trait — I need a stable per-item id for the unique `CodeLocationS`.

---

## 🤖 Claude

No such assert in our tree — that was a Scala-tree finding. Writing the synthesizer.

---

## 🤖 Claude

Now rewriting `rust_package_stores` to emit declarations and drop `coutputs`.

---

## 🤖 Claude

Now the module wiring and imports, then I'll build.

---

## 🤖 Claude

Compiles. Checking warnings, then the suites.

---

## 🤖 Claude

One regression as expected. Let me see the actual failure.

---

## 🤖 Claude

That's the milestone working — the "untested combination" flagged in the plan. Look at what it resolved to:

```
Function(FunctionNameT { template: FunctionTemplateNameT { human_name: "add_two_numbers", ... },
         template_args: [], parameters: [Int(IntT { bits: 32 }), Int(IntT { bits: 32 })] })
```

A synthesized `FunctionS` went into the store, the function-compile phase picked it up, the solver resolved its `Lookup` rules to `int`, and `add_two_numbers(3, 4)` resolved against it. The only thing failing is the *assertion*, which still expects the old design's shape.

---

## 🤖 Claude

574/175/8 — back to baseline. Checking the default config and the driver.

---

## 🤖 Claude

My panic fired exactly as designed, and it's the right one: the fixture's `make_counter() -> Counter` needs a Rust *struct* named in a rule, which is the qualified-name work we deliberately deferred. Making that a skip rather than a hard stop:

---

## 🤖 Claude

Predicted consequence: `make_counter` is now skipped, so the driver's `(make_counter()).get()` program can't resolve. Let me point the driver at the milestone this step actually proves.

---

## 🤖 Claude

Driver passes. Fixing the stale message and the warning.

---

## 🤖 Claude

The synthesized-declaration path works end to end.

## What landed

**`declarations.rs`** (new) — `synthesize_extern_function` builds a `FunctionS` with `IBodyS::ExternBody`, carrying the same `IFunctionAttributeS::Extern` the postparser attaches for a hand-written `extern func`. Each param and the return gets a rune bound by an ordinary `LookupSR`; primitives resolve straight to a kind from the builtins store, so **no `CallSR` is needed** — that answers the open question from the plan, at least for non-generic primitives.

**`importer.rs`** — `rust_package_stores` now emits `IEnvEntryT::Function(&FunctionS)` and no longer takes `&mut CompilerOutputs`, matching the core edits you approved.

## What it proves

```
[compile=stub] importable_functions -> [("add_two_numbers", RustItemId(0)), ("make_counter", RustItemId(2))]
[compile=stub] fn_sig(RustItemId(0), args []) -> params [Int(32), Int(32)] ret Int(32)
OK: a synthesized Vale declaration for a Rust function, compiled by the ordinary
    machinery, resolved a call against a real TyCtxt
```

The untested combination — a *synthesized* `FunctionS` carrying `ExternBody`, where every in-tree macro uses `GeneratedBody` and every `ExternBody` came from parsed source — works. And the injection point needed no new hook: the function-compile phase already walks the top-level stores, so the declaration was picked up, solved, and run through `make_extern_function` by the ordinary path.

The fixture test's failure was itself the proof. It resolved to `Function(FunctionNameT { human_name: "add_two_numbers", parameters: [Int(32), Int(32)] })` while still asserting the old `ExternFunction` shape — i.e. the callee is now the extern *wrapper*, an ordinary Vale function, with the `ExternFunctionNameT` prototype one level down as the `ExternFunctionCallTE` target. I updated the assertion to the new shape.

**Suites:** 574/175/8 interop, 573/175/8 default — both at baseline. 7 pre-existing warnings, no new ones.

## One thing you should weigh

**The `Counter` method milestone is parked.** `make_counter() -> Counter` is now skipped at synthesis, because a declaration names its types the way source does and a Rust-backed citizen has no Vale source-level name yet — `import_rust_types` registers it in a per-type env keyed by template id, not in any ambient namespace. So there's nothing a `LookupSR` could resolve to.

I made it a **skip** rather than a panic, so an unnameable signature makes the function un-importable instead of importable-with-a-wrong-type. The visible cost is that calling it reports "couldn't find function" for a function that exists.

I didn't invent a bare `CodeName("Counter")` to keep it green — that's precisely the unqualified-name design we agreed not to build, and it wouldn't have worked anyway without also putting the type in the ambient store, which is the collision-exposing step.

So `(make_counter()).get()` worked under the old prototype design and doesn't under this one. That's the sequencing we chose — but it's a working milestone going dark, so I'd rather you know than discover it. The `Counter` fixture and `import_rust_types` are untouched and waiting for the struct path.

Nothing committed.

---

## 👤 User

get Harmonious's opinion on all the things?

---

## 🤖 Claude

Sent. Watcher's armed, so their reply will wake me.

**What I reported:** the pivot landed and works, and each of their four answers changed what got built —

- **Identity** — I found our oracle doing their exact `Box` bug in three places, including a linear scan matching a method's *owner type by human-name string*. The twist worth telling them: our own arch doc already carries `@ATAFLBZ` as a named invariant with their `Box` example in it, so the implementation was violating a rule the design already stated. Plus the silent-merge hazard is now closed with a distinct location per item.
- **Namespacing** — their "clean namespace at one layer buys nothing beneath" amendment is why I'm treating synthetic names and provenance filtering as complementary.
- **Empty definitions** — confirmed load-bearing here, and their debuginfo-walker warning was already designed out at arch §10.4.5, citing them.
- **Drop** — the `drop_in_place` wrapper was already our architecture; what was wrong was a *todo doc* proposing the by-value shape they reverted in a day. Marked superseded with their reasoning attached.

**What I asked them:**

- Whether "put a declaration in a scope and the compiler compiles it" is a general shape or an implementation detail we're now leaning on — that came free and I'd budgeted for a new hook.
- The role change for `fn_sig`: the oracle is now consulted once per item at *synthesis* time and must lower a **generic** rustc signature structurally into our type variables, with our solver doing the instantiating — no per-instantiation query at all. Whether Sky lowers generics structurally or only ever asks at a concrete instantiation, and what bites once the signature stops being concrete (`ty::Param`, early vs late bound).
- Passed along the rustc findings in case any are new to them — particularly that clippy returns `Vec<DefId>` rather than `Option` because `memchr::memchr` resolves to two crate versions, which makes our oracle's signature already wrong by that standard.

**And one self-critique in their genre.** The skip for unnameable types silently narrows the import surface: our log shows the function was *offered* and never shows it was *dropped*. A user gets "couldn't find function" for a function that plainly exists, with nothing connecting the error to the reason. Not a vacuous pass — the compile fails loudly — but the same family: a decision made silently, discoverable only by knowing to look. I asked whether they hit a "can't represent this, so it silently isn't importable" case and what they did beyond logging, since a log only helps someone who already suspects.

Tree state unchanged: 574/175/8 interop, 573/175/8 default, driver green, nothing committed.

---

## 🤖 Claude

Reply's in, and it improves on my self-critique rather than just endorsing it.

## The headline: poison, don't drop

My instinct was to make the skip an *observable* via the log. Their correction: a log only helps someone who already suspects — **put the explanation in the diagnostic the user already sees.**

Concretely, don't drop the declaration. Register it with the reason attached, and let ordinary lookup find it. Then the use site says:

> found `make_counter`, but it can't be imported yet: its return type `Counter` has no nameable form

instead of "couldn't find function to call." One field on the declaration rather than a new observability channel, and it turns a silent narrowing into something impossible to miss. Their `@RTMEIZ` precedent does exactly this — an un-imported Rust type doesn't produce a mystery lookup failure, it produces *"Rust type `Stdout` is not imported; add `import rust.std.io.Stdout` to your source."* The constraint stayed; the constraint got a voice.

That's a cheap fix to the thing I flagged as my own weak point, and I'd like to do it.

## The scope-iteration property — build it, don't bank it

They had the same property and **had to construct it**. Their field accessors started with a special-case discovery branch, dedicated codegen, and its own symbol handling; they deleted all of it in favour of synthesizing ordinary functions injected right after parsing. Recorded payoff: "one fewer special case across the discovery, codegen, symbol-mangling, and serialization paths."

Their generalization is worth having: **synthesized is the degenerate case of parsed** — the same family as our arch doc's §1.5.5 "non-generic is the degenerate case of generic" and §1.7's "drop is just a function." Third costume, same principle.

Their practical advice: since we're now depending on the phase iterating scopes rather than the parsed denizen list, write a test that synthesizes a declaration into a scope and asserts it compiles — so a future "optimization" produces a named failure instead of a mystery.

## Structural generic lowering — they migrated *toward* it, and their stated reason for not doing it was false

They had six early-returns bailing out on type parameters, plus a conversion that panicked on the type-param arm with a comment asserting it was an architectural invariant. All deleted. The sentence worth keeping:

> The shortcut was implementation laziness … not an architectural constraint — `tcx.fn_sig(...).instantiate(...)` accepts Param-bearing args cleanly because Params are normalization terminals.

Hazards, ranked:

1. **`ty::Param` carries a positional index you must supply.** Wrong list, or right list wrong order, gives a well-formed `Param` pointing at the wrong slot — the @EarlyBinder ordering concern in structural clothing, and *worse*, because there's no concrete type to look wrong. Their advice: the `pick<A, B>` canary now tests index mapping, not substitution.
2. **Arity** — their arg builder silently discards excess type args. Structural lowering constructs arg lists far more often, so an off-by-one becomes silent.
3. **The shape trap** — their worst substitution bug produced a parser-shaped type where a resolver-shaped one was required; both representable, both plausible, and the wrong one yielded *stack garbage rather than a panic*. They eventually split the enum in two with a single promotion chokepoint. Design that boundary in first.
4. **Early vs late bound: they have nothing** — they erase all lifetimes at the boundary and have never confronted it. Flagged as a genuine gap rather than guessed at.

And they confirm `instantiate_identity()` becomes correct under the new design, but would keep the comment-on-every-call discipline — it stops being a warning and becomes documentation of intent, worth *more* now that the call is common.

## They conceded two things

Their own resolver is `Option<DefId>` first-match-wins, and clippy's `Vec<DefId>` is the correct shape — `memchr::memchr` resolving to two crate versions is a case their design can't represent. Their words: they'd been telling us the bare-name resolver "held up," and the accurate version is "held up on fixtures small enough never to contain two crate versions."

And our re-export finding reframes their own sidestep as better-justified than they'd claimed: since `std::vec::Vec` names no definition and there *is* no canonical path, keying on the import table is less a shortcut than the only well-defined key available.

## The one I'd act on beyond the poison fix

On the doc-vs-implementation gap:

> A named invariant in a doc does not enforce itself. The only invariants of ours that have actually held are the ones with a mechanical fence.

They run a grep-based CI test that fails on a forbidden pattern unless each occurrence carries an explicit `arch-fence-allow:` marker naming which sanctioned exception applies. Crude, and it works precisely because it doesn't rely on memory. We have three named sites for the bare-identifier match — exactly the input a fence wants. Their argument: *"the value isn't fixing those three — it's the fourth one, in eight months, by someone who never read the doc."*

---

Three candidate next moves, in what I'd rank as descending value: **the poison-instead-of-skip diagnostic** (fixes something I just built badly), **the scope-iteration regression test** (cheap, protects a property we now depend on), and **the `@ATAFLBZ` fence** (protects against the fourth site). Want any or all of them?

---

## 🤖 Claude

Watcher re-armed as **Vale4-opal**, resident with an empty queue.

Standing state: the synthesized-declaration path is green — 574/175/8 interop, 573/175/8 default, driver exit 0, 7 pre-existing warnings, nothing committed. Docs reflect the pivot (plan doc rewritten, arch §1.7 visibility bullet, `opaque-extern-drop.md` superseded banner, frontend-plan and callout-map banners, README reordered).

Open for your call — the three follow-ups from Harmonious, ranked:

1. **Poison instead of skip** — register the unnameable declaration with its reason so the use site says *"found `make_counter`, but its return type `Counter` has no nameable form yet"* rather than "couldn't find function to call." Fixes the weak point I flagged in my own work.
2. **Scope-iteration regression test** — pins the property that placing a declaration in a scope is sufficient to get it compiled, which we now depend on.
3. **`@ATAFLBZ` fence** — grep-based check over the three bare-identifier match sites, requiring an explicit allow-marker, so the fourth site in eight months fails loudly.

And still parked by design: the `Counter` method milestone, waiting on the struct path.

---

## 👤 User

<command-message>fire-commit</command-message>
<command-name>/fire-commit</command-name>
<command-args>with experimental</command-args>

---

## 🤖 Claude

Reading the config first, as the skill requires.

---

## 🤖 Claude

Config: rebase-and-fast-forward, `experimental` family, target `experimental`. Starting pre-flight. 0a — external repos.

---

## 🤖 Claude

Luz has an untracked curate queue — the config says drain it before committing. Let me see what's in it.

---

## 🤖 Claude

That's a Guardian pipeline crash (missing companion binary), triggered by one of my own `cd &&` commands — not a triage-able code change. Noting it and continuing pre-flight.

---

## 🤖 Claude

Luz is 1 behind origin. Bringing it current.

---

## 🤖 Claude

Decisive — upstream already ratified the resolution for this exact entry:

> `111fe9f` — *"Drop the MBXX curate-queue entry — it was a companion-program crash record, not a shield case, and had nothing to route."*

Our `002.*` is the identical shape. Draining it the same way (I've read and quoted all four files above; two are zero-length, and the crash is reproducible).

---

## 🤖 Claude

convo-8.md is the previous session's transcript — belongs with this arc. Staging precise paths, leaving the two items that aren't mine.

---

## 🤖 Claude

Clean on symlinks, test delta, and `#[ignore]`. Now the blocking gate — 0g, the config's test command, unfiltered.

---

## 🤖 Claude

Pre-flight complete. **Stopping at 0g — the gate fails, and it's the same structural condition as last time.**

## The blocker

The config's test command starts with `cargo build --manifest-path FrontendRust/Cargo.toml`, which doesn't compile:

```
error[E0432]: unresolved import `frontend_rust::backend_ffi`   src/bin/valec/midas.rs:7
error[E0433]: could not find `pass_manager` in `pass_manager`  src/bin/valec/build.rs:252
                                                               src/bin/valec/frontend.rs ×6
```

Every failure is in `src/bin/valec/`, and **none of those files are staged** — `backend_ffi` and `pass_manager` are intentionally commented out of `lib.rs` by the onion arc. `cargo nextest run` can't build its targets either, so neither backend gate can run at all. This isn't a red suite; it's a suite that can't start.

Against the bar you set for this exact condition last time — *"typing-pass should build, and some typing pass tests should pass"*:

| | result |
|---|---|
| `cargo build --lib` | **exit 0**, 8 warnings (7 pre-existing lib + 1 driver) |
| `cargo test --lib` | **573 passed** / 175 failed / 8 ignored — unchanged all session |
| `cargo test --lib --features rust_interop` | **574 passed** / 175 / 8 |
| `valec-rs` driver | exit 0 |

## Everything else is clean

| check | result |
|---|---|
| 0a Luz | was 1 behind → **fast-forwarded to current**, 0/0. Curate queue **drained** |
| 0a Guardian | clean, 0 ahead / 0 behind, all 5 submodules at pinned refs |
| 0b `DO NOT SUBMIT` | no real hits (all are docs *describing* the sweep) |
| 0c absolute paths | 2 files — see below |
| 0d broken symlinks | none |
| 0e temp-disable | no real hits (docs only) |
| 0f test delta | 0 added / 0 deleted / 0 signature-modified |
| new `#[ignore]` | none |

**The Luz queue drain is worth reporting.** It held an untracked `002.*` case — a Guardian *crash record* (the MBXX companion binary doesn't exist, so the shield couldn't spawn it), triggered by one of my own `cd &&` commands. Upstream had already ratified the resolution for the identical entry in `111fe9f`: *"Drop the MBXX curate-queue entry — it was a companion-program crash record, not a shield case, and had nothing to route."* I applied the same resolution, having read and quoted all four files first (two were zero-length). Luz is now clean but for untracked `tmp/` scratch.

**0c hits, both judgment calls I'd rather flag than silently "fix":** 7 in `convo-8.md`, all inside a verbatim transcript where rewriting would falsify the record; 1 in `synthesized-declarations-plan.md` — the §2 table naming the sibling worktrees `/Volumes/V/ValeRustInterop` and `/Volumes/V/RustInteropReiImpl`. Those are other checkouts on this machine with no relative form, same category as the README cross-repo references flagged last time.

**One thing the config's test-delta grep structurally misses:** I modified `calls_a_rust_free_function`'s *assertions* (not its signature), so the signature-based detector reports zero. The why: it asserted the callee's name was `ExternFunction`, which was the old prototype-store design's shape. Under synthesized declarations the callee is the extern *wrapper* — an ordinary `Function` — with the `ExternFunctionNameT` prototype one level down as the `ExternFunctionCallTE` target. The assertion now checks that, and still checks the params ride the name. Strictly the same coverage, updated to the correct shape.

## What I need from you

The gate can't pass on this branch regardless of my work. Say the word and I'll proceed under the bar you set — 19 files staged, target `experimental`, external repos already synced. Or if you'd rather, `fire commit temporary with experimental` is the variant that explicitly doesn't require a green gate.

---

## 👤 User

yeah, typing-pass should build, and some typing pass tests should pass

---

## 🤖 Claude

Bar confirmed. Proceeding.

---

## 🤖 Claude

Another TL advanced `experimental` with a docs commit — expected. Committing first.

---

## 🤖 Claude

Committed. Now step 4 — rebase onto `experimental`, which moved.

---

## 🤖 Claude

Blocked by the unstaged `CLAUDE.md` (not my change). Stashing it across the rebase rather than committing someone else's work.

---

## 🤖 Claude

Rebase clean. Restoring the parked file, then re-verifying after the incoming commit — as a separate step, per the skill.

---

## 🤖 Claude

Re-verified post-rebase, both unchanged. Now the ratchet.

---

## 🤖 Claude

`fire commit with experimental` is complete.

## Landed

**`8d40eff9d`** on `experimental-4`, ratcheted to `experimental` (`0692e2e0e..8d40eff9d`). Divergence 0/0 both ways. Per the config's experimental-family model the local ratchet *is* the sync — nothing pushed.

18 files, +5,701/−34: `declarations.rs` and the synthesizer, `rust_package_stores` emitting declarations instead of prototypes, the two `compiler.rs` hunks, `overload_resolver.rs` down to zero interop content, the driver and test assertions updated to the wrapper shape, and the doc set (new plan doc, arch §1.7 visibility principle, superseded banners, corrected extern-drop todo, prior session's transcript).

## Against your bar, verified twice

Once at pre-flight and again after the rebase, since `0692e2e0e` came in from another TL:

| | result |
|---|---|
| `cargo build --lib` | exit 0 |
| `cargo test --lib` | **573 passed** / 175 / 8 |
| `cargo test --lib --features rust_interop` | **574 passed** / 175 / 8 |

`cargo build` still can't compile `src/bin/valec` (references `backend_ffi`/`pass_manager`, intentionally out of `lib.rs`), so neither nextest backend can run. Unchanged by this commit; none of those files were touched.

## Externals

Both clean and current. **Luz was 1 behind and is now fast-forwarded** to `111fe9f`; Guardian 0/0 with all five submodules at pinned refs. Neither had anything to commit.

The curate queue was drained rather than committed: the untracked `002.*` entry was a Guardian *crash record* (missing MBXX companion binary, triggered by one of my own `cd &&` commands), and upstream had already ratified deleting the identical entry in `111fe9f` — *"a companion-program crash record, not a shield case, and had nothing to route."* I read all four files before removing them.

## Two leftovers, both deliberately not mine

```
 M CLAUDE.md              ← manifest-sync regen, predates this session
?? .github/ci-guide.md    ← CI doc from a prior session
```

`CLAUDE.md` blocked the rebase, so I parked it in a stash and restored it afterward rather than folding someone else's work into an interop commit. Both are unchanged and waiting for whoever owns them. `Luz/tmp/` is untracked scratch, as it's been all session.

⚠️ **Luz changed on disk** (fast-forwarded to `111fe9f`) — restart this directory's Guardian to pick it up.

---

## 👤 User

whats next?

---

## 🤖 Claude

Three tiers, and I have a clear recommendation.

## What I'd do next: generic externs

`fn pick<A, B>(a: A, b: B) -> A` instantiated at `<int, bool>`.

That's the reason we pivoted. Right now we've proven the *mechanism* — a synthesized declaration compiles through the ordinary path — but not the thing it was built for. The old design was abandoned specifically because a finished prototype can't represent a generic function, and we haven't yet demonstrated that the new one can.

It's also the only major milestone **not gated on deferred work**: no struct, so no qualified names, no `lookup_struct_template`, no extern drop. Both `int` and `bool` are nameable today.

And it's where Harmonious's hazards land, so we'd be testing the risky part rather than the easy part:

- `ty::Param` carries a **positional index** we have to supply from the surrounding item's generic list. Wrong list or wrong order gives a well-formed param pointing at the wrong slot — the @EarlyBinder ordering concern in structural clothing, and worse, because there's no concrete type to look wrong. `pick<A, B>` is exactly the canary that catches an index swap: at `<int, bool>` a wrong order yields `bool` where `int` belongs.
- It's the first real use of `instantiate_identity()` in its now-correct role.
- It exercises `make_extern_function` reading solved `template_args` off `env.id`, which I've asserted works and never verified.

## Cheap hygiene worth folding in

Small, and each protects something we now depend on:

- **Poison instead of skip** — Harmonious's correction to my own weak point. Register the unnameable declaration with its reason so the use site says *"found `make_counter`, but its return type `Counter` has no nameable form yet"* instead of "couldn't find function to call." One field, not a new channel.
- **Scope-iteration regression test** — pins that placing a declaration in a scope is sufficient to get it compiled. We're depending on it; if someone later "optimizes" that phase to walk the parsed denizen list, we get a named failure instead of a mystery.
- **`@ATAFLBZ` fence** — a grep check over the three bare-identifier match sites requiring an explicit allow-marker. The value isn't those three, it's the fourth one in eight months.

## The bigger arc, for context

The **struct path** (unparking `Counter`) is gated on *two* prerequisites, which is why I'm not proposing it first: extern drop (§8 — much cheaper now that `__vale_drop<T>` turns out to be our own architecture, but still core work in `struct_drop_macro`), and enough of the naming design that a Rust citizen can be named in a rule.

Beyond that: real `import rust.X.Y` driving the allowlist, then the walker.

My call — **generics next, with the poison fix alongside** since it's a handful of lines and closes something I flagged in my own work. Want me to start there, or would you rather unpark `Counter` and take the two prerequisites head-on?

---

## 👤 User

is there anything we can do that doesnt require core changes?

---

## 🤖 Claude

Good question — let me check the one that matters, because I think the answer is better than expected.

---

## 🤖 Claude

Yes — and it's the thing I already recommended, which makes the choice easy.

## Generic externs need **zero** core changes

I checked the three things that could have forced one:

- **The oracle's signature type is fully contained.** Only `oracles.rs` and the test harness name `RustOracle` anywhere outside `rust_interop/`, and both only hold it as a trait object — neither cares about method signatures. So reshaping `ValeSig`/`fn_sig` so a signature can say "this is type-param #0" rather than a concrete kind is entirely ours.
- **`GenericParameterS` is constructible.** All-public fields, and the typing pass already builds them from scratch (`anonymous_interface_macro.rs:398`, `:677`). No new API.
- **`make_extern_function` is already written for generic externs.** It reads `template_args` off the *solved* `env.id`, and the @PRIIROZ comment sitting in it is specifically about generic externs inheriting container params. Core is ready and waiting.

So the whole milestone is: oracle lowers a generic rustc signature structurally, synthesizer emits `generic_params` plus rules referring to them, fixture gains `fn pick<A, B>(a: A, b: B) -> A`. All inside `rust_interop/` and the fixture `.rs` files.

## Also zero-core

- **Fix the three name-matching sites** in `tyctxt_oracle.rs` — key on `DefId`, add provenance filtering. This is the `@ATAFLBZ` defect, and it's pure oracle code.
- **Reshape the oracle's returns to `Vec<DefId>`-style** — clippy's lesson about `memchr::memchr` resolving to two crate versions.
- **The scope-iteration regression test** — lives in the interop test tree.
- **The `@ATAFLBZ` fence** — a grep script plus CI wiring, not compiler code.
- **Warm-cache observability** — driver and logging oracle only. Harmonious's standing prediction, still unaddressed.

## A correction I owe you

I described the **poison-instead-of-skip** fix as "one field on the declaration rather than a new observability channel." That field would be on `FunctionS`, which is core — or it'd need a new error variant, also core. So it is *not* a zero-core item, and I mis-sold it. It's still worth doing; it just doesn't belong on this list.

Everything else on the bigger arc — the struct path (extern drop in `struct_drop_macro`), Vale's import binding, the precedence fix — is core by nature.

## Recommendation

**Generics, and the oracle name-matching fix alongside it.** The first is the milestone the whole pivot exists to enable and it's unblocked; the second is a known defect in the exact file I'd be editing anyway, so fixing it first means building the generic path on `DefId` identity rather than on string matching I'd have to unpick later.

Want me to start?

---

## 👤 User

yep start

---

## 👤 User

yep start. but start by scoping it out, and then send some questions to get Harmonious's advice, and then continue on. let me know if any core changes are needed, so i can do them manually

---

## 🤖 Claude

Scoping first. Let me look at what a generic function's declaration actually contains.

---

## 🤖 Claude

Scope is clear. Sending questions before I build.

---

## 🤖 Claude

Sent. Continuing while that's in flight — starting with the oracle's signature shape.

---

## 🤖 Claude

Now the `TyCtxtOracle` implementation — the part with the index hazard.

---

## 🤖 Claude

Now `lower_sig_ty` — this is where the index hazard lives, so I'll make the parent-offset explicit.

---

## 🤖 Claude

Their reply landed mid-edit and corrects exactly what I just wrote. Applying it.

---

## 🤖 Claude

Now `TyCtxtOracle` needs to hold the scout arena for interning param names.

---

## 🤖 Claude

Expected — the other two impls and the call sites. Fixing them.

---

## 👤 User

[Request interrupted by user for tool use]

---

## 👤 User

please edit normally

---

## 🤖 Claude

Understood — switching to the Edit tool.

---

## 🤖 Claude

Now the synthesizer — emitting generic params and referencing them directly.

---

## 👤 User

[Request interrupted by user for tool use]

---

## 👤 User

also i forgot to say, we should be doing this with good /tdd

---

## 🤖 Claude

Right — I jumped straight to implementation without a failing test. Backing up.

## Where I am

Partial implementation across 5 files, currently **not compiling** (I was mid-edit when you stopped me). Nothing committed since `8d40eff9d`.

## Baseline caveat, per the skill's step 0

The suite is **not** green: 573 passed / 175 failed / 8 ignored. The skill says stop and surface rather than reason past it — so, surfacing. Those 175 are the onion arc's known state, which you ratified twice today as the commit bar ("some typing pass tests should pass"). I'll treat **573/175/8 default and 574/175/8 interop as the fixed baseline**, and any movement in those numbers — either direction — as a stop rather than a footnote.

## Proposal

Stash the partial implementation so the first test can be seen failing for the *right* reason. On the pre-edit tree a generic import hits `panic!("cannot lower generic Rust function ... it has N generic parameter(s)")`, which is a genuine RED. Right now it'd fail with compile errors, which proves nothing.

## RFIGA

**1. A generic Rust free function is importable and callable at a concrete instantiation.**
- **R**: fixture gains `pub fn pick<A, B>(a: A, b: B) -> A`; driver's Vale program becomes `exported func main() int { return pick<int, bool>(3, true); }`; assert it typechecks and the log shows the signature came back with its parameters intact.
- **F**: run the driver; expect the "cannot lower generic Rust function" panic.
- **I**: `ValeSigType`, structural lowering keyed on param *name*, generic-param emission in the synthesizer.
- **G**: re-run the driver.
- **A**: both suites at baseline.

The instantiation is deliberately `<int, bool>` with return type `A`. If the index mapping is swapped you get `bool` where `int` belongs and `main() int` won't typecheck — Harmonious's point that this tests the *mapping*, not merely that substitution happened. `id<T>` would have passed either way.

**2. A signature Vale cannot represent is dropped, not imported.**
- **R**: fixture gains `pub fn first<I: Iterator>(i: I) -> I::Item`; assert it does **not** appear as importable, and that a Vale program calling it fails to resolve.
- **F**: run; expect it to be imported (or to panic) rather than cleanly dropped.
- **I**: the `TyKind::Alias` arm returning `None`, propagated so the whole declaration drops.
- **G**: re-run.
- **A**: both suites.

Slice 2 exists because Harmonious flagged projections as the one case where dropping bounds isn't merely lossy — `I::Item` needs the `I: Iterator` predicate to normalize at all, so it's un-resolvable rather than unbounded. They'd never run it themselves (their source language can't name an associated type), so this is ours to verify.

## Two questions before I start

**Which behaviors matter most to you** — is slice 2 worth doing now, or should I do slice 1 and treat unrepresentable-signature handling as its own later piece?

And **do you want the generic test in the driver, the `#[test]` fixture suite, or both?** The driver exercises real rustc (which is where the index mapping actually lives); the fixture oracle can't produce a `ty::Param` at all, so a fixture-side test would only cover the synthesizer half.

---

## 🤖 Claude

Watcher re-armed as **Vale4-opal**, queue empty.

Waiting on your call for the two questions above before I start the RED cycle:

1. **Scope** — slice 1 only (generic function importable and callable), or slices 1 and 2 (plus dropping un-representable signatures)?
2. **Where the test lives** — driver, `#[test]` fixture suite, or both? The index mapping only exists against real rustc, so the driver is where slice 1 has teeth; the fixture oracle can't produce a `ty::Param` at all.

Current state, so it's not a surprise later: partial implementation across 5 files, **not compiling**, nothing committed since `8d40eff9d`. My plan is to stash it so the first test fails against the old `panic!("cannot lower generic Rust function …")` — a real RED — rather than against compile errors, which would prove nothing. Say the word and I'll do that; if you'd rather I keep the work in place and write the test around it, that's a weaker RED and I'd want you to have chosen it deliberately.

---

## 👤 User

1. both 1 and 2 please
2. your call, just somewhere in a rust_interop folder

---

## 🤖 Claude

Both slices, driver-side. Starting with step 0 — stashing the partial implementation and confirming the baseline.

---

## 🤖 Claude

Stashed cleanly. Confirming the baseline before writing anything.

---

## 🤖 Claude

Baseline confirmed at 573/175/8 and 574/175/8. **Slice 1, RED** — the fixture and the Vale program first.

---

## 🤖 Claude

Now the driver's program and assertions — the actual test.

---

## 🤖 Claude

Now **F** — run it and confirm it fails for the right reason.

---

## 🤖 Claude

**Tests are correctly failing, proceeding with implementation.**

The RED is exactly the right shape: `cannot lower generic Rust function ... it has 2 generic parameter(s)` — the old design refusing at precisely the point the new one has to work. Restoring the parked implementation.

---

## 🤖 Claude

`seam.rs` is entirely dead — neither export has a caller since the pivot orphaned it. Fixing the two live call sites first.

---

## 🤖 Claude

Now removing the orphaned `seam.rs` rather than maintaining a dead call site.

---

## 🤖 Claude

Compiles. **G** — running the driver.

---

## 🤖 Claude

Lifetime collapse from storing the arena. Better fix: intern the names at construction, where the arena is already in hand.

---

## 🤖 Claude

**A normal one — stock `nightly-2025-12-09` from rustup, no fork.**

`rust-toolchain.toml` pins `channel = "nightly-2025-12-09"`, and the active sysroot is `~/.rustup/toolchains/nightly-2025-12-09-aarch64-apple-darwin`. We link rustc's internals via `rustc-dev` + `#![feature(rustc_private)]`, but that's just linking the stock nightly's own crates.

Worth separating two things, since they get conflated:

- **`~/rust`** (where I sent the resolver agent) *is* a fork checkout — it has a `fork-sysroot` directory — but that's Harmonious's tree. We read it as a source reference; we don't build against it.
- **Harmonious builds on `channel = "rustc-fork"`.** We don't, and the arch doc says we shouldn't need to yet: the **read** path — `TyCtxt` queries, `module_children`, `fn_sig`, `generics_of` — needs no fork. The four fork patches are all for the **write** side, when Vale's items have to appear in rustc's own monomorphization graph (`per_instance_mir`, `layout_of`, the partition filter, `fill_extra_modules`).

So the fork becomes necessary at codegen, not now. I asked Harmonious to confirm-or-contradict that from experience in an earlier round, since they're forked and we're not — and nothing they've said since has contradicted it.

One caveat I'd flag: `rustc --version` reports `1.94.0-nightly`, while the resolver agent surveyed `~/rust` at `1.95.0`. So the source I've been citing for `param_at`, `NameResolution`, and the `std::vec` re-export is **one version ahead** of what we compile against. Nothing I've used from it looks version-sensitive, but the `generics.rs` and `imports.rs` details are worth re-checking against 1.94 before anything depends on them structurally.

Continuing with the build.

---

## 🤖 Claude

Now the `own_generic_param_names` helper, and reverting the arena signature change.

---

## 🤖 Claude

Progress — the generic function passes; now `first` panics on `Option` (an un-imported ADT) *before* reaching the projection. My slice-2 fixture buried the alias inside `Option<…>`. Putting it in bare return position, which is what the test is actually about:

---

## 🤖 Claude

Both slices are working — look at the log:

```
fn_sig(RustItemId(3)) -> Some("generics ["A", "B"] params [Generic(0), Generic(1)] ret Generic(0)")
fn_sig(RustItemId(4)) -> None
```

The only failure is a **stale assertion of my own** — it looks for `ret Struct(StructTT`, but positions now render wrapped as `ret Kind(Struct(StructTT`.

---

## 🤖 Claude

The log double-escapes: `{rendered:?}` on an `Option<String>` escapes the inner quotes. Fixing the rendering rather than writing an assertion full of `\\\"`.

---

## 🤖 Claude

**GREEN.** Now **A** — the full suite, both configs.

---

## 🤖 Claude

Both slices are green, and the suites are exactly at baseline.

## What the driver now proves

```
fn_sig(RustItemId(3)) -> generics [StrI("A"), StrI("B")] params [Generic(0), Generic(1)] ret Generic(0)
fn_sig(RustItemId(4)) -> None
```

**Slice 1** — `fn pick<A, B>(a: A, _b: B) -> A` is read **structurally**, with its parameters intact rather than collapsed to one instantiation, and this Vale program typechecks against a real `TyCtxt`:

```vale
exported func main() int {
  return pick<int, bool>(add_two_numbers(3, 4), true);
}
```

That's the thing the whole pivot existed to make possible — the old design panicked here by construction. And it's the index mapping under test, not just substitution: returning `A` from a call at `<int, bool>` means a swapped binding yields `bool` where `int` belongs and fails to resolve. Per Harmonious, `id<T>` would have passed under either mapping.

**Slice 2** — `fn first<I: Iterator>(i: I) -> I::Item` is **declined**, not imported. The whole declaration drops rather than carrying an alias nothing can normalize.

## Three things the RED cycles caught that I'd have got wrong

- **My first slice-2 fixture tested the wrong thing.** I wrote `-> Option<I::Item>`, so the return type was an ordinary ADT and the un-imported-`Option` check fired first — it would have "passed" while proving nothing about projections. Moving the alias to bare return position fixed it, and the fixture now says why.
- **Harmonious's correction, applied mid-flight.** I'd hand-rolled `index - parent_count`; their reply said don't, and better still to key on the parameter *name*, which is safe because Rust forbids an item shadowing a parent impl's parameter name (E0403). That's what's implemented, and it sidesteps the offset arithmetic entirely.
- **The lifetime collapse** from storing the arena on the oracle (`&'s ScoutArena<'s>` forces the arena to outlive `'s`). Fixed properly by interning each item's parameter names at construction, where the arena is in hand — which is also simpler than doing it on demand.

## Two things I changed beyond the minimum — flagging both

**`seam.rs` is deleted.** After the pivot neither of its exports (`push_rust_call_candidates`, `maybe_rust_field`) had a caller anywhere; keeping it would have meant updating a dead call site for the new signature. Plan §11 already recorded it as removed by this design, and Harmonious's counsel on dead-but-constructible code was explicit — *"'never ported' isn't a guard; it's a plan to remember."* I left a comment in `mod.rs` saying what was there and why it's gone.

**The oracle log's `fn_sig` line is rendered by hand** instead of `{:?}`-ing an `Option<String>`, which was double-escaping every quote and would have forced assertions matching `\"A\"`. The log exists to be read; that was a defect in it.

## No core changes needed

Everything landed inside `rust_interop/` plus the fixture. `make_extern_function` handled the generic case unmodified — as expected, since the @PRIIROZ comment sitting in it was written for exactly this.

**Suites: 573/175/8 default, 574/175/8 interop — both at baseline.** 8 warnings, all pre-existing. Nothing committed.

Worth noting for later: `lower_ty` still **panics** on an un-imported ADT reached through a signature, where the new code path *declines*. That inconsistency is what my bad fixture stumbled into, and it's the same poison-vs-panic question Harmonious raised. Not in scope here, but it's a real edge I'd want to reconcile.

---

## 👤 User

hold for a while please, rearming watcher through the night

---

## 🤖 Claude

Holding. **Vale4-opal** is armed and resident (pid 57126) — I'll pick up any mail that arrives overnight and otherwise stay put.

State for whenever you're back:

- **Generics landed, both slices green.** `pick<A, B>` at `<int, bool>` typechecks against real rustc; `first<I: Iterator> -> I::Item` is declined rather than imported. Suites at baseline — 573/175/8 default, 574/175/8 interop, 8 pre-existing warnings.
- **Uncommitted**, on top of `8d40eff9d`: `ValeSigType` and the structural `fn_sig`, name-keyed param resolution, generic-param emission in the synthesizer, `seam.rs` deleted, the two fixture additions, the driver's program and assertions.
- **Two things I flagged for your judgement** rather than deciding alone: deleting `seam.rs`, and the hand-rendered `fn_sig` log line.
- **One loose end noted, not touched**: `lower_ty` still panics on an un-imported ADT reached through a signature, where the new path declines — same poison-vs-panic question Harmonious raised.

Sleep well.

---

## 🤖 Claude

Watcher re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 👤 User

qq: do we generate vale FunctionS wrappers for every method in a type, before they are asked for? or do we lazily make those?

---

## 🤖 Claude

Eagerly, and further than the question assumes — plus **methods don't get `FunctionS` wrappers at all yet.**

Four layers, all eager:

| layer | what it does | when |
|---|---|---|
| `TyCtxtOracle::new` | walks every allowed crate-root child, then every `inherent_impls` → `associated_items` method | oracle construction |
| `rust_package_stores` | synthesizes a `FunctionS` per importable **free function** | environment build |
| `import_rust_types` | `make_prototype_entry` per **method**, per type — a prototype, not a declaration | after global env |
| function-compile phase | `evaluate_generic_function_from_non_call` on **every** `IEnvEntryT::Function` in the top-level stores | `compiler.rs:1010-1035` |

That last one is the one I'd flag hardest: every synthesized declaration is not merely *built* eagerly, it's **compiled** eagerly, whether or not anything calls it — and `make_extern_function` runs at the end of that, registering an extern via `add_function_extern`. So an imported-but-unused function still becomes a registered extern the backend would emit a declaration for.

So the split today is: **free functions → eager `FunctionS` declarations; methods → eager prototypes.** Bringing methods onto the declaration path is the same move that's already done for free functions, and it's gated on a Rust-backed citizen being nameable at all — which is the parked qualified-name work.

**Whether eager is right is a live question.** The justification on record (in the oracle's `importable_types` doc comment) is @RTMEIZ: every Rust item must be explicitly imported, so the surface is finite and declared. That holds today with a 5-name allowlist. It gets thinner fast — `import rust.std.vec.Vec` brings ~100 inherent methods, and under the current shape we'd synthesize *and compile* a declaration for each.

Worth noting rustc itself goes the other way: the resolver survey found foreign modules are populated **on first touch** (`populate_on_access`), precisely so nothing enumerates `std`. That's the same instinct as the `VCOORD` note already sitting on `TyCtxtOracle::new` about retiring the up-front walk.

So: eager everywhere now, defensible while the import list is hand-written, and one of the first things that should become lazy once `import rust.X.Y` drives it. Not a problem today; a problem the moment the allowlist stops being three lines.

---

## 👤 User

"methods → eager prototypes" why prototypes?

---

## 🤖 Claude

Because it's the un-migrated remainder of the old design — not a considered choice. And the comment justifying it is now stale.

---

## 🤖 Claude

Three-layer answer, and the middle one is a defect I left behind.

**1. History.** Under the pre-pivot design *everything* was prototypes — the package store held `IEnvEntryT::Templata(ITemplataT::Prototype(..))` too. The pivot converted free functions to declarations and stopped there. Methods are simply the part that hasn't moved.

**2. The stated rationale is now refuted, and the comment still asserts it.** `importer.rs:12-15` says:

> *"A Rust method has no Vale AST behind it; a prototype is exactly what a call site points at."*

That premise is exactly what the pivot overturned. A Rust function *does* get a Vale AST now — a synthesized `FunctionS` — and free functions prove it. The comment reads as a considered design position, and it isn't one any more. That's my stale comment and it's actively misleading; worth fixing.

**3. The real blocker, which is not about ASTs.** A method declaration has to **name its receiver**. `struct_drop_macro`, the model for this, emits `LookupSR(template_rune, struct_a.name.get_imprecise_name(...))` — the citizen, by imprecise name. That works for a Vale struct because it lives in a store a lookup can reach.

A Rust citizen doesn't. `import_rust_types` registers it with `declare_type_outer_env`, which writes to `type_name_to_outer_env` — a **side table keyed by template id**, not any namespace a `LookupSR` resolves against. So a method declaration naming `Counter` would fail to resolve.

Which is the *same* gap that makes `make_counter` un-importable: a Rust-backed citizen has no Vale source-level name. Prototypes sidestep it because a prototype carries the receiver as a `KindT` directly — by identity, never by name.

So the honest summary: **methods are prototypes because they're the leg that hasn't been walked, and walking it is gated on exactly one thing — a Rust citizen becoming nameable.** Once that lands, the method path is the same move already made for free functions, and `make_prototype_entry` disappears entirely.

Want me to fix that stale comment now? It's a few lines, and it's the sort of thing that quietly teaches the next reader a refuted design.

---

## 👤 User

rust not naming its receiver isn't an obstacle

---

## 🤖 Claude

Let me check that rather than defend it — you may be right and I think I see how.

---

## 🤖 Claude

You're right, and I can see the mechanism now — `environment.rs:950+`:

```rust
pub fn lookup_with_imprecise_name_inner(&'t self, name, lookup_filter, get_only_nearest, interner) {
    let result = self.templatas.lookup_with_imprecise_name_inner(
        IEnvironmentT::Citizen(self), name, lookup_filter, interner);
    if !result.is_empty() && get_only_nearest { result } else { /* walk parent */ }
}
```

A `CitizenEnvironmentT` checks **its own store first**, and — unlike `PackageEnvironmentT` — it actually honours `get_only_nearest`, so a local hit short-circuits and never reaches the ambient namespaces.

So the receiver only has to be nameable **from inside the method's own environment**, not globally. We build that store. Putting the citizen into it as `IEnvEntryT::Templata(ITemplataT::Kind(KindTemplataT { kind: struct_kind }))` makes `LookupSR(CodeName("Counter"))` resolve locally — no global entry, no collision exposure, no qualified-name work. And for a non-generic Rust struct it needs no `CallSR` either: it resolves straight to a kind, exactly like `int` in the builtins store.

**The obstacle was self-inflicted.** `precompile_struct` gets this for free — its `sibling_entries` are every top-level entry of the declaring package, which includes the struct itself. We set `sibling_entries` empty, correctly, because Rust has no UFCS — and in doing so removed the thing that made the receiver nameable, without noticing that was a second consequence.

What it actually changes, splitting a case I'd wrongly collapsed:

- **Methods** — unblocked. One store entry, and the receiver resolves. The qualified-name work isn't a prerequisite after all.
- **Free functions returning a Rust type** (`make_counter() -> Counter`) — still blocked, and genuinely. That declaration lives in the *package* store, so its `LookupSR` walks the package and global namespaces, where `Counter` isn't registered. Making it ambient there is exactly the collision exposure we deferred.

So the parked `Counter` milestone splits: the method half is reachable now, the constructor half still needs naming (or the type arriving by some route other than a name).

Two things I'd still want to check before treating methods as unblocked: whether a *generic* Rust citizen needs a template-shaped templata rather than a kind (`Vec<T>` would need something for `CallSR` to apply args to, where `Counter` doesn't), and whether extern drop still gates instantiating one at all.

---

## 👤 User

i might not want the templata to be in the environment like that... i think it's only placeholders now that are in the inner environment?

or maybe its fine, as long as the struct is a StructT phrased in terms of placeholders.

but generally, i dont like it when methods are treated any differently than functions, i think that's one of rust's biggest mistakes. please dispatch an agent to look into what im saying, and how the philosophy manifests in normal vale code.

can you also please send a message to Harmonious asking them if they lazily or eagerly compute something like FunctionS, or if they just read the rust things directly without a FunctionS-like wrapper?

---

## 🤖 Claude

Two dispatches. Agent first.

---

## 🤖 Claude

Both dispatched.

**Agent** — read-only, into the env-contents and philosophy questions. I told it to lead with the two that actually decide the design:

- **Is the inner environment only placeholders?** Your claim, to be verified or refuted at the construction site — plus what a `StructTT` phrased in terms of placeholders would concretely be, and whether anything already builds one.
- **Are methods structurally distinct from functions in Vale at all?** Whether `IEnvEntryT`/`ITemplataT` distinguish them, whether overload resolution does, and every place in the typing pass that treats a method differently for any reason other than "it was declared inside braces" — or a statement that there are none. Plus how UFCS is the concrete expression of "methods are just functions."

I also asked it to argue **both sides** of my self-entry proposal, with the specific framing that a Vale struct already reaches its own name through `sibling_entries` — so I may just be reconstructing, for a citizen with no siblings, what a Vale citizen gets for free. If that makes the proposal redundant or foreign, I want it said plainly.

And the generic case: whether a method's receiver lookup needs a *template*-shaped templata for `CallSR` to apply arguments to, rather than a bare kind — and whether an equivalent exists for a type that's declared but never defined.

**Harmonious** — asked whether they build a per-item wrapper at all, or read `TyCtxt` directly at each point of use; if there is one, eager or lazy; and what forced whichever they chose. I gave them our four eager layers, flagged the compile-every-declaration one as the one I'm least comfortable with, and framed *why now*: our two paths diverge exactly along the method/function seam you object to, and I'd rather find out whether the wrapper should exist before making them symmetric around the wrong one. Also asked whether they have a rule for panic-vs-decline on "can't represent this," since our own two paths currently disagree.

Nothing else touched — generics work still uncommitted and green.

---

## 🤖 Claude

Their answer is a clear "no wrapper at all" — plus a caution against copying that.

## What Sky actually stores about a Rust item

```rust
pub struct ToylangRegistry {
    pub imports: Vec<String>,   // e.g. "std::alloc::Global"
    ...
}
```

A path string. No declaration, no prototype, no per-item object. Those strings do two jobs: become `pub use` lines in the stub crate, and form the namespace their bare-name resolver searches. Resolution goes **straight to `TyCtxt` at the point of use**, via oracle callbacks threaded down through every statement and expression.

So they're **lazy by construction, not by optimization** — there was never an eager version to retreat from. Which answers the `Vec` worry directly: they never enumerate a type's methods. Their one `inherent_impls` use is a scan for a single name, once per call site that names it. A hundred inherent methods costs a scan, never a hundred declarations, compiles, or extern emissions. They agreed our fourth layer — compiling every declaration whether or not anything calls it — *"has no analog here and is the one to be uncomfortable about."*

## But they argued against us copying them

The sharpest thing in the reply:

> We have no wrapper because our typechecker is willing to call into rustc mid-resolution. Yours would have to give up solver-based resolution to do the same.

Both of us have a stub crate; they *stop there*, and the compiled stub **is** their declaration surface — namespace, DefIds, signatures on demand. A wrapper would duplicate what rustc already maintains for them. But our wrapper buys something they can't want: **our ordinary solver runs over it**, which is exactly why generics needed zero changes outside `rust_interop/` and why the machinery for generic externs inheriting container params just worked.

Their recommendation: **keep the wrapper, attack the eagerness** — synthesize a declaration on *first reference* rather than for every importable item. Same axis my discomfort was actually pointing at, and it's what rustc's `populate_on_access` does.

They priced their own choice honestly too: every call site re-queries rustc, uncached; they documented a `(DefId, args) → deps` memo layer and never implemented it. And making their oracle tolerate `ty::Param` was *forced* by resolving against rustc mid-walk rather than against a lowered declaration. Our design front-loads that cost once per item; theirs pays it per use.

## On methods vs functions — they back you without reservation

Their own compiler law states it almost word for word:

> **Prefer treating `self` as just another parameter.** When possible, avoid separate code paths for self vs non-self parameters. If syntax separates the receiver, reassemble the full parameter list so downstream logic can handle everything uniformly.

They follow it where it counts — trait-impl methods store `self` elevated to an explicit `&ToyStruct` first parameter. But they admitted they only follow it *partly*: their oracle still has split entry points (`rust_method_return_type` vs `rust_free_fn_return_type`, `find_inherent_method` vs `resolve_rust_path`). Same asymmetry as ours, living at the query layer rather than an object layer, so cheaper to tolerate and duly tolerated.

Their read on our seam: **collapse it toward declarations for both**, not prototypes for both. The declaration is the shape the solver, generics, and UFCS all want.

## And a rule worth adopting outright

On panic-vs-decline, they have one and it's phase-based, encoded as a pair of functions — `try_resolved_to_rustc_ty` declines, `resolved_to_rustc_ty` panics:

> Decline while user input can still be at fault; panic once an earlier phase has validated the thing away.

Applied to us: an un-imported ADT reached through a signature is **user error** — our @RTMEIZ equivalent, fixed by adding an import. So `lower_ty`'s panic is on the wrong side of the line, and the structural path already has it right.

The corollary is the part I'd adopt as a standing rule: *if you keep a panicking variant, its doc comment must name the prior phase that guarantees it can't fire. If you can't name that phase, it shouldn't panic.*

The philosophy agent is still running; I'll hold on the method/function seam until it reports, since it's checking whether my self-entry proposal is even the right mechanism.

---

## 🤖 Claude

Watcher re-armed.

---

## 🤖 Claude

Substantial, and it contradicts both me and the architect in places — plus it flagged one claim as needing empirical confirmation. Verifying the decisive ones.

---

## 🤖 Claude

Verified the four decisive claims myself. Reporting with what I confirmed marked as such.

## Your philosophy instinct is the codebase's design — decisively

**Methods are erased in the postparser.** `IExpressionPE::MethodCall` becomes `FunctionCallSE { callable_expr: OverloadSet(doSomething), arg_exprs: [subject, ...] }` (`expression_scout.rs:720-810`). Past that point there is no method concept in the tree at all. That's a stronger statement than "methods are like functions" — Vale's method *syntax* has no representation beyond parsing.

Downstream confirms it: `IEnvEntryT` has no method variant, `ITemplataT` has none, an internal method and a free function both become `IEnvEntryT::Function(&FunctionS)` with the same imprecise key and the same `FunctionTemplataT` — differing only in which env sits in `outer_env`. And `grep -n "internal" typing/overload_resolver.rs` returns **zero hits**.

The agent enumerated 15 places that treat methods differently. Every one is "it was declared inside braces, so it inherits the citizen's generic params / env / vtable slot." No semantic method-ness anywhere.

So moving Rust methods from prototypes to declarations isn't just cleanup — it's the direction Vale's design already points.

## Your "inner env is only placeholders" — refuted, and I checked the construction site

`struct_compiler_generic_args_layer.rs:447-451` maps **every** entry of `inferences` into the inner store, and `interpret_results` asserts every rune has a conclusion. So `struct Foo { x int; }` puts the member-type rune → `Kind(int)` in there.

Two consumers depend on it holding more than placeholders — `infer_compiler.rs:491` and `edge_compiler.rs:516` both walk the inner env filtering for `IEnvEntryT::Templata(ITemplataT::Prototype(..))` to harvest function bounds. I confirmed the first.

Your intuition is right about the **ids** — the inner env's id is the placeholdered instantiation name — just not about the store.

## But your outer-env instinct deserves a partial yes, and my proposal has an exact precedent

`struct_compiler_core.rs:404-415`, verified — the **lambda citizen** puts its own `KindT::Struct` into its own outer store, twice: under its own name and under `Self_`:

```rust
(understruct_instantiated_name_t, IEnvEntryT::Templata(
    ITemplataT::Kind(...KindTemplataT { kind: KindT::Struct(understruct_struct_tt) }))),
(INameT::Self_(...SelfNameT { }), IEnvEntryT::Templata(
    ITemplataT::Kind(...KindTemplataT { kind: KindT::Struct(understruct_struct_tt) }))),
```

And its synthesized drop names the receiver with a bare `LookupSR(_Self)` — no `CallSR`. That is my proposal, already shipped, for the one citizen kind that (like a Rust type) has no ambient name.

**The hazard, also verified:** `compile_struct_core:144` is `_ => panic!("vcurious: unexpected entry in outer_env.templatas")`. Any non-`Function` entry in a citizen outer store trips it. It doesn't fire today — Rust types have no `StructS`, so that code never runs — but it **becomes live the moment we synthesize a `StructS`** per plan §7. The lambda path dodges it by building its env by hand and never calling `compile_struct_core`.

## A correction I owe you, and it's mine

I told you a free `func doSomething(&Bork)` is callable as `bork.doSomething()` via `sibling_entries`. **Wrong.** Verified: `sibling_entries` filters top-level namespaces for the key `struct_template_id + PackageTopLevel` — the macro-derived `drop` and constructor nested *under that struct*. Not the declaring package's contents.

The real mechanism is that `PackageEnvironmentT` is **flat and global** — every package env's `global_namespaces` is *all* top-level stores, so every free function is ambiently visible from everywhere.

That makes `importer.rs`'s header comment wrong in a second place (it already asserts the refuted "a Rust method has no Vale AST" premise). Both want fixing.

## The fork this opens

- **Self-entry in the citizen's outer store** — solves "a synthesized method can name its receiver," has exact precedent, needs no qualified-name work. But it does *not* make `Counter` writable in Vale source, and it doesn't scale to generic citizens: there's no template-shaped templata without a `StructS`, so `CallSR` has nothing to apply args to. (A bare `Kind` passed to `CallSR` is a passthrough, so zero-arg works either way.)
- **Put the Rust citizen in the `rust` package top-level store as `IEnvEntryT::Struct(&StructS)`** — parity with Vale structs, and everything falls out: `StructDefinition` templata handles generics, `precompile_struct` builds the outer env, macro drop/constructor arrive as siblings, and a Rust method becomes an ordinary `internal_method`. Cost: ambient lookup has no tiebreak (`environment.rs:164` panics on two hits) — the qualified-name work.

Plan §7 already committed us to synthesizing a `StructS` with empty members. If that holds, the second route is where this is going, and the self-entry is the version that works before qualified names land.

Worth noting the agent flagged `sibling_entries` as needing empirical confirmation because it contradicted two prior agents — I verified it directly rather than take it on trust.

---

## 🤖 Claude

Watcher re-armed.

---

## 🤖 Claude

Watcher is live (Vale4-opal, pid 2632) — that exit was just the older one being superseded.

---

## 👤 User

btw, remember the two rust interop repos in vale? which one was more recent?

---

## 🤖 Claude

**`RustInteropReiImpl` is the more recent one** — by about five weeks.

| | last activity |
|---|---|
| `/Volumes/V/RustInteropReiImpl` | **2026-06-09** — `66b49e5cd`, *"TesterRust: support stdlib + stdout assertion in rust-interop tests; ri_29 runs Catter verbatim"* |
| `/Volumes/V/ValeRustInterop` | **2026-05-04** (newest source file; no git history at all — it's an orphaned worktree) |

That matches the name and the archaeology. `ValeRustInterop` is the ancestor: it's where the abandoned experiments are preserved as commented-out corpses — the `OpaqueStructMemberT` blob-member design, `isRustOpaqueType()` gated on `packageCoord.module.str == "rust"`, the `lift` flag that baked Rust's `Vec<i32>::push` shape into the typing pass and got rolled back. "ReiImpl" is the re-implementation, and both it *and* our tree replaced that package-coordinate check with the honest `extern` attribute.

Two practical consequences worth keeping straight:

- **`RustInteropReiImpl` is a branch of *this* repo** (`rust-interop-reimpl`, also `origin/master-with-rust-interop-reimpl`), so its `FrontendRust/` is the same language as ours and its findings transfer directly — which is exactly what bit us earlier, when a survey agent read its `templata.rs` and reported the line numbers as if they were ours. It still has `hig

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

please send an agent into RustInteropReiImpl to see what they generate, and what they do for methods

---

## 👤 User

please send an agent into RustInteropReiImpl to see what they generate, and what they do for methods, and what those methods' environments look like

---

## 🤖 Claude

Dispatched, read-only into `/Volumes/V/RustInteropReiImpl`.

I told it to lead with the two sections that decide our design:

**The environments.** What's in an extern struct's outer store versus inner store, whether there's a **self-entry** (the citizen's own kind under its own name or a `Self` name), and — the hypothesis I most want tested — that their method receiver resolves *ambiently* because `extern struct Vec` is an ordinary top-level denizen in a package namespace, meaning they never confronted our problem at all. Confirm or refute.

Also whether they ever put a **prototype** in an environment store, or whether every entry is a declaration. That's the question underneath our method/function seam: is prototypes-in-environments a shape Vale ever wants, or purely our migration artifact.

**Methods versus free functions.** Every asymmetry in their handling, or a statement that there are none — plus what `make_extern_function` actually does with `GenericParametersInheritance` beyond the wire-format reshape.

Supporting: what ValeRuster literally emits (with a full generated `.vale` file quoted if one's on disk), whether methods land inside the struct's braces as internal methods, how `A` and `C` both end up on `bar` in `extern struct Foo<A> { extern func bar<C>(...) }`, what rules the method carries for its receiver, and confirmation that the extern struct gets a real definition with zero members that `translate_opaque_i` reads back.

I gave it the prior findings up front — that the 32 `ri_*` tests run through the Scala frontend while `FrontendRust` has 7 green in-tree ones, and that their generation is text-level — so it doesn't spend its budget rediscovering those, and told it to say plainly if their approach sidesteps our problem rather than solving it. That's a useful answer too.

I'll report when it lands.

---

## 🤖 Claude

Decisive report. Let me verify the one fact route 2 hinges on, in *our* tree.

---

## 🤖 Claude

Verified — `environment.rs:442` maps `INameT::StructTemplate(s)` → `CodeName(s.human_name)`, so route 2 works in our tree too. Report:

## The philosophy question — their emitter is the cleanest evidence you'll get

ValeRuster's method emitter (`ValeRuster/src/main.rs:700-730`) has **no receiver-specific code path at all**. rustdoc's `sig.inputs` has `("self", <type>)` as its first entry, and the loop just prints name-then-type like any other parameter:

```rust
for (param_name, param_type) in &method.sig.inputs {
    params_inner_str += param_name;
    params_inner_str += " ";
    params_inner_str += &valify_simple_type(...);
}
```

Which produces `extern func capacity(self &Vec<T>) i64;` — `self` as an ordinary named parameter with a spelled-out type. Static constructors (`new`, `with_capacity`) sit in the same braces, distinguished only by *not having* a `self` param. That's your position, implemented by omission.

And downstream they found **five** method/free-function asymmetries, all shallow: where the declaration is registered, what drives its compilation, parent-last generic params, the `RuneParentEnvLookupSR` filter, and `GenericParametersInheritance`. Same `FunctionS`, same `ExternBodyS`, same `make_extern_function`, same overload resolution. **No different entry kind, no prototype path.** A method is an ordinary function whose declaring env happens to be a citizen env.

`GenericParametersInheritance` turns out to be used for *exactly one thing* — a `splitAt` count for the Hammer wire-format reshape (`Vec<i32>::capacity` rather than `Vec::capacity<i32>`). It's a pure "is my id nested under a citizen?" test on the already-built prototype id. Nothing else in `make_extern_function` distinguishes a method.

## Prototypes-in-environments: a real Vale shape, but not ours

This is the sharpest correction to what we're doing. Vale *does* put prototypes in environment stores — that's the entire generic-bounds mechanism, and the store even indexes them under the prototype's own human name so `where func moo(T)T` is reachable as `CodeNameS("moo")`.

But they're **always** keyed by rune or reachable-index, and always a bound. A prototype stored under a human function name, standing in for a declaration, appears **nowhere** in their tree. Every environment entry meaning "a callable you can name" is `IEnvEntryT::Function(&FunctionA)`.

So our method-prototypes aren't a variant of a Vale idiom — they're outside it. Collapsing them to declarations moves us toward their shape.

## On the receiver: they sidestep our problem, they don't solve it

Confirmed. No self-entry anywhere in an extern struct's outer env — the only `SelfNameT` entry in their whole typing pass is for **closure understructs**, the same lambda precedent I found in our tree.

Their receiver resolves ambiently, twice over: the higher-typing pass flattens `structs_s` across every package and matches on bare human name, and the package env unions *all* top-level stores (with a comment reading `// THIS IS TEMPORARY, it pulls in all global namespaces! See issue #356`). The struct got there during indexing, as an ordinary parsed top-level denizen.

The rules for `self &Vec<T>` are entirely unremarkable: `Lookup(CodeNameS("Vec"))` → `Call([T])` → an augment for the `&`. Ordinary name lookup, no `Self`, nothing special.

Worth noting a divergence: they have `MaybeCoercingLookupSR` and `AugmentSR`; our tree has neither (`AugmentSR` is commented out of our solver, and we have plain `LookupSR`). So their rule vectors won't transcribe literally.

## The two routes, now well-evidenced

**Register the citizen in a package top-level store.** Faithful reproduction of what they do, and everything falls out — `StructDefinition` templata handles generics, `precompile_struct` builds the env, methods become ordinary internal methods. Costs the qualified-name work, since ambient lookup has no tiebreak. One caveat the agent flagged: their higher-typing matcher *hard-panics* on any name that isn't a `TopLevelStructDeclaration`, so a synthesized citizen name would need that exact shape if it passes through that stage.

**Self-entry in the citizen's outer env**, keyed by its own `StructTemplateNameT`. I verified the mechanism holds here: `environment.rs:442` maps that to `CodeName(human_name)`, so a method's `LookupSR("Counter")` finds it nearest-first and never reaches a package. Precedent is the lambda understruct, *not* their extern structs — we'd be extending the lambda pattern. Needs the `vcurious` assert at `struct_compiler_core.rs:144` relaxed.

The honest framing: route 1 is what a Vale citizen actually is; route 2 is what Vale does for citizens that have no namespace to live in. A Rust citizen is currently the second and, per plan §7, is meant to become the first.

---

## 👤 User

so Self would be a StructTemplateNameT? and we would generate top level functions, not methods inside a struct environment?

---

## 🤖 Claude

Let me check what `Self_` actually maps to, since that's the crux of the first question.

---

## 🤖 Claude

Both confirmed. Taking them in order.

## No — `Self_` and `StructTemplateNameT` are two different keys

They map to *different imprecise names*, so they answer different lookups:

| entry key | imprecise name (`environment.rs`) | matched by |
|---|---|---|
| `INameT::Self_(SelfNameT{})` | `SelfName(SelfNameS{})` — `:451` | `LookupSR(SelfNameS)`, i.e. the `_Self` spelling |
| `INameT::StructTemplate(s)` | `CodeName(s.human_name)` — `:442` | `LookupSR(CodeNameS("Counter"))`, i.e. the type's actual name |

The lambda understruct registers its kind under **both**, so both spellings resolve. That's why `struct_drop_macro` can do a bare `LookupSR(SelfNameS)` with no `CallSR` — it's using the `_Self` key, not the name key.

So it's a choice, not an identity: `Self_` if you want synthesized code to say `_Self`; `StructTemplateNameT` if you want it to say `Counter`.

## And yes — top-level functions, and that's the more Vale-native answer

I verified the desugaring (`expression_scout.rs:764-786`). `v.get()` becomes an ordinary overload call with the subject spliced in as the **first argument**:

```rust
let mut args = vec![subject1];
args.extend(tail_args1);
```

...against an `OverloadSetSE` on the bare method name. There is no method concept past that point.

So a **top-level** `get(self Counter) int` in the `rust` package store gets found by `v.get()` through the *calling* environment — because `PackageEnvironmentT` unions every top-level store, so a free function is ambiently visible from everywhere. No citizen env involvement at all. That's precisely what UFCS is, and it collapses our method/function seam to nothing: one path, one entry kind, one call site in the synthesizer.

**But it changes which problem we have**, and this is the tension worth seeing:

- **Methods inside the citizen env** → the self-entry works, receiver naming is solved locally, no qualified-name work. But methods live somewhere free functions don't, which is the asymmetry you object to.
- **Top-level functions** → maximally consistent, no asymmetry at all. But the declaration still has to name `Counter` in its `LookupSR`, and from a package env that lookup is *ambient* — so `Counter` must be in a namespace, which is exactly the collision/qualified-name work.

The self-entry only helps if the method lives inside the citizen. Choosing top-level functions means choosing to do the naming work.

Two things I'd flag before you decide. First, `get_param_environments` has **no ref-peeling arm** — `&Counter` is `BorrowRef{Struct}` and matches none of its three arms, so a borrowed receiver never reaches the citizen's outer env today. Under the top-level route that stops mattering, since resolution comes from the calling env. Second, ReiImpl's extern structs put methods *inside* the citizen — so the top-level route is more Vale-native than what either sibling implementation actually does, and we'd be the first to try it.

---

## 👤 User

top-level functions is the way forward. if we run into a collision, we should work on qualified names.
wdym "a borrowed receiver never reaches the citizen's outer env today."?

---

## 🤖 Claude

Recorded: top-level functions, qualified names when a collision forces it.

Let me show you the receiver thing rather than describe it.

---

## 🤖 Claude

Here's the concrete thing.

`get_param_environments` (`overload_resolver.rs:502-509`) matches on each argument's kind:

```rust
KindT::Struct(sr)           => vec![get_outer_env_for_type(get_struct_template(sr.id))],
KindT::Interface(ir)        => vec![...],
KindT::KindPlaceholder(kp)  => vec![...],
_ => Vec::new()
```

And post-onion, **ref-ness is a wrap inside `KindT` itself** — `CoordT` was dissolved, so `KindT` has `BorrowRef`, `OwnRef`, `ShareRef`, `WeakRef` as sibling variants of `Struct` (`types.rs:52-69`).

So `&Counter` is `KindT::BorrowRef(BorrowRefT { inner: Struct(Counter) })`. It matches **none** of those three arms, falls through to `_ => Vec::new()`, and the citizen's outer env is never consulted. There's no peeling step — the function looks one level deep and a borrow hides the struct underneath it.

**What that means concretely:** `c.get()` where `c` is borrowed contributes *no* param environment. Whatever methods live in `Counter`'s outer env are invisible to that call. Only the ambient calling env is searched.

That's why the fixture uses by-value `self`, and why the working Vale program is `(make_counter()).get()` — the parenthesized form moves rather than borrows, so the argument is a bare `Struct(Counter)` and the arm matches. I'd written that in the fixture as being about `substitute_templatas_in_kind`'s unimplemented ref-wrap arms; it turns out there are *two* independent ref-shaped gaps on that path, and this is the second.

Note this isn't interop-specific — it's Vale's own UFCS. Any Vale `x.method()` on a borrowed receiver has the same hole, which is plausibly part of the 175.

**And it's an argument for the route you just picked.** Under top-level functions, `v.get()` resolves through the *calling* environment — ambient, since `PackageEnvironmentT` unions every top-level store — so `get_param_environments` isn't on the path at all. The top-level route routes around this gap entirely, rather than waiting for the onion arc to fill those arms.

So the sequencing is cleaner than I'd framed it: methods-as-top-level-functions doesn't just remove the asymmetry, it removes our dependency on a live core gap.

---

## 👤 User

is that a bug in vale itself, or just with the rust stuff?

---

## 🤖 Claude

Good question — let me check empirically rather than reason about it.

---

## 🤖 Claude

That test name is exactly the mechanism. Reading it and its failure.

---

## 🤖 Claude

**It's Vale itself, not the Rust stuff.**

There's a pure-Vale test for exactly this, and it's failing:

```rust
fn calling_a_method_on_a_local_will_supply_borrow_ref() {
    let code = r"
struct Bork { a int; }
func doSomething(bork &Bork) int { return __copy_prim(&bork.a); }
func main() int {
  bork = Bork(42);
  return bork.doSomething();
}
";
```

No interop anywhere. It fails at `expression_compiler.rs:1963` in `dot_borrow` with a bare `unimplemented`.

Two things that sharpens, though — I was conflating them:

**The failure above is upstream of the thing I described.** `bork.doSomething()` doesn't even get as far as overload resolution; it dies in `dot_borrow` while evaluating the receiver expression. So that particular test is blocked by a different onion-arc hole, not by `get_param_environments`.

**And `doSomething` here is a *free* function, so it wouldn't need param environments anyway** — it's found ambiently through the calling env. The `get_param_environments` hole specifically affects **internal methods** (declared inside a citizen's braces), which live *only* in the citizen's outer env and have no ambient fallback.

So, precisely:

- **The borrow path through `dot_borrow` is broken in Vale today**, demonstrably, with a Vale-only test — that's the arc, and it's one of the 175.
- **The `get_param_environments` ref-peeling gap is a separate latent hole**, also Vale's rather than ours, but I haven't demonstrated it firing because the path in front of it fails first. I'd downgrade my earlier claim from "this is why our fixture uses by-value `self`" to "this is *a* reason it would fail; the one it actually hits first is `dot_borrow`."

Both are Vale-side. Neither is caused by interop, and neither is ours to fix. And both are routed around by the decision you just made — under top-level functions with a moved receiver, we touch neither.

---

## 👤 User

got it, we're blocked on that part then. send a short polite message to Vale2 asking them to put this test somewhere on their medium-term radar.
then, what can we do that isnt blocked?

---

## 🤖 Claude

Sent to Vale2-garnet.

## What isn't blocked — and one thing I'd flagged as blocked that probably isn't

I want to correct myself before listing chores, because it changes the answer.

I said methods-as-top-level-functions needs `Counter` ambiently nameable, which needs a synthesized `StructS`, which trips `struct_drop_macro`'s extern panic. **That chain has a shortcut.** We don't need a `StructS` to make a name resolvable — we need an *entry*. Putting `IEnvEntryT::Templata(ITemplataT::Kind(KindTemplataT { kind: struct_kind }))` into the `rust` package top-level store under `Counter` makes `LookupSR(CodeNameS("Counter"))` resolve ambiently, exactly the way `int` does from the builtins store. We already intern that `StructTT` in `import_rust_types`.

And no `StructS` means `compile_struct_core` never runs for it, so the drop macro never fires — the §8 gate isn't on this path at all. Scope-end drop would resolve through a synthesized top-level `drop(self Counter) void` declaration, the same shape as every other method.

So the real next slice looks reachable: **methods and drop become ordinary top-level declarations, the receiver resolves ambiently, and `make_prototype_entry` disappears.** That's the seam collapse you asked for, and it's the highest-value thing available. I'd want to verify the drop-macro claim before committing to it — but if it holds, we're not blocked on core at all.

Independent of that, all unblocked and all inside `rust_interop/`:

- **The three name-matching sites** (`tyctxt_oracle.rs:73-76`, `:240-256`, `:259-264`) — key on `DefId`, add a provenance filter. This is our own arch doc's `@ATAFLBZ` invariant, currently violated, and it's in the file I'd be editing anyway.
- **`lower_ty`'s panic → decline**, per Harmonious's phase rule: an un-imported ADT is user error, so it belongs on the declining side with a diagnostic. Our structural path already does this; the two disagree.
- **The `@ATAFLBZ` fence** — grep plus allow-marker, so the fourth site in eight months fails loudly rather than silently.
- **The scope-iteration regression test** — pins that placing a declaration in a scope is sufficient to get it compiled, which we now depend on.
- **Two stale comments in `importer.rs`** — the refuted "a Rust method has no Vale AST" premise, and my wrong claim about what `sibling_entries` are.
- **Committing the generics work**, which is green and uncommitted.

Genuinely blocked and not ours: the borrow path (`dot_borrow` + the ref-peeling gap), now with Vale2. Extern drop stays a gate only for the `StructS` route, which the above sidesteps.

My recommendation: verify the drop-macro claim, then do the seam collapse; fold the oracle `DefId` fix in since it's the same file. Want me to start with the verification?

---

## 👤 User

"methods and drop become ordinary top-level declarations, the receiver resolves ambiently, and make_prototype_entry disappears."
yeah, i want that. lets go that direction.
verify then do the seam collapse please.
let me know if theres anything needed in the core compiler

---

## 🤖 Claude

Verifying the drop-macro claim first — that's what the whole direction rests on.

---

## 🤖 Claude

**Verified — the claim holds, doubly.**

`compiler.rs:1366` is the *only* call site of `get_struct_sibling_entries`, and it's inside the indexing loop over parsed `struct_a` denizens. No `StructS` ⇒ the macro never runs ⇒ no `GeneratedBody(drop_generator)` function is ever created ⇒ `generate_function_body_struct_drop` never executes ⇒ the extern panic is unreachable.

And even if it somehow ran, `struct_drop_macro.rs:203` opens with `coutputs.lookup_struct(struct_tt.id, self)`, which resolves to `lookup_struct_template`'s `.expect("Struct template not found")` — a Rust type has no `StructDefinitionT`, so it would die there first, before reaching the extern check.

So §8's drop gate is genuinely off this path. Establishing the baseline and writing the RED.

---

## 🤖 Claude

Baseline confirmed (574/175/8, driver green). **RED** — the Vale program that should work once the seam collapses:

---

## 🤖 Claude

**Tests are correctly failing, proceeding with implementation.**

`CouldntFindFunctionToCallT` on `make_counter` with zero rejected callees — it isn't importable, because its return type isn't nameable. Exactly the gap the seam collapse closes. Now the implementation.

---

## 🤖 Claude

Now the store: the type as a nameable entry, and methods + drop as top-level declarations.

---

## 🤖 Claude

Guardian's right — I copied a `crate::` path from adjacent code. Importing it properly.

---

## 🤖 Claude

`check_defining_conclusions_and_resolve` harvests reachable bounds from a citizen's **inner** env, and we only declare the outer one. Adding it:

---

## 🤖 Claude

Both paths are live — the new top-level `get` declaration *and* the old `ExternFunction` prototype in the citizen env. That duplication is exactly what the collapse deletes.

---

## 🤖 Claude

`get` resolves now. The failure is my test program's fault — I used `+`, which isn't in scope in this minimal harness. Sequencing instead of combining:

---

## 🤖 Claude

`ensure_deep_exports` needs a `StructDefinitionT` for the Rust citizen. That's a core touchpoint — let me see its shape before deciding.

---

## 🤖 Claude

That's a core touchpoint. Applying it locally so I can enumerate everything behind it rather than reporting one at a time — I'll flag the full set for you at the end.

---

## 🤖 Claude

**GREEN**, and that was the only core touchpoint. Running the full suites.

---

## 🤖 Claude

Both are now dead — defined, never called. Deleting them completes the collapse.

---

## 🤖 Claude

The seam is collapsed and green. Both suites at baseline (573/175/8, 574/175/8), driver exit 0, 8 pre-existing warnings.

## One core change is needed — this is the only one

`FrontendRust/src/typing/compiler.rs`, in `ensure_deep_exports`'s extern-param check (~`:1541`). I applied it locally to find out what was behind it; **please review and redo it as you see fit.**

```rust
let kind_is_fine_in_extern_func = match param_type {
    // A Rust-backed citizen is extern by construction — the reserved `rust`
    // package coordinate on its id *is* the marker — so it satisfies this
    // check without a definition to read the attribute off. It has none:
    // an imported Rust type is declared, never defined.
    #[cfg(feature = "rust_interop")]
    KindT::Struct(s) if is_rust_backed(&s.id) => true,
    KindT::Struct(s) => coutputs.lookup_struct(s.id, self).attributes.iter().any(...),
    KindT::KindPlaceholder(_) => true,
    _ => false,
};
```

Plus `is_rust_backed` added to the existing `use crate::typing::rust_interop::{...}` at `:37`. Without it, `lookup_struct` hits `.expect("Struct template not found")` — the check reads an attribute off a definition a Rust type doesn't have. It's precisely the "gated one-line delegation at a definition-reading site" the plan doc predicted.

## What the collapse did

**Methods are now ordinary top-level declarations.** `rust_package_stores` builds one list — free functions *and* methods — and runs them through a single code path. The receiver is just parameter zero. `import_rust_types`' method loop is gone, as is the synthesized drop prototype.

**The Rust type is nameable**, registered in the same top-level store its functions live in, as a bare `Kind` templata — exactly how `int` reaches scope from the builtins store. That's what makes `make_counter() -> Counter` importable at all.

**`make_prototype_entry` and `concrete_kinds` are deleted.** No prototype is put into any environment by interop any more — which matches what the ReiImpl survey found: Vale does store prototypes in environments, but only ever as *bounds*, keyed by rune, never as "what function does this name refer to."

The citizen's outer env is now **empty** — and that's the point. It still has to exist, because `get_param_environments` reaches for it unconditionally on a `KindT::Struct` argument, but nothing lives in it.

Three things the RED cycles caught, in order:

1. **A missing inner env.** `check_defining_conclusions_and_resolve` harvests reachable bounds from the inner env of *every* citizen a signature mentions, unconditionally. Now declared, empty — same "absent ≠ empty" distinction as instantiation bounds.
2. **The duplication announced itself as `CouldntNarrowDownCandidates`** — two `get` candidates, the new `Function` declaration and the old `ExternFunction` prototype. That's the seam being visibly double-counted.
3. **My test program used `+`**, which isn't in scope in this harness (test package only, no builtins). Sequenced instead.

The program now proving it:

```vale
exported func main() int {
  x = pick<int, bool>(add_two_numbers(3, 4), true);
  return (make_counter()).get();
}
```

Generic call, free function, Rust type by name, and a method-as-function — all through one path.

**Drop is untested and unimplemented here.** The temporary is consumed by `get(self Counter)`, so no scope-end drop is required. Per TDD I didn't build it without a test demanding it — that's the next slice, and it'll be a top-level `drop` declaration like any other. Worth knowing the §8 gate stays off this path: no `StructS` means `compile_struct_core` never runs, so the drop macro's extern panic remains unreachable.

Nothing committed.

---

## 👤 User

go ahead and add that back in, ty

---

## 🤖 Claude

Reading "that" as **drop** — the piece I removed and deliberately didn't rebuild. RED first: a program that binds a Rust value and lets it fall out of scope.

---

## 👤 User

[Request interrupted by user for tool use]

---

## 👤 User

go ahead and add the kind_is_fine_in_extern_func back in, ty

---

## 🤖 Claude

It's still in the tree — I never reverted it, so there's nothing to re-apply. Confirming exactly what's there:

---

## 🤖 Claude

Already in place and verified — nothing to re-apply. My earlier "please review and redo it as you see fit" was an offer to hand it back, not a note that I'd reverted it; sorry for the ambiguity.

What's in the tree, in full:

- **`compiler.rs:37`** — `is_rust_backed` added to the existing `use crate::typing::rust_interop::{...}`.
- **`compiler.rs:1541-1550`** — the `#[cfg(feature = "rust_interop")]` guard arm ahead of the ordinary `KindT::Struct` arm, so a Rust-backed citizen answers the extern-param check from its package coordinate rather than from a definition it doesn't have.

Note the two arms are order-dependent: the guarded one must come first, since `KindT::Struct(s)` below it is an irrefutable match for the same shape. And with the feature off, the arm doesn't exist and the match is byte-identical to before — which is why the default suite is unmoved at 573/175/8.

That's the entire core diff for the seam collapse. Everything else lives in `rust_interop/`.

Still uncommitted, and still green: driver exit 0, 574/175/8 interop, 573/175/8 default, 8 pre-existing warnings.

---

## 👤 User

re the KindT::Struct(s) if is_rust_backed(&s.id) => true,...
could we instead have added an extern attribute to the generated struct?

---

## 🤖 Claude

Good instinct — that would be honest data instead of a cfg'd special case, and it'd satisfy *every* definition-reading site rather than one. Let me check whether a definition can be built without a `StructS`.

---

## 🤖 Claude

`StructDefinitionT` has **no `origin_struct` field** — it's all plain data we already hold. Checking what `add_struct` demands:

---

## 🤖 Claude

Only two preconditions, both trivial. Let me check what the importer already declares and what `ExternT` needs.

---

## 🤖 Claude

Everything's available. Adding the definition:

---

## 🤖 Claude

Now reverting the core guard, since the attribute makes it unnecessary.

---

## 🤖 Claude

Now `drop` is required — the real definition made it fire. That's the next slice, and the fix is the same path as everything else: a top-level `drop` declaration.

---

## 🤖 Claude

Your instinct was right, and it was better than a guard arm. **The core change is gone entirely — `compiler.rs` now has zero diff.**

## What changed

`import_rust_types` now builds a **real `StructDefinitionT`** and calls `add_struct`:

```rust
attributes: [ICitizenAttributeT::Extern(ExternT { package_coord })],
members:    &[],
sharedness: SharednessT::Single,
weakable:   false,
is_closure: false,
```

Two things made this possible that I'd assumed weren't: **`StructDefinitionT` has no `origin_struct` field** — it's plain data, every piece of which we already hold — and **`add_struct` asserts only two things**: that sharedness was declared, and that it isn't already added. No `StructS` required anywhere.

So `ensure_deep_exports` reads the `Extern` attribute off the definition exactly as it does for a hand-written `extern struct`. No special case, no `#[cfg]`, no `is_rust_backed` check in core.

And it isn't just the one site — it fixes the *class*. The plan doc predicted "8–12 gated one-line delegations at definition-reading sites"; every one of those was a site wanting to read a definition that didn't exist. They now all have one to read.

## The drop question answered itself

Adding the definition made `drop` become required — `Compiler::drop`'s `KindT::Struct` arm always resolves a destructor call, with no discard path for an owned struct. That's a genuine RED, and the fix was the same path as everything else: a synthesized top-level `drop(self Counter) void` declaration, in the same store, through the same code.

So the seam is now fully collapsed: **free functions, methods, and drop are one code path producing one entry kind.** The type's own item id supplies drop's identity — free and unique, since the type itself is a `Kind` entry carrying no location.

Worth noting the §8 gate stays off this path. No `StructS` means `compile_struct_core` never runs, so `struct_drop_macro`'s extern panic is still unreachable; drop is resolved by ordinary overload lookup finding our declaration.

## State

| | |
|---|---|
| `git diff` on core | **empty** |
| driver | exit 0 |
| default suite | 573 / 175 / 8 |
| interop suite | 574 / 175 / 8 |
| warnings | 8, all pre-existing |

One thing I'd flag rather than leave implicit: the definition claims `sharedness: Single` and `weakable: false` for every imported type. Both are true today and neither is queried from rustc — they're declared at the boundary, which is what the callout map recommends. But they're now *asserted* facts sitting in a definition rather than absent ones, so if a Rust type ever needs different answers, this is where they'd be wrong.

---

## 👤 User

sharedness: Single and weakable: false is correct. rust will never support those two things.
whats next?

---

## 👤 User

sharedness: Single and weakable: false is correct. rust will never support those two things.
whats our test coverage like?

---

## 👤 User

sharedness: Single and weakable: false is correct. rust will never support those two things.
whats our test coverage like? rust interop should have a lot of tests

---

## 🤖 Claude

Fair challenge. Let me count rather than guess.

---

## 🤖 Claude

Honest answer: **thin, and thinner than the numbers suggest.**

## What actually exists

**One** automated test: `calls_a_rust_free_function`. That's the whole difference between 573 and 574.

Everything else I've been verifying today — generics read structurally, projections declined, methods-as-top-level-functions, the type being nameable, drop resolving, the negative control — lives in the **driver**, which is a `[[bin]]`, not a `[[test]]`. `cargo test` never runs it. It only executes when I invoke it by hand.

So when I've been reporting "driver exit 0" alongside suite numbers, those are two different kinds of claim, and I should have been clearer about that. Roughly nine assertions of real coverage sit behind a binary nobody runs automatically. If someone breaks the seam tomorrow, both suites stay green.

That's exactly the genre Harmonious flagged — their standing rule is *"an integration test must run the artifact and check its output"* — and the `harness = false` wrapper to do it was on the list from the previous session and never got built.

## What has no coverage at all

- **The identity hazard** — the plan doc's top risk. Two same-named items collapsing into one candidate, silently. No test, and it's the one failure that produces no error.
- **Drop actually resolving** — it works in the driver as a side effect of the program, never asserted directly.
- A Rust type in **parameter** position (we only cover return position and receiver).
- **More than one** Rust type, or two crates.
- A **generic Rust type** (`Vec<T>`) — we cover generic *functions* only.
- The `lower_ty` panic-vs-decline inconsistency.
- Anything on the fixture-oracle path beyond that single function.

## What I'd do, ranked

1. **`[[test]]` with `harness = false`** that drives the same code path. This is the big one — it converts nine existing assertions from manual to automatic for very little work, and it's the right shape because rustc gets its own process. Everything else is worth less until this exists.
2. **The identity test** — two Rust items sharing a human name, asserting both remain callable. That's the silent-failure hazard, and per TDD I'd want to watch it fail with a shared sentinel location before trusting the DefId-derived fix.
3. **Split the driver's single program into several fixtures** — right now one Vale program carries generics, a free function, a type by name, and a method. When it fails, it fails as a lump. Separate fixtures would localize.
4. **Parameter-position and multi-type fixtures**, then a generic Rust type once `Vec<T>` is reachable.

Worth naming a structural constraint: the fixture-oracle test can't cover much of this, because `FixtureOracle` can't produce a `ty::Param` or an alias at all. Generics and projections are only testable against real rustc — so the driver-side suite is where interop coverage has to live, which makes item 1 the gate on all of it.

Want me to start with the `harness = false` wrapper?

---

## 👤 User

also, please read DBAPIZ

---

## 🤖 Claude

Read it. It changes my answer from last turn, and it flags two things in what we already have.

## Where we violate it now

**`compile_vale` is nearly the dark-box API, but returns stringly-typed data.** Its signature is `-> Result<Vec<String>, String>`. DBAPIZ rule 2 wants structured outputs — result types, not strings — and `NoStringlyTypedData-NSTDX` says the same. The real error is an `ICompileErrorT`; we're flattening it to a `String` at the boundary, which is exactly where the information should still be typed.

**`check()` lives inside the production binary** (`driver/main.rs:82`). That's test logic shipped in `main.rs`. DBAPIZ says tests call the dark-box function and assert on its output — they don't get embedded in the artifact. It also means those nine assertions can never be extended without editing the binary.

**`main()` isn't trivially thin.** Arg parsing and `build_dep_rlib` are legitimately above the boundary; `check()` is not.

And there's no `docs/architecture/dark-box-api.md`, which DBAPIZ says every project with a binary target should have.

## So my proposal from last turn was half right

I proposed a `[[test]] harness = false` that runs the driver and checks its output. The *process* mechanics are right — rustc owns the process, so the test has to be its own binary. But the *boundary* was wrong: I had the test reading stdout, which is the pure black-box shape DBAPIZ explicitly says should be the small minority.

Revised:

- **Dark-box API** — `typecheck_against_rust(tcx, allowed, vale_source) -> InteropOutcome`, where the outcome carries the oracle log and a **structured** compile result.
- **`main()`** — gather paths, build the rlib, run rustc, print the outcome. Nothing else.
- **`[[test]] harness = false`** — its own thin `main()`, same setup, asserting on the structured `InteropOutcome`. Many fixtures and programs, not one.
- **`check()` moves out of the binary** into that test.
- A couple of genuine black-box tests that spawn `valec-rs` and check exit code, to cover the thin layer.

## One real obstacle, and it's the interesting part

`ICompileErrorT<'s, 't>` is lifetime-bound to arenas that die with the callback — the containment property the driver's comments already note. So a structured error **cannot escape** `after_expansion` by reference.

The DBAPIZ-faithful fix is to convert at the boundary into an owned, lifetime-free structured value — a small enum mirroring the variants we assert on — rather than to `String`. Structured, just not holding arena refs. That's a meaningfully better cut than what's there.

Which also dissolves a justification we wrote down. `OracleCall(pub String)`'s doc comment says text was chosen *"because the log crosses a process boundary (the driver prints it, a test reads its stdout), so a stable human-readable line is the useful form."* Under the dark-box design there is no such crossing — the test reads the value directly — so the log can become a structured enum too, and assertions stop being `l.contains("generics [")`.

That last bit is worth flagging as its own finding: several of my current assertions are substring matches against a formatted string. They'd all become field access.

Want me to do this — extract the dark-box API with owned structured outputs, move `check()` into a `harness = false` test, and write the `dark-box-api.md`? It's more work than the wrapper I proposed, but it's the difference between automating nine substring assertions and having a place to put fifty real ones.

---

## 👤 User

i've been applying darkbox to a pretty large extent in a lot of ways, though it takes a bit of perspective.
we dont strictly have only end-to-end testing here in the vale compiler (parser has tests, postparser too, typing has tests, and so on). but thats only because each pass is so large.
but i still very much like the spirit of dark-box. because unit tests are bad, theyre brittle, and they cause inertia keeping us to a certain architecture, and they often arent correct in the way that end-to-end tests are.
with that in mind, tell me about our testing setup?

---

## 🤖 Claude

Let me look at the actual shape rather than characterize it from memory.

---

## 🤖 Claude

Here's the honest picture, and the headline isn't what I expected.

## The end-to-end tests are switched off

```
321  integration_tests     ← commented out of lib.rs
109  end_to_end_tests      ← commented out of lib.rs
  6  instantiating         ← commented out
  4  simplifying           ← commented out
────
241  parsing               ← running
230  typing                ← running
 84  postparsing           ← running
 21  solver                ← running
```

**430 tests aren't compiled at all.** And they're the *most* dark-box-shaped ones we have — `integration_tests` and `end_to_end_tests` run whole Vale programs through the pipeline to Vivem and check the output. Exactly the "correct in the way end-to-end tests are" layer.

What's still running is entirely per-pass — the layer you said you tolerate only because each pass is so large. So the arc has inverted the pyramid: the tests that would tell us a program still *works* are dark, and the ones that survive are the ones most coupled to structure.

And all **175 failures are in `typing`** — zero in parsing, postparsing, solver, lexing, utils. The damage is contained to one pass, which is good news, but it also means the pass under active reconstruction is the one whose tests are load-bearing right now.

## Where the boundaries actually sit

The per-pass tests are better shaped than "unit tests" would suggest. The typing tests go source-string in → `compiler_test_compilation` → `expect_compiler_outputs()` → `HinputsT` out. That's a dark-box API for the pass: structured input, structured output, nothing calling internal helpers. Then `collect_only_tnode!` walks the *output*, which is inspection of a returned value rather than reaching into internals — and your rule that `collect_` stays test-only is what keeps that boundary honest.

The brittleness risk isn't the entry point, it's what the walkers assert. A test that says "exactly one `FunctionCall` node exists with this callee" is pinned to AST shape; one that says "this program compiles and `main` returns int" isn't. We have both kinds.

## Where interop sits, specifically

Worst of both. **One** test in the running layer. The real coverage — generics, projections declined, methods-as-functions, drop, the negative control — is in a `[[bin]]` that `cargo test` never invokes. So interop's dark-box-shaped tests are dark in the same way `end_to_end_tests` is, just for a different reason: not commented out, simply never wired to a test target.

That reframes what I proposed. The `[[test]] harness = false` isn't primarily about adding coverage — it's about **un-darkening coverage that already exists**. Nine assertions that run only when I type the command.

## The thing I'd want your call on

There's a real question hiding here: **should interop tests live in the per-pass layer at all?**

The fixture-oracle test does — and it's structurally limited, because `FixtureOracle` can't produce a `ty::Param` or an alias. It can only ever test the Vale-side synthesis against canned answers, which is the shape you deleted the fixture *for* in the first place.

Everything that matters about interop needs real rustc, which means the driver, which means a `harness = false` binary. So interop's tests want to be end-to-end-shaped by nature — closer to `integration_tests` than to `typing/test/`. Which is an argument for building that suite properly now rather than growing the per-pass one.

---

## 👤 User

i've been applying darkbox to a pretty large extent in a lot of ways, though it takes a bit of perspective.
we dont strictly have only end-to-end testing here in the vale compiler (parser has tests, postparser too, typing has tests, and so on). but thats only because each pass is so large.
but i still very much like the spirit of dark-box. because unit tests are bad, theyre brittle, and they cause inertia keeping us to a certain architecture, and they often arent correct in the way that end-to-end tests are.
with that in mind, tell me about our testing setup for rust interop, and where rust interop testing is headed?

---

## 🤖 Claude

Good framing — and it reframes the question, because the typing pass already *is* a dark-box API. `compiler_test_compilation` takes Vale source and gives back `HinputsT` or a compile error; 217 tests call it. Nobody unit-tests `translate_templex`. So the question for interop isn't "should we adopt dark-box," it's **which boundary is ours, and is it as good as that one.**

## What we have

Three boundaries exist, whether or not we chose them:

| boundary | inputs → outputs | state |
|---|---|---|
| **oracle** — `RustOracle` | a rustc item → `ValeSig`, `importable_*` | no standalone tests |
| **typing pass + oracle** — `compiler_test_compilation_with_rust_oracle` | Vale source + oracle → `HinputsT` | **1** test |
| **typing pass + real rustc** — the driver | Vale source + Rust crate + allowlist → outcome | ~9 assertions, **not automated** |

The middle one is the right boundary and it's the one the rest of the compiler uses. The problem is the fixture oracle can't reach most of what matters.

## The trajectory nobody has stated yet

**The fixture oracle gets less useful over time, not more.** It structurally cannot produce a `ty::Param`, an alias, or anything rustc-shaped. Generics, projections, inherited impl parameters — none of it is expressible there. A previous session already noticed the corollary: deleting `FixtureOracle` *dissolves* the contract-test question rather than deferring it.

So interop testing converges on the rustc-hosted boundary as the primary suite, and the fixture path shrinks to whatever is genuinely oracle-independent. Which means **the driver's boundary has to become as good as `compiler_test_compilation` is** — hand it a Vale program and an allowlist, get back a structured outcome, so a test is three lines. Right now it's one hard-coded program and a `check()` function welded into the binary.

## The brittleness point lands directly on what I wrote today

Most of my driver assertions look like this:

```rust
log.iter().any(|l| l.contains("generics [\"A\", \"B\"]"))
```

That's coupled to a `Debug` format, not to behavior — the unit-test failure mode exactly. I already hit it twice today: once when `ValeSigType` wrapping turned `ret Struct(` into `ret Kind(Struct(`, and once when `{:?}` on an `Option<String>` double-escaped the quotes. Both times the *behavior* was correct and the *assertion* broke.

And the honest conclusion is stronger than "make them structured": **most of them shouldn't exist.** The Vale program can carry the assertion. `pick<int, bool>` returning `A` means a swapped index yields `bool` where `int` belongs and `main() int` doesn't typecheck — no log grep needed, and it survives any refactor of how we render logs. Same for the generic reading, the type being nameable, the method resolving, drop resolving.

What a source program *cannot* express is **vacuity** — that the oracle was consulted at all rather than something Vale-side coincidentally matching. That's the log's real and only job, and it's the one assertion that earned its keep today by catching an empty `packages_to_build`.

So the mature shape is: programs assert behavior, the log asserts consultation, and the structured outcome exists so neither is a substring match.

## One constraint that shapes all of it

`collect_` macros are test-only by your rule, and integration tests link the lib as an ordinary dependency so `#[cfg(test)]` items are invisible there anyway. The driver can't walk the AST and shouldn't. That's not a limitation to work around — it forces exactly the discipline above: assert by *what compiles*, not by *what the tree looks like*.

## Where I'd take it

A fixture matrix rather than one program. Each row is a small Rust crate plus a Vale program that compiles only if one behavior is right: generics, projection declined, method-as-function, drop, a Rust type as parameter, two types, two crates, same-named items (the identity hazard). Each test asserts "compiled" or "failed with *this* structured error," plus a negative control where the allowlist is emptied.

That's ~10 tests replacing ~9 substring assertions on a single program, all automated, none coupled to a `Debug` format.

The prerequisite is the same either way: the rustc-hosted boundary needs a real dark-box function and a `harness = false` test to run it. Everything above is cheap once that exists and impossible until it does.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

also, we shouldnt have a fixture oracle, and we should strive to get rid of it. the main question that remains for me is:
- how many tests for the typing pass do we want, invoking the actual rustc typing pass and our typing pass together? i think a lot of them.
- how many tests do we want to be truly end-to-end, invoking the actual rustc entirely (typing, mono, codegen), and our entire compiler (typing, instantiator, backend) together? i think a lot of them there as well.

i think every test should be run in the former, and every test should be run in the latter, similar to how we have a lot of tests repeated between vale src/typing and the vale backend tests which are truly end-to-end.

---

## 🤖 Claude

Agreed on all three, and the "every case in both tiers" requirement has a structural consequence worth naming up front.

## It means cases must be data, not Rust functions

If one corpus feeds two runners, a case can't be a `#[test]` fn — it has to be a fixture the runners *read*: a Rust crate, a Vale program, and an expected outcome. ReiImpl already does exactly this — 32 `ri_*.vale` files with an `// expected_exit: 42` header — and that header is the thing that lets one case mean different assertions at different depths.

That matters now rather than later: **design the outcome field for tier 2 even while only tier 1 exists.** A case says "exits 42"; tier 1 checks the weaker property it can see (typechecks, with the oracle consulted), tier 2 later checks the strong one. Otherwise we write fifty cases against a typecheck-shaped schema and rewrite them all when codegen lands.

## What each tier can actually assert

They're not nested — each catches something the other structurally can't.

**Tier 1** can assert *failure precisely*: this program must fail, with this structured error. That's most of the interesting interop surface — un-imported type, unrepresentable signature, ambiguity, wrong generic binding. It also owns the vacuity guard, since the oracle log only exists there.

**Tier 2** can assert *behavior*: it prints 42. It's the only tier that catches instantiator bugs, wrong symbol names, ABI mismatch, and the whole `@SMLRZ` wire-format reshape — none of which are visible to a typechecker. Harmonious's cautionary tale is exactly this shape: a build that exited 0 while the linker resolved against stub `unreachable!()` bodies.

So the corpus needs both, and a case that *must not compile* is inherently tier-1-only. "Every case that can run in both, does" is the honest rule, and the outcome field encodes which is which.

## Tier 2 is blocked, and not on us

Worth being plain: today the interop cargo feature deliberately **doesn't link the C++ backend** (the `build.rs` gate), pending the LLVM 16 → ~21 port. And `valec` itself doesn't build while `backend_ffi` and `pass_manager` are out of `lib.rs` for the onion arc. So tier 2 needs both the port and the relink before it can run at all.

That's fine as long as we don't let it distort tier 1 — build the corpus now, shaped so tier 2 consumes it unchanged.

## Dropping the fixture oracle has a clean dependency chain

It isn't a separate project; it falls out:

1. Dark-box API on the rustc-hosted boundary + a `harness = false` runner.
2. Move `calls_a_rust_free_function` — our *only* in-suite test, and the fixture's only consumer — into the corpus.
3. Delete `FixtureOracle`. `Oracles::none()` already says "no oracle" without an implementation.

Step 2 is the whole gate. And deleting it removes a real ongoing tax: every trait change has to be mirrored into a fake that tests nothing — `importable_functions` needed adding to it, the `ValeSigType` change touched it, and it's precisely the "decorator that inherits a default lies" hazard's cousin. A fake that must track a trait it doesn't exercise is drift with a maintenance bill.

## On "a lot"

I won't invent a number, but the axes multiply fast, and that's the point: signature shapes (primitives, generics, projections, references), item kinds (free function, method, drop, constructor), type shapes (concrete, generic, several, cross-crate), and failure modes (not imported, unrepresentable, ambiguous, wrong binding). ReiImpl's 32 is the order of magnitude to pass, not to match.

The precedent here supports the ambition — 229 typing tests, 109 end-to-end, 321 integration. Interop currently has **one**.

## What I'd build first

The corpus format and the tier-1 runner, in that order, because the format is the thing that's expensive to get wrong. Concretely: a fixture directory per case (Rust crate + Vale program + expected outcome), a dark-box function returning a structured outcome, and a `harness = false` test that walks the directory. Then port the existing driver program in as case one, and the fixture oracle's test as case two — at which point the fixture oracle has no consumers and goes.

Want me to start on the corpus format? That's the piece I'd want your eyes on before I write fifty cases against it.

---

## 🤖 Claude

Watcher armed — note it came up as **Vale4-raven** rather than `Vale4-opal`, since the old identity's registration had lapsed. Anything addressed to `opal` won't reach me now, so I'd use `raven` for further correspondence.

Standing state: seam collapse landed and green (573/175/8 default, 574/175/8 interop, driver exit 0, `compiler.rs` diff empty), nothing committed since `8d40eff9d`.

Open question from my last message: whether to start on the corpus format — a fixture directory per case (Rust crate + Vale program + expected outcome), a dark-box function returning a structured outcome, and a `harness = false` runner. That's the piece I'd want reviewed before writing cases against it.

---

## 👤 User

tier 1 also should test the compiler outputs, the typing pass output AST.

---

## 🤖 Claude

Agreed — "it typechecks" is too weak a signal for a lot of what we care about. But this collides with a constraint we've already hit, and I'd rather surface it than pick silently.

---

## 🤖 Claude

That requirement forces a real architectural fork, because of something I confirmed just now.

## The collision

`typing/mod.rs:56` is `#[cfg(test)] pub mod test;` — so `NodeRefT` and the `collect_*` macros exist **only** in the lib's own test target. They're invisible to an integration test (which links the lib as an ordinary dependency) and to a `[[bin]]` like the driver.

So **asserting on the typed AST forces tier 1 into `cargo test --lib`.** And that's exactly where running rustc is hazardous:

- `install_ice_hook` sets a **process-global** panic hook; `cargo test` runs tests as parallel threads in one process, so it races with the other 574.
- A rustc *fatal* error exits the process rather than returning — taking the whole suite with it.
- `run_compiler` establishes per-thread session globals, and I don't know whether calling it repeatedly in one process is safe. That was the original reason the driver is a binary.

Put bluntly: **AST access and process safety currently pull in opposite directions**, and I picked the out-of-process side back when we'd decided the Vale program could carry the assertions. Your requirement reopens it.

## Three ways out

**A — tier 1 lives in `cargo test --lib`.** Cases become ordinary `#[test]`s with full `collect_` access and proper libtest diffs. Requires not installing the ICE hook under test, and probably serializing the interop tests. Unknown: whether `run_compiler` tolerates repeated in-process invocation.

**B — tier 1 stays out-of-process, and the dark-box function returns a structured AST *summary*** extracted by a hand-written walk inside `rust_interop/`. Process-safe, satisfies your "production walks must be hand-written" rule, and it's DBAPIZ-shaped. Cost: we assert on a summary rather than the AST, so a wrong extractor makes tests pass wrongly — and it's a second thing to keep true.

**C — split**, most cases out-of-process for behavior, a few in-process for AST. Both sets of complexity.

## What I'd do

**A is strictly better if it works** — real AST assertions, ordinary tests, no summary layer to maintain, and tier 2 later reads the same fixtures out-of-process. It also deletes the fixture oracle immediately, since a real-rustc test in the lib test target replaces `calls_a_rust_free_function` outright.

Whether it works is a fact, not a judgement call, and it's cheap to establish: one `#[test]` in `typing/test/rust_interop/` that runs `run_compiler` over a fixture and asserts on the callee via `collect_only_tnode!`. If that passes, add a second and see whether two coexist in one process. That settles the architecture in one experiment, and it's the same code either way — if it fails I fall back to B knowing *why*, rather than having assumed.

Want me to run that experiment before we design the corpus format? It determines whether cases are `#[test]` functions or fixture directories, which is the format question you wanted eyes on.

---

## 👤 User

you know, eventually, we might be serializing the typing output AST to a file anyway. maybe we should move that up in the schedule?

---

## 🤖 Claude

Genuinely interesting — that would dissolve the fork rather than pick a side. Let me check what already exists, because the answer depends heavily on whether this is a dump or a round-trip.

---

## 🤖 Claude

Worth separating two things that got merged, because the answer differs sharply between them.

## Full serialization is genuinely the §7 project

`von/` is **103 lines** — just the `IVonData` value model (`Int/Float/Bool/Str/Object/Array`), and it's **commented out of `lib.rs:53`**. Its only consumers are the integration tests, which are themselves commented out, and they use it as Vivem's *result* type (`VonInt(42)`), not as a serializer.

**VonHammer was never ported.** Nothing writes a `.vast`. And in the Scala tree VonHammer converts the *final* H-level AST, not `HinputsT` — so even that precedent is at the wrong level for "typing output."

The scale is visible in the tree: `traverse.rs` is **1,610 lines** to walk the typed AST once. A dumper is a second walk of the same shape — and it can't reuse `traverse.rs`, both because that's `#[cfg(test)]` and because your rule says production walks are hand-written. So the arch doc's "a project, not a task" reads accurate, and it'd block interop testing behind it.

## But tier 1 doesn't need full serialization

What interop actually asserts on is small: the callee of each call, whether a body is an `ExternFunctionCall` and to what, where drops were inserted, what a generic instantiated at. That's a targeted extraction — order of a couple hundred lines, not fifteen.

And here's the convergence worth noticing: **that targeted dump is exactly option B from my last message** — the dark-box function returning a structured output. You arrived at it from serialization, DBAPIZ arrives at it from the testing boundary, and they're the same object.

Which resolves the fork properly rather than by picking a side: the runner stays out-of-process (safe from `run_compiler` and the global ICE hook), *and* tests assert on the AST — because the AST facts come across the boundary as data.

## The bit that makes it not-throwaway

Emit it as **`IVonData`**, not a bespoke struct.

That's already the tree's serialization value model, it's 103 lines, it needs no new dependency, and full serialization later produces the same shape — so the targeted dump becomes a *projection* of the eventual full one rather than something to throw away. It also kills the stringly-typed problem outright: assertions become tree queries, not `l.contains("generics [")`.

Cost: uncommenting `lib.rs:53`. Worth checking that doesn't drag the integration tests in with it.

## So: move it up, but the small end of it

I'd not move full `HinputsT` serialization up — it's a big rock, it's blocked behind nothing we need today, and its real justifications (the two-binary split needing the typed AST to cross a process boundary, the §7 cache) are separate from testing.

I *would* build the targeted `IVonData` extraction now, as the dark-box output, shaped as a subset of the eventual full dump. It unblocks tier 1 with AST assertions, it's the thing DBAPIZ wants anyway, and when full serialization lands it subsumes this rather than competing with it.

One honest caveat: a hand-written extractor can be wrong, and then tests pass wrongly. The mitigation is that its output is *also* what the driver prints, so a human reads it every run — same property that made the oracle log catch the vacuous pass.

Want me to prototype the extraction over what the current driver already produces, so you can see the `IVonData` shape before I commit to it?

---

## 👤 User

we shouldnt bring back von and vonhammer, we should just use rust's normal json capabilities i think.
tier 1 doesnt need serialization? i thought you were saying that we couldnt inspect the typing ast because it lives in another process (rustc's)?

---

## 🤖 Claude

You're right — I muddied that, and the distinction I was drawing wasn't the one I wrote.

**Tier 1 does need serialization.** The typed AST is built inside `after_expansion`, in arenas that die with the callback, in rustc's process. Any fact a separate runner asserts on has to cross that boundary, and crossing it *is* serialization. What I meant was "not full `HinputsT` serialization" — I wrote "extraction" for the subset, which hid that it's the same mechanism at smaller scope. My fault.

And agreed on `serde_json` over von. Note there's nothing to "bring back" either way — `von` is commented out of `lib.rs:53`, so it'd be *adding* it, not restoring it. Rust's own JSON is the better call.

So the real question is the one you were asking: **targeted subset, or the whole typing AST?** Let me check the obstacle that decides it.

---

## 🤖 Claude

Good — the obstacle is small and localized, which changes my recommendation toward yours.

## `HinputsT` is almost entirely serializable

It's `Vec`s and `HashMap`s of definitions — interfaces, structs, functions, edge blueprints, instantiation bounds, exports. All of it is arena refs (`&'t`), and serde serializes `&T` fine one-way. No round-trip means `MustIntern` and pointer identity don't matter at all — those only bite on *reload*.

**Exactly one cycle risk**, and it's a single field: `FunctionHeaderT::maybe_origin_function_templata: Option<FunctionTemplataT>`, where `FunctionTemplataT` holds `outer_env`. Follow that and you're into environments, which reference `parent_env` and `global_env` and recurse forever.

The cut is one `#[serde(skip)]`, or better, reducing it to the templata's id — which is the only part a test would want anyway.

That's the whole obstacle. Not fifteen hundred lines of hand-written walk — `#[derive(Serialize)]` down the definition types plus one deliberate cut.

## Which flips my recommendation

I argued for a targeted extractor on the assumption that full serialization was the §7 project. It isn't — **§7 is about a *cache*, which needs round-trip**: reload, re-intern, restore pointer identity. That's genuinely hard and genuinely a project. A one-way dump shares almost none of that difficulty, and I conflated them.

So: do the full dump. The argument that decides it is your own — you want *a lot* of tests. A targeted extractor means every new assertion risks "the extractor doesn't surface that yet," and you find out while writing the test. That's a bottleneck that gets worse exactly as the corpus grows. A full dump is written once and every subsequent test is free.

It also pays for itself beyond testing: it's the debugging artifact for "what did the typing pass actually produce," which right now nobody can see; ReiImpl's `.vast` files served exactly that role when the agent surveyed them.

## What it costs

`serde` + `serde_json` as dependencies, and `#[derive(Serialize)]` across the typed-AST types. That's wide but shallow — and it's core-tree churn on files interop otherwise doesn't touch, so it's your call rather than mine.

Two things I'd want to check before committing: whether any *other* type in the definition graph reaches an environment (I checked `ast.rs` and `citizens.rs`; expression types are the remaining surface), and whether `IdT`'s interned-pointer representation serializes to something stable and readable rather than an address.

Want me to do that check and come back with the concrete diff shape — which types need the derive, and where the cuts land?

---

## 👤 User

can you make this happen with only changes in rust_interop? i do eventually want to serialize typing pass outputs, but im not ready to design that into vale core properly quite yet.

---

## 🤖 Claude

Yes — and it's cheaper than that, because there's a path where tier 1 needs **no serialization at all**, and we haven't tested whether it's open.

## First: the experiment we skipped

Two messages back I proposed settling whether `run_compiler` can be called from a `#[test]` in the lib's own test target. We went down the serialization road instead, and that question is still unanswered — but it dominates the cost.

If it *can*: tier 1 lives in `cargo test --lib`, where `collect_*` and `NodeRefT` are already available. The AST is inspected in-process, never crosses a boundary, and **needs no serialization, no serde, and no core changes.** Cases become ordinary `#[test]`s with libtest diffs.

The three risks are each addressable or checkable: don't install the global ICE hook under test; fatal rustc errors only fire on fixture crates we control; and repeated in-process invocation is the one genuine unknown. That last is a 20-minute experiment — one test, then two, and see if they coexist.

Given you're not ready to design serialization into core, that's the path worth eliminating first, because if it works the whole question dissolves.

## If it doesn't: yes, rust_interop-only, hand-written

No derives on core types. A walk in `rust_interop/` reading the public fields of `HinputsT` and emitting `serde_json::Value` — the definition types are all `pub`-fielded, which is how we've been reading them all session.

Two things that make this less bad than the 1,610-line figure suggests:

- **The cycle is one field.** `FunctionHeaderT::maybe_origin_function_templata` is the only route from a definition into an environment. Reduce it to the templata's id and the graph is acyclic.
- **It can start small and grow.** One function per node type, added as tests demand. That reintroduces the "extractor doesn't surface that yet" bottleneck you'd rightly want to avoid — but it's a bottleneck we *control*, entirely inside our module, rather than a core commitment made early.

Worth naming a third option honestly: `serde`'s remote-derive lets us define mirror types in `rust_interop` that generate ser code for core types without annotating them. More systematic than free-hand, and field renames in core become compile errors here rather than silent drift. Costs a mirror per type, so it's only worth it if the dump goes wide.

## What I'd do

Run the experiment. If `run_compiler` survives libtest, take that — zero core changes, zero serialization, full AST access, and it deletes the fixture oracle in the same move since a real-rustc test in the lib test target replaces `calls_a_rust_free_function` directly.

If it doesn't survive, fall back to the hand-written JSON walk in `rust_interop`, and you keep the option of designing real serialization into core later without this having pre-empted it.

Either way nothing lands in core. Want me to run the experiment?

---

## 👤 User

run the experiment please

---

## 🤖 Claude

Now the experiment — two tests, since coexistence is the real unknown.

---

## 🤖 Claude

It compiles — `run_compiler` is reachable from the test target. Now the real question:

---

## 🤖 Claude

Test 1 passes — `run_compiler` *does* work from a `#[test]`, with `collect_only_tnode!` asserting against a real `TyCtxt`. Now, why did test 2 fail?

---

## 🤖 Claude

That's my assertion, not a coexistence failure — `(make_counter()).get()` has **two** calls and `collect_only_tnode!` demands exactly one. `run_compiler` reached `after_expansion` and built the AST fine the second time.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

please dont edit with scripts

---

## 🤖 Claude

Understood — Edit tool only from here.

---

## 🤖 Claude

The plural form wants an iterable; `collect_where_tnode!` (singular node, plural matches) is the right one.

---

## 🤖 Claude

Both pass serially. The remaining question is the default mode — parallel threads:

---

## 🤖 Claude

**The experiment succeeds. Option A is open.**

## Results

| | |
|---|---|
| `run_compiler` from a `#[test]` | **works** |
| `collect_where_tnode!` on the AST, against a real `TyCtxt` | **works** |
| Two `run_compiler` calls in one process, serial | **works** |
| Two in parallel (default libtest threads) | **works** |
| Full interop suite alongside all 574 others | **576 passed** / 175 / 8 |
| Default suite | 573 / 175 / 8, unchanged |

The interop count went 574 → 576. Both new tests run in the ordinary suite, in the ordinary process, with no `--test-threads=1` needed.

## What this means

**Tier 1 needs no serialization, no serde, and no core changes.** Interop cases become ordinary `#[test]`s in `typing/test/rust_interop/`, with full `collect_*` and `NodeRefT` access — the same boundary and the same tools as the other 229 typing tests. The typed AST never crosses a process boundary, so there's nothing to dump.

That also removes the reason we were discussing moving serialization up. You can design typing-pass serialization into core when you're ready to, on its own merits, rather than because testing forced it.

Three practical notes from doing it:

- **Don't install the ICE hook** in this path — it's a process-global panic hook and would hijack the other 574 tests' failures. Omitting it is all that's needed; assertions then report as ordinary test failures.
- **Assertions must run inside the callback**, because the arenas die with it. Only an owned verdict comes back out. That's a mild constraint, not a problem.
- **Per-case out-dirs** — I gave each test its own temp dir so parallel cases don't race on the same rlib path (@TMBFIZ). That's likely why parallel worked first try.

## What it unblocks

The fixture oracle can go now. A real-rustc test in the lib test target does everything `calls_a_rust_free_function` does and more, so once cases move across, `FixtureOracle` has no consumers and `Oracles::none()` already covers "no oracle."

And the corpus question simplifies: cases are `#[test]` functions with a shared `run_case` helper, not fixture directories with a schema — at least for tier 1. Tier 2 will still want fixture directories on disk when codegen lands, but that's a separate runner reading the same Rust crates, and it isn't blocked on getting a format right today.

One caveat I'd flag rather than bury: a *fatal* rustc error still exits the process and would take the suite down. Our fixture crates are ours, so it's controllable — but a malformed fixture is now a suite-wide failure rather than a single red test. Worth knowing before the corpus grows.

The experiment file is deliberately named as one. Want me to turn it into the real thing — fold the driver's cases in, delete the fixture oracle, and start growing the corpus?

---

## 👤 User

sorry im catching up. explain how it would work? how would we have collect_* and asserts in typing/test/rust_interop, how can they see into rustc's process?

---

## 🤖 Claude

Ah — that's the crux, and it's my fault for talking about "the boundary" so loosely. **rustc isn't a separate process.**

## The actual setup

We link rustc's own crates into our binary — that's what `rustc-dev` plus `#![feature(rustc_private)]` buys. `rustc_driver::run_compiler(...)` is an ordinary function call. rustc runs **in our process, on our thread**, and calls back into us:

```
#[test] fn my_case() {
    run_case(...)                                 // build fixture rlib, assemble rustc args
      └─ rustc_driver::run_compiler(args, cbs)    // rustc runs HERE, in-process
           └─ after_expansion(compiler, tcx)      // rustc calls back into us, live TyCtxt
                ├─ create Vale's arenas
                ├─ TyCtxtOracle::new(tcx, ...)    // read the live TyCtxt
                ├─ compiler_test_compilation_with_rust_oracle(...)   // Vale's typing pass
                ├─ collect_where_tnode!(...)      // walk the AST — it's right here in memory
                └─ record an owned verdict
    assert!(verdict)                              // back in the test
}
```

Nothing is serialized because nothing crosses a process. The `HinputsT` is built in arenas allocated on that stack frame, a few frames below the `#[test]`.

## The boundary I was actually worried about

Not rustc-vs-us — **the driver binary vs a test binary.** The driver is a `[[bin]]`, so a test checking its behavior would have to spawn it and read stdout, and *that's* what needed serialization. Putting the test in the lib's own test target removes the second binary entirely.

## Why the lib test target specifically

`typing/mod.rs:56` is `#[cfg(test)] pub mod test;`. So:

| where the test lives | rustc linked? | `collect_*` visible? |
|---|---|---|
| `[[bin]]` (our driver) | yes | **no** — `cfg(test)` is off |
| `[[test]]` integration test | yes | **no** — links the lib as an ordinary dep, `cfg(test)` off |
| `cargo test --lib` | yes | **yes** |

The lib's own test target is the *only* place both are available. That's the whole finding — and the experiment shows rustc tolerates being hosted there, twice, in parallel.

## The one real constraint

`TyCtxt<'tcx>` and Vale's arenas both die when `after_expansion` returns. So the walking and any borrowing must happen **inside** the callback; only owned data comes back out. That's why `ExperimentResult` holds `Vec<String>` and `bool` rather than `&PrototypeT`.

In practice it means assertions live in the callback and the `#[test]` checks a verdict — slightly less direct than a normal typing test, but the `collect_*` call itself is identical to the other 229.

## Why we thought otherwise

The driver was made a `[[bin]]` on Harmonious's advice that `run_compiler` owns the process — they wrap it in `catch_with_exit_code` + `process::exit` and never call it from a test. That's real for *fatal* rustc errors, which still exit and would take the suite down. But "can't be called from a test at all" was an inference nobody had tested, and it turns out to be false.

---

## 👤 User

how did we make #[test]s run inside after_expansion?

---

## 👤 User

how did we make #[test]s run inside after_expansion?
also, we can run the rust compiler in process? i thought you said a while ago it didnt like that?

---

## 🤖 Claude

Two good questions, and the second one is a correction I owe you.

## How the `#[test]` relates to `after_expansion`

The `#[test]` doesn't run *inside* it — it wraps it. `ExperimentCallbacks` is a plain struct we own, handed to `run_compiler` as `&mut`. rustc calls our `after_expansion` method on it, we write into `self.result`, and when `run_compiler` returns the test reads that field:

```rust
let mut callbacks = ExperimentCallbacks { vale_source, allowed, result: Default::default() };
rustc_driver::run_compiler(&rustc_args, &mut callbacks);   // rustc calls after_expansion in here
callbacks.result                                           // ...and we read what it recorded
```

So the split is: **the AST walk happens in the callback** (it must — that's the only place the arenas and `TyCtxt` are alive), and **the `assert!` happens in the `#[test]`** on an owned verdict. That last part is deliberate: a panic inside the callback happens within rustc's frame, which is what produced those fake-ICE reports earlier.

It's the ordinary Rust callback pattern; nothing clever. The only unusual bit is that the interesting work happens on the far side of someone else's function call.

## On "rustc doesn't like that" — I overstated it

What was actually established, from Harmonious, was narrower than what I turned it into:

- a rustc **fatal error** calls `process::exit` rather than returning — so it would take a whole test binary down;
- `install_ice_hook` replaces the **process-global** panic hook;
- `run_compiler` sets up **per-thread session globals**, and repeated invocation was an open question.

They said they wrap it in `catch_with_exit_code` + `process::exit` and never call it from a test. I generalized that into "rustc can't be hosted in a test process," and chose the binary on that basis. Two of those three turn out to be handled trivially: the ICE hook by simply not installing it, and repeated invocation by it just working — serially *and* in parallel.

Worth noting we've been running rustc in-process this whole time — the driver links `rustc_driver` and calls `run_compiler`. The question was never "can rustc share our process," only "can it share the *test* process," which is mechanically identical; the difference is what else is in there.

**The one hazard that survives is the first.** A fatal rustc error still exits, so a malformed fixture would kill the run rather than fail one test. Our fixture crates are ours, so it's controllable — but it's a sharper edge than a normal test, and it gets sharper as the corpus grows.

I'd like to measure that rather than leave it as a caveat: point a case at a fixture whose Rust source doesn't compile, and see whether the suite survives, dies, or is rescued by `catch_with_exit_code` (which the driver uses and my experiment deliberately doesn't). That's a five-minute check and it tells us whether corpus growth needs a guard. Want me to run it?

---

## 👤 User

file/line of test plz

---

## 🤖 Claude

All in one file: **`FrontendRust/src/typing/test/rust_interop/in_process_rustc_experiment.rs`**

| line | what |
|---|---|
| `:171` | `fn experiment_one_run_compiler_in_a_test()` — the `#[test]` at `:170` |
| `:183` | `fn experiment_two_a_second_run_compiler_in_the_same_process()` — `#[test]` at `:182` |
| `:166` | `rustc_driver::run_compiler(&rustc_args, &mut callbacks)` — where rustc is invoked |
| `:63-64` | `impl Callbacks for ExperimentCallbacks` / `fn after_expansion(&mut self, _compiler, tcx)` — where rustc calls back in |
| `:97` | `collect_where_tnode!(...)` — the AST walk, inside the callback |

Reading order for the flow is `:171` → `:166` → `:64` → `:97`, then back out to the assert at `:177`.

It's wired in at `FrontendRust/src/typing/test/rust_interop/mod.rs:1`, and both tests run in the ordinary suite — interop is at 576 passed with them, was 574.

The file is named as an experiment on purpose; nothing else depends on it, so it's cheap to delete or promote.

---

## 👤 User

do we need one of these:
```
impl Callbacks for ExperimentCallbacks {
    fn after_expansion<'tcx>(&mut self, _compiler: &RustcCompiler, tcx: TyCtxt<'tcx>) -> Compilation {
```
for every single test?

---

## 🤖 Claude

No — and the file already demonstrates that. There's **one** `impl Callbacks`, shared by both tests. `ExperimentCallbacks` carries what varies as fields:

```rust
struct ExperimentCallbacks {
    vale_source: &'static str,
    allowed: &'static [&'static str],
    result: ExperimentResult,
}
```

and `run_case(out_dir_name, vale_source, allowed)` at `:138` constructs it. Both `#[test]`s are three lines each — they call `run_case` and assert on the returned verdict. Adding a fiftieth case adds no callbacks machinery at all.

## What would actually vary per case

Two things are already parameters (source, allowlist). The third is *what you extract* — right now `after_expansion` hardcodes "collect every `FunctionCall` under `main`, check they're rust-backed."

The instinct is to pass a closure. **That's a trap here**, because of the lifetimes: `HinputsT<'s, 't>` is bound to arenas created inside the callback, so a per-case extraction closure would need higher-ranked bounds over `'s` and `'t`. Possible, unpleasant, and it'd infect every call site.

The shape that works with the grain is one rich verdict:

```rust
struct CaseOutcome {
    compiled: Result<(), CompileErrorKind>,
    callees: Vec<CalleeFacts>,     // name, params, return, rust-backed
    oracle_log: Vec<String>,
    // ...grows as cases need it
}
```

One callbacks impl, one extraction, tests assert on whichever fields they care about. When a case needs a fact the verdict doesn't carry, you add a field once and every case can use it.

That's the same "extractor has to anticipate" tradeoff I was worried about when this involved serialization — but it's cheap now. No serde, no schema, no process boundary; adding a field is a struct member and three lines in the callback. The bottleneck only bit when crossing a boundary made each addition expensive.

So the corpus shape is: one shared harness (`run_case` + one `Callbacks` impl + one `CaseOutcome`), and each case is a Vale program, an allowlist, and an assertion. Which is about as close to the other 229 typing tests as this can get.

---

## 👤 User

yeah i do want to pass a closure. can we pass a `for` closure? so we dont need to say its lifetimes up front

---

## 🤖 Claude

Worth trying rather than reasoning about — HRTBs can't express the `'s: 't` bound between quantified lifetimes, so it may or may not work. Let me check the shape first.

---

## 🤖 Claude

No `where 's: 't` on the struct — so the HRTB has a real chance. Let me try it.

---

## 🤖 Claude

The HRTB itself is **accepted** — both errors are just `Send`. `run_compiler` requires it, which incidentally reveals it runs the compilation on its own spawned thread (and explains why the per-thread session-globals worry never bit).

---

## 🤖 Claude

**Yes — `for<'s, 't>` works.** Both tests pass with the closure design.

The signature that does it:

```rust
fn run_case<R: Send>(
    out_dir_name: &str,
    vale_source: &str,
    allowed: &[&str],
    extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> R
```

and a case is now self-contained:

```rust
#[test]
fn experiment_one_run_compiler_in_a_test() {
    let rust_backed: Vec<bool> = run_case(
        "vale-interop-experiment-1",
        "\nexported func main() int {\n  return add_two_numbers(3, 4);\n}",
        &["add_two_numbers"],
        |coutputs| {
            let main = coutputs.lookup_function_by_str("main");
            let callees: Vec<&PrototypeT> = collect_where_tnode!(
                NodeRefT::FunctionDefinition(main),
                NodeRefT::FunctionCall(call) => Some(call.callable)
            );
            callees.iter().map(|c| is_rust_backed(&c.id)).collect()
        },
    );
    assert_eq!(vec![true], rust_backed);
}
```

No shared verdict struct to anticipate everything, no `CaseOutcome` to grow. Each case asks for exactly what it wants.

## Two things worth knowing from making it work

**The HRTB does more than avoid naming lifetimes — it enforces the safety rule.** `R` is fixed *outside* the `for<..>`, so it structurally cannot mention `'s` or `'t`. "Only owned data escapes the callback" stops being a discipline and becomes a compile error. That's better than the verdict-struct design, which relied on remembering.

**The only obstacle was `Send`, not lifetimes.** `run_compiler` requires `Callbacks: Send`, so `F` and `R` need it too. That's a small tax — and it incidentally tells us rustc runs the compilation on its **own spawned thread**, which explains why the per-thread session-globals worry never materialised: each `run_compiler` gets a fresh thread rather than reusing ours.

So the corpus shape is settled: one `run_case` helper, one `Callbacks` impl, and each case is a Vale program, an allowlist, and a closure — about as close to the other 229 typing tests as this can get, with real `collect_*` assertions against real rustc.

The residual hazard is unchanged and still worth measuring: a *fatal* rustc error would exit the process and take the suite with it. Want me to test that next, or start folding the driver's cases in?

---

## 👤 User

yep test it

---

## 🤖 Claude

Now the hazard test itself:

---

## 🤖 Claude

It passed — which means my fixture didn't test what I claimed. `after_expansion` runs **before type checking**, and we return `Compilation::Stop`, so rustc never reaches the type error. My fixture comment had the reasoning backwards.

A parse error happens *before* expansion, so it can actually reach us:

---

## 🤖 Claude

A genuine parse error, and the test **failed cleanly** — the harness survived. Let me see the mechanism, then confirm the blast radius on a full run.

---

## 🤖 Claude

**The hazard is smaller than feared.** A fatal rustc error fails *one test*, not the run.

## Results

| | |
|---|---|
| parse error in the stub crate, run isolated | **1 failed**, harness survived |
| same, in the full suite | **576 passed / 176 failed** — the 175 baseline plus exactly this one |

No process death, no lost run. rustc printed `error: this file contains an unclosed delimiter` / `error: aborting due to 1 previous error`, `run_compiler` returned, `callbacks.result` was `None`, and our `expect("after_expansion never ran, so nothing was extracted")` turned that into an ordinary test failure.

So the corpus needs **no special guarding** as it grows. That was the last open objection to hosting rustc in `cargo test --lib`.

## And a correction worth keeping

My first attempt at this fixture used a *type* error and **passed** — which would have been a false all-clear. The reason is structural: `after_expansion` runs **before type checking**, and our callback returns `Compilation::Stop`, so rustc never reaches the type-check phase. Type errors in a fixture's Rust source are therefore invisible to us by construction.

That cuts both ways and is worth knowing for the corpus:

- **Good:** a fixture crate that type-errors can't break a case, because we halt first. Fixtures only need to *parse*.
- **Bad:** we get no signal that a fixture is type-broken. A fixture could rot into invalid Rust and every case would keep passing, reading signatures from a crate that would never compile for real. Tier 2 would catch it; tier 1 structurally cannot.

The mitigation is nearly free: `build_dep_rlib` already compiles `mycrate.rs` with a real `rustc` subprocess and asserts success. Doing the same for `stub.rs` — a cheap syntax-and-type check before the in-process run — would close it. Not needed today; worth doing before the corpus has fifty fixtures nobody re-reads.

I've left `fixtures_broken_rust/` and `experiment_three` in place so you can see the result, but that test is *expected* to fail by design — I'd delete both, or invert it into a proper negative test asserting the failure mode, before anything is committed. Which would you prefer?

---

## 👤 User

lets wind down. capture everything about where we are and where we're going and our plans into the handoff doc and architecture doc please

---

## 🤖 Claude

No handoff doc exists — `synthesized-declarations-plan.md` has been serving that role, so I'll make it explicit. Let me find the arch doc spots that need correcting.

---

## 🤖 Claude

Both arch-doc claims need correcting — §8.10 says "no fabricated definitions" and describes the candidate-source seam, and both are now false. Adding a revision block rather than rewriting prose in place:

---

## 🤖 Claude

Now the testing strategy, which the doc has no home for yet:

---

## 🤖 Claude

Now the handoff doc — rewriting the plan doc to serve that role explicitly.

---

## 🤖 Claude

Captured. Both docs updated.

## Architecture doc

**§8.10 gained a revision block.** Option A's *naming* claim held up and is untouched — but four claims about how the typing pass *reaches* a Rust item were superseded, and they were stated as decisions, so they needed correcting rather than quietly aging: "no fabricated definitions" is now reversed, the fourth candidate source and both triggers are retired, methods are not special, and `fn_sig`'s @EarlyBinder discipline is inverted. The "why a candidate source and not a fallback" subsection is marked historical.

**New §26b — Interop Testing Strategy.** The doc had no home for testing, so this is a new section: the two tiers and why they're not nested, that every case runs in both where it can, that tier 2 is blocked on the LLVM port and the onion relink, and the measured facts about hosting rustc in `cargo test --lib` — including the four consequences that bite (never install the ICE hook, assertions inside the callback, the HRTB extractor and why `Send` is required, per-case out-dirs), plus the one that's easy to miss: `after_expansion` runs before type checking, so a fixture that type-errors is invisible to tier 1. And §26b.3 records that there is deliberately no fixture oracle.

## Handoff doc

`synthesized-declarations-plan.md` rewritten as state/plan/handoff rather than a pivot narrative — README updated to match. Eight sections: the design in one page (with why the previous one failed), what's in the tree and its baseline, decisions locked, **verified facts worth not rediscovering**, the testing plan with the next five steps in order, known defects, what's blocked elsewhere, and the @SMLRZ trap.

The facts section is the one I'd most want a future session to read first — it's the things that cost real time to establish and are cheap to get wrong again: that `StructDefinitionT` needs no `StructS`, that methods are already just functions, that the inner env isn't only placeholders, that `sibling_entries` aren't what they sound like, and that prototypes-in-environments is a real Vale shape but only for bounds.

## Final state

| | |
|---|---|
| default | 573 / 175 / 8 |
| interop | 576 / **176** / 8 |
| core diff | empty |

**One deliberate red**: `experiment_three_a_fatal_rustc_error` is designed to fail — it's the unparseable fixture proving a fatal rustc error costs one test rather than the run. That's the 176th. It and `fixtures_broken_rust/` should be deleted or inverted into a proper negative test before anything is committed; the handoff doc says so in §2.

Nothing committed since `8d40eff9d`.

---

## 👤 User

<command-message>close-session</command-message>
<command-name>/close-session</command-name>

---

## 🤖 Claude

Docs first — one number in the handoff is now stale.

---

