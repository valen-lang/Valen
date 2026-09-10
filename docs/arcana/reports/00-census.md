# Arcana census (Vale-only; Guardian, Luz, external repos excluded)

Total non-shield IDs to potentially double-check: 219 defined in tree + 10 recovered from git + 4 never defined = 233

| Category | Count |
|---|---|
| A modern Z-file in arcana/ | 19 |
| B old-style section inside docs/arcana/ multi-topic file | 73 |
| C Z-suffix defined inline in a live non-arcana doc | 15 |
| D1 only in docs/old, cited from code | 18 |
| D2 only in docs/old, cited from docs only | 6 |
| D3 only in docs/old, cited nowhere | 69 |
| E no-suffix in other live doc (not arcana/, not old) | 19 |
| G git-history only, recovered by archaeology | 10 |
| X shields (excluded from count) | 8 |
| F cited from Vale code, never defined anywhere | 4 |


## A modern Z-file in arcana/ (19)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| PFVSZ | Parameter Full-Type / Value-Type Split | src/postparsing/docs/arcana/ParameterFullTypeValueTypeSplit-PFVSZ.md | 24 | 11 |
| DSAUIMZ | Defer Slice Allocation Until Intern Miss | docs/arcana/DeferSliceAllocationUntilInternMiss-DSAUIMZ.md<br>src/postparsing/docs/architecture/interning-dual-enum.md | 20 | 8 |
| IIIOZ | Identical Inputs, Identical Outputs | docs/arcana/IdenticalInputsIdenticalOutputs-IIIOZ.md | 16 | 2 |
| PVECFPZ | Polyvalue Enums Are Closed-Set Fat Pointers | docs/arcana/PolyvalueEnumsAreClosedFatPointers-PVECFPZ.md<br>src/typing/typing-design.md | 14 | 9 |
| TNLTZACZ | Type Names Lower To Zero-Arg Calls | src/postparsing/docs/arcana/TypeNamesLowerToZeroArgCalls-TNLTZACZ.md | 14 | 2 |
| PPSPASTNZ | PostParser Synthesizes Parser AST Nodes | src/postparsing/docs/arcana/PostParserSynthesizesParserASTNodes-PPSPASTNZ.md | 11 | 8 |
| WVSBIZ | When Values Should Be Interned | docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md<br>docs/architecture/instantiator_design_2.md<br>src/typing/typing-design.md | 9 | 15 |
| IEOIBZ | Identity Equality On Identity-Bearing Types | docs/arcana/IdentityEqualityOnIdentityBearingTypes-IEOIBZ.md<br>src/typing/docs/architecture/borrowing-design.md<br>src/typing/typing-pass-todo.md | 7 | 16 |
| ENECCLZ | Exact/Non-Exact Call Candidate Lookup | docs/arcana/ExactNonExactCallCandidateLookup-ENECCLZ.md | 7 | 4 |
| SICZ | Sealed Interned Construction | docs/arcana/SealedInternedConstruction-SICZ.md | 6 | 22 |
| BDPFWDZ | By Default Pull From Where Declared | docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md | 6 | 7 |
| BRRZ | Bound Return Resolution | docs/arcana/BoundReturnResolution-BRRZ.md | 6 | 6 |
| FRMACZ | FFI Refs Move, Accessors Consume | Backend/docs/arcana/FFIRefsMoveAccessorsConsume-FRMACZ.md | 6 | 5 |
| HTSLVBDTCZ | Handle Types Is Same LLVM Value But Different Typedefs In C | Backend/docs/arcana/HandleTypesIsSameLLVMValueButDifferentTypedefsInC-HTSLVBDTCZ.md | 6 | 3 |
| EACBIPZ | Extern Aggregates Cross By Indirect Pointer | docs/arcana/ExternAggregatesCrossByIndirectPointer-EACBIPZ.md | 5 | 2 |
| ECSIIOSZ | Each Call-Site Is Its Own Solve | docs/arcana/EachCallSiteIsItsOwnSolve-ECSIIOSZ.md | 3 | 10 |
| CSCDSRZ | Complex Solve Concludes But Doesn't Solve Rules | src/solver/docs/arcana/ComplexSolveConcludesButDoesntSolveRules-CSCDSRZ.md | 1 | 0 |
| LAGTNGZ | Lambdas Are Generic Templates Not Generics | docs/arcana/LambdasAreGenericTemplatesNotGenerics-LAGTNGZ.md | 0 | 7 |
| ISNZ | ID Shorthand Notation | docs/arcana/IdShorthandNotation-ISNZ.md | 0 | 1 |

