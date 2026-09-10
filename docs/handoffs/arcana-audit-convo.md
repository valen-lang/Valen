# Claude Conversation Log

Session ID: f5574520-2c58-41ec-9609-0e8bb19f53b7
Date: 2026-09-07 00:24:14

---

## 👤 User

<command-name>/model</command-name>
            <command-message>model</command-message>
            <command-args>fable</command-args>

---

## 👤 User

<local-command-stdout>Set model to `Fable 5.1` and saved as your default for new sessions</local-command-stdout>

---

## 👤 User

note that im doing changes in this directory, but disregard that, because youll be helping me with some read-only investigations.
first, can you read good-doc.md and meta.md and get a handle on what arcana is? we'll be doing a lot of updating of arcana this session

---

## 🤖 Claude

I'll locate and read both files.

---

## 🤖 Claude

good-doc lists good-arcana as required reading, so I'll read that too.

---

## 🤖 Claude

Read all three: `docs/skills/good-doc.md`, `docs/meta.md`, and `docs/skills/good-arcana.md` (good-doc's required reading). Here's my working understanding of arcana:

**What arcana is**

- One of nine doc categories in `docs/meta.md`. It documents a *local* thing with surprising, non-obvious effects *elsewhere* in the codebase. Not background, not usage, not architecture; specifically the cross-cutting surprise.
- Each has a unique ID: uppercase initialism of the title words plus a `Z` suffix (shields use `X`). File name is `docs/arcana/<HammerCaseTitle>-<ID>.md`, e.g. `SealedInternedConstruction-SICZ.md`.
- Lives in the `docs/arcana/` of the feature that *causes* the effect, not where the effect is felt. Project-agnostic ones go in `Luz/arcana/`.
- Discovery is via `@ID` comments at every affected code site. Each reference must be inside a sentence, never bare, and kept to about one line. The doc is the full explanation; the comment just says "this is an instance of @ID".
- The `Z` suffix is also used for "advisory" docs that have no `@ID` backlinks. Those must be wired explicitly via `g_mention_in`, `g_auto_load_when_editing`, or a `## See also` link, or they're invisible.

**Writing rules from good-arcana**

- Two approval gates: title+ID first, then the drafted body inline in chat before writing to disk.
- One markdown section, no subheaders. Use bold lead-ins like `**Interactions with X:**` if structure is needed.
- Focus on *why*, not *what*. No tactical call-chain narration, no line numbers. Refer to functions, types, modules by name.
- No reader-framed sections ("cross-cutting effect:", "why this looks surprising"). State the rule as plain facts about the code.
- Include at least one concise example. Mention related arcana in prose, not as a separate section.
- Mandatory prose-reviewer rewrite pass of the whole thing after drafting.
- Then find every code site and add `@ID` references.

**Current inventory**

| Location | Count |
|---|---|
| `docs/arcana/` | 21 |
| `Luz/arcana/` | 7 |
| `src/postparsing/docs/arcana/` | 3 |
| `src/solver/docs/arcana/` | 1 |
| `Backend/docs/arcana/` | 2 |

A few files in `docs/arcana/` don't follow the naming convention (`DropFree.md`, `Generics.md`, `Environments.md`, `Impls.md`, `IRegion.md`, `ret-vs-panic-locals.md`, and `DefaultRulesShouldBeIncrementalNotInitial-DRSINI` lacks the `Z`). Worth knowing if we touch those.

Ready for the investigations.

---

## 👤 User

i want us to find all arcana, even malformed arcana. for example, there's some in docs/old. and there's a lot referenced from the code that ive lost track of.
for example, a lot of stuff mentions arcana that's in docs/old/Environments.md. look at that file and look for references real quick to see what i mean

---

## 🤖 Claude

I see the pattern. `docs/old/Environments.md` itself doesn't exist, but the situation you describe is real:

**Old-style arcana** live as `## Title (INITIALISM)` sections inside multi-topic files, with no Z suffix and no `@` prefix. Code refers to them as bare `See WTHPFE` / `see UCRTVPE` comments.

**Where the Environments ones actually are:**

| Initialism | Defined in | Code refs |
|---|---|---|
| WTHPFE | `docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md` only | 4 in `src/typing/env/function_environment_t.rs` |
| BEAFB | same old file only | 4 in `src/typing/expression/expression_compiler.rs` |
| UCRTVPE | `docs/arcana/Environments, Templatas, and Parent Entries.md` (migrated) and the old file | 4 in `function_environment_t.rs` |
| CODME, IMRFDI | `docs/arcana/Environments.md` | 0 |
| ACTE, DBTSAE, LNASC | old file only | 0 |

So WTHPFE and BEAFB are referenced from live code but only defined in `docs/old/`. The old file also has unlabelled sections (Templatas Must Remember Environment, Must Scan For Declared Runes First, etc.) that may be referenced by name rather than initialism.

**Proposed sweep** to find everything:

1. Grep all `.md` files (including `docs/old/`, `Luz/`, `src/*/docs/`) for headers of the form `(INITIALISM)` to collect every defined ID, old and new style.
2. Grep all `.rs` files for `@[A-Z]{3,}Z?` and bare `[Ss]ee [A-Z]{3,}` to collect every referenced ID.
3. Cross-reference: referenced but undefined, defined but unreferenced, defined only in `docs/old/`, defined in two places, malformed names or missing Z.

This is read-only and parallelizable, so I'd fan it out to a few Explore agents and consolidate into one table. Say the word and I'll run it.

---

## 👤 User

im sure we had a markdown doc talking about environments and stuff...

---

## 🤖 Claude

Here's the picture for the Environments docs specifically:

**Three files, two lineages.**

| File | Origin | Contents |
|---|---|---|
| `docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md` | Original Scala-era compiler doc (commit "Adding compiler docs") | 13 sections, 426+ lines. Only 5 have initialisms: WTHPFE, ACTE, DBTSAE, LNASC, BEAFB. UCRTVPE appears as a plain header. |
| `docs/arcana/Environments, Templatas, and Parent Entries.md` | Written fresh in "Added restackify" commit, 32 lines, same title as the old doc | Only UCRTVPE. Never a copy of the old file. |
| `docs/arcana/Environments.md` | Was `docs/Environments.md`, moved during the repo flatten | IMRFDI and CODME. |

So the doc you remember is the `docs/old/Compiler/Templar/` one. Live code references WTHPFE and BEAFB, and both are defined *only* there. UCRTVPE has code refs and a proper (if old-style) arcana file. CODME, IMRFDI, ACTE, DBTSAE, LNASC have zero code refs.

The `docs/old/Compiler/Templar/` directory also has `Templar.md` with another 10+ initialism sections (DFSO, SDFF, ADSDF, NSIDN, HWDD, TIFA, GDFAK, DOAOC, NDFETT, NIIRII) and `docs/HigherTypingPass.md` has KVCIE, MFDBRE, RTMHTP, STRAST. Same shape, likely the same problem.

This confirms the sweep is worth doing across all of `docs/`, `docs/old/`, `src/*/docs/`, `Luz/`, and `Backend/docs/`. Want me to fan it out now? I'd collect every `(INITIALISM)` header and every `@ID` / `see ID` code reference, then cross-reference into one table: referenced-but-undefined, defined-only-in-old, defined-twice, unreferenced, and malformed names.

---

## 👤 User

yep, ultracode fan out and identify all arcana everywhere. even ones that are mentioned but dont seem to exist, those will be important to know about too.
context: after that, i'll want us to work together to gauge how accurate they all are, but we'll get to that when we get to that.

---

## 🤖 Claude

Quick inline scout first to size the buckets, then I'll fan out.

---

## 🤖 Claude

The census workflow is running: 9 definition sweepers and 6 reference sweepers in parallel, then one resolver per orphan ID, then 4 critics with different lenses, then a second resolve round for anything the critics surface. All agents are instructed read-only. I'll consolidate the result into a report in the scratchpad when it lands.

---

## 👤 User

hold once theyre back please

---

## 🤖 Claude

You've hit your session limit · resets 9:20pm (America/New_York)

---

## 🤖 Claude

You've hit your session limit · resets 9:20pm (America/New_York)

---

## 👤 User

looks like only some finished. can you redo the ones that didnt finish, but using sonnet agents (effort low) instead of fable agents?

---

## 🤖 Claude

The 13 that finished were all 9 definition sweepers plus the 4 code reference sweepers. Everything downstream failed on the session limit. I'll edit the script so only the failed agents switch to sonnet at low effort, and resume from the run so the 13 completed ones replay from cache.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

no, please make all of them sonnet. i dont want any fable subagents ever again. ty

---

## 🤖 Claude

Understood, all agents on sonnet at low effort from now on. That means the 13 completed ones rerun too, since their options change.

---

## 🤖 Claude

Relaunched with every agent on sonnet at low effort. All 47-ish agents rerun from scratch since the model change invalidates the cache. I'll hold and consolidate the census into a report when it completes.

---

## 🤖 Claude

All 66 agents completed with no errors. Consolidating the raw JSON into a report.

---

## 🤖 Claude

Report built. Pulling the last two sections I haven't seen, then I'll summarize.

---

## 🤖 Claude

Census done and holding. Full report is at `tmp/arcana-census.md` (490 lines, 10 sections, every ID with title, file, and ref counts), raw agent data in `tmp/arcana-census-raw.json`. All 66 agents were sonnet at low effort, no failures.

**Headline counts**

| Bucket | IDs |
|---|---|
| A. Modern arcana: Z suffix, own file in an `arcana/` dir | 28 |
| B. Old-style sections inside `docs/arcana/` multi-topic files (Generics.md, Impls.md, Environments.md, DropFree.md, IRegion.md, ret-vs-panic-locals.md) | 73 |
| C. Z-suffix IDs defined inline in non-arcana docs (mostly `docs/architecture/vale-rust-interop-architecture.md`, plus borrowing-design.md, Backend/backend-design.md) | 14 |
| D1. Defined only in `docs/old/` but cited from live code | 18 |
| D2. Defined only in `docs/old/`, cited only from other docs | 6 |
| D3. Defined only in `docs/old/` or git history, cited nowhere | 74 |
| E. No-suffix IDs in other live docs (HigherTypingPass.md, docs/notes, Guardian zen files) | 54 |
| F. Referenced but defined nowhere | 34 |
| G. Shields (X suffix), listed for completeness | 63 |

**D1 is the urgent list.** Live code cites these and the only definition is under `docs/old/`: WTHPFE, BEAFB, LNASC, MSFDRF, OFCBT, NTKPRR (Environments doc), SAFHE and CSFMSEO (Environments, Closures, Overload Sets), RMLRMO (Addresses.md), IRFU, SAIRFU, SRCAMP (Infer Templar.md), LDNEIR (Parser_Scout.md), and the Backend ones MPESC, ULTMCIE, CRCISFAORC (Midas.md), SRCAO/SRCAZ (Externs and Regions.md).

**F, the lost ones, split three ways:**
- Resolver verdict "lost arcana", cited from code, no definition anywhere: SRIE, MDRTCUT, GROUPS, MCFBRBF, RPPFNG, ITN16BA, SITTX, LNTKTR, IPOMFIC, PRCCBIVRZ. The resolver reconstructed a probable title for most from the citing comment.
- Arcana files deleted in git: CADFRTZ and RRPGRZ (commit 82f9ec4b), ICIPCRZ (01652f58). Also LNTKTR was in a deleted `docs/regions/Regions.md`.
- Noise: TODO tags (CRASTBU, VTBRX), shield template placeholders (CODEX, OTHERX, TESTX), typos of shields (NAOCSCX, NMTSFX), and PAFFIB which is a truncation of AFPAFFIB.

**Caveats the critics raised** that need a human eye: VCOORD collides with the V-marker convention so its refs are unreliable; MSFDRF and LDNEIR code citations are short acronyms that could be coincidental; Generics.md's 60-odd IDs weren't individually spot-verified; docs/old "definitions" where the `(ID)` sits alone on the line below the header were caught, but prose-inline forms may still be under-counted.

---

## 👤 User

for each lost ones, please dispatch sonnet low agents into the git history to figure out what these were referring to

---

## 🤖 Claude

I'll pull each lost ID's citations from the census and hand one sonnet/low agent per ID into git history.

---

## 🤖 Claude

Dispatched 14 sonnet/low agents, one per lost ID: SRIE, MDRTCUT, GROUPS, MCFBRBF, RPPFNG, ITN16BA, SITTX, LNTKTR, IPOMFIC, PRCCBIVRZ, CADFRTZ, RRPGRZ, ICIPCRZ, UTAIRZ. Each walks `git log -S` for the ID, extracts any deleted definition verbatim, blames the citing comment back to its origin, checks the Sky prototype repo read-only for the Sky-attributed ones, and judges whether the concern still holds in the current code. The IPOMFIC agent also covers its undefined neighbours in the instantiator-design acronym list (TTTDRM, HCCSCS, NPFCASTN, DDSOT, NNSPAFOC, TNAD). I'll write the findings into the census report when they land.

---

## 🤖 Claude

All 14 came back. Appending them, including the recovered verbatim definitions, to the report.

---

## 🤖 Claude

Done. All 14 resolved; findings and recovered definition texts are in section K of `tmp/arcana-census.md`.

**Definitions recovered verbatim from git history (7)**

| ID | Title | Lost in | Still true |
|---|---|---|---|
| SRIE | Specifying Regions In Expressions | `docs/regions/Regions.md`, deleted by the FrontendRust flatten (24695a17) | yes |
| LNTKTR | Lookups Need To Know Their Region | same file, same commit | yes |
| IPOMFIC | Ignore Previous Ownerships Mutabilities From Instantiated Coords | `docs/InstantiatorRegions.md`, deleted in instantiator cleanup (d097d488) | partially |
| ITN16BA | ITables Need to be 16-Byte Aligned | `docs/SeparatedFFI.md`, deleted with Fearless FFI (b9a4dcdf) | yes, the alignment call still exists |
| ICIPCRZ | Identifiability Checks Include Parent Citizen Runes | `Frontend/docs/arcana/`, collateral of the Scala purge (01652f58) | yes, mechanism alive in the Rust port |
| CADFRTZ | Clang Arch Derived From Rust Triple | deleted deliberately with the rust-interop prototype (82f9ec4b) | no, code gone |
| RRPGRZ | Rust Resolver Precedes Generic Resolver | same commit | no, code gone |

The flatten commit also took `docs/regions/Regions.md` with it entirely, 1095 lines, which likely held more than these two IDs. IPOMFIC's deleted file also defined its neighbours HCCSCS and TTTDRM.

**Already defined, the census just missed it (1)**

- MDRTCUT: Must Delay Rule Typing Calls Until Templar. Defined in prose under an `# Astronomer` header in `docs/old/Coercing Templatas, and Overload Sets.md`. Belongs in D1.

**Never had a definition, coined inline (4).** Titles are reconstructions from the citing code.

- GROUPS: groups are read from the declaration-side GroupS, not the ceremonial templata. Same commit left a `VGB: arcana for this` note.
- MCFBRBF: delay resolving function bounds until functions are compiled. Introduced in a 2023 Scala test comment, ported verbatim.
- RPPFNG: the reserved 0th LLVM parameter for the next-gen pointer. Defined only by four inline Backend comments from the regions merge.
- SITTX: interface TAG_* values must be assigned in the same edge order at both codegen sites. Single comment, no doc, X suffix probably a slip.

**Lives in another repo (1)**

- UTAIRZ: Unsized Types Appear Inside Ref. Defined in the Sky prototype at `/Volumes/V/Harmonious/rust-interop-architecture.md`, inherited from an earlier project called erw. The Vale doc cites it deliberately as Sky's.

**Not lost after all (1)**

- PRCCBIVRZ: Pure-Rust Crates Compile Byte-Identically to Vanilla Rustc. The ID tags invariant 1 in `docs/architecture/rust-interop-design.md`. It never had its own doc, so this is a C-bucket inline definition, not a lost one.

---

## 👤 User

i want to scope out how many we'll want to doublecheck later.
ignore the ones in other repos, and ignore ones in guardian and its submodules and luz. please categorize all the remaining ones for me again?

---

## 🤖 Claude

Vale-only count is **233 non-shield IDs**. Full tables with titles, files, and ref counts are in `tmp/arcana-vale-only.md`.

| Category | Count | Where |
|---|---|---|
| A. Modern Z-file in an `arcana/` dir | 19 | `docs/arcana/` 14, `src/postparsing/docs/arcana/` 3, `src/solver/docs/arcana/` 1, `Backend/docs/arcana/` 2 |
| B. Old-style section inside a `docs/arcana/` multi-topic file | 73 | Generics.md ~60, Impls.md 7, ret-vs-panic-locals.md 5, Environments.md 2, DropFree.md 1, IRegion.md 1, Environments-Templatas-ParentEntries.md 1 |
| C. Z-suffix defined inline in a live non-arcana doc | 15 | vale-rust-interop-architecture.md 10, rust-interop-design.md 2, instantiator-design.md 1, borrowing-design.md 1, backend-design.md 1 |
| D1. Only in `docs/old/`, cited from live code | 18 | Environments-Templatas-ParentEntries 6, Midas.md 3, Infer Templar.md 3, Externs and Regions 2, Environments-Closures-OverloadSets 2, Addresses.md 1, Parser_Scout.md 1 |
| D2. Only in `docs/old/`, cited from docs only | 6 | |
| D3. Only in `docs/old/`, cited nowhere | 69 | ~14 are HGM memory-design notes, the rest are Scala-era compiler notes |
| E. No suffix, in a live doc outside `arcana/` and `docs/old/` | 19 | vale-rust-interop-architecture.md 6, HigherTypingPass.md 5, docs/notes 4, metaprogrammed-record-replay.md 2, interning-dual-enum.md 1, borrowing-design.md 1 (VCOORD) |
| G. Recovered from git history only | 10 | 7 confirmed still true or partially; CADFRTZ and RRPGRZ describe deleted code |
| F. Cited from code, never defined anywhere | 4 | GROUPS, MCFBRBF, RPPFNG, SITTX |

**How I'd scope the double-check by priority:**

- Must check, 56: A (19), C (15), D1 (18), F (4). These are cited from live code or are the canonical modern set.
- Should check, 92: B (73) and E (19). Live docs, but Generics.md alone is 60 IDs and mostly uncited.
- Probably triage, not check, 85: D2 (6), D3 (69), G (10). Decide per-ID whether to migrate or let die. The 14 HGM ones in D3 and the 2 rust-interop ones in G are almost certainly dead.

One caveat on the numbers: 8 Vale shields were filtered out of the count, and the census's D3 could be slightly over- or under-counted since old-style "ID alone on the line below the header" definitions were the hardest form for the agents to catch.

---

## 👤 User

we should dispatch sonnet low agents to doublecheck all the ones in A. have them each write up a report somewhere in docs/arcana/reports/, in a new file each, and then you (not them) please insert a link to it from the original arcana place. dont overwrite/correct the arcana, just have something like "(this arcana has inaccuracies, please see docs/arcana/reports/[whatever].md for corrections)".
does that make sense? dont launch them yet.

---

## 🤖 Claude

Makes sense. Plan before launching:

**Agents (19, sonnet/low, one per A-list arcana):** each reads the arcana doc, every `@ID` code site, and the surrounding code, then judges each claim in the doc: still true, drifted, or false, with file:line evidence. Also reports code sites that cite the ID but no longer match, and behaviour that matches the arcana but lacks an `@ID` comment. Output is a structured report, not edits.

**Me afterwards:** create the report files, then add a one-line note under each arcana's header: `(This arcana has inaccuracies; see docs/arcana/reports/<Title>-<ID>-report.md for corrections.)` Only where the agent found something; accurate ones get no note.

Three questions, first in full below:

1. Agents writing into `docs/arcana/reports/` conflicts with the project rule that spawned agents never modify project files.
2. Where reports go for arcana that live outside `docs/arcana/` (postparsing, solver, Backend).
3. Whether a clean report also gets a note or a file.

**Question 1:** The project CLAUDE.md says spawned agents may only do read-only work and must not modify any project file. Having them write to `docs/arcana/reports/` breaks that. Two options: (a) agents write reports to the scratchpad and I copy them in verbatim as new files, which satisfies the rule and is what I'd recommend, or (b) you explicitly override the rule for this task and they write directly. Which do you want?

---

## 👤 User

1. yeah, agents can write to scratchpad and you issue a cp command.
2. sorry yes, put them in a reports/ folder right there.
3. no, clean report gets no file

if no more questions, launch

---

## 🤖 Claude

Launched 19 sonnet/low checkers, one per A-list arcana. Each writes its report only to the scratchpad and only when it finds inaccuracies. When they finish I'll copy the non-clean reports into a `reports/` folder beside each arcana's own directory (`docs/arcana/reports/`, `src/postparsing/docs/arcana/reports/`, `src/solver/docs/arcana/reports/`, `Backend/docs/arcana/reports/`) and add the pointer line to each affected arcana doc.

---

## 🤖 Claude

12 of 19 got reports. Before copying, I'll read the one major-inaccuracy report and check all reports for leaked absolute paths.

---

## 🤖 Claude

Report quality looks solid, and no absolute paths leaked. Creating the three `reports/` dirs and viewing the tops of the 12 arcana docs so I can place the pointer lines.

---

## 🤖 Claude

Reports copied. Now the 12 pointer lines, each inserted directly under the H1.

---

## 🤖 Claude

Done. 12 of the 19 modern arcana got a corrections report and a pointer line under their H1; 7 were judged accurate and left untouched.

**Verdicts**

| Verdict | IDs |
|---|---|
| Accurate, no file | EACBIPZ, FRMACZ, HTSLVBDTCZ, IIIOZ, LAGTNGZ, SICZ, TNLTZACZ |
| Minor inaccuracies | BDPFWDZ, BRRZ, CSCDSRZ, DSAUIMZ, ECSIIOSZ, ENECCLZ, ISNZ, PFVSZ, PPSPASTNZ, PVECFPZ, WVSBIZ |
| Major inaccuracies | IEOIBZ |

**Where the reports landed**

- `docs/arcana/reports/` (9): BDPFWDZ, BRRZ, DSAUIMZ, ECSIIOSZ, ENECCLZ, IEOIBZ, ISNZ, PVECFPZ, WVSBIZ
- `src/postparsing/docs/arcana/reports/` (2): PFVSZ, PPSPASTNZ
- `src/solver/docs/arcana/reports/` (1): CSCDSRZ

Each report has a claims table with TRUE/DRIFTED/FALSE per claim and file:line evidence, stale citation sites, uncited sites, and for IEOIBZ a suggested rewrite. I spot-read the IEOIBZ one: its "Where" list names four types that no longer exist (FunctionA, StructA, InterfaceA, ImplA) and misfiles IEnvironmentT as manual ptr-eq when it derives. Good quality, no absolute paths leaked.

**Recurring pattern worth knowing:** several minor verdicts (BDPFWDZ, BRRZ, CSCDSRZ, ECSIIOSZ, ISNZ) are cases where the doc already carries a trailing VCOORD or ZHERE note admitting the drift, but the body text was never updated. The reports mostly confirm those self-flags and add the current line numbers.

Nothing committed. The 12 arcana edits are one added line each; `git diff --stat` will show them alongside your in-progress borrow-checker changes.

---

## 👤 User

next two batches please

---

## 🤖 Claude

Taking that as B (old-style sections inside `docs/arcana/` multi-topic files, 73) and C (Z-suffix inline in live non-arcana docs, 15). Building the per-ID args with section line ranges first, since B's sections live inside long files.

---

## 🤖 Claude

Launched 84 sonnet/low checkers: 69 for batch B (old-style sections in `docs/arcana/`, aliases INSHN, NBIFPR, AFCTD, OMCNAGP skipped since they're covered by their parent sections) and 15 for batch C (Z-suffix IDs inline in architecture docs). Each reads its own section range and citation list from the scratchpad, writes a report only if it finds drift.

Report destinations when they land:
- B: all into `docs/arcana/reports/`, pointer line inserted under the section header inside Generics.md, Impls.md, etc.
- C: beside the defining file, so `docs/architecture/reports/`, `src/typing/docs/architecture/reports/`, `Backend/reports/`. SMLRZ is only defined in a convo doc, so its report goes to `docs/arcana/reports/` instead.

Flag if you'd rather C's reports go somewhere else; I'll wait for the run before copying anything.

---

## 🤖 Claude

Batches B and C done. 35 reports landed, 35 pointer lines inserted, every pointer resolves to an existing file.

**Verdicts across the 84 sections**

| Verdict | B (69) | C (15) |
|---|---|---|
| Accurate, no file | 32 | 11 |
| Minor inaccuracies | 20 | 0 |
| Major inaccuracies | 5 | 2 |
| Obsolete | 8 | 0 |
| Not an arcana, no file | 4 | 2 |

**Major:** CCAL (MaybeCoercingLookup/Call rule kinds no longer exist), DRSINI, IGBMN, IRIIB, LAGT, and in C: ATAFLBZ, DPSFDOZ.

**Obsolete, describing removed machinery:** AUMAP (Scala-era UnreachableMoot/Hammer), CWNWMT (Hammer stage gone), DPCODODP (the bug it documents no longer reproduces, a passing test proves it), GAOFPS, IAPCGA, IMRFDI, MDESOI, SCCTT. These are candidates for deletion rather than correction.

**Not an arcana, so no report written:** AGOBF, CGADOI, CODME, OSDCE are open questions or musings with no checkable claims. TCHAPZ and TVIMDGAZ are the same in the rust-interop doc. Worth knowing when you triage what to keep.

**Where things landed**

- `docs/arcana/reports/`: 33 new B reports, 42 total with batch A's.
- `docs/architecture/reports/`: ATAFLBZ, DPSFDOZ.
- Pointer lines: Generics.md 21, Impls.md 5, ret-vs-panic-locals.md 4, vale-rust-interop-architecture.md 2, one each in Environments.md, IRegion.md, and the DRSINI file.

One report (CFWG) had an absolute repo path in its evidence column; I rewrote it to a relative form before copying. Everything else came through clean. Nothing committed.

---

## 👤 User

next batch please

---

## 🤖 Claude

Next is D1: the 18 IDs defined only under `docs/old/` but cited from live code. Building their section ranges and citation lists.

---

## 🤖 Claude

Launched 17 sonnet/low checkers for D1. SRCAO and SRCAZ share one section so they got one agent, and IRFU is an older name inside the SAIRFU section so its agent audits that section with focus on the IRFU-cited code. Reports for this batch will go into a single `docs/old/reports/` folder, since the defining files are spread across `docs/old/`, `docs/old/Compiler/`, and `docs/old/Compiler/Templar/`. I'll copy and add pointer lines when the run completes.

---

## 🤖 Claude

CRCISFAORC's report is missing from the scratchpad even though its agent claimed to write one. Checking where it went, and reading the insertion lines for the edits.

---

## 🤖 Claude

CRCISFAORC's agent reported a path but never wrote the file. I'll rerun just that one, and meanwhile copy the 5 reports that exist and add their pointer lines.

---

## 🤖 Claude

Five D1 reports are in place with pointers. The CRCISFAORC rerun is in flight; I'll copy it and add its pointer under the `(CRCISFAORC)` line in `docs/old/Compiler/Midas.md` when it lands, then give the full D1 summary.

---

## 🤖 Claude

D1 done. CRCISFAORC's rerun came back accurate (its first run's only complaint was that one of the three cited Backend sites is commented-out code, which the rerun didn't count), so no report for it.

**D1 verdicts, 17 sections**

| Verdict | IDs |
|---|---|
| Accurate, no file (11) | BEAFB, CRCISFAORC, CSFMSEO, LDNEIR, LNASC, MDRTCUT, MSFDRF, OFCBT, SAFHE, SRCAO/SRCAZ, ULTMCIE, WTHPFE |
| Minor (1) | MPESC: exit codes still right, three cited line numbers drifted |
| Major (1) | RMLRMO: the doc's conclusion is reversed. MemberLookupTE now always yields a borrow and MutateTE unwraps it, so both live citations in `src/typing/ast/expressions.rs` are misleading |
| Obsolete (3) | SAIRFU, IRFU, SRCAMP: the CoordSend / self-replacing solver-rule mechanism was never ported to Rust. Only commented-out code cites them, and convo-84 records upcast resolution moving pre-solve |

**Landed:** 5 reports in `docs/old/reports/`, pointer lines in `docs/old/Compiler/Templar/Addresses.md`, `docs/old/Compiler/Templar/Infer Templar.md` (SAIRFU's pointer names both its report and IRFU's, since they're one section), and `docs/old/Compiler/Midas.md`.

**Takeaway for D1 specifically:** 11 of 17 are accurate and load-bearing from live code, yet their only definition sits in `docs/old/`. WTHPFE, BEAFB, SAFHE, LNASC, and MDRTCUT in particular are cited from central typing-pass code. Those are the strongest candidates to migrate into `docs/arcana/` as proper Z-suffix files, and the three obsolete ones plus their dead citing code are candidates for deletion.

Remaining unchecked from the Vale-only census: E (19 no-suffix IDs in live docs, mostly the rust-interop architecture doc and HigherTypingPass.md), D2/D3 (75 in `docs/old/` with no code citations), G (10 recovered from git), and F (4 never defined).

---

## 👤 User

next batch

---

## 🤖 Claude

Next is E: the 19 no-suffix IDs defined in live docs outside `arcana/` and `docs/old/`. Dropping RTMHTPS (an old alias of RTMHTP) and VCOORD (the V-marker convention, not an arcana), that's 17. Building args.

---

## 🤖 Claude

Launched 16 sonnet/low checkers for batch E (PRCBO and PSBCBO share one appendix, so one agent covers both). I added a check to the prompt so each agent verifies its report file actually exists before returning, after the CRCISFAORC miss. Reports will go beside each defining file: `docs/architecture/reports/`, `docs/reports/` for HigherTypingPass.md, `docs/notes/reports/`, `docs/todo/reports/`, and `src/postparsing/docs/architecture/reports/`. I'll copy and add pointers when it completes.

---

## 🤖 Claude

Batch E done. 5 reports, 5 pointer lines. I removed the empty `docs/notes/reports/` dir since none of the notes-doc IDs needed a report.

**E verdicts, 16 sections**

| Verdict | IDs |
|---|---|
| Not an arcana, no file (10) | AFPAFFIB, RMHPB, WRMBVC (FFI-purity brainstorm options), KVCIE, STRAST (design-decision logs), CIDD, DRAFD, HBAB, MAMFC, MIGPROP (rust-interop §26 forward plans with no code yet), PRCBOR |
| Minor (2) | IDEPFL: 2 of its 5 dual-enum pairs don't exist as Val types. MFDBRE: pattern alive, closing reference to "the scout" is Astronomer-era |
| Major (1) | CMWAR: written as a present-tense enforced cache-write discipline, but no after_rust_analysis callback, cache, or token type exists; the handoff doc confirms it's unbuilt |
| Obsolete (2) | RTMHTP (Scala-era Scout/Templar predictor), PRCBO/PSBCBO (the retired Linear region, already framed as history in the doc) |

**Landed:** `docs/architecture/reports/` CMWAR, `docs/reports/` MFDBRE and RTMHTP, `docs/todo/reports/` PRCBO-PSBCBO, `src/postparsing/docs/architecture/reports/` IDEPFL. Pointer lines in vale-rust-interop-architecture.md, HigherTypingPass.md (2), metaprogrammed-record-replay.md, and interning-dual-enum.md.

One thing to know: interning-dual-enum.md is auto-loaded via `g_auto_load_when_editing`, so its generated `.claude/rules/*.mdc` copy won't carry the pointer line until manifest-sync next runs.

**Pattern across E:** the rust-interop §26 block mixes real invariants (RTMEIZ, NNGZ, ELASZ, all accurate in batch C) with forward plans written in the same present-tense voice (CIDD, DRAFD, HBAB, MAMFC, MIGPROP, CMWAR). Readers can't tell which is which from the text.

**Remaining unchecked:** D2/D3 (75 in `docs/old/`, no code citations), G (10 recovered from git), F (4 never defined). D3 is the big one and mostly Scala-era or HGM design notes; a cheaper triage pass (keep / migrate / delete) may suit it better than full claim-by-claim audits.

---

## 👤 User

do 40 of the 75 please

---

## 🤖 Claude

Taking the 6 D2 IDs plus the first 34 of D3 alphabetically. Building args.

---

## 🤖 Claude

Launched 40 sonnet/low checkers: the 6 D2 IDs (CSALR, CSHROOR, EHCFBD, FFOP, MMEDT, NTKPRR) and the first 34 of D3 alphabetically, ACTE through IMCBT. Since none of these are cited from live code, each agent also names the code it would be cited from, and ends its verdict with a migrate / leave / delete recommendation. Reports go to `docs/old/reports/` with pointer lines under each section header when the run completes.

---

## 🤖 Claude

Reports copied. Now the 15 pointer lines.

---

## 🤖 Claude

All 15 pointers placed. The link check flagged one item, a literal `<Title>-<ID>` placeholder somewhere. Checking where that string lives.

---

## 🤖 Claude

That's just this session's convo log quoting my earlier plan, not a real pointer. All real pointers resolve.

**Batch D2/D3 part 1 done: 40 sections, 15 reports, 15 pointer lines.**

| Verdict | Count | IDs |
|---|---|---|
| Not an arcana | 21 | FFOP, ADSDF, ARCDS, CCAUIR, CCFTS, CCKTC, CGIVSI, CITBD, DCRC, DEPAR, DFSO, DGCOF, DOAOC, ERDS, GDFAK, GFFOP, HTKST, IGFCVI, IGFIL, IMBPC, and EHCFBD (which got a report recommending deletion, since it's a Scala-only caution) |
| Obsolete | 8 | CSALR, CSHROOR, MMEDT, NTKPRR, CHRCWRT, ECFKLOO, IDNOR, IEUNDS |
| Minor | 6 | ACTE, CNE, DSDCTD, ESNTA, HWDD, IMCBT |
| Accurate | 5 | DBTSAE, DEBDA, DIPRA, DORPAR, EAET |

**Migration candidates from this slice:** the 5 accurate ones plus ACTE and DSDCTD (core claim true, only examples or one contrast drifted). All are uncited from code, and the reports name the code they'd be cited from, for example DBTSAE maps to the TemplatasStoreT versus Environment split in `src/typing/env/environment.rs`, and DIPRA to pattern_compiler dropping ignored `_` values immediately.

**Landed:** 15 reports in `docs/old/reports/` (20 total there now), pointer lines in Infer Templar.md (2), Addresses.md, Optimization.md, Midas.md (2), Environments-Templatas-ParentEntries.md (2), Parser_Scout.md (2), Environments-Closures-OverloadSets.md, Patterns.md, Namespaces.md, Templar.md, Templates.md.

**Remaining:** 35 D3 IDs (ISGFFOP through WTMERB), G (10 recovered from git, no doc in tree to annotate), F (4 never defined). Overall so far: 176 sections audited, 74 reports written.

---

## 👤 User

do all the remaining batches at once now

---

## 🤖 Claude

Remaining: 35 D3 sections, 10 git-recovered (G), and 4 never-defined (F). G and F have no in-tree doc to point at, so their agents audit the recovered or reconstructed text and the reports go to `docs/arcana/reports/` with no pointer line. Building args.

---

## 🤖 Claude

Launched the final 49 sonnet/low checkers: 35 D3 sections, 10 git-recovered definitions, and 4 never-defined IDs. The G and F agents are told to always produce a report ending in a modern-style "Suggested text" paragraph, since those are the ones with no doc in the tree. When it completes I'll copy D3 reports to `docs/old/reports/` with pointer lines, and G/F reports to `docs/arcana/reports/` without pointers, then give the whole-project tally.

---

## 🤖 Claude

The audit is complete across every Vale-only batch. Every pointer resolves to an existing report.

**Final batch (49): D3 part 2, git-recovered, never-defined**

| Kind | Accurate | Minor | Major | Obsolete | Not an arcana |
|---|---|---|---|---|---|
| D3 (35) | 2 (VDND, VWKOA) | 5 | 0 | 13 | 15 |
| G recovered (10) | 0 | 3 (ICIPCRZ, ITN16BA, PRIIROZ) | 0 | 7 | 0 |
| F never defined (4) | 1 (SITTX) | 1 (GROUPS) | 1 (MCFBRBF) | 1 (RPPFNG) | 0 |

Notable: MCFBRBF's proposed fix is not what the code does, and RPPFNG's reserved next-gen parameter was removed in the Backend fold-in, so its citing comment in `Backend/src/function/expression.cpp` is dangling. Every G and F report ends with a "Suggested text" paragraph written in the modern arcana style, ready to become a real doc if you want it.

**Whole-project tally, 225 arcana audited**

| Verdict | Count |
|---|---|
| Accurate | 70 |
| Minor inaccuracies | 49 |
| Major inaccuracies | 11 |
| Obsolete | 42 |
| Not an arcana | 53 |

Reports written: 117. Pointer lines inserted: 104 (G/F have no in-tree doc, and a few not-an-arcana verdicts produced no file).

**Where everything lives**

- `docs/arcana/reports/` 56, `docs/old/reports/` 51, `docs/architecture/reports/` 3, `docs/reports/` 2, `src/postparsing/docs/arcana/reports/` 2, and one each in `src/solver/docs/arcana/reports/`, `src/postparsing/docs/architecture/reports/`, `docs/todo/reports/`.
- Census tables: `tmp/arcana-census.md` and `tmp/arcana-vale-only.md`.

**The 11 majors, for triage first:** IEOIBZ, CCAL, DRSINI, IGBMN, IRIIB, LAGT, ATAFLBZ, DPSFDOZ, RMLRMO, CMWAR, MCFBRBF.

**Migration candidates** (accurate, uncited or cited only into `docs/old/`): WTHPFE, BEAFB, SAFHE, LNASC, MDRTCUT, CSFMSEO, OFCBT, MSFDRF, LDNEIR, SRCAO, ULTMCIE from D1, plus DBTSAE, DEBDA, DIPRA, DORPAR, EAET, VDND, VWKOA from D3, plus SITTX which needs a definition written.

Nothing is committed. The working tree now has the report files, 104 one-line pointer insertions across the docs, and your own in-progress borrow-checker changes.

---

## 👤 User

Triage the 11 majors

---

## 🤖 Claude

I'll read the verdict and claims sections of all 11 reports, then give a triage table.

---

## 🤖 Claude

All 11 read. Triage, grouped by the action each needs. Effort is my estimate for a human doing the edit with the report open.

**Tiny, apply the report's text as-is (5)**

| ID | What's wrong | Action |
|---|---|---|
| DPSFDOZ | One sentence names `is_from_vale_stubs`, which doesn't exist. Real function is `is_vale_codegen_target` in the instantiating pass, attribute-gated. | Replace the one sentence. |
| CMWAR | Written as an enforced present-tense discipline; nothing is built. The handoff doc already says so. | Prefix "Vale-specific, unbuilt. Design intent:" and reword to future tense. |
| IRIIB | Two dead bullets: `receiveUnencryptedAlienReference` is gone, and `--region-override=unsafe-fast` is gone since `Unsafe` is now always the mutable region. Subclass names are `Unsafe` and `RCImm`. | Drop one bullet, fix two names, fix one sentence. |
| IEOIBZ | The "Where" list names FunctionA/StructA/InterfaceA/ImplA, which no longer exist, and files IEnvironmentT under manual ptr-eq when it derives. | Replace the "Where" bullet with the report's rewrite. |
| MCFBRBF | Never had a doc. The comment's proposed fix, delaying bound resolution, isn't what the code does. Bounds resolve directly against substituted templata at definition time and the test passes. | Replace the citing comment in `src/typing/test/compiler_tests.rs` with the report's one-liner pointing at MFBFDP. Don't create an arcana. |

**Rewrite the body, mechanism changed but concern is alive (3)**

| ID | What's wrong | Action |
|---|---|---|
| DRSINI | Core rule is true and enforced. The "Why" and "Where the rules live" sections describe hoisting `EqualsSR` into main rules; the postparser now does the opposite and bundles it inside the default's own rules. | Rewrite those two sections. While there, rename to DRSINIZ; it's the only `docs/arcana/` Z-file without the Z. |
| ATAFLBZ | Prescribes an `is_from_vale_stubs` filter on `all_impls` walks that was never built. The same doc's §26b.6 records the real fix: delete the name-matching resolvers, derive identity from `def_path`, backstop with a grep lint. | Rewrite the entry to describe the real fix, and fix the stale row in the §26 summary table. |
| RMLRMO | Conclusion is reversed. MemberLookupTE always yields a borrow and MutateTE unwraps it; SoftLoad is dead code. Two live citations in `src/typing/ast/expressions.rs`. | Use the report's suggested rewrite, then migrate it to `docs/arcana/` as a Z-file since live code cites it, and re-point the two comments. |

**Delete, superseded (3)**

| ID | What's wrong | Action |
|---|---|---|
| LAGT | Contradicted by its own successor LAGTNGZ. Says lambdas are generics the instantiator specializes a second time; the name builder panics on exactly that. Uncited. | Delete the LAGT section body in Generics.md, leave a one-line pointer to LAGTNGZ, re-parent its subsections LHPCTLD, GLIOGN, DMPOGN. |
| CCAL | Built around `MaybeCoercingLookup` and `MaybeCoercingCall` rule kinds that no longer exist; the solver asserts they're desugared before it. Uncited. | Delete. The kind-vs-coord question it raises is settled by rune typing (the KVCIE decision). |
| IGBMN | The three-name impl scheme is gone; there's one `ImplTemplateNameT` and both-direction lookup goes through HinputsT indices. Uncited. Only the anonymous-substruct naming claim survives. | Delete, or shrink to two sentences about `AnonymousSubstructImplTemplateNameT` if you want the naming fact kept. |

**Pragmatic recommendation:** do the five tiny fixes now from the report text, delete LAGT and CCAL, and leave DRSINI, ATAFLBZ, RMLRMO, IGBMN with their pointer lines until someone next works in that area. Temporary: the pointer line plus report stays as the correction mechanism for those four.

**Principled recommendation:** for each of the 11, either rewrite the arcana body to match the code under good-arcana rules (why not what, no line numbers) or delete it and its citations, then delete the report and pointer so there's one source of truth. Migrate RMLRMO to a Z-file. Rename DRSINI to DRSINIZ. Add an explicit "unbuilt" status marker to the rust-interop §26 entries, since CMWAR and the five not-an-arcana siblings (CIDD, DRAFD, HBAB, MAMFC, MIGPROP) all present plans in the same present-tense voice as the real invariants next to them.

One cross-cutting note: `docs/meta.md` says fix recipes and plans don't get documents. The 117 reports fit that description, so treat each as disposable once its arcana is fixed or deleted, not as a permanent second layer.

---

## 👤 User

<command-message>update-handoff</command-message>
<command-name>/update-handoff</command-name>
<command-args>a new arcana-audit-handoff.md, and lets wind down for the day</command-args>

---

## 🤖 Claude

One correction to make in the handoff: MPESC isn't in the accurate list, so it doesn't belong in the migration parenthetical.

---

## 🤖 Claude

Handoff written at `docs/handoffs/arcana-audit-handoff.md`. Wound down.

**What the handoff carries**

- What arcana forms exist in this tree and where each category lives.
- The report and pointer convention, with the shell commands to recount and link-check them instead of stale numbers.
- The 11 majors as a table of present-tense facts and actions.
- Migration candidates (11 accurate but `docs/old`-only and code-cited, 7 accurate and uncited, SITTX needing a definition), deletion candidates, and the §26 unbuilt-plan problem.
- Lessons learned: the below-header `(ID)` form, agents writing outside the repo, the accurate/minor boundary being unreliable at low effort, `@TFITCX` drowning greps, and reorg commits orphaning `see ID` comments.

**Also preserved:** the Vale-only census tables as `docs/arcana/reports/00-census.md`, since `tmp/` is gitignored and the full census needed 66 agents to build.

**Working tree from this session, all uncommitted:** 11 untracked items (8 `reports/` dirs, the handoff, plus the census file inside one of those dirs) and 53 modified docs carrying pointer lines. Your borrow-checker changes are untouched alongside them. Nothing was committed and no code files were edited; MCFBRBF's comment fix is listed as an action, not applied.

---

