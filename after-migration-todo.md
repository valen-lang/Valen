# After-Migration TODO

Items to revisit after the Scala→Rust migration is complete.

- [ ] Figure out what to do instead of `allocation_map_add_impl` (`FrontendRust/src/testvm/heap.rs`). The free-fn helper is a Rust-ownership-forced factor (Round N+33, Option B) handling Scala's `val void = add(...)` partial-self constructor pattern. Scala has no `addImpl` equivalent. Possible alternatives: `OnceCell<ReferenceV>` for `void_ref` with lazy-init method (Option D), placeholder-then-overwrite (Option E), or something else not yet considered. Chose B for the smallest divergence surface (eager init, no consumer-side ripple, helper body is 1:1 Scala). Revisit when migration discipline relaxes.

- [ ] Figure out whether testvm referrers should be arena-allocated. Currently `AllocationV.referrers: HashMap<IObjectReferrerV<'v,'h,'s>, i32>` stores `IObjectReferrerV` values directly (every variant embed-by-value, after G2 landed all three of Member/Element/RegisterHold by reviving the `ExpressionIdV.path` arena-slice + adding `vivem_bump: &'v bumpalo::Bump` to `HeapV`). Alternative: intern `IObjectReferrerV` into a `'v` arena and store `&'v` refs in the map (smaller HashMap keys, but adds an interner). Scala uses immutable value semantics, so embed-by-value matches; arena would be a Rust-side optimization. Decide based on profile data once real programs run.