## B old-style section inside docs/arcana/ multi-topic file (73)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| DRSINI | Default Rules Should Be Incremental Not Initial | docs/arcana/DefaultRulesShouldBeIncrementalNotInitial-DRSINI.md | 8 | 7 |
| SFWPRL | Solve First With Predictions, Resolve Later | docs/arcana/Generics.md | 8 | 3 |
| MKRFA | Must Know Runes From Above | docs/arcana/Generics.md | 5 | 5 |
| DBDAR | Difference Between solveForDefining and solveForResolving | docs/arcana/Generics.md | 5 | 1 |
| SROACSD | Some Rules Only Apply to Call Site or Definition | docs/arcana/Generics.md | 4 | 4 |
| UCRTVPE | Unstackifieds Can Refer To Variables in Parent Environments | docs/arcana/Environments, Templatas, and Parent Entries.md<br>docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 4 | 0 |
| LCCPGB | Lambdas Can Call Parents' Generic Bounds | docs/arcana/Generics.md | 3 | 1 |
| MFBFDP | Merge Function Bounds From Different Places | docs/arcana/Generics.md | 3 | 1 |
| MDSFONARFO | Macro-Derived Sibling Functions Often Need All Rules From Original | docs/arcana/Generics.md | 3 | 0 |
| TAVWG | Tuples And Variadics With Generics | docs/arcana/Generics.md | 3 | 0 |
| CSSNCE | Call Site Solving Needs Caller Env | docs/arcana/Generics.md | 2 | 1 |
| LCNBAFA | Lambdas and Children Need Bound Arguments From Above | docs/arcana/Generics.md | 2 | 1 |
| STCMBDP | Solving Then Checking Must Be Different Phases | docs/arcana/Generics.md | 2 | 1 |
| MDATOEF | Must Declare All Type Outer Envs First | docs/arcana/Generics.md | 2 | 0 |
| SBITAFD | Substitute Bounds In Things Accessed From Dots | docs/arcana/Generics.md | 2 | 0 |
| BRCOBS | Break and Return Can Only Be Statements | docs/arcana/ret-vs-panic-locals.md<br>docs/old/Ret vs Panic, Locals (already migrated).md | 1 | 1 |
| CDFGI | Compile Dispatcher Function Given Interface | docs/arcana/Generics.md | 1 | 1 |
| DINSIE | Discard Is Not Special, It's Everywhere | docs/arcana/DropFree.md | 1 | 1 |
| OMCNAGP | Override Milano Case Needs Additional Generic Params | docs/arcana/Generics.md | 1 | 1 |
| TIBANFC | Translate Impl Bound Argument Names For Case | docs/arcana/Generics.md | 1 | 1 |
| ACEFRO | Assemble the Case Environment For Resolving the Override | docs/arcana/Generics.md | 1 | 0 |
| FODAIR | Figure Out Dependent And Independent Runes | docs/arcana/Generics.md | 1 | 0 |
| FOSFC | Figure Out Struct For Case | docs/arcana/Generics.md | 1 | 0 |
| GLIOGN | Getting Lambda Instantiation's Original Generic's Name | docs/arcana/Generics.md | 1 | 0 |
| GTCII | Get The Compiled Impl's Interface, In Terms of Dispatcher | docs/arcana/Generics.md | 1 | 0 |
| IRAGP | Incrementally Reluctantly Add Generic Placeholders | docs/arcana/Generics.md | 1 | 0 |
| NMORFI | Need Match-Only Rule For Impls | docs/arcana/Impls.md | 1 | 0 |
| RRBFS | Resolving Races Between Fallback Strategies | docs/arcana/Generics.md | 1 | 0 |
| SCCTT | Should Change Coercing to toRef | docs/arcana/Impls.md | 1 | 0 |
| UCEFO | Use Case Environment to Find Override | docs/arcana/Generics.md | 1 | 0 |
| AUMAP | Add Unreachable Moots After Panic | docs/arcana/ret-vs-panic-locals.md<br>docs/old/Ret vs Panic, Locals (already migrated).md | 0 | 1 |
| DUDEWCD | Don't Use Default Expression When Compiling Denizen | docs/arcana/Generics.md | 0 | 1 |
| IMRFDI | Interfaces Must Remember Functions Declared Inside | docs/arcana/Environments.md | 0 | 1 |
| LHPCTLD | Lambdas Have Placeholders from Containing Top Level Denizen | docs/arcana/Generics.md | 0 | 1 |
| MSAE | Must Specify Array Element | docs/arcana/Generics.md | 0 | 1 |
| AFCTD | Abstract Function Calls The Dispatcher | docs/arcana/Generics.md | 0 | 0 |
| AGOBF | Ambiguous Generic Overlapping Bound Functions | docs/arcana/Generics.md | 0 | 0 |
| BMHD | Blocks Might Have Deferreds | docs/arcana/ret-vs-panic-locals.md<br>docs/old/Ret vs Panic, Locals (already migrated).md | 0 | 0 |
| BNATON | Break Never And The Other Never | docs/arcana/ret-vs-panic-locals.md<br>docs/old/Ret vs Panic, Locals (already migrated).md | 0 | 0 |
| CCAL | Coercing Calls And Lookups | docs/arcana/Generics.md | 0 | 0 |
| CFWG | Concept Functions With Generics | docs/arcana/Generics.md | 0 | 0 |
| CGADOI | Can't Get All Descendants Of Interface | docs/arcana/Generics.md | 0 | 0 |
| CODME | Compilation Order of Denizens, Macros, Environments | docs/arcana/Environments.md | 0 | 0 |
| CWNWMT | Consecutor With Never Will Make Temporaries | docs/arcana/ret-vs-panic-locals.md | 0 | 0 |
| DMPOGN | Don't Monomorphize Parts Of Generic Names | docs/arcana/Generics.md | 0 | 0 |
| DPCODODP | Default Parameters Can Only Depend on Other Default Parameters | docs/arcana/Generics.md | 0 | 0 |
| FMBAWTUFB | Functions Must Be Associated With Type to be Used in Function Bounds | docs/arcana/Generics.md | 0 | 0 |
| GAOFPS | Giving Argument Ownership For Param Subtypes | docs/arcana/Impls.md | 0 | 0 |
| IAPCGA | Instantiator Accesses Parts of Coord Generic Args | docs/arcana/Generics.md | 0 | 0 |
| IBAMIBP | Instantiation Bound Args Match Instantiation Bound Params | docs/arcana/Generics.md | 0 | 0 |
| IBFCS | Inherit Bounds From Case Struct | docs/arcana/Generics.md | 0 | 0 |
| IGBMN | Impl Goes By Many Names | docs/arcana/Impls.md | 0 | 0 |
| INSHN | Impl Goes By Many Names | docs/arcana/Impls.md | 0 | 0 |
| IRIIB | IRegion Interface in Backend | docs/arcana/IRegion.md | 0 | 0 |
| LAGT | Lambdas Are Generic Templates | docs/arcana/Generics.md | 0 | 0 |
| MDESOI | Matching Doesnt Evaluate Struct Or Interface | docs/arcana/Impls.md | 0 | 0 |
| MLUIBTN | Must Look Up Impls By Template Name | docs/arcana/Generics.md | 0 | 0 |
| NBIFP | Need Bound Information From Parameters | docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md<br>docs/arcana/Generics.md | 0 | 0 |
| NBIFPR | Need Bound Information From Parameters | docs/arcana/Generics.md | 0 | 0 |
| NFIEFRO | Need Function Inner Env For Resolving Overrides | docs/arcana/Generics.md | 0 | 0 |
| OIRCRR | Only Include Reachables for Call Rules' Results | docs/arcana/Generics.md | 0 | 0 |
| ONBIFS | Overrides Need Bound Information From Structs | docs/arcana/Generics.md | 0 | 0 |
| OSDCE | Only See Direct Caller's Environment | docs/arcana/Generics.md | 0 | 0 |
| OWPFRD | Only Work with Placeholders From the Root Denizen | docs/arcana/Generics.md | 0 | 0 |
| PTPKCK | PlaceholderTemplata, PlaceholderKind, Coords, Kinds | docs/arcana/Generics.md | 0 | 0 |
| REMUIDDA | Require Explicit Multiple Upcasting to Indirect Descendants and Ancestors | docs/arcana/Generics.md | 0 | 0 |
| RFNTIOB | ReachableFunctionNameT instead of BoundFunctionNameT | docs/arcana/Generics.md | 0 | 0 |
| SCIIMT | Struct Can Impl Interface Multiple Times | docs/arcana/Generics.md | 0 | 0 |
| SIPWDR | Start Incremental Placeholdering With Default Region | docs/arcana/Generics.md | 0 | 0 |
| SMRASDR | Structs Member Rules Are Skipped During Resolving | docs/arcana/Generics.md | 0 | 0 |
| SRHODP | Some Rules are Hoisted Out of Default Param | docs/arcana/Generics.md | 0 | 0 |
| UINIT | Using Instantiated Names in Templar | docs/arcana/Generics.md | 0 | 0 |
| WIGOWI | WTF Is Going On With Impls | docs/arcana/Generics.md | 0 | 0 |

