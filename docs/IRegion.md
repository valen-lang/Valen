
# IRegion Interface in Backend (IRIIB)

IRegion is the main class (well, interface) responsible for managing and accessing memory in the backend.

A random sample of some of the methods on IRegion:

 * allocate: allocates a struct and populates its members.
 * constructStaticSizedArray: allocates a static sized array.
 * loadMember: loads a member from a struct.
 * receiveUnencryptedAlienReference: copies an object from another region.
 * getRuntimeSizedArrayLength: gets an array's length.

There are a handful of different subclasses:

 * UnsafeRegion: A region using no memory safety. Used for:
    * Unsafe blocks.
    * The main mutable region, if --region-override=unsafe-fast.
 * RCImmRegion: A region for immutable objects.
    * Might not be in the final design, depending on how/whether we share iso regions around.
    * Also the region behind the opaque-handle FFI: imm values cross to/from C
      as handles, not linearized buffers.
 * LinearRegion: RETIRED (2026-07). Was a bump allocator that linearized imm
   values into buffers for C and for record/replay files. FFI moved to opaque
   handles and record/replay was removed; see the PSBCBO/PRCBO appendix in
   `todo/metaprogrammed-record-replay.md` for the retired scheme and its
   successor design.

With the IRegion interface, the backend stage can compile expressions against a common lower interface.


# Notes

Need to revisit whether kinds should be associated with regions. probably so that unsafe ones' non-owning pointers can be compatible with C, and GM's ones can be fat pointers. but itd be nice if there was a different way.
