# After-Migration TODO

Items to revisit after the Scala→Rust migration is complete.

- [ ] Figure out what to do instead of `allocation_map_add_impl` (`FrontendRust/src/testvm/heap.rs`). The free-fn helper is a Rust-ownership-forced factor (Round N+33, Option B) handling Scala's `val void = add(...)` partial-self constructor pattern. Scala has no `addImpl` equivalent. Possible alternatives: `OnceCell<ReferenceV>` for `void_ref` with lazy-init method (Option D), placeholder-then-overwrite (Option E), or something else not yet considered. Chose B for the smallest divergence surface (eager init, no consumer-side ripple, helper body is 1:1 Scala). Revisit when migration discipline relaxes.

- [ ] Figure out whether testvm referrers should be arena-allocated. Currently `AllocationV.referrers: HashMap<IObjectReferrerV<'v,'h,'s>, i32>` stores `IObjectReferrerV` values directly (every variant embed-by-value, after G2 landed all three of Member/Element/RegisterHold by reviving the `ExpressionIdV.path` arena-slice + adding `vivem_bump: &'v bumpalo::Bump` to `HeapV`). Alternative: intern `IObjectReferrerV` into a `'v` arena and store `&'v` refs in the map (smaller HashMap keys, but adds an interner). Scala uses immutable value semantics, so embed-by-value matches; arena would be a Rust-side optimization. Decide based on profile data once real programs run.

- [ ] **VON wire-format parity.** Suite is at 1267/22/0 (5-region e2e 1400/0/22). Audit the 22 ignored cases for VON wire-format holds — `von_hammer` output should match what the backend reads. The ignored tests should assert the same behavior as Scala, not merely compile.

- [ ] **`final_ast` payload types.** Still on `PhantomData` in `final_ast/ast.rs` (2), `final_ast/instructions.rs` (2), `final_ast/types.rs` (3). Replace with the real field types.

- [ ] **Retire `Frontend/` (Scala).** Now that the audit trail is gone and `valec` builds via FrontendRust, the Scala `Frontend/` dir can be deleted once VON parity is proven and any remaining incidental dependencies on it are severed. 16 subdirs still present.

- [ ] **Complete the build cutover.** `bin/valec` is in FrontendRust and Backend is in-process via C ABI, so the runtime cutover landed. Confirm there's no residual Scala-`valec` invocation path (build scripts, CI, docs) and that the `frontend_rust` binary is the only frontend the project ships.