## C Z-suffix defined inline in a live non-arcana doc (15)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| RTMEIZ | Rust Types Must Be Explicitly Imported | docs/architecture/vale-rust-interop-architecture.md | 9 | 1 |
| BCHATZ | Borrow Checking Happens After Typing | src/typing/docs/architecture/borrowing-design.md | 5 | 2 |
| ATAFLBZ | All-impls Walks Need Vale-Stubs Filter | docs/architecture/vale-rust-interop-architecture.md | 5 | 0 |
| ELASZ | Early-bound Lifetime Args Synthesized | docs/architecture/vale-rust-interop-architecture.md | 3 | 0 |
| NNGZ | Non-generic is Normal-case-of-Generic | docs/architecture/vale-rust-interop-architecture.md | 3 | 0 |
| SMLRZ | The SMLRZ trap (Simplifier/Mangling Leak — Rust name-shape leaking into typing) | docs/convos/rust_interop/synthesized-declarations-plan.md | 2 | 3 |
| PRCCBIVRZ | Pure-Rust Crates Compile Byte-Identically to Vanilla Rustc | docs/architecture/rust-interop-design.md | 2 | 2 |
| BDCABIBZ | C ABI Boundary | Backend/backend-design.md | 2 | 0 |
| DPSFDOZ | DefPathStr Is For Diagnostics Only | docs/architecture/vale-rust-interop-architecture.md | 1 | 0 |
| ACRTFDZ | ABI Coerced Return Type In Function Declarations | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| GCMLZ | Generate Compile Mutex Lock | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| OWTHACBZ | On Whether to Have a Custom Backend | docs/architecture/rust-interop-design.md | 0 | 0 |
| PASDZ | Placeholder Authoritative-Source = Denizen | docs/architecture/instantiator-design.md | 0 | 0 |
| TCHAPZ | Track Caller Hidden ABI Parameter | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| TVIMDGAZ | Trait vs Impl Method DefId | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |

