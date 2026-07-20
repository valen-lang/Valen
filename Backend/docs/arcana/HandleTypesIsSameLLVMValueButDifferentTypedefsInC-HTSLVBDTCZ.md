# Handle Types Is Same LLVM Value But Different Typedefs In C (HTSLVBDTCZ)

For every exported class, like `exported struct Ship share { altitude i32; }`, we generate a C header. It holds a typedef for the handle that C code passes around, plus the class's exported functions:

```c
typedef struct vtest_Ship {
  uint64_t _reserved;  // contains pointer
} vtest_Ship;

void vtest_Ship_fly(vtest_Ship ship);
```

Each exported class gets its own typedef (vtest_Ship, vtest_Boat, etc). To the C compiler these are distinct types (C compares structs by name), so a user can't accidentally hand a vtest_Boat to the function vtest_Ship_fly.

Internally, though, the backend has no separate vtest_Ship and vtest_Boat struct: every type shares one LLVM type (a struct holding an address int) and getExternalType returns it for all of them. We decided not to give each class its own LLVM type, to keep things simpler in the backend.

Similarly, all interfaces become the same LLVM type (a struct holding an address int, plus an address int to a vtable):

```vale
sealed exported interface IShip imm {}
abstract func getFuel(virtual ship &IShip) i32;
```

```c
typedef struct vtest_IShip {
  uint64_t _reserved0;   // object pointer
  uint64_t _reserved1;   // typeinfo/vtable pointer
} vtest_IShip;
```

And soon, all concrete type weak refs become the same LLVM type (a struct holding an address int, plus a generation int), and all interface weak refs become the same LLVM type (a struct holding an address int, plus a generation int, plus a vtable int).