## D1 only in docs/old, cited from code (18)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| MPESC | Midas Process Exit Status Codes | docs/old/Compiler/Midas.md | 6 | 0 |
| SRCAO | Start RC At One | docs/old/Externs and Regions.md | 6 | 0 |
| BEAFB | Block Environments Are For Breaks | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 4 | 0 |
| LNASC | Last Names for Anonymous Substructs and Constructors | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 4 | 0 |
| ULTMCIE | Use Locals To Make Constant Into Expression | docs/old/Compiler/Midas.md | 4 | 0 |
| WTHPFE | Whether To Have Parent Function Environments | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 4 | 0 |
| CSFMSEO | Closure's Struct and Function Must See Each Other | docs/old/Environments, Closures, Overload Sets.md | 3 | 1 |
| SAFHE | Structs and Functions Have Environments | docs/old/Environments, Closures, Overload Sets.md | 3 | 1 |
| CRCISFAORC | Constraint RC Is Same Field As Owning RC | docs/old/Compiler/Midas.md | 3 | 0 |
| RMLRMO | ReferenceMemberLookup Results In Member's Ownership | docs/old/Compiler/Templar/Addresses.md | 2 | 1 |
| IRFU | Impl Rule For Upcasts | docs/old/Compiler/Templar/Infer Templar.md | 2 | 0 |
| LDNEIR | Lambdas Dont Need Explicit Identifying Runes | docs/old/Parser_Scout.md | 1 | 0 |
| MDRTCUT | Must Delay Rule Typing Calls Until Templar | docs/old/Coercing Templatas, and Overload Sets.md | 1 | 0 |
| MSFDRF | Must Scan For Declared Runes First | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 1 | 0 |
| OFCBT | Ordinary Functions Can Be Templates | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 1 | 0 |
| SAIRFU | Send and Impl Rules For Upcasts | docs/old/Compiler/Templar/Infer Templar.md | 1 | 0 |
| SRCAMP | Some Rules Can Add More Puzzles | docs/old/Compiler/Templar/Infer Templar.md | 1 | 0 |
| SRCAZ | Start RC At One | docs/old/Externs and Regions.md | 1 | 0 |

## D2 only in docs/old, cited from docs only (6)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| FFOP | First Final Owned Pointer | docs/old/HGM V16, V17, V18.md | 0 | 3 |
| CSALR | Complex Solve As Last Resort | docs/old/Compiler/Templar/Infer Templar.md | 0 | 1 |
| CSHROOR | Can Sometimes Have Read-Only Owning References | docs/old/Compiler/Templar/Addresses.md | 0 | 1 |
| EHCFBD | Equals and Hash Code Forbidden By Default | docs/old/Compiler/Optimization.md | 0 | 1 |
| MMEDT | Modules Must Export Dependencies Themselves | docs/old/Compiler/Midas.md | 0 | 1 |
| NTKPRR | Need To Know Parent Rules and Runes | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 0 | 1 |

## D3 only in docs/old, cited nowhere (69)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| ACTE | Adding Constructors To Environments | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 0 | 0 |
| ADSDF | Automatically Detect Simplifiable Drop Functions | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| ARCDS | Anonymous Runes Considered Deeply Satisfied | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |
| CCAUIR | Compiler Cant Add to User's Identifying Runes | docs/old/Parser_Scout.md | 0 | 0 |
| CCFTS | Can Coerce Functions to Structs | docs/old/Coercing Templatas, and Overload Sets.md | 0 | 0 |
| CCKTC | Can Coerce Kinds To Coords | docs/old/Coercing Templatas, and Overload Sets.md | 0 | 0 |
| CGIVSI | Combined Generation for IVSIs | docs/old/HGM V20.md | 0 | 0 |
| CHRCWRT | Can't Have Rule Components Without Rule Type | docs/old/Parser_Scout.md | 0 | 0 |
| CITBD | Check Inner Tethers Before Destroy | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| CNE | Closures Need Environments | docs/old/Environments, Closures, Overload Sets.md | 0 | 0 |
| DBTSAE | Difference between TemplatasStore and Environment | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 0 | 0 |
| DCRC | Double Constraint Refs Contradiction | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |
| DEBDA | Declare Everything Before Defining Anything | docs/old/Compiler/Midas.md | 0 | 0 |
| DEPAR | Dealias Extern Params and Returns | docs/old/Externs and Regions.md | 0 | 0 |
| DFSO | Drop, Free, Shared, Owned | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| DGCOF | Dot Gets Components Or Fields | docs/old/Parser_Scout.md | 0 | 0 |
| DIPRA | Destroy Ignored Patterns Right Away | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| DOAOC | Droppable Option, Arrays, and other Containers | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| DORPAR | Downcasting Owning Ref Produces A Result | docs/old/Impls.md | 0 | 0 |
| DSDCTD | Destructuring Shared Doesnt Compile To Destroy | docs/old/Compiler/Templar/Patterns.md | 0 | 0 |
| EAET | Extern and Export Tests | docs/old/Compiler/Tests.md | 0 | 0 |
| ECFKLOO | Elide Checks For Known Live On/Off | docs/old/Compiler/Midas.md | 0 | 0 |
| ERDS | Encrypted References, Different Stacks | docs/old/Externs and Regions.md | 0 | 0 |
| ESNTA | Each Step Needs Template Args | docs/old/Compiler/Namespaces.md | 0 | 0 |
| GDFAK | Generate Destructors For All Kinds | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| GFFOP | Generationed FFOPs | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| HTKST | How To Know Somethings' Template | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |
| HWDD | How We Derive Drop | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| IDNOR | Impls Dont Need Ordered Runes | docs/old/Parser_Scout.md | 0 | 0 |
| IEUNDS | Unknown Needs DeeplySatisfied | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |
| IGFCVI | Inner Generations For Complex Varying Inlines | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| IGFIL | Inner Generations For Inline Interfaces | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| IMBPC | Impls Must Be with Parent or Child | docs/old/Impls.md | 0 | 0 |
| IMCBT | Interface Methods Can Be Templates | docs/old/Compiler/Templates.md | 0 | 0 |
| ISGFFOP | Inline Struct Generationed FFOP | docs/old/HGM V20.md | 0 | 0 |
| IST | Interior Scope Tether | docs/old/HGM V21.md | 0 | 0 |
| KIWOT | Know If We're the Only Tether | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| LHRSP | Lambdas Have Readwrite Self Parameters | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 0 | 0 |
| LTFIVOFFOP | Lazy Tethering For Inline Varying Objects' FFOPs | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| MAPOWM | Messages Are Pre-Order, With Metadata | docs/old/Compiler/Midas.md | 0 | 0 |
| MDMIA | Matching Doesnt Match Into Arguments | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |
| MEDBR | we must execute deferreds before a return | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| MEDP | Must Explicitly Destructure Packs | docs/old/Parser_Scout.md | 0 | 0 |
| MINAAN | Matching Imprecise Names Against Absolute Names | docs/old/Names.md | 0 | 0 |
| MLIOET | Must Look In Override Env Too | docs/old/Compiler/Virtuals.md | 0 | 0 |
| MTIV | Must Tether Inline Varyings | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| NDFETT | No Destructor For Empty Tuple Type | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| NIIRII | Need Interface Identifying Rune In Impl | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| NOTAN | Need Optional Template Args in Name | docs/old/Compiler/Namespaces.md | 0 | 0 |
| NSIDN | Need Separate IDrop and Drop Names | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| NTERT | Need Types on Every Rule and Templex | docs/old/Coercing Templatas, and Overload Sets.md | 0 | 0 |
| PEAME | Parents and Environments Are Mutually Exclusive | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 0 | 0 |
| PVSBUFI | Possible Values Shouldnt Be Used For Inference | docs/old/Parser_Scout.md | 0 | 0 |
| RCKC | Do Runes Capture Kinds or Coordinates? | docs/old/Parser_Scout.md | 0 | 0 |
| RMLHTP | ReferenceMemberLookup Has TargetPermission | docs/old/Compiler/Templar/Addresses.md | 0 | 0 |
| ROS | Representing Overload Sets | docs/old/Environments, Closures, Overload Sets.md | 0 | 0 |
| RSTIV | Representing Scope Tethering In VAST | docs/old/Compiler/Midas.md | 0 | 0 |
| RWKILC | Rune With Kind Is Like Call | docs/old/Parser_Scout.md | 0 | 0 |
| SASP | Suffix Adds Size Parameter | docs/old/Compiler/Midas.md | 0 | 0 |
| SDFF | Separate Drop and Free Functions | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| SMCMST | Solver Must Choose Most Specific Type | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |
| TIFA | The Immutable Free Anomaly | docs/old/Compiler/Templar/Templar.md | 0 | 0 |
| TMRE | Templatas Must Remember Environment | docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md | 0 | 0 |
| UCCRH | Undead Cycle / Constraint Ref Hybrid | docs/old/HGM V16, V17, V18.md | 0 | 0 |
| UMSEIR | User Must Specify Enough Identifying Runes | docs/old/Parser_Scout.md | 0 | 0 |
| UTSLIG | Use Type Stability for Less Inner Generations | docs/old/HGM V20.md | 0 | 0 |
| VDND | Voids Dont Need Discarding | docs/old/Voids and Discarding.md | 0 | 0 |
| VWKOA | Vivem Weaks Keep Objects Allocated | docs/old/Compiler/Vivem.md | 0 | 0 |
| WTMERB | Whether To Merge Equal Rules Beforehand | docs/old/Compiler/Templar/Infer Templar.md | 0 | 0 |

## E no-suffix in other live doc (not arcana/, not old) (19)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| IDEPFL | Postparser Interning: Dual-Enum Pattern For Lookups | src/postparsing/docs/architecture/interning-dual-enum.md | 1 | 4 |
| MFDBRE | Must Forward Declare Before Rules Evaluated | docs/HigherTypingPass.md | 1 | 0 |
| PRCBO | Pointers in Registers Can Be Offsets | docs/todo/metaprogrammed-record-replay.md | 0 | 2 |
| CMWAR | Cache-Must-Write-At-Rust-analysis | docs/architecture/vale-rust-interop-architecture.md | 0 | 1 |
| PSBCBO | Pointers in Serialize Buffers Can Be Offsets | docs/todo/metaprogrammed-record-replay.md | 0 | 1 |
| RTMHTPS | Recursive Types Must Have Types Predicted In Scout | docs/HigherTypingPass.md | 0 | 1 |
| AFPAFFIB | Alternatives For Purity Across the FFI Boundary | docs/notes/ContextWordNotes.md | 0 | 0 |
| CIDD | Comptime If Deterministic Discipline | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| DRAFD | Dangle-Region-Annotation-Flows-Drop | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| HBAB | Honest at Boundary, Always | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| KVCIE | Kind vs Coord, Implicit and Explicit | docs/HigherTypingPass.md | 0 | 0 |
| MAMFC | Missing-Annotation Mut-effect Fail-Closed | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| MIGPROP | Migratory Propagation | docs/architecture/vale-rust-interop-architecture.md | 0 | 0 |
| PRCBOR | PRCBO Reasoning | docs/notes/LinearRegionNotes.md | 0 | 0 |
| RMHPB | Region Metadata Has Pure Bit | docs/notes/ContextWordNotes.md | 0 | 0 |
| RTMHTP | Recursive Types Must Have Types Predicted | docs/HigherTypingPass.md | 0 | 0 |
| STRAST | Strongly Typed Rule AST | docs/HigherTypingPass.md | 0 | 0 |
| VCOORD | (referenced only, per-function unique name concept) | src/typing/docs/architecture/borrowing-design.md | 0 | 0 |
| WRMBVC | Write Region Metadata Bit on Virtual Call | docs/notes/ContextWordNotes.md | 0 | 0 |

## G git-history only, recovered by archaeology (10)

| ID | Title | Recovered from | Still true |
|---|---|---|---|
| CADFRTZ | Clang Arch Derived From Rust Triple | git:82f9ec4b^:docs/arcana/ClangArchDerivedFromRustTriple-CADFRTZ.md | no |
| HCCSCS | Handling Collapsed Coords and Subjective Coords Simultaneously | git:d097d488^:docs/InstantiatorRegions.md | ? |
| ICIPCRZ | Identifiability Checks Include Parent Citizen Runes | git:790fc183:Frontend/docs/arcana/IdentifiabilityChecksIncludeParentCitizenRunes-ICIPCRZ.md (full file, 58 lines) | yes |
| IPOMFIC | Ignore Previous Ownerships Mutabilities From Instantiated Coords | git:d097d488^:docs/InstantiatorRegions.md lines 283-290 | partially |
| ITN16BA | ITables Need to be 16-Byte Aligned | git:a502ea0e:docs/SeparatedFFI.md (section added at that commit; later renamed content persisted to git:b9a4dcdf^:docs/SeparatedFFI.md before deletion) | yes |
| LNTKTR | Lookups Need To Know Their Region | git:24695a17^:docs/regions/Regions.md lines 596-618 | yes |
| PRIIROZ | container-chain fold (title unknown) | git:d3507636 (never had a doc file) | ? |
| RRPGRZ | Rust Resolver Precedes Generic Resolver | git:82f9ec4b^:docs/arcana/RustResolverPrecedesGenericResolver-RRPGRZ.md | no |
| SRIE | Specifying Regions In Expressions (SRIE) | git:24695a17^:docs/regions/Regions.md lines 533-579 | yes |
| TTTDRM | Time Travel To Determining Region Mutabilities | git:d097d488^:docs/InstantiatorRegions.md | ? |

## X shields (excluded from count) (8)

| ID | Title | File(s) | code refs | doc refs |
|---|---|---|---|---|
| TFITCX | Types Fit Into These Categories | docs/shields/TypesFitIntoTheseCategories-TFITCX.md<br>src/typing/typing-design.md | 293 | 21 |
| AASSNCMCX | Arena-Allocated Structs Should Not Contain Malloc'd Collections | docs/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md<br>src/typing/typing-pass-todo.md | 0 | 18 |
| SPDMX | (shield, referenced only as 'SPDMX-B' / 'SPDMX exception B' in a comment convention note) | src/typing/typing-pass-todo.md | 0 | 7 |
| ATDCX | Arena Types Don't Clone | docs/shields/ArenaTypesDontClone-ATDCX.md | 0 | 5 |
| AFEOX | Allowed File Extensions Only | docs/shields/AllowedFileExtensionsOnly-AFEOX.md | 0 | 2 |
| UCMTRSX | Use Collect Macros To Recursively Search | git:HEAD:FrontendRust/docs/shields/UseCollectMacrosToRecursivelySearch-UCMTRSX.md | 0 | 2 |
| MLVFX | Multi-Line Vale Fixtures | docs/shields/MultiLineValeFixtures-MLVFX.md | 0 | 1 |
| MIMBEX | Mig Impls Must Be Empty | git:HEAD:FrontendRust/docs/shields/MigImplsMustBeEmpty-MIMBEX.md | 0 | 0 |

## F cited from Vale code, never defined anywhere (4)

| ID | Reconstructed title | Introduced by | Still true |
|---|---|---|---|
| GROUPS | Groups are declaration-side (GroupS), not templata-side | f8fa7f1c Rung-1 borrow checker: the joint-argument check catches unsafe aliasing | yes |
| MCFBRBF | Must Compile Functions Before Resolving Bounds (of Functions) | 21092ef5 Post-generics cleanup: Made instantiation bound params match up with th | yes |
| RPPFNG | Reserved Parameter Position For Next-Gen (pointer) | b93cf939 "Merging backend from experimental regions branch (#597)" (2023-06-19) | yes |
| SITTX | Same Interface Tags Twice | bb075e70 Backend FFI boundary does no reference counting: refs move/consume acro | yes |
